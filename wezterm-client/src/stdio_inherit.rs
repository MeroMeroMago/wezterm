//! Stop a spawned child -- and anything that child leaves running -- from
//! inheriting *this* process's stdio handles on Windows.
//!
//! # Why this exists
//!
//! `wezterm cli` auto-starts a `wezterm-mux-server` when it cannot connect
//! to a mux (see `client.rs`, `Reconnectable::unix_connect`). That server
//! daemonizes and outlives the `wezterm cli` process that started it.
//!
//! If `wezterm cli` was itself launched by some other program with its
//! stdout/stderr connected to pipes -- which is what any wrapper, script,
//! or TUI that wants to read `wezterm cli`'s output does -- then those pipe
//! handles leak into the long-lived daemon. The wrapper waits for EOF on
//! the pipes, EOF never comes (the daemon still holds the write ends open),
//! and the wrapper hangs forever even though `wezterm cli` itself exited
//! seconds ago.
//!
//! # Why configuring the child's `Stdio` is not enough
//!
//! The intuitive fix -- `cmd.stdout(Stdio::null())` -- does **not** work,
//! and this is the whole reason this module exists rather than a one-line
//! change at the call site.
//!
//! Rust's `std::process::Command` always calls `CreateProcessW` with
//! `bInheritHandles = TRUE`. Windows then duplicates *every* handle in the
//! calling process that is marked `HANDLE_FLAG_INHERIT` into the new
//! process -- not merely the three handles named in `STARTUPINFO`. A pipe
//! handle our own parent gave us arrives already marked inheritable, so it
//! is passed down regardless of what we set the child's stdio to. The child
//! never uses it, never closes it, and hands another copy to *its* children.
//!
//! MEASURED on Windows 11 (26200), rustc 1.97: with a grandchild that
//! merely sleeps, a parent waiting for EOF on its child's stdout saw
//!
//! | child's stdio            | parent saw EOF? |
//! |--------------------------|-----------------|
//! | `Stdio::inherit()`       | no (hung)       |
//! | `Stdio::null()`          | no (hung)       |
//! | `Stdio::piped()`         | no (hung)       |
//! | `null` + flag cleared    | yes, in 14ms    |
//!
//! Clearing `HANDLE_FLAG_INHERIT` on our own std handles for the duration
//! of the spawn is what actually breaks the chain. Because the immediate
//! child then never possesses those handles at all, no descendant of it can
//! inherit them either -- breaking the first link is sufficient.
//!
//! Note this does not interfere with a later, deliberate `Stdio::inherit()`
//! spawn: Rust implements `inherit` by explicitly duplicating the handle as
//! inheritable at spawn time, which does not depend on the source handle's
//! own flag.

#[cfg(windows)]
mod imp {
    use winapi::shared::minwindef::DWORD;
    use winapi::um::handleapi::{GetHandleInformation, SetHandleInformation, INVALID_HANDLE_VALUE};
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::{
        HANDLE_FLAG_INHERIT, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
    };
    use winapi::um::winnt::HANDLE;

    pub(super) const STD_IDS: [DWORD; 3] =
        [STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, STD_ERROR_HANDLE];

    fn usable(h: HANDLE) -> bool {
        !h.is_null() && h != INVALID_HANDLE_VALUE
    }

    /// `Some(true)` if `h` is marked inheritable, `Some(false)` if it is
    /// not, `None` if the handle could not be queried at all.
    pub(super) fn get_inherit(h: HANDLE) -> Option<bool> {
        if !usable(h) {
            return None;
        }
        let mut flags: DWORD = 0;
        if unsafe { GetHandleInformation(h, &mut flags) } == 0 {
            return None;
        }
        Some(flags & HANDLE_FLAG_INHERIT != 0)
    }

    /// Set or clear `HANDLE_FLAG_INHERIT` on `h`. Returns whether it worked.
    pub(super) fn set_inherit(h: HANDLE, on: bool) -> bool {
        if !usable(h) {
            return false;
        }
        let value = if on { HANDLE_FLAG_INHERIT } else { 0 };
        unsafe { SetHandleInformation(h, HANDLE_FLAG_INHERIT, value) != 0 }
    }

    /// Clears `HANDLE_FLAG_INHERIT` on this process's std handles, and puts
    /// back exactly what it found when dropped. Hold one of these across a
    /// `Command::spawn` whose child must not capture our stdio.
    ///
    /// Handles that were already non-inheritable are left alone and are not
    /// touched on restore, so this is safe to nest or to use on a process
    /// whose stdio is a console rather than a pipe.
    pub struct NoInheritStdio {
        /// Handles this guard actually changed, and must therefore restore.
        cleared: Vec<HANDLE>,
    }

