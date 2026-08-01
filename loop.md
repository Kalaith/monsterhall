# Monsterhall — Tower Depth Loop

Run with the `/loop` skill, e.g. `/loop Read loop.md in this directory and run exactly one iteration of it.`
(Add an interval like `/loop 45m ...` only if you want a wall-clock cadence; otherwise let it self-pace.)

---

## Mission

Monsterhall's **management engine is finished**. Assignments, day resolution, contracts, patrons,
debt milestones, guild rooms, buildings, eggs, mutations, events, and a 365-day balance harness all
work. What the game is short on is **tower**: the thing the entire fiction is about is three floors
deep and runs out before the debt chain does.

Target: **a 25-floor tower**, authored as five bands of five, each band a place with its own hazards,
species, rewards, gate, and reasons to care — and the systems around it (missions, species, buildings,
contracts, events, prose) grown to match. Build it so floor 26 is a data entry, not a refactor.

Every iteration must leave the tower **deeper or denser**, and must leave the game playable and
balanced. This is a game heading for a paid itch.io release: unfinished-feeling content is worse
than absent content.

## Prime directive: content depth over new systems

- Default to **data-only work** in `assets/data/*.json`. That is the win condition, not a fallback.
- Touching Rust is allowed when — and only when — the content *needs* it:
  - a `#[serde(default)]` field on an existing schema struct so authored content can say something
    the schema can't yet express,
  - a surface that shows authored content the player currently can't see (a floor list that scrolls,
    a journal note, a preview row),
  - a curve that was tuned for 3 floors and breaks at 25,
  - a bug the new content exposes.
- **Do not** add a new subsystem, a new resource, or a new player verb. If an idea needs one, write it
  into **Deferred** at the bottom and pick something else.
- Breadth without texture is not depth. Twenty-five floors that differ only by a `difficulty` number
  are one floor printed twenty-five times.

## Every floor must earn its place

Before authoring a floor, answer these in one line each in your iteration notes. If you can't, the
floor isn't designed yet.

1. **What does it do that no other floor does?** (a hazard family, a species pool, a reward skew,
   a mission it uniquely permits)
2. **What gates it, and what does the player have to have built or grown to get in?**
   (`requires_building_ids`, `required_roster`, and the depth chain — not just "the last one")
3. **What decision does it create?** (a run worth risking your best scout on, a floor you skip
   because the shallower one pays your debt this week)
4. **How does the player learn it exists before they can enter it?** (a journal line, an event, a
   contract that names it, a building description that promises it)

A floor with no inbound reference is invisible content. Wire it both ways.

---

## Where the depth is thin (measured 2026-08-01)

Re-count these from the JSON each iteration rather than trusting the table.

| Axis | Now | Target | Notes |
|---|---|---|---|
| **Tower floors** | **3** | **25** | `assets/data/floors.json`, depth 1–3, difficulty 20/34/48. The whole point of the game. |
| Missions | 4 | 8–10 | GDD names 6 types; `missions.json` has 4. Rescue/Retrieval and Contract Fulfilment are unwritten. |
| Species | 8 | 14–18 | Deep bands need companions you can only get down there. |
| Mutations | 3 | 10+ | The corruption payoff. One mutation per two species is thin. |
| Buildings | 12 | 20+ | Buildings are the only floor gate today (see below). |
| Guild rooms | 4 | 7–8 | Four rooms for a 20-companion roster. |
| Traits | 10 | 16+ | Traits drive contract fit and role assignment. |
| Contracts | 12 | 25+ | Should reference named floors and deep species. |
| Events | 38 | — | Healthy. Deep-floor events are the gap, not event count. |
| Patron tiers | 3 | 4–5 | `patron_tiers.json`; upkeep bands already reference tier 4. |
| Debt milestones | 7 | — | ~455 days of chain against 3 floors of tower. Mismatched, in the tower's favour to fix. |
| Relic drops | declared, **never read** | wired | `relic_drop_ids` parses in `types.rs:457` and nothing consumes it. Named relics are free depth. |

### Standing themes, several iterations each

- **Band identity** — five floors that share a biome, hazard family and species pool, with an
  escalating reason to go deeper inside the band. Author a band as a unit, not floor-by-floor.
- **Deep species** — species whose eggs only appear below a certain depth, so the roster you finish
  with is evidence of how deep you got.
- **Missions as stances** — the four current missions bias materials/eggs/relics/residue. Deep floors
  want missions that only make sense down there (rescue, sealed extraction, mapping a route home).
- **Relics as objects** — turn `relic_drop_ids` from dead data into named finds with descriptions,
  journal entries, and a reason a patron wants one.
