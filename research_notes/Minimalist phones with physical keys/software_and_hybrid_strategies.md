# Software and Hybrid Strategies for Cutting Smartphone Screen Time (as of 2026-09-28)

Research note on method: WebSearch worked, but WebFetch was blocked by the egress proxy for most primary sources (PMC/OUP, Apple Support, Verizon, Georgetown, Tom's Guide, news-medical, whatifididnt.com). Many findings below therefore come from search-result snippets, not full-page reads. Each one is flagged where that matters. Treat exact prices and feature claims from vendor or affiliate blogs (autonomous.ai, habitdoom, unhookd, getblocc, blok.so, blankspaces.app, getfinit, FaithLock) as marketing-adjacent: many of them sell a competing blocker.

## 1. Physical-token blockers and software locks on a normal smartphone

### Takeaway
Physical-token blockers (Brick, Bloom, Unpluq and many clones) use an NFC tag or card plus Apple's Screen Time API (Family Controls) or Android accessibility/usage permissions. Their main value is friction, and none of them is bypass-proof. Brick ($59, no subscription) has the strictest reputation, and Bloom (about $39 card, free app) is the cheaper, more flexible option. Among software-only tools, one sec has the best peer-reviewed evidence (PNAS 2023). Opal and ScreenZen are the most-recommended "hard block" and "gentle friction" apps in 2026 roundups.

### Cited Findings
**Brick**
- Brick is an app plus a physical puck. Apps you choose stay blocked until you tap the phone to the Brick again ("unbrick"). — [NBC Select review](https://www.nbcnews.com/select/shopping/brick-phone-app-blocker-review-rcna259740); [Cybernews Brick review 2026](https://cybernews.com/reviews/brick-phone-blocker-review/)
- Price: $59 one-time, no subscription fees. — [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/); [autonomous.ai Brick vs Bloom](https://www.autonomous.ai/ourblog/brick-vs-bloom) (sells a competing "Autonomous Key")
- Emergency override: 5 Emergency Unbricks per device. After those are used, you request more through a form, usually handled within 48 hours. — search snippet summarizing [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/) / [Scribbright](https://scribbright.com/brick-phone-blocker-review/) (full page not fetched)
- Known bypass: without Strict Mode, you can turn Brick off in iOS Screen Time settings, or on Android find it under Accessibility settings. Strict Mode removes the ability to delete the app or change those settings during a session. — search snippet ([Digital Reviews Network](https://www.digitalreviews.net/reviews/mobile/brick-phone-distraction-device-review/) / [Cybernews](https://cybernews.com/reviews/brick-phone-blocker-review/))
- Tap works "immediately", placement doesn't matter much, and the interaction is "hard to mess up". — [autonomous.ai](https://www.autonomous.ai/ourblog/brick-vs-bloom) (competitor blog)
- Other mainstream reviews exist: Forbes Vetted (2026), Consumer Reports (paywalled), NBC Select, and an AOL/Independent piece on the "two-inch magnet". I did not read their verdicts. — [Forbes](https://www.forbes.com/sites/forbes-personal-shopper/article/brick-review/); [Consumer Reports](https://www.consumerreports.org/electronics-computers/cell-phones/can-brick-solve-my-screen-time-problem-a3152096219); [AOL](https://www.aol.com/articles/two-inch-magnet-promises-curb-050000341.html)

**Bloom**
- NFC card, about $39, with a free app. It is built for "controlled access" (short breaks, scheduled focus windows, per-app control) rather than a full lockout. — [autonomous.ai](https://www.autonomous.ai/ourblog/brick-vs-bloom)
- Bloom added a strict mode in 2026 that stops you force-closing the app to escape a block, after early reviewers found that gap. — [autonomous.ai](https://www.autonomous.ai/ourblog/brick-vs-bloom)
- Rule of thumb from that comparison: if you will abuse a five-minute break, buy Brick. If you want breaks or a child mode, Bloom does more for about $20 less. — [autonomous.ai](https://www.autonomous.ai/ourblog/brick-vs-bloom)

**Unpluq**
- Unpluq "pioneered" the NFC-tag blocking category. The tag clips to your keys or can be given to someone else for accountability. It has schedules and runs on a subscription: about $79 for the tag with the first year included. — [autonomous.ai](https://www.autonomous.ai/ourblog/brick-vs-bloom) (renewal price after year 1 not confirmed)

**Other token blockers seen in 2026 roundups (not verified in depth)**
- BLOCC, Autonomous Key, Habit Doom, Blok, unhookd and others are marketed as Brick alternatives. Several of the comparison pages come from these vendors. — [getblocc.com](https://www.getblocc.com/blog/blocc-vs-brick.html); [habitdoom.com](https://habitdoom.com/blog/brick-alternatives-iphone); [unhookd](https://unhookd.app/blog/best-physical-phone-blockers-2026); [autonomous.ai 7 alternatives](https://www.autonomous.ai/ourblog/best-brick-alternatives-for-iphone-and-android)
- According to the autonomous.ai comparison, the Autonomous Key "stays locked by default so there is no session to force-close", but it has "the shortest track record". — [autonomous.ai](https://www.autonomous.ai/ourblog/brick-vs-bloom) (self-promotional)

**Software-only friction and blocking apps**
- one sec: a peer-reviewed field experiment (PNAS, 2023; Max Planck Institute for Human Development and Heidelberg University; 280 participants over 6 weeks) found:
  - 36% of attempts to open a target app were abandoned after the one sec pop-up;
  - attempts to open target apps fell 37% versus week 1;
  - actual opening of target apps fell 57% after six weeks;
  - users reported more satisfaction with their app use.
  — [PNAS: Grüning et al., "Directing smartphone use through the self-nudge app one sec"](https://www.pnas.org/doi/10.1073/pnas.2213114120); [one sec research page](https://one-sec.app/max-planck-study/)
- How one sec works: a pop-up with a deliberation message, a short wait, and the option to dismiss opening the app. — [PNAS](https://www.pnas.org/doi/10.1073/pnas.2213114120)
- ScreenZen (free, iOS and Android): micro-interventions (delays, short timed access windows, streaks) plus a minimalist launcher that hides blocked apps. Opal is recommended for "hard boundaries" and the "strongest possible block". — [FaithLock ScreenZen vs Opal](https://www.getfaithlock.com/resources/screenzen-vs-opal) (competitor blog); [ScreenZen App Store](https://apps.apple.com/us/app/screenzen-screen-time-control/id1541027222); [Opal App Store](https://apps.apple.com/us/app/opal-screen-time-control/id1497465230); [MakeUseOf on Opal](https://www.makeuseof.com/opal-screen-time-limiting-app-helps-use-phone-less/)
- Minimalist launchers (Minimalist Phone, Olauncher, Blank Spaces, Dumbify and others) take a "make the device boring" approach instead of hard locks. — [search summary / Habi roundup](https://habi.app/insights/best-screen-time-apps/); [Blank Spaces blog](https://www.blankspaces.app/blog/one-sec-vs-blank-spaces) (vendor)
- Freedom (the blocker used in the Castelo RCT, see section 3) is a cross-device app, website and internet blocker. — [Freedom](https://freedom.to/); [Wikipedia: Freedom](https://en.wikipedia.org/wiki/Freedom_(application))

### Inferences
- On iOS every third-party blocker (Brick, Bloom, Opal, one sec, ScreenZen) sits on Apple's Screen Time / Family Controls framework, so blocking strength differs mainly in how each app locks down its own settings ("strict mode"). The final escape hatch is usually deleting the app or revoking Screen Time permission. A more robust trick sometimes suggested is setting a Screen Time passcode you don't know (a partner sets it). Not verified in a fetched source.
- On Android, blockers use Accessibility or Usage Access permissions. These are easier to revoke unless the app blocks the Settings screens.
- Evidence ranking for app-level tools: one sec has the only peer-reviewed RCT-style field data found. Brick, Bloom and Unpluq have reviews and testimonials but no independent studies were found.
- For this user, a token blocker is cheap ($39–$79) and can be tried in days, so it's worth testing before buying a dedicated minimalist device.

### Gaps
- I couldn't read full product pages or independent reviews (The Verge, Wired, Wirecutter): fetches were blocked. Wirecutter or Verge verdicts on Brick/Bloom for 2026 are unconfirmed.
- Current Unpluq renewal price, Blok and Tapuz pricing, and "Ground" (named in the brief): no reliable info found.
- No independent comparative study of launchers (Olauncher, Niagara, Minimalist Phone, Blank Spaces, Dumbify, the "Dumbphone" iOS app) against blockers was found.
- iOS Focus modes plus grayscale, Android Digital Wellbeing/Bedtime and Samsung Modes: I have no fetched sources on their effectiveness. Grayscale's effect is plausibly supported by small studies (for example, Holte and Ferraro), but those weren't verified in this session.

## 2. Pairing a Clicks keyboard with a locked-down smartphone vs. buying a dedicated device

### Takeaway
The user already owns a Clicks keyboard. A smartphone plus a Clicks keyboard (case or Power Keyboard) plus a token or software blocker gives the physical-key feel and full modern essentials for $0–$59 extra. A dedicated device like the Clicks Communicator ($499 list) adds a separate phone and plan. The trade-off is that on the smartphone, self-control still depends on the lock software.

### Cited Findings
- Clicks Power Keyboard:
  - battery-powered Bluetooth QWERTY that snaps magnetically to MagSafe iPhones and Qi2 Android phones;
  - slide-out design that hides behind the phone;
  - also works with tablets, PCs and TVs;
  - 2,150 mAh battery with 5W wireless charging, backlit.
  — [Liliputing](https://liliputing.com/clicks-power-keyboard-is-a-magnetic-thumb-keyboard-wireless-power-bank-for-your-phone/); [Clicks product page](https://clicks.tech/products/powerkeyboard); [CNN Underscored](https://www.cnn.com/cnn-underscored/reviews/clicks-power-keyboard)
- Power Keyboard pricing: pre-orders opened at $79 (announced January 2026), with retail planned at $109 and a Spring 2026 launch. Search snippets as of now show $99. — [9to5Google, Jan 2 2026](https://9to5google.com/2026/01/02/clicks-is-making-a-magnetic-keyboard-that-slides-out-from-pixel-10-and-any-other-phone/); [AppleInsider](https://appleinsider.com/articles/26/01/02/clicks-has-swapped-iphone-keyboard-cases-for-a-new-magsafe-keyboard-battery); [brandclickx $99 review](https://brandclickx.com/clicks-power-keyboard-review/) (the current price conflicts between sources)
- Reviews: "fun", delivers on its promise, "snappy, well-made keys", lots of customization, basic wireless charging. — [CNN Underscored](https://www.cnn.com/cnn-underscored/reviews/clicks-power-keyboard); also [Fast Company](https://www.fastcompany.com/91589909/clicks-power-keyboard-review), [Tech Advisor (Pixel 10 Pro)](https://www.techadvisor.com/article/3029955/clicks-power-keyboard-hands-on-review-perfect-for-pixel-10-pro.html), [Cybernews](https://cybernews.com/reviews/clicks-power-keyboard-review/)
- Clicks keyboard cases exist for iPhone 17, 16, 15 and 14, plus Pixel 10, Galaxy S25 and Motorola Razr. The iPhone 17 case is billed as "thinner, lighter". — [Clicks iPhone 17](https://clicks.tech/products/clicks-keyboard-for-iphone-17); [Clicks shop-all](https://www.clicks.tech/collections/shop-all); [9to5Google](https://9to5google.com/2026/01/02/clicks-is-making-a-magnetic-keyboard-that-slides-out-from-pixel-10-and-any-other-phone/)
- Clicks Communicator: a message-centric Android phone with a hardware keyboard. $499 at launch, with early-bird paths of $399 paid in full or a $199 deposit. — [Gadget Hacks](https://smartphones.gadgethacks.com/news/clicks-communicator-physical-keyboard-phone-launches-2026/); [RedShark News (CES 2026)](https://www.redsharknews.com/clicks-ces-2026-power-keyboard-communicator)

### Inferences
- The Clicks keyboard doesn't reduce screen time by itself: it improves typing. Screen-time reduction still has to come from a blocker or launcher. The combination most analogous to a "keyboard dumbphone" is:
  - a minimalist launcher (or ScreenZen's launcher);
  - a Brick or Bloom token locking social apps, browsers and video;
  - Messages, Phone, Maps, Wallet, bank and 2FA left unblocked.
- A dedicated device (Communicator, or a keyboard dumbphone covered by other researchers) gives structural separation and no unlock path, which the Castelo RCT suggests is the active ingredient (see section 3). The cost is a second device or number and weaker app compatibility.

### Gaps
- Whether the Communicator has shipped and its real-world reviews as of late September 2026 was not verified here (likely covered by the dedicated-device researcher).
- Whether the Power Keyboard's Bluetooth HID works with Brick/Opal lock screens or during Focus modes: no info.

## 3. Hybrid two-device setups (dumbphone + watch, ring or card for payments and essentials)

### Takeaway
Payments are the easy part: a contactless card or a passive NFC payment ring (Tapster, McLear) needs no phone at all. A cellular smartwatch as a standalone "smartphone replacement" is the hard part, because both Apple Watch and Wear OS watches need a smartphone for initial setup.

On the Apple side:
- Apple Watch For Your Kids (Family Setup) runs standalone but disables card-based Apple Pay.
- A normal cellular Apple Watch paired to an iPhone left in a drawer is the common workaround.

Pixel Watch LTE can make calls and use Google Wallet without the phone nearby after setup. Carrier number-sharing (NumberSync, NumberShare, DIGITS) links a smartphone's number to a watch. Only T-Mobile DIGITS was historically described as including feature phones.

### Cited Findings
**Apple Watch**
- Family Setup lets a parent or guardian's iPhone set up a cellular Apple Watch for someone without an iPhone. The watch then has its own number and Apple ID and works standalone. — [Apple Support 109036](https://support.apple.com/en-us/109036) (search snippet; page fetch blocked); [US Mobile](https://www.usmobile.com/blog/how-to-set-up-apple-watch-for-family-without-an-iphone/)
- Every Apple Watch needs an iPhone for initial activation and pairing. — [US Mobile / search summary](https://www.usmobile.com/blog/how-to-set-up-apple-watch-for-family-without-an-iphone/)
- Limitation: Apple Pay with credit or debit cards in Wallet is not available with Apple Watch For Your Kids. Apple Cash Family is available for users under 18 (US only). — [Apple Support 109036](https://support.apple.com/en-us/109036) (via snippet)
- Family Setup requirements quoted in older snippets: iPhone 6s or later, Apple Watch Series 4 or later (GPS + Cellular) or SE, and a Family Sharing group. These are likely outdated: current requirements would reference newer iPhone and iOS versions. — [Apple Support](https://support.apple.com/en-us/109036) (flag: older info)
- Carriers support standalone-mode activation for "Apple Watch For Your Kids", for example Verizon. — [Verizon KB 234270](https://www.verizon.com/support/knowledge-base-234270/)
- People have used just an Apple Watch as their phone (one writer sold their iPhone). Calls, texts and Siri dictation for longer messages work. — [Redeeming Productivity](https://redeemingproductivity.com/how-to-use-an-apple-watch-as-a-standalone-phone/)
- Tom's Guide ran a first-person piece: "The best dumbphone is the one I'm already wearing on my wrist". — [Tom's Guide](https://www.tomsguide.com/phones/the-best-dumbphone-is-the-one-im-already-wearing-on-my-wrist-and-it-made-me-return-to-life-more) (not fetched; details unverified)
- Direct cellular on Apple Watch needs an eligible activated plan, supported carrier, region, software and coverage. — [Techpod](https://www.itechpod.com/blogs/techpod-blog/dumbphone-phone-light-full-smartphone)

**Wear OS (Pixel Watch / Galaxy Watch)**
- A Pixel Watch with an active LTE plan can be used without being connected to a smartphone. The Pixel Watch 2 LTE can place calls with no phone nearby. — [Google Pixel Watch Community thread](https://support.google.com/googlepixelwatch/thread/222012671/can-i-use-pixel-watch-with-lte-without-a-cell-phone?hl=en); [Notebookcheck Pixel Watch 2 LTE review](https://www.notebookcheck.net/Google-Pixel-Watch-2-LTE-smartwatch-in-review-Where-are-the-improvements.796541.0.html)
- Cards added to Google Wallet on the watch live on the watch. Google services need LTE or a paired phone in Bluetooth range. — [Google Pixel Watch Help: contactless payments](https://support.google.com/googlepixelwatch/answer/12661007?hl=en)
- Wear OS added Google Maps directions without the phone (older news; date not verified). — [Tom's Guide](https://tomsguide.com/news/wear-os-now-offers-google-maps-directions-without-your-phone)

**Carrier number sharing**
- AT&T NumberSync: use your smartphone number on tablets, smartwatches and smart speakers (calls and texts on compatible smartwatches). — [AT&T NumberSync](https://www.att.com/features/numbersync/)
- Verizon NumberShare: share "your smartphone's number" with compatible smartwatches and connected devices. — [Verizon NumberShare](https://www.verizon.com/solutions-and-services/numbershare/); [Verizon FAQ](https://www.verizon.com/support/numbershare-faqs/)
- T-Mobile DIGITS: one number across smartphones, PCs, cellular smartwatches "and even feature phones". This is 2016 launch coverage and may be outdated. — [TechCrunch 2016](https://techcrunch.com/2016/12/07/t-mobile-digits)
- XDA discussion of LTE watch, number sharing and "standalone" mode interactions. — [XDA Forums](https://xdaforums.com/t/lte-watch-numbershare-numbersync-digits-and-standalone-mode.4037257/)

**Payment rings and cards**
- Tapster makes passive-NFC rings, bracelets and keychains for contactless payment, with no battery, card or phone needed. Trustpilot reviews are generally positive. — [Tapster](https://gotapster.com/); [Trustpilot](https://www.trustpilot.com/review/gotapster.com)
- McLear RingPay links a debit or credit card and offers cash back. McLear made the first NFC door-unlock ring. — [Newbega NFC ring comparison](https://www.newbega.com/news/nfc-ring) (older / unverified)
- Kerv (K-Ring) launched in 2017 as a wearable prepaid Mastercard. Coverage is 2017–2019 and mostly UK-oriented, so current US availability is unverified. — [Tech for Travel](https://techfortravel.co.uk/tech-review-k-ring/); [Smart Ring News](https://www.smartringnews.com/posts/top-5-contactless-payment-rings-features-specs-pricing-comparison)

### Inferences
- The easiest hybrid for a US user, all unverified in a fetched source:
  - a dumbphone for calls and SMS, which may lose iMessage/RCS group-chat features;
  - a contactless physical card or passive payment ring for payments;
  - an old iPhone or iPad kept at home on Wi-Fi for banking apps, app-based 2FA and ride-hailing.
- A cellular Apple Watch paired to an iPhone in a drawer keeps full Apple Pay (not the Family Setup version). This is widely reported, but it keeps the iPhone "in the system", and some watch features still route through it.
- Number sharing is designed around a smartphone as the primary line. Putting a dumbphone and a watch on one number is officially documented only for DIGITS, historically. Otherwise it needs a separate watch number or call forwarding.

### Gaps
- I couldn't confirm the 2026 state of Galaxy Watch standalone use with Samsung Wallet, Garmin Pay or Fitbit Pay, or Curve, and whether Wear OS watches can still be set up and then run with the phone permanently removed.
- The dumbphone-plus-watch-on-one-number configuration for each carrier in 2026 is unverified: the Verizon FAQ fetch was blocked.
- Whether Apple's Family Setup allows an adult (18+) as the "family member" with limited features: the Apple doc wasn't fetched.

## 4. Handling 2FA, banking, rideshare and maps without a smartphone

### Takeaway
Most essentials have workarounds, but ride-hailing and app-only banking remain the most common friction points. The typical hybrid answer is a Wi-Fi-only "drawer device" plus hardware or desktop 2FA.

### Cited Findings
- Watch-based TOTP apps (for example, 2FA Hub) can move 2FA codes to a smartwatch so the iPhone isn't needed to generate tokens. — [2FA Hub App Store](https://apps.apple.com/cd/app/2fa-hub/id1538363570); [Wikipedia: Comparison of OTP applications](https://en.wikipedia.org/wiki/Comparison_of_OTP_applications)
- Light Phone users note you can't use Uber with a dumbphone and adapt with alternatives like buses. — search summary of [AOL "ditched iPhone" piece](https://www.aol.com/news/ditched-iphone-used-dumb-phone-155645304.html) / [Techpod](https://www.itechpod.com/blogs/techpod-blog/dumbphone-phone-light-full-smartphone)
- Some people run a hybrid: an older smartphone for banking, navigation and essential apps, and a dumbphone as the primary device on workdays or weekends. — [Adobotech, Aug 2026](https://www.adobotech.net/2026/08/why-dumbphones-are-making-comeback.html) (low-authority blog)
- A Substack essay on "the dumbphone tax" discusses the costs of these workarounds. — [carmellaguiol Substack](https://carmellaguiol.substack.com/p/the-dumbphone-tax/comments) (not read)

### Inferences
- Hardware security keys (FIDO2/passkeys, e.g., YubiKey) and desktop banking cover most 2FA and banking needs. SMS 2FA works on any dumbphone. Rideshare can be booked via the web (m.uber.com) on a laptop, or by phone in some markets. Treat all of this as unverified: not sourced in this session.

### Gaps
- No fetched sources on Light Phone III directions quality, Uber's web or phone booking in 2026, or banks that require app-only 2FA.

## 5. Evidence on what works

### Takeaway
The strongest causal evidence is Castelo et al. (PNAS Nexus, 2025). Blocking mobile internet for 2 weeks, while keeping calls and texts, roughly halved screen time and improved attention, mental health and well-being. That supports "dumbphone-like" restriction (by device or software) over mild nudges. For software nudges, one sec's field study shows a 57% drop in target-app opens. Evidence on long-term dumbphone adherence and people switching back is anecdotal. The best figure found is a Light Phone customer survey: 78% bought it to cut screen time.

### Cited Findings
- Castelo, Kushlev, Ward, Esterman and Reiner, "Blocking mobile internet on smartphones improves sustained attention, mental health, and subjective well-being", PNAS Nexus 4(2), pgaf017 (Feb 2025). A month-long randomized trial in which 2 weeks of blocked mobile internet improved attention, mental health and subjective well-being. — [PNAS Nexus](https://academic.oup.com/pnasnexus/article/4/2/pgaf017/8016017); [PubMed](https://pubmed.ncbi.nlm.nih.gov/39967678/); [MedicalXpress](https://medicalxpress.com/news/2025-02-smartphones-benefits.html)
- Design: 467 participants (mean age 32) used the Freedom app to block all mobile internet on their phones for 14 days. Calls and texts remained available (this last point is from memory and press coverage; not verified in a fetched source). — [USRTK HealthWire](https://usrtk.org/healthwire/blocking-mobile-phone-internet-may-boost-mood-mental-health-attention/); [Georgetown](https://www.georgetown.edu/news/digital-detox-reduce-screen-time-benefits/) (via search snippet)
- Screen time: among participants who kept the block at least 10 of 14 days, average daily screen time fell from 314 to 161 minutes (about 5.2 hours to 2.7 hours). — search snippet attributed to [USRTK](https://usrtk.org/healthwire/blocking-mobile-phone-internet-may-boost-mood-mental-health-attention/) / [Georgetown](https://www.georgetown.edu/news/digital-detox-reduce-screen-time-benefits/)
- Press coverage: [Healio](https://www.healio.com/news/hematology-oncology/20250409/blocking-internet-on-mobile-devices-improves-wellbeing-mental-health); [News-Medical](https://www.news-medical.net/news/20250219/Blocking-mobile-internet-for-two-weeks-improves-mental-health-and-well-being.aspx); an Advisory Board 2026 briefing on social media detox also cites it: [Advisory Board, Apr 2026](https://www.advisory.com/daily-briefing/2026/04/14/social-media-detox)
- one sec PNAS field study (2023): 280 participants over 6 weeks; 57% fewer target-app opens; 36% of open attempts abandoned; 37% fewer attempts versus week 1. — [PNAS](https://www.pnas.org/doi/10.1073/pnas.2213114120)
- Light Phone survey: 78% of customers bought it to cut screen time. — [CBC News](https://www.cbc.ca/news/business/is-the-flip-phone-back-why-some-people-are-switching-to-dumbphones-1.7236222)
- Motivations for switching include addiction ("not a source of enjoyment anymore"), parents protecting children, cost, durability and battery life. — [CBC News](https://www.cbc.ca/news/business/is-the-flip-phone-back-why-some-people-are-switching-to-dumbphones-1.7236222)

### Inferences
- Castelo et al. is effectively a trial of a software-enforced "dumbphone" on the user's own smartphone. It suggests removing mobile internet, not the smartphone form factor or a keyboard, drives the benefit, so a strictly locked smartphone can plausibly reproduce much of the effect.
- Compliance is the weak point: the screen-time result is reported for the subgroup who kept the block at least 10 of 14 days, which implies many didn't. That argues for high-friction tokens (Brick-style) or a physically separate device for people with weak self-control.

### Gaps
- Exact figures from memory and not verified this session: the Castelo compliance rate (I recall about 25% kept the full two weeks), the "91% improved on at least one outcome" figure, and the comparison of effect sizes to antidepressants. The full text was blocked.
- No rigorous survey was found on the share of dumbphone switchers who return to smartphones, or why. The Quora and press pieces returned are anecdotal. r/dumbphones threads weren't fetched.
- No studies isolating grayscale, Focus modes or minimalist launchers were verified this session.
