# Brick (getbrick.com / getbrick.app): NFC phone-blocker, as of 2026-09-28

Method note: WebFetch to getbrick.com and apps.apple.com was refused by the egress proxy (EGRESS_BLOCKED). Everything below comes from WebSearch result snippets. The search tool often merges snippets from several results, so a claim is sometimes attributed to the page that most plausibly produced it. Where that attribution is uncertain, the claim says so. The company uses both getbrick.com (current primary storefront) and getbrick.app (older domain, still used for some pages and the hello@getbrick.app email). The help center is at support.getbrick.com, which mirrors brick.frontkb.com.

## Product lineup, prices, subscriptions

### Takeaway
Brick is still one product: a $59 one-time, battery-free NFC puck with a free app and no subscription. It comes in three colors, and multi-buy is 2 for $106.20 (10% off). I found no evidence of a "Brick 2" or a new hardware model as of late Sept 2026. Bulk orders for business or schools are quote-only. Brick launched at $49 in 2023.

### Cited Findings
- Brick is a "$59, battery-free NFC puck"; it is a one-time payment covering hardware and lifetime app access — [Cybernews review 2026](https://cybernews.com/reviews/brick-phone-blocker-review/); [Habit Doom](https://habitdoom.com/blog/brick-alternatives-iphone)
- "Your purchase includes both the physical Brick and full access to the free companion Brick app — no subscriptions, no extra fees"; developer is Brick LLC (Wisconsin) — snippet attributed among [App Store listing](https://apps.apple.com/us/app/brick-ditch-distractions/id6448794069) / [Brick FAQ](https://getbrick.com/pages/faq)
- Amazon now sells it as "The Brick Smartphone Access Blocker, Subscription-Free Phone Lock… High-Grade Magnet & Anti-Slip Silicone" — [Amazon listing](https://www.amazon.com/Brick-Phone-Blocker-Device-Lock-Screen/dp/B0GQ6VV79M)
- Colors: Grey, Charcoal, White. One Brick $59; two Bricks $106.20 (10% discount). The snippet is attributed to [Writer Gadgets](https://writergadgets.com/brick-app-review/) or [whatifididnt.com](https://whatifididnt.com/blog/brick-phone-app/) (merged snippet). The official product page is [getbrick.com/products/grey-brick](https://getbrick.com/products/grey-brick).
- One search snippet claimed "a single Brick product is priced at $15" from the getbrick.com product/bulk pages. This is **unverified and likely a misread** (possibly a sale, accessory, or per-unit bulk figure). Every other source says $59. — [getbrick.com/products/grey-brick](https://getbrick.com/products/grey-brick); [Bulk Bricks page](https://getbrick.app/pages/bulk-bricks)
- Brick is "now HSA & FSA eligible" — snippet from a 2026 search on Brick features (source page among [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/) / [getbrick.com](https://getbrick.com/); attribution uncertain)
- ABC News/GMA ran "I tried the viral Brick app-blocker for 2 weeks. It's now on sale for back-to-school" (Aug–Sept 2026), so periodic discounts exist — [ABC News/GMA](https://abcnews.com/GMA/Shop/brick-review/story?id=134547488)
- Bulk and organization orders: "does not support wholesale… not for resale"; quotes come by emailing hello@getbrick.app; the business page mentions priority support for large orders — [Bulk Bricks](https://getbrick.app/pages/bulk-bricks); [For Business](https://getbrick.com/pages/forbusiness)
- Launch history: about 2,000 devices sold after the mid-September (2023) launch "at a price point of $49"; devices were initially 3D-printed in co-founder Nasgowitz's basement — [BizTimes](https://biztimes.com/jump-start-germantown-startup-brick-aims-to-temporarily-block-distractions-on-smartphones/)
- One Brick can be shared: "Multiple people can use the same Brick device… Each person must create their own separate account" (families, roommates, partners) — snippet from placement/tips search, likely [Brick FAQ](https://getbrick.com/pages/faq) or [whatifididnt.com](https://whatifididnt.com/blog/brick-phone-app/)
- Searches for a new 2026 model turned up nothing; review coverage through Aug/Sept 2026 describes the same device — [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/); [EFTM July 2026](https://eftm.com/2026/07/brick-review-adding-a-physical-roadblock-to-end-your-doom-scrolling-277956)

### Inferences
- The price rose from $49 (2023 launch) to $59. The exact date of the increase was not found.
- A "family pack" is effectively not needed, because one Brick works with several accounts and phones. The 2-pack discount is the only multi-buy found.

### Gaps
- I could not load the live product page to confirm current colors, 3-packs, or sale price as of Sept 28, 2026.
- The "$15" figure could not be explained or verified.
- There is no confirmation of any accessory or subscription add-on. The App Store in-app-purchase list could not be fetched.

## How it works: iOS vs Android (and generic Android keyboard phones)

### Takeaway
On iOS, Brick uses Apple's Screen Time API (FamilyControls + ManagedSettings) to "shield" apps and silence their notifications. The puck is a passive NFC tag that the app must read to toggle. On Android (12+), blocking runs through an Accessibility Service, and Strict Mode adds Device Admin. The Android version is widely considered weaker and less popular.

### Cited Findings
- iOS mechanism: Brick "uses the Screen Time API (FamilyControls + ManagedSettings) to shield every app except your allow-list." The snippet came up alongside an open-source clone discussion and is attributed to the [boring-phone GitHub README](https://github.com/MellevdB/boring-phone) / [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/) (merged snippet).
- NFC works like contactless payments. Tapping the phone on Brick immediately blocks the selected apps and silences their notifications — [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/)
- The app must be open to Brick and Unbrick, and NFC must be on. If the puck won't scan, support suggests testing with the free "NFC Tools" app — [Brick support: "Brick won't scan?"](https://support.getbrick.com/en/articles/6572865)
- You can Brick without the puck by holding the virtual Brick on the app homepage for 5 seconds, but unbricking still needs the physical device (or an Emergency Unbrick) — snippet via [Brick FAQ](https://getbrick.com/pages/faq) / [support](https://support.getbrick.com/)
- The puck has no battery, no charging, and a magnet on the back; it is about 1.5–2 in. and made of 3D-printed plastic — [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/); [Digital Reviews Network](https://www.digitalreviews.net/reviews/mobile/brick-phone-distraction-device-review/)
- Android support: "Brick currently works with Android devices running version 12.0 or later" — [Brick Android page](https://getbrick.com/pages/android); [Google Play listing](https://play.google.com/store/apps/details?id=com.brickllc.brick&hl=en_US)
- On Android, Brick "requires Android's Accessibility Service to detect when apps are launched… it only monitors app package names… no personal content or data is accessed" — [Google Play listing](https://play.google.com/store/apps/details?id=com.brickllc.brick&hl=en_US)
- "If you turn Strict Mode on, Brick will also need Device Admin Permission… to prevent unauthorized app uninstallation" (Android) — [Brick help center, Android Support](https://brick.frontkb.com/en/categories/1717121-android-support)
- "Every Android blocker runs through an Accessibility Service that you can switch off in Settings, so no Android option is truly bypass-proof." The same source says Brick is "iPhone-first," with about 3,960 Play reviews against more than 50,000 iPhone ratings — [Habit Doom, Brick alternative for Android](https://habitdoom.com/blog/brick-alternative-android). Note that Habit Doom is a competitor.
- On iOS you can block "any iPhone app (except the dialer, per Apple policy)" plus selected Safari websites — snippet among [App Store](https://apps.apple.com/us/app/brick-ditch-distractions/id6448794069) / [Pocket-lint](https://www.pocket-lint.com/brick-device-phone-app-blocker/)

### Inferences
- Generic Android keyboard phones (Unihertz Titan series, Clicks Communicator) should in principle run Brick if they run Android 12+, include Google Play, and have **NFC hardware**, since Brick relies only on standard Accessibility/Device Admin APIs and NFC. On these phones Brick inherits Android's weaknesses: Accessibility can be switched off, and there are OEM multi-window quirks. Verify NFC on the specific model before buying.
- iOS enforcement is stronger because Apple's Screen Time shields are applied at the OS level, and Strict Mode blocks the delete and Screen Time escape routes.

### Gaps
- I found no reports (Reddit or otherwise) of Brick on Unihertz Titan or Clicks Communicator specifically, and I could not confirm NFC presence on those models from this research.
- I couldn't get the current Google Play rating or install count. The Habit Doom figure of about 3,960 reviews is from an undated 2026 snapshot.

## Modes, schedules, Strict Mode, Emergency Unbricks, lost Brick, escape routes

### Takeaway
Brick supports multiple named Modes, each either a block list or an allow list ("choose which apps NOT to block"), plus Safari website blocking and scheduled auto-Bricking. Strict Mode prevents deleting the app or changing Screen Time permissions while Bricked. There are 5 Emergency Unbricks, which don't auto-refresh; you get more by submitting a form or emailing support, with a turnaround of about 48 hours. If you lose the Brick, Emergency Unbricks are the only way out, and after that you need a new puck or a support refill.

### Cited Findings
- Custom modes (Work, Family, Sleep, Gym, Study, Focus…); each mode can block "up to 50" apps — snippet among [App Store](https://apps.apple.com/us/app/brick-ditch-distractions/id6448794069) / [Google Play](https://play.google.com/store/apps/details?id=com.brickllc.brick&hl=en_US)
- "Up to ten custom modes… each blocking its own set of apps plus a chosen list of Safari websites" — snippet from the WSJ/Freedom search, attribution uncertain (likely [Freedom vs Brick](https://freedom.to/blog/freedom-vs-brick/))
- Allow-list option: "choose which apps to block, or flip it and choose only the apps you want to keep — everything else gets locked" — [whatifididnt.com](https://whatifididnt.com/blog/brick-phone-app/) / [Pocket-lint](https://www.pocket-lint.com/brick-device-phone-app-blocker/) (merged snippet)
- Schedules: "set your phone to automatically Brick at set times each day" — [App Store listing](https://apps.apple.com/us/app/brick-ditch-distractions/id6448794069). One comparison says "Brick's scheduling only lets you set a start time" (you still tap to end), while Unpluq has start and end times — [whatifididnt: Unpluq vs Brick](https://whatifididnt.com/blog/unpluq-vs-brick/). This may be outdated.
- Website blocking: you can block Safari or any browser app, or block specific sites like YouTube.com without blocking the whole browser — [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/). However, Brick "cannot block specific website links in non-Safari browsers" — [Brick "Need help blocking?"](https://getbrick.com/pages/need-help-blocking)
- Strict Mode "makes it impossible to delete the Brick app from your phone or disable the app through your phone's screen time settings." You toggle it in Brick Settings — [Brick help center](https://brick.frontkb.com/en/articles/6572993); [whatifididnt.com](https://whatifididnt.com/blog/brick-phone-app/)
- "In the past, deleting the Brick app would unbrick your phone" (without Strict Mode, deleting still restores access) — [whatifididnt.com](https://whatifididnt.com/blog/brick-phone-app/)
- Strict Mode can also disable using emergency unBricks: "Strict Mode is a feature that prevents you from bypassing Brick, like deleting the app, or using your emergency unBricks" — snippet from the features search, attribution uncertain ([Pocket-lint](https://www.pocket-lint.com/brick-device-phone-app-blocker/) / [Writer Gadgets](https://writergadgets.com/brick-app-review/)). This conflicts with other sources that treat Emergency Unbricks as the out even in Strict Mode ([help center](https://brick.frontkb.com/en/articles/6572993)).
- Emergency Unbricks: "You get five Emergency Unbrick per device" — [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/). Contradicting this, "five being your lifetime total per account… fill out a form… most requests handled within 48 hours" — [whatifididnt.com](https://whatifididnt.com/blog/brick-phone-app/). Another source says the count can be reset "within two business days by filling out an online form" — [Writer Gadgets](https://writergadgets.com/brick-app-review/) / [influencerhub](https://influencerhub.blog/brick-review-honest-experience/). NBC says you "have to email Brick customer support to refresh them" — [NBC Select](https://www.nbcnews.com/select/shopping/brick-phone-app-blocker-review-rcna259740)
- Lost Brick: "Emergency Unbricks let you unlock your phone without your Brick… You get 5, so you're covered if you lose your Brick" — [Brick FAQ](https://getbrick.com/pages/faq). "If your phone is bricked and you run out of Emergency Unbricks, the only way back into blocked apps is by scanning the actual device" — [Brick help center](https://brick.frontkb.com/en/articles/6572993)
- The home screen shows a timer of how long you've been Bricked, and a summary of blocked notifications appears when you end a session — [NBC Select](https://www.nbcnews.com/select/shopping/brick-phone-app-blocker-review-rcna259740); [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/)

### Inferences
- "Per device" vs "per account" is ambiguous. The safest buyer framing is "5 total, not auto-refreshing; refills by support request (~2 business days)."
- The 50-apps-per-mode limit matches Apple's FamilyControls selection-token limit, so allow-list mode is the practical way to go "essentials-only" with many apps.
- A lost Brick plus Strict Mode plus 0 emergency unbricks means you wait on support or buy a new $59 puck. No remote unlock was documented.

### Gaps
- Could not confirm whether a *different* Brick (for example, a spouse's) can unbrick your phone, or whether each account is paired to one specific puck.
- Could not confirm current schedule end-time support (2026 app version).
- Could not confirm whether Strict Mode disables Emergency Unbricks by default or only optionally.

## Known bypasses, bugs, complaints, failure modes

### Takeaway
Main documented bypasses:
- **iOS:** deleting the app or changing Screen Time permissions (closed by Strict Mode), browser/in-app-browser links (partially mitigated), and non-Safari browsers for specific sites.
- **Android:** turning off Accessibility, and opening blocked apps in a floating "mini window."

Other complaints are setup bugs and crashes, forgetting to allow-list essentials (you must unbrick to edit), NFC scan hiccups, and a disliked navigation redesign. Official iOS 26 NFC breakage for Brick was not confirmed.

### Cited Findings
- Android: "users can bypass all bricked apps by opening them in a mini window" (from Google Play reviews) — [Habit Doom](https://habitdoom.com/blog/brick-alternative-android); also surfaced in [Google Play listing (en_IN)](https://play.google.com/store/apps/details?id=com.brickllc.brick&hl=en_IN) snippet
- Browser links: "Safari and browser blocking works but some hyperlinks in other apps can bypass it, though there is a documented workaround" — [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/). Loopholes include "hyperlinks in note apps" — [whatifididnt.com](https://whatifididnt.com/blog/brick-phone-app/)
- General iOS Screen Time loopholes (not Brick-specific) include in-app browsers and the Passwords app's browser — [Apple Developer Forums](https://developer.apple.com/forums/thread/127748)
- Setup: "Several app crash reports… during setup"; "easy to forget to whitelist essential apps, such as banking or ride-share, which requires unbricking to edit" — [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/)
- App Store complaint: a "new navigation scheme… unintuitive… insufficiently tested"; the developer replied it had shipped bug fixes. One reviewer mentions "premium subscribers," which is odd given no subscription exists; this may refer to an older or legacy offering and is unverified — [App Store UK reviews](https://apps.apple.com/gb/app/brick-ditch-distractions/id6448794069?see-all=reviews)
- Forbes testers "searched for ways to hack the Brick without breaking it on regular and Strict modes"; the review notes "quirks that may annoy some users" — [Forbes Vetted](https://www.forbes.com/sites/forbes-personal-shopper/article/brick-review/)
- Strictness cuts both ways: if you leave the house without the puck, "you cannot unBrick remotely, which is much stricter than most apps" — [NBC Select](https://www.nbcnews.com/select/shopping/brick-phone-app-blocker-review-rcna259740)
- iOS 26 has generic Core NFC developer reports (NFC reader not working after upgrade, "Go to Settings" alert not navigating in beta) — [Apple Dev Forums 800624](https://developer.apple.com/forums/thread/800624); [798236](https://developer.apple.com/forums/thread/798236). These are not tied to Brick.

### Inferences
- On iOS, turn on Strict Mode on day 1 and block all browsers, or at least use website blocks. Without Strict Mode, Brick is only "friction," because deleting the app escapes it.
- On Android, the protection is inherently weaker: Accessibility can be toggled off, and there is the mini-window bypass. Treat it as a speed bump.

### Gaps
- Direct Reddit threads (r/nosurf, r/digitalminimalism, r/dumbphones) did not surface in search results, so Reddit sentiment is **not captured**.
- No confirmed report of a specific iOS update breaking Brick. No data on notification leaks, or on "only one app at a time" behavior.

## Reviews, ratings, company background

### Takeaway
Press coverage is broadly positive. Reviewers say it works because you "can't cheat," while noting it's a stopgap, not a cure. The iOS App Store rating is about 4.9 from tens of thousands of ratings. The company is Brick LLC, Germantown/Milwaukee, Wisconsin, founded in 2023 by UW-Madison grads TJ Driver and Zach Nasgowitz. It is bootstrapped, with no outside funding reported and no Shark Tank appearance found (competitor Bloom was on Shark Tank).

### Cited Findings
- iOS rating: 4.94 from 32.6K reviews — [Bitrise app benchmark](https://bitrise.io/resources/tools/app-navigator/apps/ios/com.brickllc.brick-app). Habit Doom says "more than 50,000 iPhone ratings" (later snapshot?) — [Habit Doom](https://habitdoom.com/blog/brick-alternative-android). The official listing is at [App Store](https://apps.apple.com/us/app/brick-ditch-distractions/id6448794069).
- NBC Select (Feb 2026), 2-week test, largely positive: "I loved it… made meeting certain goals… so much easier"; it works because it "doesn't give the option to cheat out of it" — [NBC Select](https://www.nbcnews.com/select/shopping/brick-phone-app-blocker-review-rcna259740)
- Consumer Reports: it helped cut screen time (some days under 3 hrs) but "didn't solve the problem outright"; the reviewer calls it "one of the better stopgap solutions" — [Consumer Reports](https://www.consumerreports.org/electronics-computers/cell-phones/can-brick-solve-my-screen-time-problem-a3152096219) / [MSN mirror](https://www.msn.com/en-us/news/technology/can-brick-solve-my-screen-time-problem/ar-AA1TmgVe)
- Forbes Vetted (2026), tested more than a month: "a real game-changer if you're constantly missing important moments" — [Forbes Vetted](https://www.forbes.com/sites/forbes-personal-shopper/article/brick-review/)
- Other 2026 reviews: [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/), [ABC News/GMA](https://abcnews.com/GMA/Shop/brick-review/story?id=134547488), [EFTM (July 2026)](https://eftm.com/2026/07/brick-review-adding-a-physical-roadblock-to-end-your-doom-scrolling-277956), [Pocket-lint](https://www.pocket-lint.com/brick-device-phone-app-blocker/), [Marie Claire UK](https://www.marieclaire.co.uk/life/health-fitness/brick-phone-detox-device-review), [Apartment Therapy](https://www.apartmenttherapy.com/brick-app-review-37523373), and the year-long user review at [whatifididnt.com](https://whatifididnt.com/blog/brick-phone-app/)
- WSJ coverage: "Brick is building a devoted user base of phone addicts who want to stop doomscrolling"; press has also included the Washington Post and Vogue — [WSJ on Threads](https://www.threads.com/@wsj/post/DaNH-nVGzDG/brick-is-building-a-devoted-user-base-of-phone-addicts-who-want-to-stop); [WSJ on X](https://x.com/WSJ/status/2071889183421947916)
- SSENSE framed Brick as "a Status Symbol for the Aspirationally Offline" — [SSENSE](https://www.ssense.com/en-us/editorial/technology/how-brick-became-a-status-symbol-for-the-aspirationally-offline)
- Founders: TJ Driver and Zach Nasgowitz, 2023, Germantown, WI; "totally bootstrapped"; declined to share sales, but saw a "very big jump" heading into 2026 — [BizTimes](https://biztimes.com/jump-start-germantown-startup-brick-aims-to-temporarily-block-distractions-on-smartphones/); [BizTimes company profile](https://biztimes.com/company/brick/); [GenZ Entrepreneurship](https://genzentrepreneurship.com/2025/10/13/zach-nasgowitz-tj-driver-brick/)
- Shark Tank: none found for Brick. Bloom (a rival) is marketed "as seen on Shark Tank" — [unhookd](https://unhookd.app/blog/best-physical-phone-blockers-2026)

### Inferences
- The 4.9 rating comes from a hardware-gated audience of committed buyers, so it likely overstates satisfaction compared with the general population.

### Gaps
- No Wired, The Verge, NYT/Wirecutter, Tom's Guide or Mashable Brick review surfaced in searches. Their verdicts are unknown, not negative.
- YouTube reviews were not researched, since snippets aren't useful for video.
- The date of the most recent funding and sales data is unknown.

## Comparison with closest rivals

### Takeaway
Brick ($59 one-time) sits between cheaper one-time options (Bloom stainless card, about $39; free open-source Foqos with your own NFC tag or QR code) and subscription models (Unpluq about $79/yr; Blok yearly plan). Reviewers rank Brick among the hardest to bypass when the puck is kept out of reach.

### Cited Findings
- Unpluq about $79/yr is costlier long-term than Brick ($59) or Bloom ($39); Foqos, Bloom and the Autonomous Key are one-time, and Blok is a yearly plan — [autonomous.ai](https://www.autonomous.ai/ourblog/best-brick-alternatives-for-iphone-and-android) / [whatifididnt: Unpluq vs Brick](https://whatifididnt.com/blog/unpluq-vs-brick/) (merged snippet)
- Bloom is a stainless steel card that you tap near the top of the phone; Brick is a magnetic plastic cube — [whatifididnt: Bloom vs Brick](https://whatifididnt.com/blog/bloom-vs-brick/)
- Unpluq has start and end schedules, "pause for today," and a post-unlock 5-minute timer; Brick schedules only a start — [whatifididnt: Unpluq vs Brick](https://whatifididnt.com/blog/unpluq-vs-brick/)
- "The hardest to bypass are physical blockers you can put out of reach: Brick, the Autonomous Key, Bloom in child mode, and Blok's system-level blocking" — [whatifididnt: 7 best physical blockers](https://whatifididnt.com/blog/physical-phone-blocker/)
- Foqos is a free, open-source iOS app that locks apps behind an NFC tag or QR code, an "alternative to Brick, Opal, ScreenZen, Unpluq…" — [Foqos GitHub](https://github.com/awaseem/foqos)
- Blok supports barcodes on iOS (any scannable code becomes a key) — [Blok App Store](https://apps.apple.com/ie/app/blok/id6477306136)
- Other rivals: BLOCC ([BLOCC vs Brick](https://www.getblocc.com/blog/blocc-vs-brick.html)) and Freedom, a software-only tool ([Freedom vs Brick](https://freedom.to/blog/freedom-vs-brick/))

### Inferences
- Most comparison sites are rival vendors (Habit Doom, BLOCC, Blok, unhookd, Freedom, Autonomous), so treat their Brick criticisms with caution.

### Gaps
- Prices for Bloom, Blok and Unpluq were not independently verified here. Another researcher is presumably covering those products.

## Practical tips: placement and "essentials only" configuration

### Takeaway
Keep the Brick somewhere that takes deliberate effort to reach: fridge, office whiteboard, car glove box, or with a partner. Use allow-list mode for an "essentials only" phone. Before your first Brick, add everything you truly need (Wallet, Maps, banking, Uber/Lyft, Messages, Phone, authenticator, 2FA, calendar), because editing requires unbricking. Enable Strict Mode, and block all browsers or at least specific sites.

### Cited Findings
- The magnet sticks to fridges, car dashboards, and filing cabinets. Users keep it in the glove box or trunk after work; one user leaves it in the car and blocks social and streaming for 10–12 hours at a time. Keeping it out of the office means "you have to physically leave the room" to unlock — [Pocket-lint](https://www.pocket-lint.com/brick-device-phone-app-blocker/); [whatifididnt.com](https://whatifididnt.com/blog/brick-phone-app/) (merged snippet)
- Allow-list flip ("choose only the apps you want to keep") — [whatifididnt.com](https://whatifididnt.com/blog/brick-phone-app/)
- A common mistake is forgetting to allow-list banking or ride-share, which requires unbricking to fix — [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/)
- Block Safari or all browsers, or only specific domains (YouTube.com, Instagram.com) — [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/); non-Safari site-level blocking isn't supported — [Brick "Need help blocking?"](https://getbrick.com/pages/need-help-blocking)
- The phone dialer can't be blocked (Apple policy), so calls always work — snippet among [App Store](https://apps.apple.com/us/app/brick-ditch-distractions/id6448794069) / [Pocket-lint](https://www.pocket-lint.com/brick-device-phone-app-blocker/)
- Partners and roommates can share one Brick with separate accounts — [Brick FAQ](https://getbrick.com/pages/faq) (via snippet)

### Inferences
- Suggested essentials-only allow list (US): Phone, Messages, Wallet/Apple Pay, Maps/Google Maps, bank and credit-card apps, Uber/Lyft, authenticator and password manager, Calendar, Camera, Music/Podcasts (optional), work MFA. Block: social, YouTube, all browsers (Safari, Chrome), news, shopping, games. Since in-app browsers and the Passwords app can leak web access, allow-list mode beats a block list here.
- Storing the Brick with a partner or at the office adds accountability. Keep in mind that the 5 Emergency Unbricks remain the escape hatch unless Strict Mode disables them.

### Gaps
- There is no official Brick guidance on a recommended "essentials" configuration. The list above is inference.
