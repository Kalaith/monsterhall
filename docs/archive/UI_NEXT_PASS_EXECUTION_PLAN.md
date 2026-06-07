# UI Next Pass Execution Plan

## Purpose

This document converts [docs/UI_NEXT_PASS_MANDATE.md](/H:/WebHatchery/RustGames/monsterhall/docs/UI_NEXT_PASS_MANDATE.md) into a code-facing implementation plan for the current Rust UI codebase.

It answers:

- which files need to change
- which shared primitives should be introduced first
- which screens should be rebuilt first
- how to sequence the work so the UI stops drifting screen by screen

## Current UI Surface

Primary UI entry points:

- [src/ui/mod.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/mod.rs)
- [src/ui/screens/mod.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/mod.rs)

Shared rendering helpers today:

- [src/ui/core.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/core.rs)
- [src/ui/art.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/art.rs)
- [src/ui/view_models.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/view_models.rs)

State types that drive UI decisions today:

- [src/state/game_state.rs](/H:/WebHatchery/RustGames/monsterhall/src/state/game_state.rs)

Highest-priority gameplay screen files:

- [src/ui/screens/town_overview.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/town_overview.rs)
- [src/ui/screens/town_overview_sections.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/town_overview_sections.rs)
- [src/ui/screens/guild_jobs.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/guild_jobs.rs)
- [src/ui/screens/guild_jobs_sections.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/guild_jobs_sections.rs)

Secondary gameplay screen files:

- [src/ui/screens/guest_management.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/guest_management.rs)
- [src/ui/screens/guest_management_sections.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/guest_management_sections.rs)
- [src/ui/screens/expedition_planning.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/expedition_planning.rs)
- [src/ui/screens/chamber_management.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/chamber_management.rs)
- [src/ui/screens/chamber_management_sections.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/chamber_management_sections.rs)
- [src/ui/screens/town_management.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/town_management.rs)
- [src/ui/screens/monster_profile.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/monster_profile.rs)
- [src/ui/screens/main_menu.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/main_menu.rs)
- [src/ui/screens/opening.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/opening.rs)
- [src/ui/screens/settings.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/settings.rs)

## Structural Problems In Code Terms

The current UI quality issues map to code-level issues:

- too much per-screen layout logic embedded directly in screen files
- shared buttons exist, but shared screen chrome does not
- selected state is inconsistent and often encoded as label text
- footer and top utility patterns are duplicated across screens
- compare-heavy cards are hand-built separately in each screen
- status and error feedback are mostly detached from the triggering control
- spacing is mostly hard-coded per file instead of system-driven

This means the next pass must start by changing the UI architecture, not just rewriting individual layouts.

## Required New Shared Modules

Create these modules before rebuilding the heaviest screens.

### 1. `src/ui/layout.rs`

Purpose:

- centralize spacing, margins, section gaps, and standard heights

Responsibilities:

- spacing constants
- outer safe margins
- standard panel paddings
- standard button heights
- standard footer heights
- standard card spacing

Expected constants:

- `SPACE_4`
- `SPACE_8`
- `SPACE_12`
- `SPACE_16`
- `SPACE_24`
- `SPACE_32`
- `OUTER_MARGIN`
- `PRIMARY_BUTTON_H`
- `SECONDARY_BUTTON_H`
- `UTILITY_BUTTON_H`
- `FOOTER_H`

Why:

- current spacing is duplicated and ad hoc across `town_overview_sections`, `guild_jobs_sections`, `expedition_planning`, `guest_management_sections`, and `town_management`

### 2. `src/ui/theme.rs`

Purpose:

- centralize semantic color names instead of letting screens choose accent intent ad hoc

Responsibilities:

- semantic colors
- panel tier colors
- selected-state colors
- disabled-state colors
- local feedback colors

Expected semantic colors:

- `PRIMARY`
- `POSITIVE`
- `WARNING`
- `DANGER`
- `NEUTRAL`
- `INFO`

Why:

- `core.rs` currently defines button visuals, but semantic meaning is still implicit and inconsistent

### 3. `src/ui/chrome.rs`

Purpose:

- hold shared screen chrome that is currently repeated across many files

Responsibilities:

- top utility bar
- shared screen title block
- footer action bar
- panel tier wrappers
- shared status strip

Expected functions:

