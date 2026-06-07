# Screens Reduction Implementation Plan

## Goal
Reduce [`src/ui/screens.rs`](/H:/WebHatchery/RustGames/monsterhall/src/ui/screens.rs) from 2764 lines to under 1500 lines without changing game behavior, while moving the codebase toward the standards in [`CODE_STANDARDS.md`](/H:/WebHatchery/RustGames/monsterhall/CODE_STANDARDS.md).

This plan treats `< 1500` as a checkpoint, not the final clean-state target. `CODE_STANDARDS.md` sets a hard limit of 800 lines and recommends splitting by responsibility much earlier.

## Current State
- `src/ui/screens.rs` is 2764 lines.
- The file mixes four responsibilities that should not live together:
  - UI action definitions
  - shared drawing helpers
  - screen rendering
  - screen-facing formatting and lookup helpers
- Several draw functions exceed the function-size guidance in `CODE_STANDARDS.md`:
  - `draw_town_overview`: 445 lines
  - `draw_guest_management`: 356 lines
  - `draw_chamber_management`: 311 lines
  - `draw_guild_jobs`: 276 lines
  - `draw_expedition_planning`: 258 lines
- `src/data/ui_text.rs` and [`assets/data/ui_text.json`](/H:/WebHatchery/RustGames/monsterhall/assets/data/ui_text.json) already exist and are loaded, but `screens.rs` still contains player-facing copy and formatted UI labels.

## Refactor Constraints
- Follow `CODE_STANDARDS.md`:
  - split by responsibility
  - keep UI code dumb
  - move player-facing static text out of screen code
  - prefer straightforward, maintainable structure
- Preserve behavior during the reduction pass.
- Do not mix this refactor with gameplay changes.
- Keep `main.rs` and `game.rs` call sites simple by preserving the current public draw API until the split is complete.

## Reduction Strategy
The fastest safe path to `< 1500` is not micro-cleanup inside `screens.rs`. It is extracting entire responsibility groups out of the file in a sequence that gives immediate line-count wins and cleaner ownership.

String migration must happen before most file splitting. If screen modules are extracted first while copy remains embedded, the project will just spread hardcoded player-facing text across more files and make the cleanup slower.

Target module structure:
- `src/ui/actions.rs`
- `src/ui/view_models.rs`
- `src/ui/components/hover_card.rs`
- `src/ui/screens/`
  - `mod.rs`
  - `loading.rs`
  - `main_menu.rs`
  - `opening.rs`
  - `town_management.rs`
  - `day_results.rs`
  - `settings.rs`

This first pass is intentionally scoped to:
- centralize player-facing static copy in JSON first
- then move enough code out of `screens.rs` to get below 1500 lines
- then continue toward the standards-compliant end state

## Measured Extraction Opportunities
- Shared top-level helpers at the top of `screens.rs`: about 123 lines
  - `format_resource_cost`
  - `describe_building_effects`
  - `is_mouse_over_rect`
  - `draw_hover_card`
- Screen-facing formatting and lookup helpers at the bottom: about 394 lines
- Smaller screen draw functions:
  - `draw_loading_screen`: 50 lines
  - `draw_main_menu`: 105 lines
  - `draw_opening_chapter`: 144 lines
  - `draw_town_management`: 182 lines
  - `draw_day_results`: 146 lines
  - `draw_settings_modal`: 88 lines

If these groups are extracted cleanly, the remaining monolith should land around 1530 lines before import cleanup. That is close, but not yet safe. The plan therefore includes one additional reduction step inside a large screen area rather than relying on formatting noise to cross the line.

## Implementation Phases

### Phase 1: Complete String Migration First
Finish the remaining migration of player-facing static text from `src/ui/screens.rs` into [`assets/data/ui_text.json`](/H:/WebHatchery/RustGames/monsterhall/assets/data/ui_text.json) and keep [`src/data/ui_text.rs`](/H:/WebHatchery/RustGames/monsterhall/src/data/ui_text.rs) in sync.

Priority order:
- shared labels
- onboarding lines
- main menu text
- opening chapter text
- town overview labels
- town management labels
- brothel management labels
- guest management labels
- chamber management labels
- expedition planning labels
- day results labels
- settings labels

Why:
- this enforces the clean-coding rule before the file is split across modules
- it prevents hardcoded copy from being redistributed into new files
- it reduces future review noise because wording changes stay in one data source

Acceptance:
- no new screen module is created with embedded player-facing static text
- all migrated text is loaded through `data.ui_text`
- validation is updated for every newly added required string

### Phase 2: Extract Actions
Create `src/ui/actions.rs` and move `UiAction` there.

Why:
- removes one non-rendering responsibility from `screens.rs`
- makes screen modules easier to import independently

Acceptance:
- `UiAction` is no longer defined in `src/ui/screens.rs`
- `src/ui/mod.rs` re-exports `UiAction`