- **Corruption and mutation** — deep floors already push `corruption_pressure`; the mutation table has
  three entries. Deep runs should visibly change the companions who make them.
- **Prose** — floor `description`, `hazard_tags`, `ui_text.json`, journal, event text. The tower is
  supposed to feel dangerous and old. Prose is content.

---

## Ordered phases

Do not author band 2 before the scaffolding in phase 0 is done — 25 floors on today's plumbing
produces an unplayable game and a balance harness that fails for the wrong reasons.

### Phase 0 — make 25 floors possible (do these first, roughly in order)

These are the known hard blockers. Each is one iteration or less.

1. ~~**Floor list is capped at four.**~~ **Done 2026-08-01** — paged window in
   `expedition_planning_sections.rs`, see Ledger. Still open, and the natural next slice: the
   **roster lists**. `expedition_planning.rs` team panel and `contract_desk_sections.rs:556` both
   `.take(6)` against a 20-companion population cap, so most of a late-game roster cannot be
   assigned; `town_management.rs:142` takes 10 of the buildings, which will bite as buildings grow
   past 20. The team panel is height-bounded 2-column cards, so it needs paging of its own shape
   rather than a copy of the floor list.
2. ~~**Floors can only be unlocked by buildings.**~~ **Done 2026-08-01** — survey chain in
   `day_cycle/surveys.rs`. Floors declare `requires_surveyed_floor_ids` + `required_surveys`;
   expeditions bank survey points on the floor they ran (missions carry `survey_value`, scout route
   is worth 2); a daily sweep opens any floor whose chain and buildings are both satisfied.
   **No floor uses it yet** — band 1 is its first real customer, and authoring those floors is what
   proves the mechanism against real data.
3. **The depth curve is linear and tuned for 3.** In `src/engine/depth.rs:139`,
   `hazard_risk = depth * 2 + hazard_tags.len() * 3 + mission modifier`, and it feeds
   `success_bonus -= hazard_risk / 3` and `injury_risk_delta`. At depth 25 that is ~50 hazard before
   tags, against `expedition_injury_threshold: 55` and `expedition_relic_reward_threshold: 88` in
   `config.json`. Reshape it (band-relative depth, diminishing curve, or preparation scaling that
   grows with the tower) so deep floors are hard, not arithmetically impossible.
4. **`relic_bonus` hard-codes `floor.depth >= 3`** (`depth.rs:165`). Make it data or band-relative.
5. **Save compatibility.** `config.json` carries `save_version: 9` and `content_version: 1.9.0`.
   Any new state field must be `#[serde(default)]` and wired through `state/persistence.rs`, and
   `validate_game_state_references` (`engine/validation.rs`) must still accept old saves. Bump
   `content_version` when content lands; bump `save_version` only when the save shape actually changes.

### Phase 1 — author the bands

Five bands of five floors. Suggested shape, adjust freely, but decide the whole band before writing
the first floor of it:

| Band | Depths | Existing anchor | Identity to establish |
|---|---|---|---|
| 1 | 1–5 | `floor_1_slick_cellars` | Service tunnels. Salvage, residue slicks, crawlspaces. Teaches the loop. |
| 2 | 6–10 | `floor_2_molten_baths` | Heat and vents. Preparation punishes the careless. |
| 3 | 11–15 | `floor_3_gilded_kennels` | Ruined luxury, beast pens, ambush. Escorts and handlers matter. |
| 4 | 16–20 | — | New. Something the guild is not ready for: corruption, sealed archives, the tower's own logic. |
| 5 | 21–25 | — | New. The endgame band. `tower_core` already exists as a magic egg source (`day_cycle/eggs.rs:7`) — that thread wants paying off. |

Per band, one iteration each for: the five floors themselves; the species/eggs that only drop there;
the building or room that gates it; the contracts/events/journal prose that point at it.

### Phase 2 — density

Once the shaft is 25 deep: missions, relics as named objects, mutations, traits, patron tier 4–5,
deep-floor events, and the prose pass. Rotate axes; don't grind one.

---

## One iteration

### 1. Orient (cheap)
- `git status` and `git log --oneline -8` — know what the last iteration did.
- Read the **Ledger** at the bottom. Do not repeat the last two iterations' axis.
- Re-count the thin axes from the JSON before trusting the table above.
- Confirm which phase you're in. Phase 0 blockers outrank everything else.