- `draw_top_utility_bar(...)`
- `draw_screen_header(...)`
- `draw_footer_bar(...)`
- `draw_primary_panel(...)`
- `draw_support_panel(...)`
- `draw_utility_panel(...)`
- `draw_inline_status(...)`

Why:

- nearly every screen manually draws a title, subtitle, and `Settings` button
- footers are inconsistent and currently screen-local

### 4. `src/ui/components.rs`

Purpose:

- reusable high-level UI pieces instead of screen-local custom rows

Responsibilities:

- semantic badge
- chip row
- selected card shell
- disabled card shell
- entity card layout
- compact metric tiles
- empty state block

Expected functions:

- `draw_badge(...)`
- `draw_chip(...)`
- `draw_chip_row(...)`
- `draw_entity_card_frame(...)`
- `draw_metric_tile(...)`
- `draw_empty_state(...)`

Why:

- worker cards, candidate cards, egg rows, and building info blocks should stop being custom one-offs

### 5. `src/ui/feedback.rs`

Purpose:

- localize and standardize interaction feedback

Responsibilities:

- inline error messaging
- inline success/confirmation text
- recent-action feedback rendering helpers

Possible first version:

- helper functions that render local status lines near selected panels
- no animation system required in the first pass

Why:

- current `last_error` rendering is typically detached at the bottom of screens

## Required Updates To Existing Shared Files

## `src/ui/core.rs`

Keep:

- font scaling
- base text draw helpers
- base button rendering hooks

Change:

- move spacing constants out to `layout.rs`
- move semantic color meaning out to `theme.rs`
- keep only low-level primitives here
- ensure all button styles read from semantic theme values instead of hard-coded color intent

Add:

- single selected treatment helper
- single disabled treatment helper

## `src/ui/view_models.rs`

Add view-model helpers for decision-oriented UI:

- building affordability state
- building unlock destination summaries
- worker readiness summaries
- worker predicted result summaries
- guest candidate pass/fail chip data
- egg readiness state labels
- expedition risk state labels
- daily priority summary for town overview

Why:

- screen files should not keep re-encoding decision logic directly into layout code

## `src/ui/actions.rs`

Review whether new direct navigation actions are needed.

Likely additions:

- go-to-context actions where current screen should route directly to related work
- possible contextual follow-up actions after hatch/build/assignment

Do not add speculative actions unless the screen flow actually needs them.

## `src/state/game_state.rs`

Do not reshape core save data lightly.

Only add UI-facing transient state if necessary for:

- stronger selected-state handling
- direct contextual follow-up flow
- local feedback state that cannot be derived from existing status strings

Prefer keeping persistent game state untouched and deriving presentation state in UI helpers where possible.

## Screen Refactor Order

## Phase 1: Shared System Pass

Target files:

- [src/ui/core.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/core.rs)
- `src/ui/layout.rs` new
- `src/ui/theme.rs` new
- `src/ui/chrome.rs` new
- `src/ui/components.rs` new
- `src/ui/feedback.rs` new
- [src/ui/mod.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/mod.rs)

Work:

1. Introduce shared layout constants.
2. Introduce semantic color definitions.
3. Extract shared top utility bar.
4. Extract shared footer bar.
5. Define selected and disabled state rendering.
6. Define reusable badge/chip/card primitives.
7. Export new modules from `src/ui/mod.rs`.

Done means:

- no new screen work is started before these primitives exist
- screens stop hand-rolling common chrome

## Phase 2: Proof Screens

Rebuild the two heaviest proof screens first.

### A. Town Overview

Target files:

- [src/ui/screens/town_overview.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/town_overview.rs)
- [src/ui/screens/town_overview_sections.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/town_overview_sections.rs)
- [src/ui/view_models.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/view_models.rs)

Current problem:

- the screen behaves like a dashboard of all tracked systems

Required rewrite:

- replace the current “many equal panels plus large nav block” structure with:
  - daily priority module
  - compact summary strip
  - reduced roster
  - footer bar

Specific tasks:

1. Add a `daily priority` view model helper.
2. Compress campaign/resources/debt/guest pressure into a compact summary band.
3. Reduce monster roster cards to identity + current assignment + condition summary + actions.
4. Remove the current oversized action panel pattern.
5. Rebuild navigation using the shared footer bar.
6. Make one action visually dominant based on current state.

Done means:

