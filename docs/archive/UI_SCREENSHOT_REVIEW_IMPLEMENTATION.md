# UI Screenshot Review Implementation Plan

## Fresh Review Basis

This review is based on a fresh management-flow capture taken on `2026-04-12` using:

- `python tmp_screens\play_ui_test.py --scenario management --save-preset management`

Fresh screenshots were written to:

- `tmp_screens\captures_2026_04_12\fresh_review_management`

Screens reviewed from the fresh capture:

- `main_menu.png`
- `town_overview.png`
- `town_management.png`
- `guild_jobs.png`
- `expedition_planning.png`

Additional reference screenshots from `tmp_screens\captures_2026_04_11` were used for:

- `main_menu_settings.png`
- `new_game_opening.png`
- `monster_profile.png`
- `chamber_management.png`
- `guest_management.png`

This pass is intentionally focused on the questions:

1. Is everything currently on screen helping the player right now?
2. Do screens behave predictably and follow the same patterns?
3. Can the player move forward without backing out through multiple screens?
4. Does the UI clearly react to interaction?
5. Are colors doing enough semantic work?
6. Is clarity winning over cleverness?

## Executive Summary

The UI is in better shape structurally than the earlier pass, but it still has three systemic problems:

- Too much information is visible at once, even when only a small portion is actionable.
- Navigation and button treatment are not yet consistent enough across screens.
- The player still spends too much time reading labels and state text instead of reading hierarchy.

The next pass should optimize for decision speed, not decoration. The priority is not more panels, more labels, or more flavor chrome. The priority is making each screen answer three questions instantly:

- What screen am I on?
- What matters right now?
- What is the best next action?

## Fresh Review Findings

### 1. Screen content is still too eager

Current issue:

- Several screens show secondary context permanently even when it is not needed for the current task.
- This makes the game feel more like a dashboard than a game flow.
- The heaviest cases remain `town_overview`, `guild_jobs`, `guest_management`, and `expedition_planning`.

What to change:

- Hide or compress low-value context until it becomes relevant.
- Prefer one strong active panel and one support panel over three equally loud panels.
- Move passive summaries into compact status strips, badges, or collapsible subsections.

Examples:

- `Town Overview`: the summary row, roster, onboarding, and action area all compete. The player does not need full campaign, resources, debt, and guest-pressure detail at full size every visit.
- `GuildJobs Management`: room description, worker preview, and worker list all repeat related information. The screen can be reduced to room selection + room summary + assignable roster.
- `Expedition Planning`: floor details, mission row, priority grid, worker actions, and preview all demand attention simultaneously.

### 2. Pattern consistency is still not strong enough

Current issue:

- Similar actions are presented in different ways across screens.
- Some screens use styled buttons and panel chrome consistently; others still use more basic controls and flatter states.
- Footer navigation varies from screen to screen, which forces the player to re-scan the bottom of the screen every time.

Implementation evidence in the current UI code:

- Shared button styles exist in `src/ui/core.rs`, but some screens still use raw `button(...)` instead of `primary_button`, `secondary_button`, and `utility_button`.
- This is visible in `main_menu.rs`, `settings.rs`, `guest_management_sections.rs`, and `chamber_management_sections.rs`.

What to change:

- Standardize one shared top utility bar.
- Standardize one shared footer action bar pattern.
- Standardize one selected-state treatment that is not just `"Selected"` appended to labels.
- Standardize one card pattern for selectable entities:
  - portrait or thumbnail
  - name
  - current status
  - one compact secondary detail row
  - fixed action area

### 3. Dead ends still exist in management flows

Current issue:

- Some screens let the player move laterally to the next logical task, but others force a return to town first.
- This creates unnecessary backtracking and makes the game feel more menu-driven than flow-driven.

Examples:

- `Town Management` includes a direct `Expedition Desk` shortcut, which is good.
- `Guest Management` includes a direct `GuildJobs Desk` shortcut, which is also good.
- `GuildJobs Management` does not offer the same kind of lateral movement to `Guest Management` or `Town Planner`.
- `Chamber Management` offers `Return to Town` and `Expedition Desk`, but no quick route to the next likely task after hatch completion.
- The egg review state in `Chamber Management` uses `Close`, which returns to the list, but not a forward action like `Hatch and Go to Roster` or `Hatch and Open Profile` when appropriate.

What to change:

- Add “go do the thing” links in every management footer.
- For each screen, define the two most likely next tasks and surface them directly.

Recommended footer rule:

- Left: `Back to Town`
- Center-left: lateral shortcut 1
- Center-right: lateral shortcut 2 or contextual shortcut
- Right: primary confirm or end-of-day action

