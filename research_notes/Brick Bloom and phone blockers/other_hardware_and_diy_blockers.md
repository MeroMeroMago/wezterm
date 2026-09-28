# Other Hardware and DIY Phone Blockers (not Brick or Bloom), as of 2026-09-28

Research method note: direct page fetches were blocked by the network proxy for most sites (unhookd.app, whatifididnt.com and techcrunch.com all returned EGRESS_BLOCKED). Almost every finding below comes from WebSearch result snippets, attributed to the page the snippet came from. GitHub repo metadata (stars, last-updated dates) came from the GitHub search API. Many snippets come from competitors' marketing blogs (blok.so, foqos.app, habitdoom.com, autonomous.ai, getblocc.com, whatifididnt.com with affiliate codes), so treat their comparisons as biased. Prices are US$ unless noted, and none were checked on a live checkout page.

## 1. NFC and token blockers: which exist and are current in 2026?

### Takeaway
The paid "tap a token" market has grown well past Brick and Bloom. Current products include:
- **Unpluq**: tag plus a subscription.
- **Blok**: NFC devices plus an app subscription.
- **Autonomous Key**: $9, no subscription, began shipping in August 2026.
- **ScreenZen Halo**: $49 Bluetooth proximity pebble, no subscription.
- **BLOCC**: €39.99 one-time.
- **Norma**: steel disc, iPhone only, no subscription.
- **Unbranded Amazon devices**, such as Fruaros.

Several apps (Jomo, Offtime, focusLock, AppToken, DetoxTap) let you use your own NFC tag. I found no current product called "Tapuz", "Ground", "Unbrick", "Lock-in", "Stay Focused Key", "Focus Dock" or "Hold", and no Opal hardware.

### Cited Findings

