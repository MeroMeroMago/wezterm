# Software-only screen-time blockers and built-in OS tools (iOS and Android), as of 2026-09-28

Method and source caveats (read first):
- Direct page fetches were blocked by the egress proxy (shiftyourphone.com, appblock.app and techlockdown.com fetches all failed). Everything below comes from WebSearch result snippets and the search engine's summaries of them, each credited to the page it came from. Prices come from snippets and can drift or vary by region, so check the live App Store or Play listing before quoting.
- Many 2026 "best blocker" roundups are published by competing app makers: Habit Doom, Blok, unhookd, Screen Time Index, ScreenBuddy, Lockpact, Password Locker, WaitToUnlock, Blank Spaces, Mado, enough., Pauso, Habi, Lock In and Off-Switch. Treat their rankings and criticisms as commercially motivated. Independent sources used where available: Apple and Google support docs, Apple Developer Forums, PNAS, PNAS Nexus, AER/NBER, peer-reviewed journals, Android Police, Android Authority, 9to5Google, How-To Geek, WhistleOut, Zapier, TechRadar, The Hacker News and Wikipedia.
- I found no Wirecutter, The Verge or Wired roundup of app blockers from 2026 through search. Reddit threads (r/nosurf, r/digitalminimalism) did not come up as direct results, so user sentiment below comes from app-store review summaries and review sites.

## 1. Major apps: price, platform, blocking strength, features, evidence, sentiment

### Takeaway
On iOS, every third-party blocker (Opal, one sec, ScreenZen, Jomo, Clearspace, Refocus, Freedom, Blank Spaces and others) runs on Apple's Screen Time API. So they share a ceiling: the blocks are only as strong as Screen Time itself, they break when Apple's API is buggy, and they can be turned off by revoking the app's Screen Time access unless a Screen Time passcode guards that switch. The strongest paid "can't-quit" modes are Opal Deep Focus, Freedom Locked Mode, Jomo Strict/Hard mode, Refocus Strict Mode and AppBlock Strict Mode. Prices run from free (ScreenZen, Olauncher) through about $20–60 a year (one sec, Jomo, Clearspace, Refocus, Freedom) to about $100 a year (Opal). Only two apps have strong outside evidence: one sec (PNAS 2023) and Freedom, which was the tool used in a PNAS Nexus 2025 RCT.

### Cited Findings