### 4. Feedback is adequate on buttons but weak in the rest of the UI

Current issue:

- Buttons have hover and pressed states in shared styles, which is good.
- Outside buttons, the UI gives limited confirmation that a selection, assignment, or state change has happened.
- There is little in-context feedback near the interacted element.

What to change:

- Add explicit selected outlines or highlight fills to selected cards, rows, and panels.
- Add lightweight transient confirmation text near the interacted element:
  - `Assigned to Vanilla Suite`
  - `Priority set to Safe`
  - `Floor selected`
- Add disabled states with visible reasons instead of allowing ambiguous failure.
- Prefer local feedback over bottom-of-screen error text whenever possible.

Current weak spots:

- Selection feedback in list screens often relies on the button label changing rather than the whole row/card changing.
- Error feedback is usually detached from the failed control and rendered low on the screen.
- Guest eligibility and chamber outcome review still rely too much on reading text instead of reading a state color or badge.

### 5. Color semantics are underused

Current issue:

- Accent colors currently carry mood and chrome more than meaning.
- The UI does not yet use color strongly enough to distinguish:
  - available
  - blocked
  - risky
  - assigned
  - selected
  - success
  - warning
  - failure

What to change:

- Green = ready, affordable, valid, assigned successfully.
- Red = invalid, danger, debt pressure, injury risk, destructive action.
- Yellow/amber = warning, partial fit, caution, limited availability.
- Blue = neutral selection and navigation.
- Gray = inactive, unavailable, background utility.

Concrete uses:

- `Town Management`: `Available` and `Build Selected` should read as clearly positive when affordable.
- `Guest Management`: eligible candidates should be clearly green; failing candidates should show red failure chips, not just warning-colored prose.
- `Expedition Planning`: risk should escalate from neutral to amber to red based on injury score.
- `Chamber Management`: `Ready`, `Review Required`, `Unknown`, and `Locked` should each have distinct badge colors.

### 6. Several screens still choose “show everything” over clarity

Current issue:

- The UI often exposes the full internal model instead of the minimum decision surface.
- This is most visible where the game asks the player to compare candidates.

What to change:

- Replace sentence-style summaries with short comparable fields.
- Prefer chips, labels, and stat rows over prose paragraphs.
- Reserve full descriptive text for a selected-detail panel, not every list entry.

Target rule:

- If the player is deciding between options, every option should be comparable in 2 seconds.

## Screen-by-Screen Improvements Still Worth Making

## Main Menu

Still needs improvement:

- The separate logo lockup and center menu composition still split attention.
- `Settings` is still a small utility button inside the menu panel instead of feeling like a global utility control.
- Empty scenic space is still doing more atmospheric work than navigational work.

Recommended next changes:

- Unify title, subtitle, and primary actions into a single focal block.
- Move `Settings` into a top utility bar that matches the rest of the game.
- Make the primary call to action more dominant than every other option on the screen.

## Opening Screen

Still needs improvement:

- The layout is directionally correct, but onboarding text and action emphasis can be stronger.
- This is a sequence screen, so it should feel sequential rather than like a general information page.

Recommended next changes:

- Increase body size and line spacing.
- Add a visible step marker or chapter progression cue.
- Make the continue/build CTA visually dominant and consistent with later primary actions.

## Town Overview

Still needs improvement:

- It remains the busiest screen in the game.
- The bottom action block still occupies too much visual weight relative to the active state of the town.
- The player sees more high-level campaign detail than they usually need for the next click.

Recommended next changes:

- Compress the top summary row into fewer, denser cards.
- Promote one “today’s problem” or “next recommended action” panel.
- Reduce the visual height of the action area and increase the specificity of the actions.
- Make roster cards show only the most decision-relevant state by default.

## Monster Profile

Still needs improvement:

- It is serviceable, but it still reads as a stats page rather than a character page with actionable meaning.
- Conditions, traits, and history can be made more scannable.

Recommended next changes:

- Reduce the top summary band.
- Turn traits and conditions into clearer chips or stat cards.
- Surface current role, readiness, and best next use more clearly than raw detail.

## Town Management

Still needs improvement:

- The screen is cleaner than before, but it is still text-forward.
- The selected building area still asks the player to read several different text blocks before acting.
- Progression is present but not yet immediately informative.

Recommended next changes:

- Turn `Cost`, `Passive Effects`, and `Unlocks` into stronger visual modules with clearer color semantics.
- Make affordability instantly readable without reading the cost string.
- Add a direct “build unlocks this, take me there” link when a building unlocks a room or floor.
- Convert progression into milestone chips rather than status sentences.

