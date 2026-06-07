# UI Final Punch List

## Purpose

This document converts the latest screenshot review into a short final pass checklist.

It answers:

- which original requirements are fully met
- which are only partially met
- why the remaining gaps still matter
- what must be true before the UI can be considered complete

Review basis:

- [main_menu.png](/H:/WebHatchery/RustGames/monsterhall/tmp_screens/captures_2026_04_12/phase5_main_menu/main_menu.png)
- [town_overview.png](/H:/WebHatchery/RustGames/monsterhall/tmp_screens/captures_2026_04_12/final_review_town_seq/town_overview.png)
- [town_management.png](/H:/WebHatchery/RustGames/monsterhall/tmp_screens/captures_2026_04_12/final_review_town_seq/town_management.png)
- [guild_jobs.png](/H:/WebHatchery/RustGames/monsterhall/tmp_screens/captures_2026_04_12/final_review_brothel_seq/guild_jobs.png)
- [guest_management.png](/H:/WebHatchery/RustGames/monsterhall/tmp_screens/captures_2026_04_12/final_review_guest_seq/guest_management.png)
- [chamber_management.png](/H:/WebHatchery/RustGames/monsterhall/tmp_screens/captures_2026_04_12/final_review_chamber_seq/chamber_management.png)
- [expedition_planning.png](/H:/WebHatchery/RustGames/monsterhall/tmp_screens/captures_2026_04_12/final_review_expedition_seq/expedition_planning.png)
- [monster_profile.png](/H:/WebHatchery/RustGames/monsterhall/tmp_screens/captures_2026_04_12/final_review_profile_seq/monster_profile.png)

## Verdict

The UI now meets the structural direction of the original requirements, but it does not yet fully satisfy every finish condition.

Current state:

- strong improvement in screen hierarchy
- strong improvement in navigation consistency
- real reduction in dead-end flows
- remaining issues in feedback strength, semantic color discipline, and final polish

The remaining work is a finish pass, not another major redesign.

## Master Checklist

### Requirement 1: Show Only What Helps The Current Decision

- [x] `Main Menu` now has one main focal block.
- [x] `Town Overview` now emphasizes `Today's Priority`.
- [x] `GuildJobs Management` now emphasizes the selected room decision.
- [x] `Guest Management` now emphasizes the selected request.
- [x] `Town Management` now has a clearer dominant decision panel and direct next-step guidance.
- [x] `Chamber Management` now presents a cleaner single-outcome hatch decision in simple egg states.
- [ ] `Monster Profile` still shows more structure than the current decision actually needs.

Done means:

- each screen has one dominant question
- one dominant panel
- one dominant action
- support information is visibly secondary

### Requirement 2: Responsive And Predictable

- [x] shared top utility bar is consistent
- [x] shared footer action bar is consistent
- [x] selected states now follow the same visual grammar more often
- [x] major management screens use similar layout logic
- [ ] profile and remaining non-management screens still need the exact same polish standard as the core management screens

Done means:

- screens use the same interaction patterns
- button hierarchy is stable everywhere
- selected, inactive, and utility states read the same way across the whole UI

### Requirement 3: Limit Dead Ends

- [x] footer shortcuts now reduce backtracking between management screens
- [x] most gameplay screens now offer lateral movement instead of forcing repeated back-outs
- [ ] some screens still need stronger contextual `take me there` affordances tied to the selected decision

Done means:

- any screen with a related next action has a direct route to it
- the player rarely needs to back out through multiple screens to continue the flow

### Requirement 4: Feedback

- [x] selected rows, cards, and panels now react visually more clearly
- [x] status strips are more visible than before
- [ ] action confirmation is still too subtle in many places
- [ ] static screenshots suggest interaction acceptance still relies too much on small text status updates
- [ ] local confirmation near the affected control is still missing in several flows

Done means:

- every meaningful click causes an obvious nearby response
- hover/down/selected/accepted states are visible
- failure states appear close to the failed action
- the UI never feels frozen after input

### Requirement 5: Clear Colors

- [x] green, warning, and danger colors now do real work in several screens
- [x] selected and positive states are more legible than before
- [ ] semantic color usage is still inconsistent
- [ ] some important states still read as generic blue or neutral chrome instead of meaning

Done means:

- green consistently means good/ready/valid
- red consistently means danger/failure/blocking
- warning consistently means caution or pending pressure
- neutral chrome never competes with semantic state color

### Requirement 6: Clarity Over Cleverness

- [x] the UI is clearer than the earlier prototype-style version
- [x] management screens are less box-heavy and more decision-led
- [ ] some screens still compress too much information into small text areas
- [ ] some remaining placeholder-grade art still lowers the sense of finish

Done means:

- no screen requires line-by-line interpretation to understand the next move
- copy is product-facing, not implementation-facing
- comparison screens rely on strong structure first and prose second

### Global Finish Rule: No Overlapping Text Or UI Elements

- [ ] no text should clip into adjacent panels
- [ ] no labels should collide with neighboring values
- [ ] no buttons should overlap borders, badges, or thumbnails
- [ ] no panel header should intersect body content
- [ ] no footer content should collide with the action label band
- [ ] long strings must truncate, wrap cleanly, or shrink before collision occurs