- the first thing the player sees is what matters today
- the overview no longer reads like five panels of equal weight

### B. GuildJobs Management

Target files:

- [src/ui/screens/guild_jobs.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/guild_jobs.rs)
- [src/ui/screens/guild_jobs_sections.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/guild_jobs_sections.rs)
- [src/ui/view_models.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/view_models.rs)

Current problem:

- room details, worker preview, and worker roster all compete at the same weight

Required rewrite:

- replace the current panel stack with:
  - selected room decision panel
  - `Assigned Here` list
  - `Available` list
  - footer bar with lateral shortcuts

Specific tasks:

1. Merge room details and worker preview into one selected-room panel.
2. Split workers into `Assigned Here` and `Available`.
3. Convert worker rows into shared entity cards.
4. Reduce worker summaries to one current-state line and one prediction line.
5. Add direct footer shortcuts to `Guest Desk` and `Town Planner`.
6. Move detached error text into the selected room or affected worker area where possible.

Done means:

- the screen reads like casting staff into roles, not scanning logs

## Phase 3: Comparison Screen Cleanup

## Guest Management

Target files:

- [src/ui/screens/guest_management.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/guest_management.rs)
- [src/ui/screens/guest_management_sections.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/guest_management_sections.rs)
- [src/ui/view_models.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/view_models.rs)

Required rewrite:

- selected request becomes the dominant panel
- candidate cards adopt shared entity card anatomy
- failure reasons become compact badges instead of wrapped prose
- campaign context gets demoted

Specific tasks:

1. Add request requirement chip helpers in `view_models.rs`.
2. Add candidate pass/fail badge data helpers.
3. Replace current text-heavy candidate rows with card layout.
4. Move reward/deadline/species/skill/history requirements into chips.
5. Route footer through shared footer bar.

## Expedition Planning

Target files:

- [src/ui/screens/expedition_planning.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/expedition_planning.rs)
- [src/ui/view_models.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/view_models.rs)

Required rewrite:

- priority selection becomes compact
- team composition becomes the dominant panel
- preview becomes a compact strip instead of a third full reading band

Specific tasks:

1. Add expedition risk and reward summary helpers.
2. Convert worker assignment rows into shared entity cards.
3. Replace the current large priority block with a compact segmented control style layout.
4. Color-code risk and success using semantic theme values.
5. Move footer to shared footer bar.

## Chamber Management

Target files:

- [src/ui/screens/chamber_management.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/chamber_management.rs)
- [src/ui/screens/chamber_management_sections.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/chamber_management_sections.rs)
- [src/ui/view_models.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/view_models.rs)

Required rewrite:

- eggs become more visual readiness cards
- review state becomes a proper decision panel
- `Close` becomes contextual wording

Specific tasks:

1. Add egg readiness status helpers.
2. Replace text rows with card-based egg list items.
3. Convert species outcomes into selectable cards with known/unknown/locked states.
4. Add direct forward shortcuts after hatch-ready or hatch-complete actions if flow allows.
5. Route footer actions through shared footer bar.

## Phase 4: Economy And Character Screens

## Town Management

Target files:

- [src/ui/screens/town_management.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/town_management.rs)
- [src/ui/view_models.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/view_models.rs)

Required rewrite:

- selected building panel becomes more visual and decision-led
- cost and unlocks become semantically readable
- progression becomes milestones instead of text summary

Specific tasks:

1. Add building affordability and unlock chip helpers.
2. Replace large text blocks with metric tiles and chip rows.
3. Add direct linked-destination affordances where a building unlocks another destination.
4. Move footer to shared footer bar.

## Monster Profile

Target files:

- [src/ui/screens/monster_profile.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/monster_profile.rs)
- [src/ui/view_models.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/view_models.rs)

Required rewrite:

- make current role and readiness more important than encyclopedic detail

Specific tasks:

1. Add best-next-use summary helpers.
2. Convert traits and conditions to chips/stat cards.
3. Reduce top band height and improve role emphasis.

## Main Menu, Opening, Settings

Target files:

- [src/ui/screens/opening.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/opening.rs)
- [src/ui/screens/settings.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/settings.rs)

Required rewrite:

- move to shared chrome and shared hierarchy rules

Specific tasks:

1. Opening gets stronger sequence emphasis and clearer primary action.
2. Settings adopts shared button classes and utility structure instead of raw button usage.

