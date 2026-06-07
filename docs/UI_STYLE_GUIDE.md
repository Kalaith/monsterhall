# UI Style Guide

## Scope

This is the overall UI style guide for Monsterhall. It governs layout, hierarchy, component behavior, writing style, and visual rules.

Use `docs/UI_THEME_SHEET.md` as the color and mood companion to this guide. The theme sheet defines the Monsterhall Guild Hall palette; this guide defines how screens should be composed and how UI information should behave.

## Design Principle

Every screen should answer three questions immediately:

- Where am I?
- What matters right now?
- What can I do next?

If a panel, sentence, metric, or action does not help answer one of those questions, collapse it, demote it, or remove it.

## Screen Anatomy

Use this hierarchy for management screens:

1. Screen header or selected context.
2. One dominant decision panel.
3. One support area for comparison or secondary context.
4. Footer navigation and contextual actions.

Avoid screens with three or more equally loud panels. The UI should guide the eye from context to decision to action.

## Layout Rules

- Use stable panel dimensions and fixed action columns where content can vary.
- Keep outer margins, panel padding, and gaps sourced from `src/ui/layout.rs`.
- Do not let hover states, selected labels, or dynamic text resize cards.
- Prefer horizontal chip rows for compact facts.
- Prefer compact empty states over empty framed panels.
- Avoid placing cards inside larger decorative cards.
- Reserve large panels for decision-making content, not static labels.

## Information Compression

The UI should avoid repeating facts.

- A selected object name can appear in the list and selected detail title.
- A selected object name should not also appear in the art caption.
- A state should appear in one status pill or strip.
- A same-state action button should be hidden.
- Zero-value metrics should be hidden unless they are meaningful warnings.
- Empty states should be shown once.

## Component Standards

### Buttons

- Primary button: one dominant action for the current screen or selected object.
- Secondary button: valid alternative action.
- Utility button: navigation, settings, close, and low-risk controls. It should share the secondary button color family, not use a separate dark-blue treatment.
- Danger button: destructive actions, quit, debt pressure, forced tradeoffs.
- Premium/confirm button: build, unlock, high-value confirmation.

Rules:

- Do not show more than one primary action in the same decision area.
- Do not show a button that simply repeats the current state.
- Disabled actions need either a visible reason or should be hidden if the reason is obvious.
- Action rows should stay compact and close to the content they affect.
- Footer navigation buttons use the same secondary style unless they are the current primary screen action.

### Cards

Use one shared entity-card anatomy for monsters, workers, contracts, and assignment candidates. Field order is fixed:

- portrait
- name/species
- current state
- one key value line
- fixed-width action rail

The action rail belongs on the right edge of the card. It should have stable width across cards on the same screen, use one vertical stack or one compact row, and hide same-state actions.

The key value line should be the one fact that drives the current screen's decision:

- Town roster: skill summary or condition summary.
- Guild Hall worker: projected earnings or fit.
- Expedition member: instability, readiness, or expedition value.
- Patron candidate: eligible/blocked reason.

Do not add a second paragraph to cards. If a card needs more explanation, the selected detail panel is carrying too much work.

Cards used for comparison should keep the same field order, portrait size, text offsets, and action rail width across screens whenever the layout width allows it.

### Selected Item Headers

Selected item panels use one shared header sequence:

- title
- one status pill
- one short description
- one action or compact action group
- compact metadata chips

This applies to selected buildings, rooms, eggs, floors, requests, and profile summary panels. Keep the selected title as the only large name in the detail area. Art captions, support rows, and action hints should not repeat it.

The status pill should summarize the active state only once. Secondary facts such as build count, cost, category, difficulty, risk, possible species, tier, training, and projected output belong in compact metadata chips after the action area.

Metadata chips should be short nouns or values such as `Built Out`, `Cost`, `Category`, `Risk: High`, `1 possible species`, or `Tier 1`. Avoid full-sentence status rows inside selected item headers.

### Container Sizing

Containers should be only as large as the decision they contain.

- Assigned/Available regions collapse or shrink when there are few items.
- Team panels should not reserve space for six cards when the fixture has three.
- Traits and other passive reference panels should be compact unless they contain multiple meaningful entries.
- Top status strips should be slim metric/status rows, not large panels with one line of text.

Large empty interiors are treated as a layout bug unless they are intentionally preserving room for a known high-frequency content state.

### Chips And Badges

Use chips for compact facts:

- cost
- risk
- reward
- source
- grade
- availability
- assignment count
- requirement pass/fail

Use badges for state:

- selected
- ready
- blocked
- warning
- danger
- locked
- assigned

