# UI Next Pass Mandate

## Purpose

This document turns the latest screenshot review and follow-up critique into a stricter implementation mandate.

The goal of the next pass is not to “polish the current UI.”

The goal is to stop building screens like state-inspection tools and start building them like decision surfaces.

Core rule:

- Every screen gets one dominant question.
- Every screen gets one dominant panel.
- Every screen gets one dominant action.

If an element does not help answer the current screen question, it should be:

- compressed
- demoted
- hidden behind selection
- moved to a different screen

## Diagnosis

The UI still reads as early-stage because the problem is structural, not decorative.

Current failure pattern:

- too many medium-importance boxes
- too many panels speaking at once
- too much text in compare-heavy screens
- weak hierarchy between primary and secondary actions
- inconsistent interaction patterns across screens
- too little local feedback after non-trivial actions
- colors used mostly for mood instead of meaning

This creates a prototype/admin-tool feeling even when the layout is technically functional.

## Design Mandate

The next pass must optimize for decision speed.

This means:

- fewer visible concepts per screen
- stronger hierarchy
- more consistent interaction patterns
- less prose in selection/comparison views
- more semantic color
- stronger local response to user actions

The target feel is:

- immediate
- predictable
- readable
- game-like

Not:

- inspectable
- exhaustive
- box-heavy
- text-heavy

## Non-Negotiable Rules

### 1. One Question Per Screen

Each screen must have one explicit player question.

Approved screen questions:

- `Main Menu`: start or continue?
- `Town Overview`: what should I do today?
- `Town Management`: what is worth building next?
- `GuildJobs Management`: who should work where?
- `Guest Management`: which worker can fulfill this request?
- `Chamber Management`: which egg is ready, and what should I hatch?
- `Expedition Planning`: who goes, and how risky is it?
- `Monster Profile`: what is this character good for right now?

Any content that does not support the screen’s question must be demoted.

### 2. One Dominant Panel Per Screen

Each screen may have:

- one primary decision panel
- one support panel
- one action band
- one optional compact summary

Do not build screens with four equally loud panels.

### 3. One Dominant Action Per Screen

Each screen needs one visually strongest action.

Examples:

- `Main Menu`: `New Campaign`
- `Town Overview`: recommended next destination or `End Day` when relevant
- `Town Management`: `Build Selected`
- `GuildJobs Management`: `Assign`
- `Guest Management`: `Assign to Request`
- `Chamber Management`: `Hatch`
- `Expedition Planning`: `Confirm Team` or equivalent primary commit action

Primary actions must be visually distinct from navigation and utility.

## Visual System

This pass must define and enforce a small shared visual grammar.

### Text Hierarchy

Use only three text sizes for normal screen UI:

- `Display`: screen title and major panel emphasis
- `Section`: panel titles, selected entity names, key stats
- `Body`: standard labels, values, and support detail

Optional:

- `Caption`: only for helper text, hint text, and compact metadata

Rules:

- Do not invent per-screen text scales.
- Headers must feel obviously louder than body text.
- Support text must never visually compete with the primary decision content.

### Panel Tiers

Use only three panel tiers:

- `Tier 1 Primary`
  - strongest border or fill contrast
  - largest title treatment
  - reserved for the main decision area
- `Tier 2 Support`
  - normal panel treatment
  - used for context needed to support the current decision
- `Tier 3 Utility`
  - lightest chrome
  - used for settings, footer utility, minor metadata

Rules:

- Not every box deserves a border.
- If two panels are not equally important, they must not look equally important.

### Semantic Colors

Define six semantic colors and use them consistently:

- `Primary`: selected neutral action, navigation emphasis
- `Positive`: ready, valid, affordable, assigned, success
- `Warning`: caution, partial fit, deadline pressure, review required
- `Danger`: invalid, blocked, harmful, destructive, high risk
- `Neutral`: background utility, low-priority chrome, inactive state
- `Info`: passive explanation, helper context, non-critical metadata

Rules:

- Green means good. Use it.
- Red means bad. Use it.
- Do not spend semantic colors on decoration.
- Accent color must do state work, not just style work.

### Button System

Only three button classes:

- `Primary`
- `Secondary`
- `Utility`

Rules:

- One primary action per screen or sub-state.
- Lateral navigation uses secondary.
- Settings, back, close, and low-priority actions use utility unless they are the current dominant action.
- Disabled buttons must look intentionally disabled and show why.

### Selected State

Define one selected-state treatment and use it everywhere.

Selected state must not rely only on appending `"Selected"` to button labels.

Selected treatment should combine:

- stronger outline or fill
- stronger title/value color
- optional selection badge or left accent bar

### Disabled State