### 2. Choose one slice
Pick **one** coherent slice, sized to finish and verify in a single iteration. Good sizes:
- one phase-0 blocker, end to end;
- one band of five floors, with their gates, hazards, egg pools and prose;
- 2–3 species that share a band, with their eggs, traits, preferred rooms and hatching costs;
- one mission type plus the floors that permit it and the contracts that ask for it;
- one relic family: named drops, the floors they come from, the patrons who want them, journal text;
- one building/room that gates a band, with its costs, unlocks and description.

### 3. Author it, data-first
Write it into `assets/data/`. Match the existing entries' shape and voice exactly — read three
neighbours before adding a fourth. Balance numbers belong in JSON, never as constants in Rust
(`CODE_STANDARDS.md`, data-driven design is a hard rule).

### 4. Wire it in
New content must be reachable and referenced. Typically: a gate that opens it, an egg pool that
includes it, a contract or event that names it, a journal or UI line that tells the player it exists.

### 5. Verify
- `cargo fmt -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test` — this includes the **balance harness** in
  `src/engine/validation/validation_tests/` (30 / 90 / 180 / 365-day simulations with policy
  assertions on roster size, population cap, debt survival and expedition cadence). New floors will
  move those numbers. **Re-tune the content or the thresholds deliberately and say which you did —
  never delete or loosen an assertion to make a red test green.**
- Extend `data/depth_validation.rs` / `data/validation/*` when new content encodes a rule worth
  protecting (a gate that must reference a real floor, a band that must have five members, an egg
  pool that must be non-empty).
- Visual check via the capture harness when the slice changes something on screen —
  `..\macroquad-toolkit\scripts\capture_ui.ps1 -Scenes <scene>`.
- Then run `.\publish.ps1` from this directory. That is the sanctioned end-to-end validation path per
  `AGENTS.md`; report honestly if it fails.

### 6. Commit
One commit per iteration, in the catalog's style: subject narrates the change in Monsterhall's own
voice and ends with a plain-terms parenthetical tag; body is honest prose — problem, change,
reasoning. No `feat:`/`fix:` prefixes. See `rust_management/docs/COMMIT_STYLE.md`.

### 7. Log it
Append one line to the **Ledger**: date, axis, what landed, what the next iteration should know
(especially: which balance numbers moved). Move anything deliberately skipped into **Deferred**.
Tick off anything this closed in `TODO.md`. Keep both lists tight — this file is the loop's memory.

---

## Hard constraints (violating these breaks the build or the game)

- **Every id referenced must exist.** `data/validation/*.rs` and `data/depth_validation.rs` run at
  load and fail hard: floor→building, floor→species, floor→mission, contract→trait, room→building,
  contract→follow-up. A typo'd id is a startup crash, not a warning.
- **Roles and niches are a closed set.** `depth_validation.rs:95` accepts only `comfort`,
  `performance`, `hatchery`, `corruption`, `corruption_adept`, `hatchery_specialist`, `performer`,
  `delver`, `versatile`. Adding a role means adding it there *and* to `monster_role()` in
  `engine/depth.rs:33`, which infers roles from stats — a role nothing infers is a dead role.
- **Hazard tags may not be blank**, and events with situation pressure must set `situation_days`
  (`depth_validation.rs:62`, `:68`).
- **New save fields must be `#[serde(default)]`** and wired through `state/persistence.rs`, so
  existing saves keep loading.
- **Keep the day cycle deterministic under seed.** Randomness goes through the existing
  `gen_range` helpers and seeded simulation (`seed_simulation` in the validation tests) — never a
  fresh RNG, never wall-clock time.
- **800-line limit on every `.rs` file**, non-test lines. Six files are already at 600–790
  (`day_cycle/resolution.rs` 790, `engine/guest.rs` 779, `ui/view_models.rs` 775,
  `town_overview_sections.rs` 738). If your change touches one, extract a cohesive responsibility as
  part of the task. **No new `mod.rs`.**
- **The vocabulary is settled — don't reintroduce the old one.** This game was reskinned from an
  earlier premise, and a rename pass (2026-08-01) removed the leftovers: ids now match their display
  names, companions are *companions* (never "girls"), patrons are *patrons* (never "clients"), and
  contracts are *contracts* (never "guest requests"). The one-way migration map lives in
  `state/persistence.rs` (`RENAMED_CONTENT_IDS`) — **it is history, not a naming reference.** Author
  new ids in the current fiction, and make a new id read the same as the name the player sees.
- **Never write anything under `D:\xampp\htdocs`** — it is a publish target only.
- Art is mostly placeholder (`TODO.md`): only backdrops and one icon atlas exist. New species carry a
  `portrait_key`; expect it to render as placeholder and don't block on art.

## Reference