Badges should be short. Prefer `Affordable`, `Blocked`, `Ready`, `Risk: Low`, and `Assigned` over sentence fragments.

### Empty States

Empty states should be short and useful:

- title: what is absent
- body: why it matters or when it changes
- optional action: the next useful destination

Do not show multiple empty states for the same absence. For example, contract desk should not show no requests, no request selected, and no eligible companions at the same time.

### Art

Art should support the active decision.

- Use art captions only when they add information not already in the selected title.
- Do not use art panels as duplicate labels.
- Important art can use restrained gold edges.
- Art frames should share the same panel and border system as the rest of the UI.

## Navigation

Management screens should use consistent footer structure:

- left: back to town or previous major context
- middle: one or two lateral shortcuts to related work
- right: primary contextual action when relevant

Avoid forcing the player to return to town just to reach the next likely task. Guild Hall, patron, hatchery, expedition, and town management should expose related destinations directly when useful.

## Text Style

Write UI copy as game-facing operational language, not implementation notes.

Prefer:

- `No active requests today`
- `Check again tomorrow or build demand sources in town`
- `No workers assigned`
- `Hatch (45 gold / 6 residue)`
- `Risk: Low`

Avoid:

- `Campaign loaded from save`
- `Status: Plan expedition`
- `Build now. The cost is covered...`
- repeated explanations of the same state
- prototype or MVP language on player-facing screens

Full-width prose should be rare. Use it for one of:

- mood or flavor
- actionable guidance

Do not use both in the same panel.

## Color Semantics

Follow `docs/UI_THEME_SHEET.md` for exact colors. Use color for meaning:

- Purple: selected, focus, active destination.
- Wine red: pressure, residue, danger, debt.
- Gold: premium value, reward, upgrades, notable value.
- Green: valid, ready, successful, assigned.
- Amber: caution, partial fit, limited resource.
- Gray-violet: passive chrome, inactive, disabled support.

Do not use color only as decoration. If two colors appear on the same screen, their meaning should be understandable from context.

## Screen-Specific Guidance

### Main Menu

- Keep the first viewport focused on title, two-line premise, three actions, and supporting art.
- `New Campaign` should be the dominant action.
- Settings belongs in compact utility chrome.
- Avoid dev-build or prototype wording.

### Town Overview

- Lead with today's priority.
- Keep campaign/resource/debt data compact.
- Highlight the destination related to the priority instead of duplicating the same CTA.
- Roster cards use the shared entity-card anatomy: portrait, name/species, current state, one key value line, action rail.

### Town Management

- Availability lives in list icon and selected status pill.
- Cost, category, and built count belong in a chip row.
- Unlocks and progression should be actionable or compact.

### Guild Hall Management

- Selected room title owns the room name.
- Empty assigned state should collapse.
- Worker cards use the shared entity-card anatomy and a fixed action rail.
- Zero projections caused by no workers should become `No workers assigned`.

### Hatchery Management

- Hatchery header is for hatchery-level facts.
- Selected egg detail is for egg-specific facts.
- Locked outcome appears once.
- Single outcomes render as compact result cards.
- Hatch cost belongs in the hatch action.

### Expedition Planning

- Floor name belongs in list and selected title only.
- Risk, success, and main reward get priority.
- Zero projected rewards are hidden unless important.
- Team cards use the shared entity-card anatomy and should not keep a tall panel when only a few team members exist.

### Contract Desk

- No requests should render one empty state.
- Request requirements should become chips.
- Candidate pass/fail data should use badges, not wrapped prose.
- Summary data belongs in a slim strip when requests exist.

### Settings

- Display mode state belongs in the selector only.
- Save feedback belongs near Save.
- Quit should be visually separated from Save and Close.

## Implementation Expectations

Shared UI rules should live in shared files:

- `src/ui/layout.rs`: spacing, heights, stable dimensions.
- `src/ui/theme.rs`: palette and semantic colors.
- `src/ui/chrome.rs`: headers, footers, panel wrappers.
- `src/ui/components.rs`: cards, chips, badges, empty states, metric tiles.
- `src/ui/feedback.rs`: inline success/error/status feedback.

Screen files should compose these primitives. They should not hand-roll common card, footer, badge, or empty-state behavior.

## Review Checklist

Before a UI change is accepted, check:

- Does the screen have one dominant decision area?
- Is any fact repeated more than twice?
- Are empty states collapsed to one message?
- Are same-state buttons hidden?
- Are status and availability in consistent places?
- Are metrics chips or tiles instead of prose?
- Does color communicate state rather than decoration?
- Are destructive actions separated from utility actions?
- Does all text fit without overlap at supported resolutions?
- Do screenshot captures confirm the hierarchy?