Define one disabled-state treatment and use it everywhere.

Disabled treatment should combine:

- muted fill
- muted text
- reduced contrast
- local reason text or tooltip when relevant

## Spacing System

The next pass must use a disciplined spacing system.

Use a fixed spacing scale across the UI:

- `4`
- `8`
- `12`
- `16`
- `24`
- `32`

Recommended usage:

- outer margins: `24`
- panel padding: `16`
- gap between related controls: `8`
- gap between distinct sections: `16` or `24`
- compact row height: consistent across comparable lists
- primary button height: consistent across screens
- secondary button height: consistent across screens

Rules:

- Stop using ad hoc visual spacing per screen.
- Comparable UI patterns must share the same vertical rhythm.
- If a screen feels prototype-like, spacing drift is likely part of the reason.

## Content Budget

Every screen must stay within a content budget.

Default budget:

- 1 primary panel
- 1 support panel
- 1 action band
- 1 optional compact summary strip

Comparison screens budget:

- 1 selected-detail panel
- 1 candidate list
- 1 action footer
- optional compact state strip

Rules:

- Do not keep all useful context visible just because it exists.
- Repeated explanation belongs in one selected-detail panel, not in every row.
- Compare screens should optimize for side-by-side readability, not prose completeness.

## Card Anatomy

All compare-heavy entity cards should use the same anatomy.

Card structure:

1. visual anchor
   portrait, icon, or thumbnail
2. identity
   name, type, or role
3. current state
   assignment, readiness, or status
4. decision-critical summary
   one short line or compact chip row
5. action area
   fixed-position buttons

Rules:

- Cards should not read like mini reports.
- If a row needs two wrapped prose blocks, it is too dense.
- Extra explanation belongs in the selected detail panel.

## Feedback and Motion

Feedback must extend beyond button hover states.

Every meaningful click should trigger at least one of:

- selected row/card highlight changes immediately
- detail panel updates immediately
- local confirmation line appears near the interacted element
- small state animation or value transition

Required local feedback examples:

- `Assigned to Vanilla Suite`
- `Priority set to Safe`
- `Build queued`
- `Not enough materials`
- `Request requirements not met`

Rules:

- Prefer local response over distant bottom-of-screen error text.
- Use motion sparingly but purposefully.
- Repeated actions must stay fast; do not slow the loop with ornamental animation.

## Navigation Rules

The UI must minimize dead ends.

Every management screen footer should support:

- return to town
- at least one lateral shortcut to the most likely next task
- one current primary action when relevant

Preferred footer pattern:

- left: `Back to Town`
- middle-left: related screen shortcut
- middle-right: second related screen shortcut or contextual shortcut
- right: primary action

Examples:

- `GuildJobs Management`
  - `Back to Town`
  - `Guest Desk`
  - `Town Planner`
  - context-specific primary action
- `Chamber Management`
  - `Back to Town`
  - `Monster Roster` or profile shortcut after hatch
  - `Expedition Desk`
  - `Hatch`

Rules:

- Do not force repeated backtracking through town overview.
- When a screen implies the next step, offer a shortcut.

## Art-to-UI Relationship

Art should support structure, not sit beside it.

Requirements:

- each screen should have one stronger motif
- thumbnails and portraits need consistent framing
- art should reinforce the screen accent and hierarchy
- portraits should feel integrated into cards, not pasted into them

Rules:

- decorative background must not compete with information layers
- supporting art should help scanning and identity recognition
- framing and silhouette clarity matter more than decorative detail

## Screen Mandates

## Main Menu

Question:

- start or continue?

Primary panel:

- unified menu focal block

Primary action:

- `New Campaign`

Mandates:

- combine title and menu into one composition
- move `Settings` to shared top utility bar
- remove MVP/dev-build copy such as `This build...`
- rewrite supporting copy so it sells the actual core gameplay loop, not the implementation state
- if supporting text remains, it should read as flavor-forward or fantasy-forward setup, not patch-note text
- make `New Campaign` clearly louder than `Continue` and `Quit`
- make the title page feel like a finished front door, not a functional launcher

## Town Overview

Question:

- what should I do today?

Primary panel:

- daily priority / recommended action module

Primary action:

- next destination or end-of-day action based on current state

Mandates:

- compress the top summary harder
- stop treating the overview like a full dashboard
- reduce roster cards to decision-relevant state only
- shrink the bottom nav area so it stops behaving like a second screen
- surface one clear “today’s problem” block

## Town Management

Question:

- what is worth building next?

Primary panel:

- selected building decision panel

Primary action:

- `Build Selected`

Mandates:

- make affordability obvious without reading cost prose
- convert unlocks into chips and linked destinations
- show why the building matters now
- turn progression into milestones, not sentences