- `monsterhall_gdd.md` — the design document. Floors, missions and species should serve it; it
  already names mission types and building categories that don't exist yet.
- `TODO.md` — the standing backlog. Several items are balance questions this loop will answer.
- `docs/UI_STYLE_GUIDE.md`, `docs/UI_THEME_SHEET.md` — layout, hierarchy, copy rules, palette.
- `AGENTS.md`, `CODE_STANDARDS.md` — project rules and the validation path.
- `README.md` — the stated premise; content should serve it.

## Stop conditions

Stop the loop and report if:
- `publish.ps1` fails twice for the same reason;
- the balance harness can't be satisfied without loosening an assertion — that's a design decision,
  bring it to the user;
- a phase-0 blocker turns out to need a genuinely new system rather than a schema field;
- the same slice fails verification twice.

---

## Ledger

<!-- One line per iteration. Newest at the bottom. -->

- **2026-08-01 — phase 0, blocker 1 (floor list cap).** `expedition_planning.rs` drew
  `available_floors.take(4)`; replaced with a paged window in a new
  `expedition_planning_sections.rs`. The page is derived from the selected floor's index, not
  stored — no new state, no new `UiAction`, no save impact. Panel fits 6 rows; past 6 floors the
  last row becomes a `<` / `>` pager with an "n-m of N" label, stepping a whole page. Rendering for
  ≤6 floors is byte-identical to before, so today's 3-floor tower is unchanged. 8 unit tests on the
  window logic (34 total, was 26). **Next iteration should know:** (a) the roster lists are still
  capped — `expedition_planning.rs` team panel and `contract_desk_sections.rs:556` both `.take(6)`
  against a 20-companion cap, and that panel is height-bounded 2-column cards so it needs a
  different treatment than the floor list; (b) the capture harness photographs **only the main
  menu** (`src/main.rs:83` — the scene env var is parsed but unused), so no screen except the menu
  can be visually verified; every UI slice in this loop is unit-test-verified only until that is
  fixed. Balance numbers unmoved (pure UI).

- **2026-08-01 — phase 0, blocker 2 (floor unlock chain).** Floors could only be opened by
  `building.unlocks.floor_ids`, which cannot scale to 25. Added a survey chain: `TowerFloorData`
  gains `requires_surveyed_floor_ids` + `required_surveys` (both `#[serde(default)]`),
  `MissionData` gains `survey_value` (default 1; `scout_route` set to 2, giving that mission its
  first unique reason to exist), `PlayerTownState` gains `floor_surveys` (`#[serde(default)]`, so
  old saves load untouched). `day_cycle/surveys.rs` records a survey when an expedition pays out
  and sweeps once per day to open floors whose chain **and** `requires_building_ids` are satisfied
  — which also makes `requires_building_ids` live data for the first time; it was previously
  validated and never enforced. Load-time validation rejects a chain naming an unknown floor,
  itself, or zero surveys. **Also extracted `day_cycle/upkeep.rs`** (5 upkeep functions, 171 lines)
  because `resolution.rs` sat at 792 against the 800 hard limit and had no room for the hook; it is
  now 625. 9 tests added (43 total, was 34) — 8 unit tests on the chain, plus one that runs the
  real 30-day cycle and asserts surveys actually accumulate, because the unit tests would all pass
  even if the `resolve_day` hook were dead code. Balance reports byte-identical: the mechanism is
  dormant until a floor declares a chain. **Next iteration should know:** phase 0 items 3 and 4
  (the linear depth curve, and `relic_bonus`'s hard-coded `depth >= 3`) are the last blockers
  before band authoring, and both are in `engine/depth.rs`; they are worth doing together since
  both are the same "tuned for 3 floors" problem.

## Deferred (needs a new system or a decision; not for this loop)

- Retiring `RENAMED_CONTENT_IDS` in `state/persistence.rs` once no pre-`save_version: 10` saves are
  plausibly in circulation. Harmless to keep; delete it at the 1.0 release, not before.
- Renaming the `guest` vocabulary that survives in code (`engine/guest.rs`, `guest_appeal`,
  `guest_name_template`, `NavContracts`' underlying flow) — a guild hall legitimately has guests, so
  this is a judgement call about whether "guest" and "patron" are the same thing. ~400 sites; decide
  the model first.
- Full art pass: portraits, room/floor thumbnails, patron art, egg art, story CGs (`TODO.md`).
- Named adventurer parties, rival guilds, factions, gear loadouts, tower bosses — all listed as
  expansion opportunities in the GDD; all new systems.
- Direct expedition control or combat. The game is explicitly about preparation, not fighting.