### Phase 3: Extract Screen View Models
Create `src/ui/view_models.rs` and move screen-facing formatting and lookup helpers there.

Move:
- resource formatting
- name lookup helpers
- guest requirement labels
- egg summary helpers
- onboarding line assembly
- sex skill and history summary helpers

Why:
- this is pure formatting/read-only logic, not rendering
- it directly matches the standards rule to split by responsibility

Acceptance:
- the bottom helper block is removed from `src/ui/screens.rs`
- screen modules import formatting helpers instead of defining them locally

### Phase 4: Extract Shared Hover/Card Helpers
Create `src/ui/components/hover_card.rs` for hover-card behavior and its mouse-hit helper.

Move:
- `is_mouse_over_rect`
- `draw_hover_card`

Optional:
- keep `format_resource_cost` and `describe_building_effects` in `view_models.rs`

Acceptance:
- no generic component helper remains embedded in `src/ui/screens.rs`

### Phase 5: Split Low-Risk Screens First
Create `src/ui/screens/` and move these draw functions into dedicated files:
- `loading.rs`
- `main_menu.rs`
- `day_results.rs`
- `settings.rs`

Why this order:
- these screens are smaller
- they have fewer cross-screen dependencies
- they are the safest place to prove the module pattern

Acceptance:
- each file owns one draw function and any screen-local constants only
- `src/ui/screens/mod.rs` re-exports the same public draw functions used today

### Phase 6: Move the Two Medium Screens Needed To Cross the Target
Move:
- `opening.rs`
- `town_management.rs`

Why:
- together they remove enough code to make the line count reduction deterministic instead of marginal
- `town_management` also carries building-description pressure that belongs beside its own screen

Acceptance:
- `src/ui/screens.rs` drops below 1500 lines
- no draw function longer than 100 lines is introduced in the new files without being further split into local helpers

### Phase 7: Reduce One Large Remaining Screen Internally
Do not stop the moment the file count is below 1500. Use the same pass to split one large remaining screen into local helper functions inside its own eventual module.

Best candidate:
- `draw_town_overview`

Suggested helper slices:
- top summary panels
- debt panel
- guest pressure panel
- onboarding panel
- action button panels

Why:
- `draw_town_overview` is the largest function in the file
- it most clearly violates the function-size guidance
- reducing it early lowers future merge risk

Acceptance:
- the selected screen is no longer one scrolling function
- helper functions are screen-local and named by panel responsibility

## Line Budget
Expected reduction after Phases 2 through 6:
- actions extraction: about 40 lines
- view-model extraction: about 394 lines
- component helper extraction: about 31 lines
- low-risk screen split: about 389 lines
- opening split: about 144 lines
- town management split: about 182 lines

Projected remaining size:
- `2764 - (40 + 394 + 31 + 389 + 144 + 182) = about 1584 lines`

That is still too high. To make the target deterministic, Phase 7 must also remove at least 85 more lines from `screens.rs`.

Recommended deterministic target:
- extract `draw_town_overview` next instead of partially slicing it in place

Safer practical target after adding that step:
- `1584 - 445 = about 1139 lines`

This is the recommended stopping point for the reduction pass because it gives margin for imports, comments, and future edits instead of landing just below the threshold.

## Clean Coding Rules For The Work
- One file owns one concept.
- One screen module owns one screen.
- Shared drawing helpers belong in `components/`.
- Shared formatting helpers belong in `view_models.rs`.
- Screen modules may read state and return `Option<UiAction>`, but must not mutate game state.
- New functions should target 20 to 50 lines and stay below 100 lines.
- Static player-facing strings belong in `assets/data/ui_text.json`, not in Rust screen code.
- Do not introduce fallback text literals in draw functions except for development-only error paths.

## Acceptance Checklist
- [ ] `src/ui/screens.rs` is under 1500 lines
- [ ] string migration is completed before the majority of screen extraction begins
- [ ] `UiAction` has been moved out of `src/ui/screens.rs`
- [ ] formatting and lookup helpers have been moved out of `src/ui/screens.rs`
- [ ] hover-card logic has been moved into a reusable component module
- [ ] at least six draw functions have been moved into `src/ui/screens/`
- [ ] extracted screens use `data.ui_text` for static copy
- [ ] `cargo fmt` passes
- [ ] `cargo clippy` runs with no new warnings introduced by the refactor
- [ ] the game still builds and screen transitions still work

## Follow-Up After The `< 1500` Checkpoint
Reaching `< 1500` should immediately be followed by the full standards-compliant split:
- move `guild_jobs`
- move `guest_management`
- move `chamber_management`
- move `expedition_planning`
- delete or reduce `src/ui/screens.rs` to a thin module export layer only

Final desired state:
- no UI monolith remains
- no screen file exceeds the standards hard limit
- no screen draw function is a multi-hundred-line procedure

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