## Chamber Management

Still needs improvement:

- The inventory list is still mostly textual.
- Review mode still reads like a temporary overlay state rather than a strong decision screen.
- The current flow ends too often in “close and go back” rather than “finish and continue.”

Recommended next changes:

- Convert eggs into cards with stronger readiness states.
- Turn outcome rows into distinct cards with clear known/unknown status.
- Add forward shortcuts after hatch-ready or hatch-complete actions.
- Replace generic `Close` with a more explicit verb when the player is exiting a review state.

## GuildJobs Management

Still needs improvement:

- This is still the densest screen.
- Room details and worker preview remain too similar in purpose.
- Worker rows still carry too much text for a compare-and-assign screen.

Recommended next changes:

- Merge room details and worker preview into one selected-room summary.
- Split worker list into `Assigned Here` and `Available`.
- Give each worker card one predicted result line and one current-state line, not two dense summary blocks.
- Add a direct shortcut to `Guest Desk` because brothel assignment and guest fulfilment are tightly related.

## Guest Management

Still needs improvement:

- The screen has a reasonable three-panel structure, but candidate comparison still depends too much on text.
- The context panel is helpful, but it currently steals attention from the selected request.

Recommended next changes:

- Turn request requirements into chips with pass/fail states.
- Move candidate failures into short red badges rather than wrapped prose.
- Make the selected request panel visually dominant over the campaign context panel.
- Add a direct shortcut from a request to the requested room where useful.

## Expedition Planning

Still needs improvement:

- This screen still feels like two screens layered together.
- Priority selection and roster assignment are both high-attention tasks and currently compete.
- The bottom preview area is useful, but it adds a third reading band after the player has already parsed two others.

Recommended next changes:

- Collapse priority controls into one concise horizontal selector.
- Convert team members into compact assignment cards matching brothel/guest selection patterns.
- Push preview results into a slimmer summary strip until the team composition changes.
- Add stronger color coding for risk and expected success.

## Immediate System Fixes

These are the highest-value implementation changes because they improve many screens at once.

### A. Standardize interaction components

- Replace remaining raw `button(...)` usage with shared styled button helpers.
- Add one reusable selected-card state and one reusable disabled-card state.
- Add one shared badge component for:
  - success
  - warning
  - danger
  - selected
  - locked

### B. Standardize navigation structure

- Add a shared top utility bar for settings and future meta actions.
- Add a shared footer navigation template for all management screens.
- Add direct lateral shortcuts between related work screens instead of routing everything through town.

### C. Improve in-place feedback

- Add local success/failure feedback near the clicked control.
- Replace detached bottom error text with inline validation where possible.
- Add hover help or short hint text for important controls that have non-obvious outcomes.

### D. Reduce comparison load

- Replace sentence summaries with two-line comparable cards.
- Move descriptive prose into selected-detail areas only.
- Ensure any list used for assignment has the same data order on every screen.

## Priority Order

### Phase 1: Predictability and dead-end removal

- Standardize button and selection styles across every screen.
- Add consistent footer navigation to all management screens.
- Add direct “take me there” shortcuts between related screens.

### Phase 2: Comparison clarity

- Refactor brothel, guest, and expedition candidate lists into consistent card patterns.
- Add semantic badges and pass/fail chips.
- Reduce prose density in assignment screens.

### Phase 3: Information reduction

- Compress town overview summary content.
- Merge duplicated panels in brothel and expedition flows.
- Move secondary context out of the main decision path.

### Phase 4: Final hierarchy polish

- Tune accent use so color means state, not just decoration.
- Improve selected-state visuals.
- Refine main menu and opening composition.

## Concrete Implementation Tasks

1. Replace all remaining raw button calls on gameplay screens with shared styled button helpers.
2. Add reusable badge and pill components with semantic colors.
3. Build a shared management footer component with contextual shortcuts.
4. Add inline status confirmation messaging for assignment and selection actions.
5. Refactor worker and candidate lists to a single card layout shared across brothel, guest, and expedition screens.
6. Add “go there” actions where one screen unlocks or implies a next destination.
7. Reduce summary text on town overview and management screens.
8. Replace generic close/back labels with action-specific labels where the player is exiting a sub-state.

## Success Criteria

The next UI pass should be considered successful if:

- The player can identify the primary action on every screen within 1-2 seconds.
- Moving between related management tasks no longer requires repeated returns to town.
- Assignment screens support quick comparison without reading full sentences.
- Selection and assignment changes are visible immediately near the interacted element.
- Color semantics communicate validity, risk, and success clearly.
- Each screen shows only the information needed for the decision being made on that screen.