**Opal (iOS; Android also available)**
- Opal Pro costs about $99.99 a year (about $8.29 a month) or $19.99 billed monthly. A one-time lifetime plan costs $399, and verified students get 50% off — [Headway Opal review 2026](https://makeheadway.com/blog/opal-app-review/). Blok also frames it as "$100/year" — [Blok](https://www.blok.so/resources/opal-app-review-is-it-worth-100-year-for-screen-time-management) (competitor).
- Deep Focus is Opal's strongest mode. A session cannot be ended early, deleting the app does not lift the block, and it blocks websites too, which closes the Safari and private-browsing loophole. Deep Focus sits behind the paywall, so the free tier is "closer to a trial" — [Headway](https://makeheadway.com/blog/opal-app-review/).
- Known bypass: going to Settings > Screen Time and switching off "Apps with Screen Time Access". This "plagues virtually all third-party distraction management apps on iOS". To counter it, Opal tells users to set a Screen Time passcode so its access can't be revoked — [Password Locker blog](https://password-locker.com/blog/post/opal-workarounds-a-solution-to-disabling-screen-time-access/) (competitor); [Opal Help: How to lock Opal's Screen Time access](https://opalapp.com/help/how-to-lock-opals-screen-time-access).
- If you delete Opal to escape a session, the apps may stay blocked. Opal has a documented "hard reset" procedure for Screen Time settings — [Opal Help](https://opalapp.com/help/how-do-i-hard-reset-my-screen-time-settings).
- Opal for Android is on Google Play — [Opal Help: Introducing Opal for Android](https://opalapp.com/help/introducing-opal-for-android). It is "not as powerful as the one on iPhone" because Android has no equivalent Screen Time API — [Headway](https://makeheadway.com/blog/opal-app-review/).
- Sentiment in one roundup: most polished design and analytics, but "its blocking is bypassable and runs about $8 a month" — [Unstar ranking 2026](https://unstar.app/blog/opal-forest-freedom-one-sec-jomo-screen-time-apps-ranked-2026) (competitor).

**one sec (iOS, Android, plus browser and desktop)**
- Evidence: Grüning, Riedel & Lorenz-Spreen, PNAS 2023, "Directing smartphone use through the self-nudge app one sec" — [PNAS](https://www.pnas.org/doi/10.1073/pnas.2213114120). Reported results:
  - Target-app openings fell 57% after 6 weeks of use.
  - In 36% of attempts, users closed the target app after the intervention.
  - Attempts to open target apps fell 37% compared with the first week.
  - Sources: [one sec research page](https://one-sec.app/max-planck-study/); [finit review](https://getfinit.com/blog/one-sec-review). One review gives the sample as 280 participants over 6 weeks — [learnofchrist review](https://learnofchrist.com/resources/one-sec). I could not verify the sample size in the paper itself.
- Mechanism: a friction or pause screen (a breathing exercise, a mirror, a random friction task) before the app opens. It is not a hard block — [learnofchrist review](https://learnofchrist.com/resources/one-sec). "It does not hard-block, so willpower still matters" — [Unstar](https://unstar.app/blog/opal-forest-freedom-one-sec-jomo-screen-time-apps-ranked-2026).
- Price: one sec pro is $19.99 a year on the US App Store, and monthly plans start around $3.99 — [WhistleOut / one sec pro help](https://tutorials.one-sec.app/en/articles/3036418); [one sec site](https://one-sec.app/). Other sources say "$2.99/month up to $99.99 lifetime" — [Zapier via search summary](https://zapier.com/blog/stay-focused-avoid-distractions/). Pricing is inconsistent across sources.
- One pro subscription covers iPhone, iPad, Android and computer — [one sec platforms](https://one-sec.app/platforms/).
- The one sec developer (Frederik Riedel) has publicly documented major Screen Time API bugs; see section 5 — [riedel.wtf](https://riedel.wtf/state-of-the-screen-time-api-2024/).

**ScreenZen (iOS and Android, free)**
- Fully free with optional tips: no subscription and no paywall — [Habit Doom](https://habitdoom.com/blog/screenzen-alternative-iphone) (competitor); [screenzen.co](https://screenzen.co/).
- App Store rating is 4.8 stars across more than 30,000 reviews, with more than 500,000 monthly active users as of 2026 — [Habit Doom](https://habitdoom.com/blog/screenzen-alternative-iphone). The MAU figure comes from a competitor summary and is unverified.
- Features: a countdown or pause before the app opens, blackout scheduling, gesture unlocks, app timers and custom messages — [WhistleOut](https://www.whistleout.com/CellPhones/Apps/screenzen-app-review); [Nibble](https://nibble-app.com/blog/screenzen).
- Weakness: it "doesn't actually block apps… it can make Instagram annoying to open". Users also report inconsistent website blocking and permission bugs — [unhookd](https://unhookd.app/blog/screenzen-worth-it-review) (competitor).

**Freedom (iOS, Android, Mac, Windows, Chrome, Linux)**
- Premium costs $8.99 a month, $3.33 a month billed yearly (about $40 a year), or $99.50 lifetime. The same snippet also mentions a $199 "Freedom Forever" plan and a $12.99 monthly price, so figures conflict across pages — [freedom.to/premium](https://freedom.to/premium); [SaaSpartout](https://saaspartout.com/marketplace/freedom/).
- The free plan has only "Start Now" sessions. Locked Mode, scheduling and unlimited devices need Premium — [Freedom Help Center](https://support.freedom.to/en/articles/13764747-what-s-included-in-free-and-premium-plans).
- Main strength: blocks sync across devices, which stops someone switching to the laptop — [Unstar](https://unstar.app/blog/opal-forest-freedom-one-sec-jomo-screen-time-apps-ranked-2026). Reported to have 3 million users — [Blok](https://www.blok.so/resources/best-app-blockers-for-iphone-in-2026) (competitor).
- Evidence: Castelo et al., PNAS Nexus, published Feb 18, 2025. This was a preregistered RCT with n = 467. Participants installed Freedom to block all mobile internet for 2 weeks while calls and texts kept working. It improved subjective well-being, mental health and sustained attention, and 91% improved on at least one measure. Sustained attention improved by about as much as the decline seen over roughly 10 years of age — [PNAS Nexus / Oxford Academic](https://academic.oup.com/pnasnexus/article/4/2/pgaf017/8016017); [PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC11834938/); [News-Medical](https://www.news-medical.net/news/20250219/Blocking-mobile-internet-for-two-weeks-improves-mental-health-and-well-being.aspx).

**Clearspace (iOS, Android)**
- $6.99 a month, $44.99–59.99 a year, or a $79.99 family plan. Students get it free — [Lockpact review](https://lockpact.app/blog/clearspace-app-review/) (competitor); [App Store](https://apps.apple.com/us/app/clearspace-reduce-screen-time/id1572515807).
- Features: budgets, pauses, schedules, streaks and reports. User reviews are largely positive ("only blocker that's truly worked"), and some claim they went from 8+ hours a day to 2–3 hours. Complaints cover exercise-tracking glitches and cost — [App Store reviews](https://apps.apple.com/us/app/clearspace-reduce-screen-time/id1572515807?see-all=reviews&platform=iphone).

**Jomo (iOS)**
- Pricing tiers:
  - Free: 1 session.
  - $29.99 a year: strict mode and up to 10 sessions.
  - $14.99 a year: student plan.
  - $99.99 one-time: all features, plus sharing with 5 family members.
  - Source: [ColdIQ](https://coldiq.com/tools/jomo); [App Store](https://apps.apple.com/us/app/jomo-screen-time-blocker/id1609960918).
- Strict Mode locks rules for a chosen number of days, and "Hard mode refuses any bypass". Users call strict mode a "game changer". Complaints cover recent tracking and sync bugs and difficulty editing rules — [Screen Time Index](https://screentimeindex.com/posts/jomo-app-review/) (competitor); [App Store reviews](https://apps.apple.com/us/app/jomo-screen-time-blocker/id1609960918?see-all=reviews&platform=iphone).

**Refocus (iOS and Mac; I found no evidence of an Android version)**
- Strict Mode prevents unblocking. Unlock options are a passcode, an NFC tag, waiting out a duration, a copy-text challenge, or a Pomodoro limit (5 unblocks a day, 25 minutes each, with a 5-minute cooldown). Other features: app limits, location-based blocks, and a Mac app — [App Store](https://apps.apple.com/us/app/refocus-app-blocker-limits/id1645639057); [refocusapp.co](https://www.refocusapp.co/).
- Pro costs about $7.99–9.99 a month or $49.99–59.99 a year, and the free tier covers core blocking — [App Store listing via search](https://apps.apple.com/us/app/refocus-block-apps-websites/id1645639057).

**AppBlock (Android, iOS, browser extensions)**
- Strict Mode stops you changing or disabling a block before it expires. The higher level ("Disable AppBlock uninstalling") also stops uninstall until the time is up — [AppBlock help: Strict Mode on Android](https://appblock.app/help/android/strict-mode-all/).
- Premium costs $4.99 a month, $29.99 a year or $89.99 lifetime, with a 7-day trial. Reported 15M downloads and a 4.7 rating — [Nubia Magazine](https://nubiapage.com/appblock-review-2026-extension-app-price-free-plan/). Another source gives 4.7 stars from about 238k reviews and 10M+ Play downloads — [Habit Doom Android](https://habitdoom.com/blog/best-app-blockers-android-2026).
- Called "the most complete dedicated blocker" on Android — [Habit Doom](https://habitdoom.com/blog/best-app-blockers-android-2026) (competitor).
- AppBlock itself acknowledges that Android's Advanced Protection mode changes stop its blocking; see section 2 — [AppBlock notice](https://appblock.app/androids-new-advanced-protection-affects-appblock-from-blocking/). Title only, because the fetch was blocked.

**StayFree (Android, with iOS and browser versions)**
- Its in-app blocker can hide YouTube Shorts, Instagram Reels and Stories without blocking the whole app. Other features: limits by site, category, schedule or daily time, keyword blocking and PIN protection — [Chrome Web Store](https://chromewebstore.google.com/detail/stayfree-website-blocker/elfaihghhjjoknimpccccmkioofjjfkf?hl=en-US); [stayfreeapps.com](https://stayfreeapps.com/).
- I could not find current premium pricing (see Gaps).

**Lock Me Out (Android, by TEQTIC)**
- Free tier: 5 lockouts, 10 apps, 5 websites and 5 locations. Premium unlocks unlimited use plus "prevent uninstallation and tampering". It is sold as monthly, yearly or one-time, but I could not retrieve the prices — [TEQTIC](https://www.teqtic.com/lock-me-out); [Softonic](https://lock-me-out-app-blocker.en.softonic.com/android).
- Its strictest mode is "lock screen only", which allows nothing but the lock screen. It offers password protection for entry, uninstall and tampering, plus temporary emergency access — [Play listing via AppGrooves](https://appgrooves.com/android/com.teqtic.lockmeout/lock-me-out-app-blocker-and-website-blocker/teqtic).
- Rated 4.47 from about 9.9k ratings. Last update April 20, 2026 — [AppBrain](https://www.appbrain.com/app/lock-me-out-app-site-blocker/com.teqtic.lockmeout).

**Digital Detox (Android)**
- The original "Digital Detox" app says a session cannot be cancelled once started. The phone is unusable apart from the emergency dialer until the session ends — [tirl.org](https://tirl.org/software/digitaldetox/).
- "Digital Detox: Focus & Live" (Urbandroid) has a "no cheating" setting that you can only get around by paying — [Google Play](https://play.google.com/store/apps/details?id=com.urbandroid.ddc&hl=en_US).
- Open-source alternative: DetoxDroid on F-Droid, which allows a timed pause and then resumes automatically — [F-Droid](https://f-droid.org/en/packages/com.flx_apps.digitaldetox/).

**Regain (Android)**
- About $10 a month. In Strict Mode you cannot end a focus session early or uninstall Regain until the timer runs out — [Giodella test](https://giodella.com/android-app-blockers/); [regainapp.ai](https://regainapp.ai/).

**Stay Focused (Android and iOS)**
- Strict Mode stops you changing settings. Deactivating it can require a PIN, an expiration date, a QR scan, or typing random text. Settings can be made stricter during Strict Mode, but loosening stays locked — [stayfocused.me](https://www.stayfocused.me/); [App Store](https://apps.apple.com/us/app/stay-focused-app-site-blocker/id1658592224).
- I did not find a price.

**BlockSite (Android, iOS, browser extensions)**
- Premium is about $10.99 a month, with in-app purchases from $3.99 to $38.99. The free tier allows only 3 blocked sites — [Cisdem review](https://www.cisdem.com/resource/blocksite-review.html); [SiteBlocker comparison](https://siteblocker.app/blog/siteblocker-vs-blocksite/) (competitor).
- Complaints include a July 2026 Trustpilot report of being asked to renew after paying a one-time $100, criticism of its data-collection history, and concerns about breaking after Android updates — [Trustpilot](https://www.trustpilot.com/review/blocksite.co); [Cisdem](https://www.cisdem.com/resource/blocksite-review.html).

**Cold Turkey (Windows and Mac desktop only; no phone app)**
- Pro is a one-time $45 covering all of the owner's computers, with no subscription — [Ascension review](https://ascensionapp.ai/cold-turkey-blocker-review); [getcoldturkey.com](https://getcoldturkey.com/).
- There are seven lock types, from a timer up to a delay of 40 days. While a block is locked, the uninstaller fails and the browser extension stays enabled — [Cold Turkey user guide](https://getcoldturkey.com/support/user-guide/).
- Widely called the "toughest" blocker, but only for desktops. It is useful alongside phone blockers so you can't shift to the laptop.

**Launchers and home-screen "dumbifiers"**
- Olauncher (Android): open source and free, with an optional Olauncher Pro. It shows only date, time and a few app names — [Google Play](https://play.google.com/store/apps/details?id=app.olauncher&hl=en_US); [Android Authority minimalist launchers 2026](https://www.androidauthority.com/best-minimalist-android-launchers-2026-3709772/).
- Minimalist Phone (Android launcher): covered in the [Android Authority 2026 launcher test](https://www.androidauthority.com/best-minimalist-android-launchers-2026-3709772/) and in [How-To Geek: "This Android launcher helped me cut my phone use in half"](https://www.howtogeek.com/this-android-launcher-helped-me-cut-my-phone-use-in-half/). I did not get its exact price.
- Blank Spaces (iPhone): replaces the home screen with widgets and hides app icons. Costs $3.99 a month, $17.99 a year or $23.99 lifetime, after a 7-day trial — [App Store](https://apps.apple.com/us/app/blank-spaces-launcher/id1570856853).
- Dumbify (iPhone): a minimalist launcher, $4.99 one-time with no in-app purchases. Reviews are positive, though some say it adds little over free widget launchers — [dumbifyapp.com](https://dumbifyapp.com/); [App Store reviews](https://apps.apple.com/us/app/dumbify/id6480082872?see-all=reviews&platform=iphone).
- "Dumb Phone" app (iOS, dumbphone.so): a minimal home-screen launcher that began as a feature of focusedOS. WhistleOut describes it as "free" — [dumbphone.so](https://dumbphone.so/); [WhistleOut](https://www.whistleout.com/CellPhones/Apps/dumbphone-homescreen-app-review); [App Store](https://apps.apple.com/us/app/dumb-phone-launcher-themes/id6451376191).
- Launchers on iOS only change the home screen. Apps stay reachable through Search and the App Library, so they create friction rather than a block (inference from how they work, not stated in a source).

**Unpluq (iOS and Android)**
- The app needs a Premium subscription. The physical Tag is optional and works only with Premium. Software-only "digital barriers" are available: walking/steps, scrolling, charging, random, or QR code — [Unpluq FAQ](https://www.unpluq.com/pages/faq); [Unpluq subscription](https://www.unpluq.com/products/unpluq-subscription-only). One review calls the NFC tag "$26" — [whatifididnt](https://whatifididnt.com/blog/unpluq-review/).

### Inferences
- For a US iPhone user, the strongest software-only setup is a paid strict mode (Opal Deep Focus, Jomo Hard mode, Freedom Locked Mode or Refocus Strict) plus a Screen Time passcode held by someone else, which blocks revoking the app's Screen Time access and deleting the app. Without that passcode, every iOS blocker can be defeated in under a minute from Settings.
- Friction apps (one sec, ScreenZen) have the best effectiveness evidence (one sec's PNAS study) but the weakest enforcement. They suit people who want to cut impulsive opens, not people who want to be forcibly locked out.
- Prices: free (ScreenZen, Olauncher), under $30 a year (one sec, Jomo, AppBlock, Blank Spaces), about $40–60 a year (Freedom annual, Clearspace, Refocus), and about $100 a year (Opal). These annual costs compare with a one-time hardware token (Brick or Bloom).

### Gaps
- Current StayFree premium, Stay Focused premium, Lock Me Out premium and Minimalist Phone launcher prices were not retrievable from snippets.
- Opal's Google Play rating and whether Deep Focus exists on Android were not confirmed.
- I could not confirm Refocus on Android.
- The one sec PNAS sample size and the exact study parts could not be checked against the paper, since the fetch was not attempted or was blocked.
- No Wirecutter, Verge or Wired coverage from 2026 surfaced.

## 2. Android: which blockers resist uninstall and bypass best, and which work on generic Android (Unihertz Titan, Clicks Communicator)?

### Takeaway
On Android, blockers are overlays driven by an Accessibility Service, sometimes backed by Device Admin for uninstall protection. They "cover" apps rather than truly lock them, and a determined owner can defeat them via Accessibility settings, force-stop, safe mode, clearing data, or ADB. The strongest options for uninstall and tamper resistance are Lock Me Out (premium anti-uninstall and tamper protection, plus a lock-screen-only mode), AppBlock Strict Mode (uninstall disabled), Regain Strict Mode, Stay Focused Strict Mode and the Digital Detox apps that can't be cancelled. New in 2026: Android 17's Advanced Protection Mode revokes Accessibility access from non-accessibility apps, which breaks these blockers when AAPM is on. Because the Clicks Communicator ships with Google Play (Android 17) and the Titan 2 Elite ships with Android 16, standard Play Store blockers should install on them. I found no device-specific test.

### Cited Findings
- "Android has no API that prevents an app opening, so blockers draw a screen over the app after it launches, making an Android block a cover rather than a lock" — [Habit Doom Android](https://habitdoom.com/blog/best-app-blockers-android-2026) (competitor).
- "No Android app blocker is genuinely unbypassable, because blocking runs on an Accessibility Service the phone's owner can switch off in Settings". Common bypasses: clearing the app's data, force-stopping it, booting into safe mode, and uninstalling via ADB — [Habit Doom: Is there an Android blocker you cannot bypass?](https://habitdoom.com/blog/android-app-blocker-that-cant-be-bypassed) (competitor).
- Device Admin: the Device Administration API gives apps elevated privileges, and apps holding it are protected from normal uninstall — [Tech Lockdown: How to enforce apps on Android](https://www.techlockdown.com/articles/enforce-apps-android). But the user can revoke admin rights. Some blockers add an accessibility watcher to stop that, and VPN-based blockers reconnect themselves, "but users can still work around this" — [BetterFilter GitHub](https://github.com/Jolt151/BetterFilter) (open-source filter project).
- Safe mode disables all non-preinstalled apps, which makes it a practical bypass for blocker apps — [XDA: Blocking safe mode and factory reset](https://xdaforums.com/t/blocking-safe-mode-and-factory-reset-on-android.4603707/); [Blok: 7 Android methods ranked](https://www.blok.so/resources/how-to-block-apps-on-android-7-methods-ranked-by-how-well-they-actually-work) (competitor).
- Lock Me Out premium: "prevent uninstallation and tampering so that you can't get out of lockouts". It has a "lock screen only" mode and temporary emergency access — [TEQTIC](https://www.teqtic.com/lock-me-out); [Softonic](https://lock-me-out-app-blocker.en.softonic.com/android).
- AppBlock Strict Mode level 2 prevents uninstalling until time is up — [AppBlock help](https://appblock.app/help/android/strict-mode-all/).
- Regain Strict Mode: you can't end early or uninstall until the timer runs out — [Giodella](https://giodella.com/android-app-blockers/).
- Digital Detox (tirl): can't be cancelled, and the phone is limited to the emergency dialer — [tirl.org](https://tirl.org/software/digitaldetox/).
- Android 17 (Beta 2, March 2026): with Advanced Protection Mode on, apps not flagged `isAccessibilityTool` lose AccessibilityService access. Apps that already had it get it revoked, with the message "Restricted by Advanced Protection Program" and no override short of turning AAPM off. Automation tools and launchers are affected as well as blockers — [The Hacker News](https://thehackernews.com/2026/03/android-17-blocks-non-accessibility.html); [Android Authority](https://www.androidauthority.com/android-17-beta-2-advanced-protection-mode-accessibility-apps-3648860/); [Android Police](https://www.androidpolice.com/advanced-protection-mode-android-17-beta-accessibility/). AppBlock has published a notice that the change affects its blocking — [AppBlock](https://appblock.app/androids-new-advanced-protection-affects-appblock-from-blocking/).
- Android 13 had already restricted Accessibility for sideloaded apps, which matters if someone installs a blocker APK from outside Play — [Esper](https://www.esper.io/blog/android-13-sideloading-restriction-harder-malware-abuse-accessibility-apis).
- Clicks Communicator:
  - Fully compatible with Google Play. It was first planned for Android 16 and confirmed in late May 2026 to launch on Android 17 — [Wikipedia](https://en.wikipedia.org/wiki/Clicks_Communicator).
  - Launch price $499 through Sept 30, 2026, then $649 MSRP, shipping in December 2026 — [GlobeNewswire press release, Sept 18, 2026](https://www.globenewswire.com/news-release/2026/09/18/3364666/0/en/Clicks-Communicator-Gets-Upgraded-Specs-and-Expanded-Ecosystem.html?f=22#038;fvtc=5&).
  - Positioned as a secondary device to your main smartphone — [9to5Google](https://9to5google.com/2026/01/02/clicks-communicator-android-phone-keyboard-messages/).
- Unihertz Titan 2 Elite: launches with Android 16 and gets 5 years of OS updates through Android 20 — [Android Authority](https://www.androidauthority.com/unihertz-titan-2-elite-3646393/). Android Police covered running Niagara Launcher on the Titan 2 — [Android Police](https://www.androidpolice.com/niagara-launcher-unihertz-titan-2/).

### Inferences
- Rough strength ranking on Android for a self-binding user (my inference from the listed feature sets, not independent testing):
  1. Lock Me Out premium with uninstall and tamper protection, or AppBlock Strict Mode with uninstall disabled.
  2. Regain, Stay Focused and Digital Detox strict or can't-cancel modes.
  3. StayFree and ScreenZen, which are friction or limit tools.
  4. Digital Wellbeing, which is designed as a nudge.
  All of them lose to safe mode, ADB or a factory reset unless a parent or MDM-style control (Family Link) sits above them.
- On Clicks or Titan-class generic Android phones with Play Services, the same Play-store blockers should work, since they rely only on standard Accessibility and Device Admin APIs. Vendor battery-optimisation killing of background services is a common risk on less mainstream Android skins, but no source specific to these devices was found.
- Users who turn on Android 17 Advanced Protection for security will lose most third-party blockers, which forces a choice between security hardening and blocking.

### Gaps
- There are no hands-on reports of specific blockers (Lock Me Out, AppBlock) running on the Clicks Communicator (not shipping until December 2026) or the Titan 2 Elite.
- I found no independent, non-vendor tests ranking Android blockers by bypass resistance. Most rankings come from competing app makers.
- The exact AppBlock mitigation for AAPM was unreadable because the fetch was blocked.

## 3. Built-in OS tools: iOS Screen Time, Focus and grayscale; Android Digital Wellbeing, Samsung and Pixel modes; Family Link and Screen Time via a partner

### Takeaway
Built-in tools are free and system-level, but they are nudges by default. iOS Screen Time becomes a real lock only when a Screen Time passcode is set, ideally by someone else, and "Block at Downtime" or "Block at End of Limit" is enabled. Since iOS 26.4 that passcode can also guard revoking third-party blockers' Screen Time access, though a Face ID bug has been reported. Android Digital Wellbeing's Focus mode and app timers have one-tap or 5-minute overrides by design. Family Link gives stronger outside control, but Google aims it at children and teens, so it is awkward to use on an adult account.

### Cited Findings
- iOS setup: Settings > Screen Time > Downtime (schedule), plus App Limits by category or app. With a Screen Time passcode set, "Block At Downtime" appears, and extra time needs the passcode (15 minutes, 1 hour or all day) — [Apple iPhone User Guide: Set up Screen Time for yourself](https://support.apple.com/en-lk/guide/iphone/set-up-screen-time-for-yourself-iphbfa595995/14.0); [Apple: Use Screen Time](https://support.apple.com/en-lamr/108806).
- A forgotten Screen Time passcode can be reset with the Apple Account email and password used to set it — [Apple Support 102677](https://support.apple.com/en-mide/102677). So if you give a friend the passcode but keep your own Apple ID credentials, you can reset it yourself. That is a key weakness unless the passcode is tied to a separate Apple Account you don't control.
- Tools that close this gap:
  - WaitToUnlock generates the password for a fresh "burner" Apple Account, which you use to set the Screen Time passcode, and then locks it away — [WaitToUnlock blog](https://waittounlock.com/blog/lock-yourself-out-of-screen-time) (vendor).
  - Password Locker releases a stored passcode only after a time delay or a tedious task — [password-locker.com](https://password-locker.com/) (vendor).
- iOS 26.4: revoking an app's Screen Time permission can now be made to require the Screen Time passcode instead of Face ID or the device passcode. It is not on by default. This closed the long-standing loophole where adults disabled blockers with their own Face ID — [Tech Lockdown: iOS 26.4 update](https://www.techlockdown.com/articles/ios26-update-screen-time-protected-app-permissions). An Apple Developer Forums thread reports a bug where iOS 26.4 still asks for Face ID instead of the Screen Time passcode — [Apple Developer Forums 821959](https://developer.apple.com/forums/thread/821959).
- Known Screen Time loopholes and fixes (from a parental-control source, but they apply to self-control):
  - Changing the time zone or clock: fix by keeping "Set Time Zone Automatically" on and setting Location Services to "Don't Allow Changes".
  - Deleting and reinstalling apps: fix by using the content restriction that disallows deleting apps.
  - "One More Minute" repeated indefinitely: fix by turning on "Block at End of Limit".
  - Source: [Protect Young Eyes: 12 iOS Screen Time hacks](https://www.protectyoungeyes.com/blog-articles/12-ingenious-screen-time-hacks-how-to-beat-them). A developer-forum thread also reports bypass by manually changing the date and time — [Apple Developer Forums 809318](https://developer.apple.com/forums/thread/809318).
- Grayscale on iOS: Settings > Accessibility > Display & Text Size > Color Filters > Grayscale. Assign it to the Accessibility Shortcut and a triple-click of the side button toggles it — [Apple Support 111773](https://support.apple.com/en-sa/111773); [How-To Geek](https://www.howtogeek.com/i-tried-my-iphones-color-filters-here-are-my-favorites/).
- Android Digital Wellbeing:
  - Focus mode lets you tap a paused app for a 5-minute override — [Android Police](https://www.androidpolice.com/2020/08/06/digital-wellbeing-lets-you-override-focus-mode-for-5-minutes-apk-mirror/) (2020 article, but the behaviour persists).
  - App timers grey out the icon, which can be removed or changed in settings. Google built them as "a nudge, not a hard block" — [How-To Geek](https://www.howtogeek.com/443322/how-to-set-app-time-limits-and-block-apps-on-android/); [Technipages](https://www.technipages.com/how-to-set-up-digital-wellbeing-on-android/).
- Pixel Bedtime Mode (Digital Wellbeing > Ways to disconnect) turns on grayscale, silences notifications and dims the wallpaper on a schedule — [Zapier](https://zapier.com/blog/bedtime-mode-android/); [Android Police](https://www.androidpolice.com/android-bedtime-mode-set-up-use/). Newer Pixel "Modes" and Samsung "Modes and Routines" allow multiple modes, each with its own schedule, replacing single-schedule Focus mode — [Android Police](https://www.androidpolice.com/using-androids-built-in-focus-modes-wrong-one-setting-fixed-everything/); [Digital Feng Shui Guide](https://www.digitalfengshuiguide.com/blog/android-focus-mode-multiple-schedules).
- Samsung Sleep mode (formerly Bedtime mode) is part of Modes and Routines. It can turn on grayscale and restrict which apps can be used — [MakeUseOf](https://www.makeuseof.com/bedtime-mode-sleep-mode-android-samsung/); [Samsung US support](https://www.samsung.com/us/support/answer/ANS10001357/).
- Family Link: supervision is designed for children and teens. Teens aged 13 to 17 need parental approval to stop supervision — [Google For Families Help](https://support.google.com/families/answer/9055704?hl=en). As a self-control tool, Family Link "has very limited protection, and some features that might be useful for self-control are not available" — [Tech Lockdown Family Link review](https://techlockdown.com/blog/family-link-review).

### Inferences
- For an iPhone owner, the strongest free software setup is:
  1. Downtime and App Limits with Block at Downtime / Block at End of Limit.
  2. The Screen Time passcode set by a partner, or tied to a burner Apple Account you don't know the password to.
  3. "Don't Allow Changes" on Location Services / time zone, and on deleting apps.
  4. The iOS 26.4 option that makes revoking blocker apps' Screen Time access require the passcode.
  Even then, the Apple-ID reset path and the Face ID bug are residual risks.
- On Android the built-in tools are weaker than on iOS. Supervising your own adult account through a partner's Family Link is a workaround against Google's intended use. Its reliability for adults is not documented.

### Gaps
- There is no official Google documentation on supervising an adult (18+) account with Family Link. I did not confirm whether Google currently allows it.
- Apple's own documentation of the iOS 26.4 "protect apps with Screen Time access" toggle was not found; only Tech Lockdown and developer-forum reports were.
- Pixel "Wind Down" naming: sources describe it as Bedtime mode (Wind Down was the older name). No 2026 Pixel-specific doc was retrieved.

## 4. Commitment tricks and bypass prevention

### Takeaway
Most software blocks fail because the person who set them up also controls the escape hatch: Settings toggles, uninstall, Apple ID resets, safe mode. Commitment tricks work by moving the key somewhere hard to reach: a partner's passcode, a burner account, time-delayed release, uninstall-locked strict modes, or multi-day locks (Jomo strict, Cold Turkey locks up to 40 days).

### Cited Findings
- Tricks in use:
  - Giving the Screen Time passcode to a friend, or setting a passcode you don't memorise — [Apple Developer Forums discussion via search](https://developer.apple.com/forums/thread/714651).
  - Password Locker as an "alternative to friends setting your Screen Time password" — [password-locker.com](https://password-locker.com/landing-page/pages/a2e674hg/).
  - A burner Apple Account for the passcode — [WaitToUnlock](https://waittounlock.com/blog/lock-yourself-out-of-screen-time).
  - Time-based passcode apps on Android — [Google Play: Screen Lock – Time Password](https://play.google.com/store/apps/details?id=com.adriadevs.screenlock.ios.keypad.timepassword&hl=en_US).
- Password Locker describes entering and deleting digits over and over, so that by the end you've forgotten the code — [Password Locker blog](https://password-locker.com/blog/post/how-to-lock-yourself-out-of-screen-time/).
- Multi-day locks and challenges:
  - Jomo strict mode locks rules for a chosen number of days — [Screen Time Index](https://screentimeindex.com/posts/jomo-app-review/).
  - Cold Turkey locks run up to 40 days, with the uninstaller disabled — [Cold Turkey guide](https://getcoldturkey.com/support/user-guide/).
  - Refocus and Stay Focused offer NFC, QR or random-text unlock challenges — [Refocus App Store](https://apps.apple.com/us/app/refocus-app-blocker-limits/id1645639057); [stayfocused.me](https://www.stayfocused.me/).
  - Unpluq offers software-only barriers (steps, QR, etc.) — [Unpluq FAQ](https://www.unpluq.com/pages/faq).
- Bypass vectors and their preventions:
  - iOS: revoking Screen Time access (prevent with a Screen Time passcode and the iOS 26.4 protection) — [Password Locker](https://password-locker.com/blog/post/opal-workarounds-a-solution-to-disabling-screen-time-access/); [Tech Lockdown](https://www.techlockdown.com/articles/ios26-update-screen-time-protected-app-permissions).
  - iOS: changing the time zone, deleting apps, or "One More Minute" (prevent with Location "Don't Allow Changes", the no-deleting restriction, and Block at End of Limit) — [Protect Young Eyes](https://www.protectyoungeyes.com/blog-articles/12-ingenious-screen-time-hacks-how-to-beat-them).
  - Android: turning off Accessibility, revoking Device Admin, force-stop, safe mode, clearing data, ADB — [Habit Doom](https://habitdoom.com/blog/android-app-blocker-that-cant-be-bypassed); [XDA](https://xdaforums.com/t/blocking-safe-mode-and-factory-reset-on-android.4603707/).
  - Switching devices, e.g. to a laptop (prevent with Freedom's cross-device sync or Cold Turkey on desktop) — [Unstar](https://unstar.app/blog/opal-forest-freedom-one-sec-jomo-screen-time-apps-ranked-2026).
- Emergency-access design: Lock Me Out offers temporary emergency access — [Softonic](https://lock-me-out-app-blocker.en.softonic.com/android). Jomo users praise strict mode for keeping emergency access while blocking otherwise — [App Store reviews](https://apps.apple.com/us/app/jomo-screen-time-blocker/id1609960918?see-all=reviews&platform=iphone).

### Inferences
- The hardware tokens (Brick, Bloom, Unpluq Tag) compete by making the key physical and remote. The software equivalents are a partner-held passcode, a burner Apple account, or a time delay. All of these are free or cheap, but the Apple-ID reset path and Android safe mode keep them short of airtight, and the same Screen Time API limits apply to the token-based iOS products too.
- The practical "strong" iOS recipe needs only the built-in Screen Time passcode, set by someone else. A paid app mainly adds better UX, scheduling, sessions and friction screens.

### Gaps
- There is no systematic data on how often users bypass partner-held passcodes, or on relapse rates for commitment devices on phones.
- I could not reach Reddit (r/nosurf, r/digitalminimalism) directly to quantify common bypass stories.

## 5. Effectiveness evidence and common failure modes

### Takeaway
Rigorous evidence exists for: self-set limits (AER 2022: −17% social media use), friction nudges (one sec PNAS 2023: −57% target-app openings), full mobile-internet blocking via Freedom (PNAS Nexus 2025 RCT: better well-being, mental health and attention in 2 weeks) and grayscale (roughly 22–50 minutes a day less in several studies). Broader detox meta-analyses find small well-being gains (SMD about 0.2–0.3) and mixed effects on total usage. The main failure modes are self-bypass, Screen Time API bugs on iOS 26, platform changes (Android 17 AAPM), subscription fatigue, and friction apps that users simply click through.

### Cited Findings
- Allcott, Gentzkow & Song, "Digital Addiction" (AER 2022; field experiment in 2020, n = 1,933) — [NBER](https://www.nber.org/papers/w28936); [AEA](https://www.aeaweb.org/articles?id=10.1257%2Faer.20210867); [Stanford PDF](https://web.stanford.edu/~gentzkow/research/DigitalAddiction.pdf).
  - Giving people a screen-time limit tool reduced social media use by 24 minutes a day (17%) over weeks 4–9.
  - A $2.50-per-hour payment cut use by 56 minutes a day (39%).
  - The authors estimate that self-control problems cause 31% of social media use.
  - A 2026 Brookings/Hamilton Project policy paper revisits these results — [Hamilton Project](https://www.hamiltonproject.org/publication/paper/digital-addiction-evidence-and-policy-implications/).
- one sec (PNAS 2023): 57% fewer target-app openings after 6 weeks. In 36% of attempts users dismissed the app — [PNAS](https://www.pnas.org/doi/10.1073/pnas.2213114120); [one sec research](https://one-sec.app/max-planck-study/). Caveat: co-author Riedel is one sec's developer.
- Castelo et al. 2025, PNAS Nexus, n = 467: two weeks of blocking mobile internet via Freedom improved well-being, mental health and sustained attention, and 91% improved on at least one outcome — [PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC11834938/); [PubMed](https://pubmed.ncbi.nlm.nih.gov/39967678/).
- Grayscale:
  - Holte & Ferraro (2020 online, Social Science Journal): one week of grayscale cut daily phone use by 37.9 minutes — [Taylor & Francis](https://www.tandfonline.com/doi/full/10.1080/03623319.2020.1737461).
  - A review of grayscale studies reports 22–50 minutes a day less screen time from a week or more of grayscale — [Dekker & Baumgartner 2024, Mobile Media & Communication](https://journals.sagepub.com/doi/10.1177/20501579231212062); [ScienceDirect 2023](https://www.sciencedirect.com/science/article/pii/S2451958823000271).
- Meta-analysis (2024, 18 studies, n = 8,147): digital detox improved subjective well-being (SMD 0.21) and psychological well-being (SMD 0.27) — [Cyberpsychology, Behavior & Social Networking](https://journals.sagepub.com/doi/10.1089/cyber.2023.0742).
- A 2025 RCT on planning a digital detox found no significant effect on total smartphone usage time — [Computers in Human Behavior 2025](https://www.sciencedirect.com/science/article/pii/S0747563225000718). An umbrella review of digital-addiction interventions also exists — [JMIR/PMC](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC11862776/).
- Failure mode, iOS API instability:
  - iOS 26 brought "heavy regressions" reported to Apple since June 2025 and still unaddressed 10 months later. Problems include DeviceActivity thresholds not firing (which also affects Apple's own limits), shields not updating, random tokens handed to shield extensions, and corrupted usage data (up to ~20 hours a day for one domain) — [riedel.wtf](https://riedel.wtf/state-of-the-screen-time-api-2024/); [Apple Developer Forums 819997](https://developer.apple.com/forums/thread/819997); [one sec help: Screen Time API issues](https://tutorials.one-sec.app/en/articles/3036354).
  - An App Limit bug was reported on iOS 26.3.1 — [Apple Community](https://discussions.apple.com/thread/256258057).
  - Third-party apps such as Opal break when the core Screen Time feature breaks — [ScreenBuddy](https://www.screenbuddyapp.com/blog/screen-time-not-working) (competitor).
- Failure mode, platform policy: Android 17 AAPM revokes blockers' Accessibility access — [Android Authority](https://www.androidauthority.com/android-17-beta-2-advanced-protection-mode-accessibility-apps-3648860/).
- Failure mode, self-circumvention cycles: users describe "endless cycles of blocking and unblocking" with Cold Turkey and Screen Time — [drgore Substack](https://drgore.substack.com/p/blockit). "There will always be a way to get around a user app that tries to stop itself from being disabled" — [BetterFilter GitHub](https://github.com/Jolt151/BetterFilter).
- Failure mode, friction apps: people get used to the prompts and push through them. ScreenZen "can make Instagram annoying to open" but can't make it inaccessible — [unhookd](https://unhookd.app/blog/screenzen-worth-it-review) (competitor).
- Failure mode, subscription and billing friction: e.g. the BlockSite renewal complaint from July 2026 — [Trustpilot](https://www.trustpilot.com/review/blocksite.co).

### Inferences
- The evidence suggests that the type of intervention matters more than the brand. Full blocking (Castelo) and financial incentives (Allcott) show the biggest effects, self-set limits and friction show moderate effects, and grayscale is a cheap add-on. Only one sec and Freedom (as a tool) have peer-reviewed support for the specific app. No study found compares a software blocker with a hardware token (Brick or Bloom) head to head.
- In late 2026, reliability risk sits more with the platform (iOS 26 Screen Time API bugs, Android 17 AAPM) than with any single app. That affects software blockers and iOS hardware-token products alike, since the tokens also use the Screen Time API on iPhone.

### Gaps
- There is no long-term (6+ month) retention or relapse data for any consumer blocker app.
- No independent survey quantifies user sentiment across apps; App Store ratings are the only broad signal.
- No peer-reviewed evaluation exists of Opal, ScreenZen, Jomo, Clearspace, AppBlock, Lock Me Out or the launchers. Vendor claims (e.g. Clearspace users going "8+ hours to 2–3 hours") are anecdotal.