## Phase 5: Title Page Finish

Target files:

- [src/ui/screens/main_menu.rs](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens/main_menu.rs)
- `assets/data/ui_text.json`
- optionally [src/data/ui_text.rs](/H:/WebHatchery/RustGames/monsterhall/src/data/ui_text.rs) if type changes are needed

Current problem:

- the title page still reads like an MVP launcher
- supporting text includes implementation-facing language such as `This build...`
- the composition is functional but not yet persuasive

Required rewrite:

- give the title page its own final polish phase after shared chrome is stable
- make the copy sell the actual core gameplay loop
- make the title page visually consistent with the rest of the game UI while still feeling like an entry screen

Specific tasks:

1. Remove MVP/dev-build wording from title-page copy.
2. Rewrite support copy around the core loop:
   raise monsters, manage the brothel, build the tower, raid for growth, survive debt.
3. Rebalance the title composition so `New Campaign` is unquestionably dominant.
4. Keep the shared top utility bar, but make the rest of the screen feel less like a settings launcher and more like a finished game front door.
5. Ensure typography, spacing, and panel hierarchy match the shared system introduced in Phase 1.

## File-Level Checklist

## Must Add

- `src/ui/layout.rs`
- `src/ui/theme.rs`
- `src/ui/chrome.rs`
- `src/ui/components.rs`
- `src/ui/feedback.rs`
- optional follow-up: `src/ui/cards.rs` if `components.rs` grows too broad

## Must Update

- `src/ui/mod.rs`
- `src/ui/core.rs`
- `src/ui/view_models.rs`
- `src/ui/actions.rs`
- `src/ui/screens/mod.rs`
- `src/ui/screens/town_overview.rs`
- `src/ui/screens/town_overview_sections.rs`
- `src/ui/screens/guild_jobs.rs`
- `src/ui/screens/guild_jobs_sections.rs`
- `src/ui/screens/guest_management.rs`
- `src/ui/screens/guest_management_sections.rs`
- `src/ui/screens/expedition_planning.rs`
- `src/ui/screens/chamber_management.rs`
- `src/ui/screens/chamber_management_sections.rs`
- `src/ui/screens/town_management.rs`
- `src/ui/screens/monster_profile.rs`
- `src/ui/screens/main_menu.rs`
- `src/ui/screens/opening.rs`
- `src/ui/screens/settings.rs`

## Should Avoid

- large save-state changes in `src/state/game_state.rs` unless strictly needed
- one-off per-screen fixes before shared primitives land
- adding more screen-local helper files that duplicate future shared components

## Testing And Validation

Use the existing capture flow:

- [tmp_screens/play_ui_test.py](/H:/WebHatchery/RustGames/monsterhall/tmp_screens/play_ui_test.py)

Add validation expectations per phase:

### After Phase 1

- all screens still render
- no regressions in navigation
- top utility bar and footer bar render consistently on migrated screens

### After Phase 2

- capture `Town Overview` and `GuildJobs Management`
- verify the first visual focal point matches the intended dominant panel
- verify selected state is visible without text suffixes

### After Phase 3

- capture guest, expedition, and chamber screens
- verify compare-heavy entries no longer depend on wrapped prose
- verify semantic colors are doing real state work

### After Phase 4

- full screenshot pass across main gameplay flow
- update the review docs if the dominant-question rule is still being violated anywhere

### After Phase 5

- capture the main menu again
- verify no title-page copy references build state, prototype state, or implementation progress
- verify the page sells core gameplay immediately and feels consistent with the rest of the UI

## Recommended Working Order

1. Build shared primitives.
2. Migrate `Settings` and one simple screen to prove the new chrome.
3. Rebuild `Town Overview`.
4. Rebuild `GuildJobs Management`.
5. Rebuild the other compare-heavy screens.
6. Rebuild economy and character screens.
7. Do a dedicated title-page finish pass.
8. Do one final spacing/type/color consistency sweep.

## Completion Criteria

This plan is complete only when:

- shared UI primitives exist and are actually used
- `Town Overview` and `GuildJobs Management` clearly feel different from the current box-heavy style
- compare-heavy screens use shared card anatomy
- footer and top utility patterns are consistent
- selected and disabled states no longer depend on text labels alone
- the codebase is less screen-specific and more system-driven than it is today
- the title page copy no longer reads like MVP/internal-build messaging