Done means:

- every captured screen is readable at a glance
- no text overlaps any border, panel, card, badge, thumbnail, or button
- no values spill outside their intended box

## Screen Punch List

## Main Menu

Status:

- mostly complete

What is working:

- one focal block
- `New Campaign` is clearly dominant
- copy now sells the game loop instead of talking about build state

What still needs work:

- key art is still placeholder-grade
- the right-side art block still feels temporary relative to the rest of the screen

Checklist:

- [x] no MVP/dev-build wording
- [x] one focal composition
- [x] dominant start action
- [ ] title art block feels finished, not placeholder
- [x] no overlapping text or controls in the current capture

## Town Overview

Status:

- strong

What is working:

- clear dominant priority panel
- compact snapshot
- roster is easier to parse
- footer gives direct navigation

What still needs work:

- roster cards are still text-dense in the lower half
- some metric labels are visually close in weight to the true priority

Checklist:

- [x] answers `what should I do today?`
- [x] primary action is visible
- [x] dead ends reduced
- [x] semantic color is helping
- [x] no overlapping text or controls in the current capture

## Town Management

Status:

- improved and close

What is working:

- selected building is now more prominent
- cost, effects, unlocks, and milestones are separated
- next destination affordance exists

What still needs work:

- still reads as several adjacent admin blocks
- unlocks area can feel empty and low-signal
- selected-building panel does not yet strongly answer `build this next because...`

Checklist:

- [x] affordability is visible
- [x] next destination affordance exists
- [x] one dominant decision outweighs most support panels
- [ ] unlocks still need more meaning when a building has no major unlock payload
- [x] no overlapping text or controls in the current capture

## GuildJobs Management

Status:

- strong

What is working:

- room selection is stable
- selected room is clearly dominant
- assigned and available workers are split cleanly
- footer shortcuts are useful

What still needs work:

- worker cards could still show outcome/readiness a bit more strongly than identity text

Checklist:

- [x] answers `who should work where?`
- [x] weight hierarchy is clear
- [x] dead ends reduced
- [x] no overlapping text or controls in the current capture

## Guest Management

Status:

- strong, with minor finish issues

What is working:

- selected request is clearly the primary decision
- candidate cards are easier to compare
- blocked versus eligible states are visible

What still needs work:

- some candidate detail text is very compressed
- failure reasons can still become cramped in narrow cases

Checklist:

- [x] answers `who fits this request?`
- [x] selected request is dominant
- [x] semantic states are visible
- [ ] candidate failure detail should breathe more cleanly
- [x] no overlapping text or controls in the current capture

## Chamber Management

Status:

- improved and close

What is working:

- left inventory plus right decision panel is a better structure
- selected egg is clear
- hatching action is visible

What still needs work:

- too much unused space in simple egg states
- selected panel still feels inspection-heavy
- duplicate hatch actions create some ambiguity

Checklist:

- [x] selected egg is clear
- [x] inventory is easier to scan
- [x] selected panel now reads more like a hatch decision
- [x] duplicate action presentation was reduced in the simple egg case
- [x] no overlapping text or controls in the current capture

## Expedition Planning

Status:

- improved but still not finished

What is working:

- team area is now clearly important
- rewards and risk are visible
- floor selection is compact

What still needs work:

- mission, priority, and risk tiles are still too similar in weight
- the priority choice is visible, but not yet cleanly grouped as one compact control

Checklist:

- [x] answers `who goes and how risky is it?`
- [x] team assignment is prominent
- [x] risk is visible
- [x] priority grouping now reads faster than the previous flat row
- [x] no overlapping text or controls in the current capture

## Monster Profile

Status:

- not finished

What is working:

- readiness and best-next-use are more visible than before
- footer shortcuts are consistent

What still needs work:

- subtitle still contains placeholder/future-facing wording
- this screen still feels like a dossier instead of a sharp decision surface
- too many medium-importance boxes remain

Checklist:

- [x] readiness is visible
- [x] best-next-use is surfaced
- [x] placeholder/future-facing copy was removed
- [ ] panel count and hierarchy still need one more simplification pass
- [x] no overlapping text or controls in the current capture

## Final Required Pass

These items should be completed before calling the UI done:

- [ ] remove all placeholder, future-facing, or implementation-facing copy
- [x] tighten `Town Management` so one next-build decision clearly dominates
- [x] tighten `Chamber Management` so the selected state feels action-led, not report-led
- [x] tighten `Expedition Planning` priority grouping and reduce equal-weight controls
- [ ] simplify `Monster Profile` further so readiness and next-use matter more than dossier layout
- [ ] strengthen local action feedback near the clicked control
- [ ] enforce semantic color consistently across all screens
- [ ] run one final screenshot sweep and verify no overlapping text or UI elements on every captured screen

## Release Bar

The UI can be considered complete when:

- every original requirement is fully checked
- no captured screen shows overlapping text or UI elements
- no visible copy sounds like placeholder, MVP, or future-work commentary
- the player can tell the next action in under a second on each primary screen
