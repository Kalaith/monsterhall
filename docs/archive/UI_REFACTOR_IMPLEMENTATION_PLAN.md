# UI Refactor Implementation Plan

## Goal
Refactor the current UI code so it is:
- split by screen/domain instead of one giant file
- built from reusable view helpers instead of repeated manual panel/button code
- driven by JSON text content instead of embedded player-facing strings in Rust
- easier to extend without growing another monolithic `screens.rs`

This plan treats the current [`src/ui/screens.rs`](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens.rs) as transitional code to be broken apart.

## Current Problems
- `src/ui/screens.rs` is too large and mixes unrelated responsibilities.
- Player-facing copy is embedded directly in Rust.
- Screen functions repeat the same panel/header/status/button-row patterns.
- Formatting helpers, lookup helpers, onboarding copy, and screen rendering live in the same file.
- Layout constants are hardcoded inside each screen instead of being organized into reusable patterns.

## Target Structure

### `src/ui/mod.rs`
- thin exports only
- no screen logic

### `src/ui/actions.rs`
- `UiAction`
- only UI event definitions

### `src/ui/view_models.rs`
- screen-facing formatting helpers
- no drawing
- examples:
  - resource labels
  - debt summary labels
  - guest requirement labels
  - egg summaries
  - monster summaries

### `src/ui/components/`
- reusable drawing blocks
- examples:
  - `top_bar.rs`
  - `summary_panels.rs`
  - `navigation_grid.rs`
  - `monster_card.rs`
  - `egg_card.rs`
  - `request_card.rs`
  - `status_box.rs`
  - `hover_card.rs`

### `src/ui/screens/`
- one file per screen
- examples:
  - `loading.rs`
  - `main_menu.rs`
  - `opening.rs`
  - `town_overview.rs`
  - `town_management.rs`
  - `guild_jobs.rs`
  - `guest_management.rs`
  - `chamber_management.rs`
  - `expedition_planning.rs`
  - `day_results.rs`
  - `settings.rs`

### `src/data/ui_text.rs`
- UI text catalog data structures
- deserialized from JSON

### `assets/data/ui_text.json`
- all player-facing static text
- screen titles
- subtitles
- panel titles
- button labels
- empty-state text
- onboarding text
- generic labels like `Gold`, `Materials`, `Return To Town`

## Rules

### Rule 1: No embedded player-facing text in screen code
Allowed in Rust:
- ids
- internal debug/error-only text during development if not user-facing
- format templates that combine runtime values with JSON-provided label fragments only if needed

Not allowed in Rust:
- panel titles
- menu labels
- tutorial lines
- empty-state messages
- descriptive text
- repeated stat labels like `Gold`, `Debt`, `GuildJobs`, `Return To Town`

### Rule 2: One screen file owns one screen
Each screen file may:
- define its draw function
- use shared components
- use view-model formatting helpers

Each screen file should not:
- define unrelated helper stacks
- define copy catalogs inline
- carry formatting helpers for other screens

### Rule 3: Repeated layout patterns become components
If a panel/button/status pattern appears in more than one screen, move it into `src/ui/components/`.

### Rule 4: Screen code should compose, not format raw domain state directly
Formatting should happen in view-model helpers before draw calls where practical.

## Implementation Steps

### Step 1: Introduce UI text data domain
- Add `UiTextData` structs in `src/data/ui_text.rs`
- Add `assets/data/ui_text.json`
- Wire it into the main data loader
- Start with these sections:
  - `common`
  - `main_menu`
  - `town_overview`
  - `town_management`
  - `guild_jobs`
  - `guest_management`
  - `chamber_management`
  - `expedition_planning`
  - `day_results`
  - `settings`

Acceptance:
- game loads all UI text from JSON
- missing required UI text fails validation clearly

### Step 2: Extract `UiAction`
- Move `UiAction` out of `screens.rs` into `src/ui/actions.rs`
- Update imports everywhere

Acceptance:
- no action enum remains inside screen-rendering files

### Step 3: Extract shared format/view helpers
- Move these kinds of helpers out of `screens.rs` into `src/ui/view_models.rs`:
  - resource formatting
  - species/room/debt name lookup wrappers
  - guest requirement labels
  - sex skill/history summaries
  - egg labels
  - onboarding line assembly

Acceptance:
- `screens.rs` no longer contains domain formatting helpers at the bottom

### Step 4: Extract shared components
- Create reusable components for:
  - settings button/top bar
  - standard screen subtitle block
  - summary panel rows
  - two-column navigation grid
  - error/status panel
  - hover card

Acceptance:
- repeated panel/button code is replaced by component calls on at least 3 screens

### Step 5: Split `screens.rs` by screen
- Move each draw function into `src/ui/screens/<screen>.rs`
- Keep exact behavior during the split

Suggested order:
1. `loading`
2. `main_menu`
3. `settings`
4. `day_results`
5. `town_overview`
6. `town_management`
7. `guild_jobs`
8. `guest_management`
9. `chamber_management`
10. `expedition_planning`
11. `opening`

Acceptance:
- original `screens.rs` is deleted or reduced to module exports only

### Step 6: Replace embedded static text with JSON references
- Screen by screen, replace literals with `data.ui_text...`
- Keep dynamic runtime values formatted using JSON-backed labels

Examples:
- `"Town Actions"` becomes `data.ui_text.town_overview.town_actions_title`
- `"Return To Town"` becomes `data.ui_text.common.return_to_town`
- onboarding/tutorial lines move entirely to JSON

Acceptance:
- no direct player-facing static strings remain in screen modules

### Step 7: Normalize screen-local layout
- Each screen gets clearly named layout constants near the top
- Repeated magic numbers should become grouped local constants or component defaults

Acceptance:
- no screen function should read like a raw list of unrelated coordinates

### Step 8: Add validation and tests for UI text coverage
- Validate required JSON sections/fields at load time
- Add focused tests for:
  - missing required UI text sections
  - empty button labels
  - missing screen title strings

Acceptance:
- text-domain mistakes fail fast instead of silently falling back to Rust literals

## Recommended Delivery Order

### Phase 1: Infrastructure
- UI text JSON domain
- `UiAction` extraction
- shared view-model helpers

### Phase 2: Easy screen split
- loading
- main menu
- settings
- day results

### Phase 3: Core management screens
- town overview
- town management
- brothel management

### Phase 4: Remaining operational screens
- guest management
- chamber management
- expedition planning
- opening chapter

### Phase 5: Hardening
- delete legacy `screens.rs`
- validate all text content
- final string audit

## String Migration Checklist
- [ ] main menu text moved to JSON
- [ ] opening chapter text moved to JSON
- [ ] town overview labels moved to JSON
- [ ] town management labels moved to JSON
- [ ] brothel management labels moved to JSON
- [ ] guest management labels moved to JSON
- [ ] chamber management labels moved to JSON
- [ ] expedition planning labels moved to JSON
- [ ] day results labels moved to JSON
- [ ] settings labels moved to JSON
- [ ] onboarding lines moved to JSON
- [ ] shared labels moved to JSON

## Completion Criteria
- `src/ui/screens.rs` no longer exists as a monolith
- all player-facing UI text is loaded from `assets/data/ui_text.json`
- screen modules are short enough to reason about independently
- shared layout patterns are components instead of copy-pasted blocks
- adding a new screen or changing wording no longer requires editing a giant mixed-responsibility file