**Unpluq (NL company; iOS and Android)**
- The tag costs $26.50 and is sold without a subscription. Premium subscriptions are listed "from $29.00". A "Tag + Premium Subscription" bundle and a Family plan are also sold — [Unpluq store](https://www.unpluq.com/collections/all); [Unpluq Tag only](https://www.unpluq.com/products/unpluq-tag-only); [Premium Subscription](https://www.unpluq.com/products/premium-subscription); [Unpluq Family](https://www.unpluq.com/pages/unpluq-family-subscription)
- Whatifididnt says Premium costs $5.33/month (about $64/year) and that "you can't use the tag without a subscription". It is also the source of the WHATIFIDIDNT 20% discount code, so it has an affiliate relationship — [whatifididnt Unpluq review](https://whatifididnt.com/blog/unpluq-review/); [Unpluq vs Brick](https://whatifididnt.com/blog/unpluq-vs-brick/)
- Other sources describe it as "about $79 for the tag with the first year included". A Vice article cites a $35/yr Unpluq+ subscription with the first year included in a $79 tag price, which looks like older pricing — [Habit Doom](https://habitdoom.com/blog/brick-alternatives-iphone); [Vice](https://www.vice.com/en/article/unpluq-tag-dumb-capabilities-smartphone/)
- **Price conflict**: sources give the year-2+ subscription as $29, $35 or $64 per year. The store page's "from $29" may be a promotional or entry tier. The tag does not work without an active plan (whatifididnt).
- Unlock challenges include tapping patterns, shaking the phone, scrolling a long page, scanning the NFC tag, steps (Android only) or a QR scan. Paid plans can lock up to 49 apps with unlimited schedules — [whatifididnt Unpluq review](https://whatifididnt.com/blog/unpluq-review/)
- MIT's student paper reviewed it in October 2025 — [The Tech](https://thetech.com/2025/10/16/unpluq-review)

**Blok (blok.so; iPhone, uses the Screen Time API)**
- Blok sells an NFC card, keychain or magnet. It markets anti-deletion protection, an emergency unblock system, social accountability and "Key Intelligence" logging of unlocks — [Blok best app blockers](https://www.blok.so/resources/best-app-blockers-2026); [Blok iPhone list](https://www.blok.so/resources/best-app-blockers-for-iphone-in-2026)
- Pricing is $59.99/yr or $9.99/mo for the app. NFC devices start at $29 and are sold separately — [Blok vs Opal / Blok resources](https://www.blok.so/resources/blok-vs-opal-screen-time-app). TechCrunch-derived coverage also gives "Blok ($29)" — [Yahoo/TechCrunch](https://tech.yahoo.com/home/articles/9-key-physically-locks-most-152537717.html)
- Blok writes its own SEO comparisons (for example "Blok vs Foqos", "Why you can't fight software with software"), so it is a biased source — [Blok vs Foqos](https://www.blok.so/resources/blok-vs-foqos-which-app-blocker-actually-works)

**Autonomous Key (Autonomous Labs; iOS 15+ and Android 8+)**
- Costs $9 with no subscription or premium tier. One key pairs with multiple phones. About 3 inches long, with no battery — [TechCrunch, 2026-08-01](https://techcrunch.com/2026/08/01/this-9-key-physically-locks-your-most-addictive-apps/); [Yahoo syndication](https://tech.yahoo.com/home/articles/9-key-physically-locks-most-152537717.html)
- Funded on Kickstarter. It was "in beta" and began shipping in August 2026 — [Kickstarter](https://www.kickstarter.com/projects/autonomousai/autonomous-key/); [TechCrunch](https://techcrunch.com/2026/08/01/this-9-key-physically-locks-your-most-addictive-apps/)
- Strictness model: apps are locked by default with no break feature. Each tap opens a 60-minute window, then apps re-lock automatically — [Autonomous blog, Brick vs Bloom](https://www.autonomous.ai/ourblog/brick-vs-bloom) (vendor source). There is also an independent review — [Digital Reviews Network](https://www.digitalreviews.net/reviews/mobile/autonomous-key-review/)

**ScreenZen Halo (Bluetooth, not NFC; iOS, Android and tablets)**
- A $49 Bluetooth pebble that blocks chosen apps whenever the phone is within a set radius (up to about 50 ft), for example the bedroom. The ScreenZen app is free with no subscription, and one Halo works with unlimited devices. It uses replaceable batteries that last "a couple of years" — [ScreenZen Halo product page](https://screenzen.co/products/halo); [Apartment Therapy review](https://www.apartmenttherapy.com/halo-screenzen-review-37554918); [Amazon listing](https://www.amazon.com/ScreenZen-Halo-by/dp/B0FVM77699); [whatifididnt Halo review](https://whatifididnt.com/blog/screenzen-halo/)
- Apartment Therapy found it "worked well most of the time". The block sometimes activated with a delay, and updates improved this — [Apartment Therapy](https://www.apartmenttherapy.com/halo-screenzen-review-37554918)
- **Answer to "Halo"**: the Halo that exists in 2026 is ScreenZen's proximity device, not an NFC tap token.

**BLOCC (getblocc.com; iPhone)**
- An NFC tag at €39.99 one-time, with no subscription and lifetime premium for 3 devices. Features include gamification (tower building, 50+ achievements) and charity donations tied to focus time — [BLOCC site](https://www.getblocc.com/); [BLOCC vs Brick](https://www.getblocc.com/blog/blocc-vs-brick.html). It has a Trustpilot page — [Trustpilot](https://www.trustpilot.com/review/getblocc.com)
- I found no US-dollar price.

**Norma (nor.ma; iPhone only)**
- A 70 mm stainless steel NFC disc that blocks apps and silences their notifications. No subscription and no battery. The app requires the physical disc. An optional weighted magnetic stand costs €20. The disc's own price did not appear in snippets — [Norma](https://nor.ma/); [Norma shop](https://www.nor.ma/shop); [App Store](https://apps.apple.com/us/app/norma-focus/id6757099829); [curated.supply listing](https://www.curated.supply/products/unit-disc)

**Amazon "Brick-style" devices**
- Fruaros NFC Phone Blocker ("Moonstone Grey") is sold on Amazon. You set a focus mode for chosen apps and websites, then tap the device to start or stop. It does not block signals and is not a lockbox. Price not visible in the snippet — [Amazon Fruaros](https://www.amazon.com/Fruaros-Physical-Distractions-Routines-Moonstone/dp/B0H69TJPST). Amazon has a "brick phone blocker" search category — [Amazon search](https://www.amazon.com/brick-phone-blocker/s?k=brick+phone+blocker)

**Apps that use your own NFC tag (tag is BYO or optional)**
- **Jomo** (iOS): can block or unblock apps with an NFC tag, but only NTAG215 tags work. Also integrates with Siri Shortcuts — [Jomo Help Center](https://help.jomo.so/en/article/how-to-unblock-block-apps-with-a-nfc-tag-ng6zur/); [Jomo Shortcuts help](https://help.jomo.so/en/article/how-to-use-jomo-with-siri-shortcuts-17p3ehp/); [App Store](https://apps.apple.com/us/app/jomo-screen-time-blocker/id1609960918)
- **offtime** (useofftime.com): you put a QR or NFC "physical key" out of reach. The site says there is "no bypass until you scan your physical key". Pricing not found — [offtime](https://www.useofftime.com/). This is a different product from the older Android app "OFFTIME". I could not confirm whether that app is still active.
- **focusLock** (iOS): any NFC tag **or your Apple Watch** can be the key — [App Store focusLock](https://apps.apple.com/us/app/focuslock-your-key-to-focus/id6504141777)
- **AppToken** (iOS; bills itself as "the first screen time app that uses a physical NFC tag as your key") — [App Store](https://apps.apple.com/uy/app/apptoken/id6746128005). **DetoxTap** (an NFC card unlocks blocked apps) — [App Store](https://apps.apple.com/us/app/-/id6757342619). **VantaFocus NFC** (iOS) — [App Store](https://apps.apple.com/py/app/app-blocker-vantafocus-nfc/id6742754205). **KAIROS** — [App Store](https://apps.apple.com/app/id6748263354). **BrainKey** — [App Store](https://apps.apple.com/mx/app/brainkey-focus-block-apps/id6758040354). **Dodier NFC** — [App Store](https://apps.apple.com/mx/app/dodier-nfc/id6746334723). Pricing and ratings for these smaller apps were not found.

**Opal**
- Opal is software-only in 2026 (no NFC hardware found), at about $100/yr. Its "Deep Focus" sessions are described as very hard to bypass — [Blok Opal review](https://www.blok.so/resources/opal-app-review-is-it-worth-100-year-for-screen-time-management); [FaithLock Brick vs Opal](https://www.getfaithlock.com/resources/brick-vs-opal); [autonomous.ai physical blocker explainer](https://www.autonomous.ai/ourblog/physical-app-blocker)

**Price range overview**
- "Physical app blockers range from free apps that use a sub-dollar NFC tag up to finished devices at $99, with a couple of subscription options around $60 a year" — [autonomous.ai explainer](https://www.autonomous.ai/ourblog/physical-app-blocker). The $99 device was not identified in the snippet.

### Inferences
- The most competitive new entrant is **Autonomous Key**: $9, no subscription, iOS and Android, and locked by default with 60-minute windows. That makes it structurally stricter than Brick-style toggles, but it only began shipping in August 2026, so there is little long-term user feedback yet.
- **Unpluq and Blok are the only notable subscription-gated token products.** Their lifetime cost is well above one-time options:
  - Unpluq: about $26.50 + $64/yr, if the whatifididnt figure is right.
  - Blok: $29+ device + $59.99/yr.
- **Halo is a different design**: it blocks by proximity (e.g., "no scrolling in bed") rather than requiring a tap to unlock, so it suits location-based habits better than all-day blocking.

### Gaps
- I found **no evidence that products called "Tapuz", "Ground", "Unbrick", "Lock-in", "Stay Focused Key", "Focus Dock" or "Hold" exist** as phone-blocking hardware in 2026. Searches returned nothing relevant. "Hold" exists as a student reward app, but I did not confirm it in this session. Treat these names as unverified or non-existent.
- The exact Unpluq renewal price is unresolved ($29 vs $35 vs $64 per year). So are the prices of the Norma disc, the Fruaros device and the offtime app.
- I found no AliExpress-specific "Brick clone" listings in the snippets.
- Aside from Foqos, I found no App Store rating counts for the smaller NFC apps.

## 2. Physical lock-away devices (timed lockboxes, Yondr, Aro)

### Takeaway
Physical containment is the most bypass-resistant category. **kSafe** ($59 Mini/Medium, $69 XL; timer from 1 minute to 10 days; no overrides) and **Mindsight** (from $39.95; "Fortress Mode" with email-only override) are the main timed lockboxes. Yondr sells a **Home Tray** (about $215–$249) for consumers; its pouches are mainly an institutional product. **Aro** is a charging box plus a membership (about $9.99/mo) with **no physical lock**, so it is the least strict.

### Cited Findings
- **kSafe / Kitchen Safe**:
  - The timer runs from 1 minute to 10 days and "remains locked until the timer reaches zero. No overrides!" — [kSafe site](https://www.thekitchensafe.com/); [kSafe FAQ](https://www.thekitchensafe.com/pages/faq)
  - Mini fits phones up to 5.8" (iPhone SE / Galaxy S10); Medium fits larger phones — [kSafe site](https://www.thekitchensafe.com/)
  - Mini and Medium cost $59 each; XL (fits an iPad Mini) costs $69 — [Mashed](https://www.mashed.com/1071329/heres-what-happened-to-kitchen-safe-after-shark-tank/)
  - Brainstamped calls it "cheaper than any app blocker subscription and far harder to bypass" and cites "$50 one-time" — [Brainstamped 2026 review](https://brainstamped.com/digital-wellness/guides/kitchen-safe-phone-lock-box-review-2026/). A 4-week test review also exists — [lockboxtimer.com](https://lockboxtimer.com/ksafe-lock-box-review/)
  - **Caveat**: modern phones such as iPhone 15/16/17 Pro Max may not fit the Mini. Size checks needed.
- **Mindsight Timed Lock Box**:
  - From $39.95. Timer from 1 minute to 30 days — [Mindsight product](https://mindsightnow.com/products/timed-lock-box); [whatifididnt lock box guide](https://whatifididnt.com/blog/phone-lock-box/)
  - Three modes:
    - Simple: a basic lockbox.
    - Standard: a countdown plus an unlock code.
    - Fortress: a countdown with no code; for emergencies you email support for an override — [Mindsight FAQ](https://mindsightnow.com/pages/faq); [Amazon](https://www.amazon.com/Mindsight-Unplug-Cravings-Willpower-Wellness/dp/B0DD54LLRG)
- **Vaydeer metal time lock box**: 2.6 L metal container with a USB-C charging pass-through — [whatifididnt lock box guide](https://whatifididnt.com/blog/phone-lock-box/). Price not captured.
- **Yondr**:
  - Pouches are magnetic locking bags, mainly for schools and venues — [Wikipedia](https://en.wikipedia.org/wiki/Yondr); [Yondr how it works](https://www.overyondr.com/phone-locking-pouch)
  - Used pouches sell on eBay — [eBay listing](https://www.ebay.com/itm/277704340516)
  - An Alibaba buying guide claims pouches are sold outright at $30–$35 each for replacements — [Alibaba guide](https://electronics.alibaba.com/buyingguides/yondr-pouch-guide-how-to-choose-use-effectively). This is a low-quality aggregator source.
  - **Caveat**: pouches need a magnetic unlocking base, so a pouch alone gives an individual little self-control value.
- **Yondr Home Tray** (consumer product):
  - A lockable box lined with Faraday fabric that blocks 5G, Wi-Fi, Bluetooth, GPS and cell signal, with a charging pass-through — [Yondr At Home](https://www.overyondr.com/home-tray); [Global Day of Unplugging](https://www.globaldayofunplugging.org/products-we-love-collection/yondr)
  - Snippets give prices of $215 and $249 — [Pittco Management listing](https://www.pittcomanagement.com/products/phone-lock-box-yondr-home-tray-lockable-signal-blocking-phone-lock-box/13478304/)
  - whatifididnt says it does *not* block signal and has 11 cable holes — [whatifididnt lock box guide](https://whatifididnt.com/blog/phone-lock-box/). **This conflicts** with Yondr's own signal-blocking description; the two may describe different versions.
- **Aro (goaro.com, by Reclaimwell / Aro Technologies, Knoxville)**:
  - A smart box that holds and charges up to 4 phones, plus an app that tracks phone-free minutes and streaks. One membership covers the household — [Aro membership](https://www.goaro.com/membership); [Sleep is a Skill store](https://www.sleepisaskill.com/store/aro-smart-box)
  - Membership is "less than $9/month" per family. One bundle gives 6 months free, then $9.99/month billed annually — [NBC4i press release](https://www.nbc4i.com/business/press-releases/ein-presswire/737156553/aro-launches-new-app-to-help-families-reduce-screen-time/). Aro later added a standalone app — [Teknovation](https://www.teknovation.biz/free-standing-app-adds-to-product-mix-for-knoxvilles-aro-technologies/). The box price was not found.
  - Criticism: a large upfront cost plus a mandatory subscription, and "you can retrieve your phone anytime since there is no physical lock" — [Mindsight Aro alternative](https://mindsightnow.com/blogs/mindful-matters/aro-box-alternative-the-best-screen-time-solution-for-real-digital-balance) (competitor source); [Medium, "the Bluetooth smart box that no one asked for"](https://medium.com/@westwise/aro-the-bluetooth-smart-box-that-no-one-asked-for-a74eaccb0989); [The Analog Family Substack](https://katherinemartinko.substack.com/p/a-fancy-phone-box-to-help-you-unplug/comments)

### Inferences
- A timed lockbox is the only option in this whole landscape that software cannot bypass.
- Its downsides: the phone is unusable for everything, including calls, maps and 2FA, and you must check that your phone fits.
- kSafe (no override at all) and Mindsight Fortress Mode are the strictest. Aro works as an honor system with gamification.

### Gaps
- I found no current Aro box price and no confirmation that Aro is still actively selling in late 2026.
- I could not verify whether Yondr sells pouches directly to individuals, or at what price, from Yondr itself.

## 3. DIY and free methods (iOS and Android), including open-source Brick clones

### Takeaway
The standout free Brick replacement on iPhone is **Foqos**:
- open-source (MIT-style GitHub project by awaseem, about 842 stars, actively updated as of 2026-09-28)
- rated 4.9 on the App Store
- works with any NTAG213 sticker or a printed QR code

On Android, free options include:
- **TapBlok** (open source)
- **nfcGuard** (open source)
- **Lock – NFC App Blocker** (Android 14+)
- Foqos Android ports

Plain iOS Shortcuts NFC automations can switch Focus modes, but they cannot block apps. Samsung Routines can be triggered by NFC but cannot turn on app-blocking Modes.

### Cited Findings

**Foqos (iOS)**
- Free, open source, no account, no ads, no subscription or in-app purchases. Uses Apple's Screen Time API. Blocking profiles can be started by NFC tag, QR code or barcode, timer, Shortcuts or schedule. Blocks apps and websites. Data stays on the device — [foqos.app](https://www.foqos.app/); [GitHub awaseem/foqos](https://github.com/awaseem/foqos)
- It works with standard NTAG213 tags and offers a printable 3D "NFC brick" and keychain design for 25 mm tags — [foqos.app](https://www.foqos.app/); [Foqos vs Brick](https://www.foqos.app/brick-app-alternative.html)
- GitHub: about 842 stars and 150 forks. Created 2024-10-08. Last updated 2026-09-28, so it is **active** — GitHub API search, [awaseem/foqos](https://github.com/awaseem/foqos)
- App Store: 4.9/5 from about 814 ratings, with 198 reviews on AppsHunter. MWM lists "100k+ downloads" — [App Store reviews](https://apps.apple.com/us/app/foqos-tap-to-block/id6736793117?see-all=reviews&platform=iphone); [AppsHunter](https://appshunter.io/ios/app/6736793117/reviews); [MWM](https://mwm.ai/apps/foqos-tap-to-block/6736793117)
- Praise: "absolutely premium without a price", schedules, the QR option. Complaints: occasional bugs and crashes, and the calendar automation sometimes shuts off — [App Store reviews](https://apps.apple.com/us/app/foqos-tap-to-block/id6736793117?see-all=reviews&platform=iphone)
- Press: "This $9 Alternative to the Brick" (Foqos plus a 20-pack of NFC tags for $8.99 on Amazon) — [Dorm Therapy](https://www.dormtherapy.com/foqos-app-diy-brick-phone-lock-100012155); [Yahoo Lifestyle](https://www.yahoo.com/lifestyle/articles/didn-t-want-splurge-brick-220000242.html); [Everyday Reading, "A Free Way to DIY a Phone Brick"](https://everyday-reading.com/a-free-way-to-diy-a-phone-brick/)
- Ecosystem: "Family Foqos" (3 emergency unblocks per reset period, configurable from 2–8 weeks) — [family-foqos.app](https://family-foqos.app/). There is also an ESP32 desk companion, a Mac fork and an ActivityWatch importer — GitHub search (rachelworld/foqos-companion, milomaurer23/foqos-for-mac, RTnhN/aw-importer-foqos)
- **Android ports** of Foqos by third parties exist: nish261/foqos-android (updated 2026-09-06), Elias02345/Foqos-Android (updated 2026-09-28) and KatyBlumer/FoqosAndroid. Each has about 3 stars, so they are early-stage — GitHub API search
- The Elias02345 port has a "strict mode" that blocks Settings and the uninstaller, plus a 3-tap "break glass" emergency unblock. It admits this is "a best-effort deterrent… A sufficiently motivated user can always reach a recovery path (Safe Mode, ADB, factory reset, disabling the accessibility service)" — [GitHub Elias02345/Foqos-Android](https://github.com/Elias02345/Foqos-Android)
- Competitor critique exists: [Blok vs Foqos](https://www.blok.so/resources/blok-vs-foqos-which-app-blocker-actually-works) (biased source)

**Android free and open-source NFC blockers**
- **TapBlok**: free and open source, no IAP or tracking. An NFC tag or QR code starts or ends a session. Offers opt-in broadcast actions so Tasker, MacroDroid or Samsung Routines can start and stop sessions. GitHub cajdata/TapBlok has about 41 stars and was updated 2026-09-20 — [tapblok.com](https://tapblok.com/); [GitHub](https://github.com/cajdata/TapBlok/). It was posted to Hacker News as "Show HN: I made a app that uses NFC as a physical switch" — [HN](https://news.ycombinator.com/item?id=42782295). (The HN post may be a different app; not verified.)
- **nfcGuard – NFC Focus Lock**: on Google Play, with schedules. GitHub Andebugulin/nfcGuard has about 20 stars and was updated 2026-09-27 — [Google Play](https://play.google.com/store/apps/details?id=com.andebugulin.nfcguard); GitHub API search
- **Lock – NFC App Blocker** (com.nathanb.lock):
  - Free, described as open source, with no analytics, account, ads or internet connection.
  - Pairs up to 5 tags and supports configurable grace periods, emergency unlocks and stats. Requires Android 14+ — [Google Play](https://play.google.com/store/apps/details?id=com.nathanb.lock&hl=en)
- Smaller projects include tagout, FocusPocus and nfcAppBlocker — [GitHub tagout](https://github.com/Diamondlight8/tagout); [GitHub FocusPocus](https://github.com/cdwilliams40/FocusPocus)

**iOS DIY without a dedicated blocker app**
- Shortcuts has a personal automation for NFC: Automation → + → NFC → scan the tag, then add actions such as Set Focus — [NFCore guide](https://nfcore.app/guides/ios-shortcuts-nfc-automation); [Apple Shortcuts guide](https://support.apple.com/en-tj/guide/shortcuts/apde31e9638b/ios)
- DIY write-ups:
  - "Making My Own NFC 'Focus Brick' to Dumb Down my iPhone" — [Jacob Desforges](https://jacobdesforges.com/nfc-focus-brick/)
  - The "Bricked" Shortcut, which can be triggered by an NFC scan or a Focus — [heliomass.com](https://heliomass.com/posts/108-bricked---a-shortcut-for-reducing-distractions/)
- **Screen Time with a passcode held by a trusted person**:
  - The known weakness is that "all you need is a passcode to bypass Screen Time" — [BGR](https://www.bgr.com/2235201/phone-blocking-app-brick-alternatives/)
  - Developers report bypasses via changing the date and time — [Apple Developer Forums](https://developer.apple.com/forums/thread/809318)
  - The Screen Time passcode is not requested when revoking a third-party app's Screen Time permission; in iOS 26.4, Face ID is requested instead — [Apple Dev Forums 758333](https://developer.apple.com/forums/thread/758333?page=2); [Apple Dev Forums 821959](https://developer.apple.com/forums/thread/821959)

**Android built-ins and automation**
- MacroDroid can use an NFC-tag trigger by storing the tag UID — [Droid Rooter NFC recipes](https://www.droidrooter.com/blog/android-nfc-automation-recipes-2026)
- Digital Wellbeing, Focus Mode and Family Link are free built-ins — [Blok Android methods](https://www.blok.so/resources/how-to-block-apps-on-android-7-methods-ranked-by-how-well-they-actually-work)
- **Samsung limitation**: Routines can be triggered by NFC tags, but "Routines can't restrict app usage like modes can" and "Routines also can't turn on a mode". Users have requested this as a feature — [Samsung Community](https://us.community.samsung.com/t5/Samsung-Apps-and-Services/Please-add-the-ability-for-a-Routine-to-activate-a-mode/td-p/2686965); [Android Authority](https://www.androidauthority.com/samsung-routines-nfc-tags-superpowers-how-3694682/)

### Inferences
- **For a US iPhone user, Foqos plus a pack of NTAG213 stickers (about $9 for 20) is a practical free Brick replacement.** It has comparable Screen Time API-based strictness, is actively maintained and is well rated.
- Its bypass resistance is roughly the same as Brick's, because both rely on the same Screen Time API. Neither is lockbox-level.
- The trusted-person-passcode trick is stronger when you use Apple's built-in Screen Time limits. It does *not* protect third-party blockers from being revoked via Settings, based on developer-forum reports.
- On Android, TapBlok, nfcGuard and Lock cover the same use case for free. But Android blockers depend on accessibility services, which a motivated user can disable (Safe Mode, ADB), so they are generally weaker than iOS Screen Time blocking.
- **iOS Shortcuts alone cannot hide or block apps.** It can only switch Focus modes, which filter notifications and home screens. This is inferred from the absence of a block-app action in the sources; Focus is not verified to block app launches. That is why Foqos-type apps are needed.

### Gaps
- I found no Reddit-sourced sentiment on r/nosurf, r/digitalminimalism, r/shortcuts or r/tasker; searches returned only App Store and vendor quotes.
- I found no independent bypass testing of Foqos on iOS (for example, whether deleting the app ends the block, or whether it has a strict or anti-delete mode like Bloom's). I could not confirm this.
- Play Store install counts and ratings for TapBlok, nfcGuard and Lock were not captured.

## 4. Smart ring, watch or Clicks keyboard as the "key"; reviews and bypass comparison

### Takeaway
- **focusLock** (iOS) is the only product I found that uses an **Apple Watch** as the unlock key.
- I found **no product that uses a smart ring or a Clicks keyboard as a blocker key**.
- The Clicks Communicator ($399–$499 Android keyboard phone, shipping Q4 2026) is instead a "distraction-free companion phone" meant to let you leave your main phone elsewhere.

**Bypass-resistance ranking, from sources and inference:**
1. Timed lockbox (kSafe no-override; Mindsight Fortress)
2. Lock-by-default token (Autonomous Key)
3. Brick-style NFC toggles with anti-delete or strict mode (Bloom strict mode, Blok anti-deletion, Foqos)
4. Proximity (Halo)
5. Aro (no lock) and plain Screen Time or Focus DIY

### Cited Findings
- focusLock: "use any NFC tag or your Apple Watch as the key" — [App Store focusLock](https://apps.apple.com/us/app/focuslock-your-key-to-focus/id6504141777)
- **Clicks Communicator**:
  - Unveiled at CES 2026; an Android phone with a physical keyboard.
  - Price is reported as $399 in some sources and $499 in others (possibly early-bird versus retail); due in Q4 2026.
  - Has a programmable "Clicks Key" and a hardware switch for airplane mode.
  - Pitched as a "companion phone" so the "fun and distraction" phone can be left in another room — [Wikipedia](https://en.wikipedia.org/wiki/Clicks_Communicator); [WebProNews ($399)](https://www.webpronews.com/clicks-communicator-399-android-phone-with-physical-keyboard-unveiled-at-ces-2026/); [BigGo ($499)](https://biggo.com/news/202601050821_Clicks-Communicator-Android-Phone-Physical-Keyboard-Launch); [Deep Tech Base](https://deeptechbase.com/clicks-communicator-the-anti-distraction-smartphone/)
- Bloom's strict mode stops you from deleting apps until the time is up. Without it, deleting the blocker app unblocks everything — [BGR](https://www.bgr.com/2235201/phone-blocking-app-brick-alternatives/). This shows that app deletion is the common bypass for iOS NFC blockers.
- Hardware tokens add friction because "software can always be turned off" — [Blok](https://www.blok.so/resources/physical-phone-blocker-why-app-blockers-dont-work) (vendor)
- Android blockers are "best-effort… Safe Mode, ADB, factory reset, disabling the accessibility service" — [Foqos-Android](https://github.com/Elias02345/Foqos-Android)
- Aro has no physical lock — [Mindsight](https://mindsightnow.com/blogs/mindful-matters/aro-box-alternative-the-best-screen-time-solution-for-real-digital-balance); kSafe has no overrides — [kSafe](https://www.thekitchensafe.com/)

### Inferences
- A smart ring would not work as an NFC key for these apps: rings are normally NFC *payment* or emulation devices, not readable NTAG tags. This is not verified.
- Any NFC blocker app that accepts arbitrary tags could in principle accept an NFC sticker stuck to a watch band or ring. This is speculative; no source confirms it.
- The Clicks *iPhone keyboard case* has no blocker-key function in any source found.
- For the report: the practical choice is between two things.
  - Free software friction: Foqos, TapBlok, etc.
  - Paid hardware: Autonomous Key $9, Halo $49, BLOCC about €40, Unpluq and Blok on subscription.
  - The paid hardware mostly buys polish and anti-delete features, not fundamentally stronger enforcement. Only a timed lockbox removes the phone physically.

### Gaps
- I found no source on smart-ring-as-key products (Oura, Ultrahuman, RingConn) or on Clicks-keyboard-as-key products.
- I found no systematic head-to-head bypass test across products. The ranking above is an inference from each product's stated design.
- Reddit sentiment could not be retrieved, since direct fetches were blocked and search did not return Reddit threads.