    // The guard only ever stores raw std handles and restores them on the
    // thread that drops it; nothing about it is thread-affine.
    unsafe impl Send for NoInheritStdio {}

    impl NoInheritStdio {
        pub fn new() -> Self {
            let mut cleared = Vec::new();
            for id in STD_IDS {
                let h = unsafe { GetStdHandle(id) };
                // Only touch handles that are actually inheritable, so the
                // restore cannot invent a flag that was never set. Note the
                // same HANDLE can appear under two ids (stdout and stderr
                // pointing at one console); clearing twice is harmless and
                // restoring twice is idempotent.
                if get_inherit(h) == Some(true) && set_inherit(h, false) {
                    cleared.push(h);
                }
            }
            Self { cleared }
        }

        /// How many std handles this guard actually had to change. Zero is
        /// normal and fine -- it means nothing of ours was inheritable.
        pub fn cleared_count(&self) -> usize {
            self.cleared.len()
        }
    }

    impl Default for NoInheritStdio {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Drop for NoInheritStdio {
        fn drop(&mut self) {
            for h in self.cleared.drain(..) {
                set_inherit(h, true);
            }
        }
    }
}

#[cfg(not(windows))]
mod imp {
    /// No-op outside Windows. Unix does not have the blanket
    /// `bInheritHandles` behaviour this guards against: Rust sets `CLOEXEC`
    /// on descriptors it owns, so an `exec`ed child does not silently
    /// acquire our pipes.
    pub struct NoInheritStdio;

    impl NoInheritStdio {
        pub fn new() -> Self {
            Self
        }
        pub fn cleared_count(&self) -> usize {
            0
        }
    }

    impl Default for NoInheritStdio {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub use imp::NoInheritStdio;

#[cfg(all(test, windows))]
mod tests {
    use super::imp::{get_inherit, set_inherit, STD_IDS};
    use super::NoInheritStdio;
    use winapi::um::handleapi::{CloseHandle, DuplicateHandle};
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::processthreadsapi::GetCurrentProcess;
    use winapi::um::winnt::{DUPLICATE_SAME_ACCESS, HANDLE};

    /// A handle we own outright, marked inheritable, so the flag tests do
    /// not depend on how the test runner happened to wire up our stdio.
    fn inheritable_handle() -> HANDLE {
        let mut dup: HANDLE = std::ptr::null_mut();
        let ok = unsafe {
            let me = GetCurrentProcess();
            DuplicateHandle(me, me, me, &mut dup, 0, 1, DUPLICATE_SAME_ACCESS)
        };
        assert_ne!(ok, 0, "DuplicateHandle failed");
        dup
    }

    #[test]
    fn inherit_flag_can_be_read_cleared_and_put_back() {
        let h = inheritable_handle();
        assert_eq!(
            get_inherit(h),
            Some(true),
            "a handle duplicated with bInheritHandle=TRUE should read as inheritable"
        );
        assert!(set_inherit(h, false), "clearing the flag should succeed");
        assert_eq!(get_inherit(h), Some(false), "flag should now read clear");
        assert!(set_inherit(h, true), "setting the flag should succeed");
        assert_eq!(get_inherit(h), Some(true), "flag should be back");
        unsafe { CloseHandle(h) };
    }

    #[test]
    fn get_inherit_rejects_a_bogus_handle() {
        assert_eq!(get_inherit(std::ptr::null_mut()), None);
        assert!(!set_inherit(std::ptr::null_mut(), false));
    }

    /// The property that actually matters: while the guard is alive, none
    /// of our std handles is inheritable, so `CreateProcessW`'s blanket
    /// `bInheritHandles = TRUE` has nothing of ours to hand down. This
    /// holds whatever state the test runner left our stdio in.
    #[test]
    fn no_std_handle_is_inheritable_while_the_guard_is_held() {
        let guard = NoInheritStdio::new();
        for id in STD_IDS {
            let h = unsafe { GetStdHandle(id) };
            assert_ne!(
                get_inherit(h),
                Some(true),
                "std handle {id:#x} was still inheritable while the guard was held"
            );
        }
        drop(guard);
    }

    /// The guard must be a round trip: whatever it found, it puts back. A
    /// process that logs to its own stderr after spawning must not have had
    /// its handle flags quietly rewritten.
    #[test]
    fn the_guard_restores_exactly_what_it_found() {
        let snapshot = || {
            STD_IDS
                .iter()
                .map(|id| get_inherit(unsafe { GetStdHandle(*id) }))
                .collect::<Vec<_>>()
        };
        let before = snapshot();
        {
            let _guard = NoInheritStdio::new();
        }
        assert_eq!(
            before,
            snapshot(),
            "guard did not restore the std handle inherit flags it changed"
        );
    }
}
