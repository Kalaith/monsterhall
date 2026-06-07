# UI Duplicate And Weak Screen Resolution Plan

## Purpose

This plan converts `feedback.md` into implementation work. The goal is to remove repeated facts, collapse dead empty states, and make every screen expose one clear decision surface instead of several competing panels.

This is not a full redesign. It is a compression pass over the current UI.

## Global Implementation Rules

Apply these rules before tuning any individual screen.

1. A fact can appear in at most two places.
   - Allowed: list label plus selected detail title.
   - Avoid: list label, art caption, selected title, status text, and action hint all repeating the same name or state.
2. Empty UI collapses.
   - If a panel has no actionable content, hide it or reduce it to one compact empty state.
   - Never show multiple panels explaining the same absence.
3. Status has one home.
   - Current state belongs in a pill or compact status strip near the selected item title.
   - Do not repeat the same state in body prose and same-state buttons.
4. Actions are contextual.
   - Hide actions that only restate the current state.
   - Keep a single dominant action per screen or selected object.
5. Metrics are chips, not paragraphs.
   - Gold, lust, risk, eggs, grade, count, source, availability, and assignment state should use chips or compact metric tiles.
6. Full-width prose is scarce.
   - Use full-width text for either flavor or actionable guidance, not both in the same panel.

## Shared Work First

The fastest way to prevent another screen-by-screen drift is to add or complete reusable UI primitives before editing individual layouts.

### Target Files

- `src/ui/components.rs`
- `src/ui/chrome.rs`
- `src/ui/layout.rs`
- `src/ui/theme.rs`
- `src/ui/view_models.rs`

### Required Primitives

Add or standardize:

- `draw_status_pill(kind, label)`
- `draw_chip(kind, label, value)`
- `draw_chip_row(chips)`
- `draw_empty_state(title, body, action)`
- `draw_entity_card(...)` with a fixed action column
- `draw_selected_detail_header(title, status_pills)`
- `draw_metric_strip(metrics)`
- `draw_management_footer(primary, shortcuts)`

### Shared Behavior Rules

- Entity cards always use the same anatomy: portrait or icon, name, role/state line, one metric or prediction line, fixed-width action column.
- Selected detail panels own selected-object title and status.
- Art captions describe the image only when the image adds new information. They must not repeat the selected title.
- Disabled buttons must include a nearby reason or be hidden if they are not useful.

## Phase 1: Highest-Impact Compression

These changes remove the largest visible duplication without requiring new game logic.

### Guest Management

Current issue: empty state is repeated as `No Requests`, `No Request Selected`, and `No active request selected`.

Implementation:

- When there are no active requests, render one full-screen empty state in the main content area.
- Hide the selected request panel.
- Hide eligible girls entirely.
- Keep only a compact economy/context strip if it has useful non-empty information.
- When requests exist, use:
  - request list
  - selected request panel
  - eligible candidate cards
  - compact summary strip

Done criteria:

- No state renders more than one "nothing selected/no request" message.
- The empty screen has one clear next suggestion: check tomorrow or build demand sources.

### GuildJobs Management

Current issue: room name and room state repeat across list item, art caption, detail title, projected output, and empty assignment panel.

Implementation:

- Remove room names from art captions when the selected detail title is visible.
- Keep room status in the selected room header only.
- Replace `0 gold / 0 lust` projections with `No workers assigned` when output is zero because the room is empty.
- Collapse empty `Assigned Here` to an 80-100px empty state.
- Rebuild worker cards with:
  - portrait
  - name/species
  - one prediction line
  - fixed-width right action column
- If only one available worker exists, allow the card to widen or center rather than leaving a large dead column.

Done criteria:

- Selected room name appears only in list and selected detail title.
- Empty assignment state does not dominate the screen.
- Worker card buttons no longer crowd text.

### Town Overview

Current issue: priority CTA, footer navigation, save-state strip, and roster state all compete.

Implementation:

- Remove the permanent `campaign loaded from save` strip.
- Keep `Today's Priority` to one sentence and one CTA.
- If the priority target matches a footer destination, highlight that footer destination instead of duplicating a second large CTA.
- Roster cards show name, species, role/state, and key stats.
- Hide either the idle status pill or same-state idle action.
- Use responsive roster layout so a single card does not leave a large empty area.

Done criteria:

- The first focal point is the current daily priority.
- No roster card shows both an idle state and an idle action.
- Footer and priority stop presenting the same destination at equal weight.

## Phase 2: Repeated State And Availability

These screens are less broken than the empty-state cases, but they still repeat status and availability too aggressively.

### Town Management

Current issue: availability appears as icon, status text, build sentence, and active button.

Implementation:

- Keep availability in two places:
  - list icon
  - selected detail status pill