## GuildJobs Management

Question:

- who should work where?

Primary panel:

- selected room decision panel

Primary action:

- `Assign`

Mandates:

- merge room details and worker preview
- split roster into `Assigned Here` and `Available`
- reduce worker card content to prediction plus current state
- add direct shortcut to `Guest Desk`
- stop showing duplicate explanation across list and detail views

## Guest Management

Question:

- which worker can fulfill this request?

Primary panel:

- selected request panel

Primary action:

- `Assign to Request`

Mandates:

- convert requirements into pass/fail chips
- move failure reasons into compact red badges
- demote campaign context so it stops competing with the selected request
- add contextual route to the requested room where useful

## Chamber Management

Question:

- which egg is ready, and what should I hatch?

Primary panel:

- selected egg review panel

Primary action:

- `Hatch`

Mandates:

- convert eggs into more visual readiness cards
- turn outcomes into distinct selectable cards
- replace generic `Close` with clearer contextual wording
- add forward actions after hatch-ready or hatch-complete states

## Expedition Planning

Question:

- who goes, and how risky is it?

Primary panel:

- team composition panel

Primary action:

- expedition commit action

Mandates:

- stop treating priority and team selection as equal full-screen tasks
- compress priority into one concise selector
- make risk color-coded and immediate
- keep preview compact unless team or mission changes

## Monster Profile

Question:

- what is this character good for right now?

Primary panel:

- role and readiness summary

Primary action:

- contextual next-use action if applicable

Mandates:

- reduce top summary bulk
- present traits and conditions as chips or stat cards
- emphasize current usefulness over encyclopedic detail

## Implementation Sequence

### Phase 1: Shared System Pass

Build reusable primitives first:

- top utility bar
- footer action bar
- primary/secondary/utility button enforcement
- selected state
- disabled state
- badge and chip components
- reusable card shell
- local confirmation/status messaging
- shared spacing constants

Deliverable:

- no gameplay screen should still be using ad hoc interaction treatment

### Phase 2: Proof Screens

Rebuild these first:

- `Town Overview`
- `GuildJobs Management`

Reason:

- they are the heaviest proof points
- if these improve, the whole game will immediately feel more mature

Deliverable:

- both screens obey the one-question / one-dominant-panel / one-dominant-action rule

### Phase 3: Comparison Screen Cleanup

Apply the shared card and feedback system to:

- `Guest Management`
- `Expedition Planning`
- `Chamber Management`

Deliverable:

- compare-heavy screens no longer rely on dense wrapped prose

### Phase 4: Economy and Character Polish

Apply the system to:

- `Town Management`
- `Monster Profile`
- `Opening`

Deliverable:

- all major screens share one visual grammar

### Phase 5: Title Page Finish

Apply a dedicated title-page pass to:

- `Main Menu`

Deliverable:

- the title page no longer reads like an MVP or internal build
- supporting copy focuses on core gameplay fantasy and loop clarity
- title page chrome, spacing, and button treatment fully match the shared UI system

## Code-Level Work Items

1. Add shared spacing constants and stop using ad hoc pixel spacing in screen files.
2. Create a reusable top utility bar component and apply it everywhere.
3. Create a reusable footer action bar with contextual shortcut slots.
4. Replace remaining raw button usage on gameplay screens with shared styled buttons.
5. Add reusable badge, chip, selected, and disabled state helpers.
6. Add local inline feedback helpers so status and error messages can render near the relevant interaction.
7. Create a shared entity card component or drawing pattern for workers, candidates, eggs, and similar list items.
8. Refactor `Town Overview` around a daily-priority module instead of an all-systems dashboard.
9. Refactor `GuildJobs Management` around selected-room assignment flow instead of parallel text panels.
10. Reduce compare-screen prose by at least 30%.
11. Remove placeholder/dev-build copy from the title page and replace it with core-gameplay-focused messaging.
12. Give the title page its own final hierarchy/composition pass after the shared system is stable.

## Acceptance Criteria

The next pass is successful only if:

- every screen has a clearly dominant panel and action
- the player does not need to read the whole screen to know what matters
- selected state is obvious without text suffixes
- disabled state is obvious and explainable
- colors carry semantic meaning consistently
- backtracking between related management tasks is reduced
- compare-heavy screens can be parsed in 2 seconds
- the UI feels like a game decision surface, not a debug dashboard
- the title page no longer contains MVP/dev-build phrasing and feels visually consistent with the in-game UI

## Final Rule

Do not ask whether a screen feels complete.

Ask:

- does the player know the question immediately?
- does the screen show only what helps answer it?
- is the best next action obvious?

If the answer is no, cut more before polishing more.