- Replace build affordance prose with short chips such as `Affordable`, `Blocked`, or `Built`.
- Turn cost, category, and built count into a single chip row.
- Remove `Next Destination: Return To Town` unless it becomes an actionable destination.
- Merge or reduce `Effects` and `Progression Web` so one support panel is dominant.
- Widen the left list rail or use shorter building display names for long labels.

Done criteria:

- There is no full sentence saying the active build button is affordable.
- The lower support area has one clear purpose.
- Long building names do not truncate awkwardly.

### Chamber Management

Current issue: source, locked outcome, grade, and outcome state repeat between inventory and selected detail.

Implementation:

- Top bar contains chamber-level info only: egg count and optional source summary.
- Selected detail contains egg-specific source and outcome info.
- Show `Locked Outcome` once as a status pill below the egg name.
- If there is only one possible outcome, render a compact result card instead of a wide outcomes area.
- Move hatch cost into the button label: `Hatch (45 gold / 6 lust)`.
- Simplify inventory cards to thumbnail, rarity, one-line outcome state, and review action.

Done criteria:

- `Locked Outcome` is rendered once for the selected egg.
- Single-outcome eggs no longer create an oversized empty lower panel.
- Hatch cost appears where the player acts.

### Expedition Planning

Current issue: floor name, idle state, status strip, and zero metrics repeat or crowd the decision.

Implementation:

- Remove floor name from art captions when the detail title is visible.
- Replace `Status: Plan expedition` with useful plan state such as `1 member assigned` and `Risk: Low`.
- Hide zero-value reward metrics unless they change the decision.
- Highlight only success chance, main reward, and injury risk by default.
- Rebuild team member cards with entity-card anatomy and fixed action columns.
- If a worker is idle, show useful actions such as assign/rest rather than an idle button and idle chip together.

Done criteria:

- Floor name appears only in list and selected detail title.
- Zero projected rewards do not consume equal visual weight.
- Risk and success are readable in one glance.

## Phase 3: Modal And Entry Screens

These screens are not the main source of duplicate state, but they define perceived quality.

### Settings

Current issue: display mode is repeated by `Fullscreen: Off`, mode tiles, and near-identical save states.

Implementation:

- Remove the `Fullscreen: Off` row.
- Let the display-mode selector carry current state.
- Make resolution options a 2-column selected-tile grid.
- Place save feedback directly above or inside the save button zone.
- Separate utility actions from destructive navigation:
  - utility row: Save, Close
  - destructive row or confirm flow: Quit Game
- Tighten vertical spacing between display mode and resolution.

Done criteria:

- Fullscreen state is represented once.
- Quit does not visually compete with save/close.
- Save feedback is near the save action.

### Main Menu

Current issue: top utility bar is oversized and copy is too airy for the amount of decision making on the screen.

Implementation:

- Shrink top utility bar height by 30-40%.
- Anchor settings as a compact top-right icon/button.
- Tighten premise copy to two lines.
- Narrow the right art panel slightly so title and primary CTA dominate.
- Remove framing that does not support the main focal block.

Done criteria:

- The visible structure is title, two-line premise, three actions, and art.
- `New Campaign` reads as the dominant action.
- Background tower art supports the composition without competing with the foreground art panel.

## Phase 4: Cross-Screen Consistency Sweep

After screen-specific work, run a final pass for the recurring duplicate patterns.

Check every screen for:

- Name duplication: list label, art caption, selected title.
- Empty duplication: no-content state shown in more than one panel.
- State duplication: state chip, same-state button, and state prose all visible together.
- Availability duplication: icon, text, affordance sentence, and button all saying the same thing.
- Metric overload: zero or secondary metrics shown at equal weight with useful metrics.
- Action row clutter: permanent actions visible when they are not relevant.

## Validation

Use the central screenshot flow after each phase:

- `python tmp_screens\play_ui_test.py --scenario review_suite --save-preset management --background --compare-threshold 2.0`

Review these captures first:

- `tmp_screens/playtests/current/review_suite__guest_management.png`
- `tmp_screens/playtests/current/review_suite__guild_jobs.png`
- `tmp_screens/playtests/current/review_suite__town_overview.png`
- `tmp_screens/playtests/current/review_suite__town_management.png`
- `tmp_screens/playtests/current/review_suite__chamber_management.png`
- `tmp_screens/playtests/current/review_suite__expedition_planning.png`
- `tmp_screens/playtests/current/review_suite__settings.png`
- `tmp_screens/playtests/current/review_suite__main_menu.png`

## Final Acceptance Criteria

The compression pass is complete when:

- no screen shows the same fact in more than two places
- empty states collapse to one useful message
- same-state buttons are hidden or replaced with meaningful actions
- art captions do not duplicate selected titles
- status and availability have consistent homes
- compare-heavy cards share one anatomy
- every screen has one visible dominant decision or action
