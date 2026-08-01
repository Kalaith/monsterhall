# Monsterhall — Tower Depth Loop

Run with the `/loop` skill, e.g. `/loop Read loop.md in this directory and run exactly one iteration of it.`
(Add an interval like `/loop 45m ...` only if you want a wall-clock cadence; otherwise let it self-pace.)

---

## Mission

Monsterhall's **management engine is finished**. Assignments, day resolution, contracts, patrons,
debt milestones, guild rooms, buildings, eggs, mutations, events, and a 365-day balance harness all
work. What the game is short on is **tower**: the thing the entire fiction is about is three floors
deep and runs out before the debt chain does.

Target: **a 25-floor tower** — **reached 2026-08-02**. Authored as five bands of five, each band a place with its own hazards,
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
| **Tower floors** | **25** ✅ | **25** | **Target met.** All five bands authored, depths 1–25, difficulty 20→104 against a ceiling of 120. Every floor unlocks and is reached in a 365-day campaign; the guild's preferred destination is the Tower Core at the bottom. |
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
| Relic drops | **28 named** ✅ | named objects | `relics.json` gives every declared drop a name, description and discovery note, reported in the day log when found. Validated both ways: no unknown drop, no unfindable relic. Still no *patron* who wants one — that is the remaining half. |

### How the planner values a floor — read this before authoring one

Measured from `policy_jobs::expedition_growth_score`, and it is not intuitive:

| term | worth |
|---|---|
| **an egg** | **120–180** (180 when the inventory is empty, 15 once pending eggs cover demand) |
| **a relic** | **70** |
| a material | **2** |
| a residue | **1** |
| injury risk | **−2 per point** |

So a floor is chosen on **eggs and relics**; materials are almost noise and residue is nothing.
Raising Menagerie Walk from 70 to 98 materials changed the campaign by literally zero — one relic is
worth 35 materials. Injury scales with difficulty, so a deep floor must pay in eggs and relics just
to break even.

**Rewards must rise monotonically with depth.** Injury climbs every floor, so a floor that pays less
than the one above it while costing more is simply skipped — and because a chain needs the *deepest
known* floor surveyed, one skipped floor stalls everything beneath it. Band 4 stalled outright until
its eggs and relics were made to ramp 2→3 and 8→12 across depths 16–20.

**A band's deepest floor is the doorway to the next band**, so it must be the best prize in its band,
not its hardest afterthought. `auction_floor` (d15) sat unrun until it was raised above `broodpens`
above it; that single change opened band 4.

**And the filter that hides floors entirely:** when the guild wants eggs the planner skips every
mission whose `reward_focus` is not eggs. A floor with no `egg_hunt` in `mission_ids` is invisible
for most of the campaign no matter what it pays. Adding `egg_hunt` to three band-3 floors took the
tower from stalling at depth 12 to running all fifteen floors and producing eight rank-5 companions.

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
3. ~~**The depth curve is linear and tuned for 3.**~~ **Done 2026-08-01** — coefficients moved to
   `config.json`, and survey familiarity now subtracts from hazard, so investment scales with the
   tower. See the Ledger for the reward-threshold ceiling that band authoring must respect.
4. ~~**`relic_bonus` hard-codes `floor.depth >= 3`.**~~ **Done 2026-08-01** — a floor yields relics
   when it declares `relic_drop_ids`, which also turns that dead field live.
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

- **2026-08-01 — phase 0, blockers 3 + 4 (depth curve and relic gate).** Reading the real math first
  changed the diagnosis: `success_score` subtracts `floor.difficulty` and `injury_risk_score` adds
  it, so **authored `difficulty` is the dominant depth dial**, not the engine's `depth * 2` term. At
  the current +14/floor slope, floor 25 would want difficulty ~356 against a success ceiling near
  180. The real problem was that *nothing on the player's side scaled with the tower*. Fixed by
  (a) moving `depth_hazard_per_floor`, `hazard_tag_risk` and the familiarity numbers into
  `config.json` per the data-driven rule, and (b) making banked surveys subtract from a floor's
  hazard, capped by `max_survey_familiarity_relief` — so a floor is brutal on first contact and
  becomes routine once walked, which gives the survey system from the last iteration a second job.
  `relic_bonus` now keys off `floor.relic_drop_ids` instead of `depth >= 3`, turning another dead
  field live; `floor_2_molten_baths` has declared `molten_collar` all along and never yielded it.
  Added a load-time ceiling (`max_floor_difficulty`, 120) so band authoring cannot write a floor
  nobody can beat. 4 tests (47 total, was 43). **Balance moved, slightly and in one place only:**
  365-day `tower_materials` 10061 -> 10073 (+0.1%), from marginally better success scores. Roster,
  buildings, eggs, relics, gold, residue and debt are all byte-identical, and no assertion was
  touched. **Next iteration should know:** the validation ceiling of 120 is deliberately generous —
  the *practical* ceiling is lower and set by the reward thresholds, not by success going negative.
  `expedition_egg_reward_threshold: 68` and `expedition_relic_reward_threshold: 88` mean a floor
  around difficulty 90+ still "succeeds" but stops paying eggs and relics. **Band authoring must
  spread difficulty across roughly 20-100 over 25 floors (~+3/floor), not continue the early
  +14/floor slope.** Phase 0 is now clear except the roster-list cap (item 1's leftover); band 1
  authoring can begin.

- **2026-08-01 — phase 1, band 1 authored (depths 1-5).** Four new floors: **Drowned Larder** (2,
  d24, best raw materials), **Lamp Gallery** (3, d27, best residue, first band-1 relic and the only
  corruption pressure), **Cistern Stair** (4, d30, vertical, best egg *grade*), **Foreman's Rest**
  (5, d32, relic + the band gate, needs `tower_route_cartography`). Each is chained to the one above
  by surveys; Molten Baths moved to depth 6 and Gilded Kennels to 11 as their band anchors, with
  difficulty and content untouched. Molten Baths gained `scout_route`. 5 discovery events name the
  new floors before the guild can reach them. **The survey chain is proven against real data** —
  `drowned_larder` and `lamp_gallery` both opened by survey alone.
  **Two findings that matter more than the content:**
  (1) **A serial survey chain stalls on any link that is not worth running.** The first tuning made
  band 1 uniformly weaker than floor 1, so the policy never picked it and floors 3-5 never opened —
  with every balance assertion still green, because a floor nobody visits changes nothing. Fixed by
  making each floor **best-in-class on exactly one axis** rather than uniformly scaled. Added
  `validation_tests/probe.rs` (`cargo test probe_floor_usage -- --ignored --nocapture`) which prints
  unlocked/surveys per floor; **run it whenever a band is authored or rewards move** — the standard
  reports show missions, not floors, and hide this failure completely.
  (2) **The tower does not pay the scarce resource.** Every floor's `baseline_rewards.gold` is 0, on
  the originals too. The guild ends a 365-day run sitting on ~62k residue it cannot spend while gold
  is what buildings, upkeep and debt all consume. A "best residue" floor therefore has no pull —
  which is why `lamp_gallery` unlocked and still went unrun. Bands 2-5 will hit this harder.
  **Balance moved substantially and needs a decision before more bands land:** ending gold +91%
  (630k -> 1.20M), expedition days 178 -> 238, expedition eggs 38 -> 248, hatches 23 -> 33,
  buildings 15 -> 17. Roster still caps at 20, debt still survives, **no assertion was loosened or
  touched**. `TODO.md` lists the day-365 target range as an open question, so there is no yardstick
  to tune against yet — that is the blocker for band 2.

- **2026-08-01 — the escort economy (requested design change, not a loop slice).** Income now comes
  from escorting adventurers who pay for the calibre of companion they get, and companions draw
  wages that climb with that calibre. `companion_food_gold_per_day` (flat 4/head) became
  `companion_base_wage_gold` scaled by rank and accumulated skill; the rank ladder runs **1-5**
  instead of 1-3 (thresholds `[3, 5, 10, 17]` chosen so ranks 1-3 land on exactly the grades they
  always did, with 4-5 as headroom reaching ~depth 16 — the old ladder topped out on floor 3, so
  every floor below it added nothing to the roster); patron tiers carry `minimum_quality_rank` and a
  companion is booked by the best clientele she **qualifies for**, falling back to an understrength
  fee only if she suits none. **Income:wage went 57:1 -> 4:1.**
  **Two traps worth remembering.** (1) Stretching the rank ladder silently *cut* income at existing
  depths until the thresholds were pinned to the old grades — raising costs and cutting pay at once.
  (2) The high-yield rooms served only rank-3+ clientele, so most of the roster took the
  understrength penalty on every shift and income fell to 37%; every room now also serves
  `local_delvers`, and the grand clientele are the upgrade a strong escort unlocks in that same room.
  **Deliberate content re-tunes, both stated rather than hidden:** `broker_compact_6` cut 300k -> 180k
  (it demanded 8,571 gold/day, a 3.4x rate spike over the milestone before it, and was unreachable
  once gold stopped being nearly free); and three assertions changed — the two roster-fills-the-cap
  ones relaxed with reasoning in place, and "all seeds reach the final debt window" **traded for the
  stricter** "no seed may miss a payment" plus "most reach it", which now holds at max 0 missed.
  **Result:** roster 20, 8 of 10 seeds at cap, zero missed payments, ending gold 1.2M -> 2.5k.
  **Next iteration should know:** buildings fell 17 -> 9 and expedition days 238 -> 62 — the guild is
  now poor enough that it neither builds nor delves as freely, which is the honest cost of a real
  wage bill against a 7-floor tower. Bands 2-5 are what feed it: deep floors produce rank 4-5
  escorts, and `frontier_factions` at 240% is waiting for them. Re-measure this table after band 2.

- **2026-08-02 — phase 1, band 2 authored (depths 7-10), and the tower finally pays.** Four floors:
  **Flue Warrens** (7, d37, the hot route down), **Slagworks** (8, d40, best materials in the game),
  **Boiler Cathedral** (9, d43, best relics), **Cinder Nursery** (10, d46, best egg grade — the
  guild's first rank-4 escorts). Four discovery events. **Three engine fixes the content forced,
  each one a thing the tower could not do without:**
  (1) **Route survey crediting** (`surveys.rs::route_survey_count`) — a survey requirement is met by
  the named floor *or anything deeper*, because a party running depth 10 walks depth 3 to get there.
  Without it a shallow floor that is never the best available run stalls every floor beneath it
  forever, which is exactly what had happened to `lamp_gallery` and `flue_warrens`.
  (2) **Rewards must scale steeply with difficulty.** Measured: a floor at difficulty 37 needs
  roughly **2.3x** the baseline of a difficulty-24 floor before the planner will ever choose it,
  because difficulty is subtracted from success and success feeds the reward bonus. Materials alone
  is the wrong lever — **relics are what make a deep floor worth the risk** (Molten Baths competes
  at 28 materials because it also carries a relic and an egg grade).
  (3) **`reward_threshold_depth_relief_pct` (60).** The egg and relic bars were flat (68 / 88) while
  `success_score` already had difficulty subtracted, so depth was charged twice and deep floors
  almost never yielded eggs. The roster was **11 rank-2 and 9 rank-3, zero rank-4**, meaning the
  entire escort economy's payoff could never arrive. With the bars made depth-relative, rank 4
  appears, buildings go 9 -> 14, gold 66k -> 198k.
  **Result:** 8 of 10 seeds fill the roster, zero missed payments, buildings avg 14.3, and **4 of 10
  campaigns now fully clear Founder's Due** — the game is winnable for the first time. One stale
  assertion rewritten with reasoning in place: it literally read "current three-floor samples should
  not fully clear Founder's Due" and now checks that winning happens but is not guaranteed.
  **Next iteration should know:** (a) **arcane residue is still a dead resource** — ~100k accumulates
  unspent, so any floor whose identity is "best residue" will never be chosen; it needs a sink before
  band 3, or stop using it as a floor axis. (b) `floor_3_gilded_kennels` (d11) is unlocked all
  campaign and **never run** — it is building-gated, sits above band 2's rewards, and needs
  rebalancing when band 3 anchors on it. (c) Run `probe_floor_usage` after every band; the standard
  reports show missions, not floors, and would have hidden all of this.

- **2026-08-02 — phase 1, band 3 authored (depths 12-15); the tower reaches rank 5.** Four floors:
  **Menagerie Walk** (12, d52), **Handler's Vault** (13, d56, gated on `species_archive`),
  **Broodpens** (14, d60, wants a rank-3 Minotaur Porter), **The Auction Floor** (15, d64). Four
  discovery events. `floor_3_gilded_kennels` rebalanced — it had sat unlocked and unrun all campaign
  because band 2 overtook it; it is now run heavily.
  **The finding that unblocked everything** is written up under "How the planner values a floor"
  above: the planner skips every non-egg mission whenever the guild wants eggs, so a floor without
  `egg_hunt` is simply invisible. Three floors gained it and the tower went from stalling at depth 12
  to running all fifteen. Reward tonnage was never the problem — I raised Menagerie Walk from 70 to
  98 materials and the campaign replayed byte-identically.
  **Also:** chain requirements dropped to 1 survey below band 1 (band 1 keeps 2 as a teaching pace).
  With route crediting the guild is delving constantly by then, and 2-3 surveys per link priced deep
  floors out of a 365-day campaign entirely.
  **Result:** all 15 floors reached; roster ranks **[0, 1, 8, 3, 8]** — eight rank-5 escorts, so
  `frontier_factions` at 240% and the 1000% rank multiplier are both live for the first time. Gold
  199k -> 671k, expedition days 62 -> 87, buildings avg 13.6, 18-20 companions on every seed, 4 of 10
  clear Founder's Due, **zero missed payments**. 52 tests green, no assertion touched this iteration.
  **Next iteration should know:** (a) residue is no longer dead — band 2's relic floors unlocked the
  `relic_residue_condenser` (25k residue, 15 relics, repeatable x10) and residue now swings
  10k-98k across seeds instead of piling up; the earlier "give residue a sink" note is stale.
  (b) `auction_floor` (d15) unlocks but still shows 0 surveys — the deepest floor is always the last
  to be worth it; expect the same for the bottom of each new band. (c) Bands 4-5 (16-25) are all that
  remain; difficulty should run ~68-100 and the ceiling `max_floor_difficulty` is 120.

- **2026-08-02 — phase 1, band 4 authored (depths 16-20); the tower turns purposeful.** Five floors:
  **Sealed Archive** (16, d68, warded from the inside), **Cartogram Vault** (17, d72, maps showing
  floors that do not exist yet), **The Reliquary** (18, d76, wants a rank-3 Moth Archivist; a
  catalogue something is still keeping), **Instrument Halls** (19, d80, machinery still running),
  **The Understair** (20, d84, where the stonework stops being built for people). Five events.
  **Two authoring rules learned here and promoted into the guidance above**, because both stalled the
  band outright before they were understood: rewards must rise **monotonically** with depth (my own
  band-4 draft had Sealed Archive paying less than the floor above it while costing more), and a
  band's **deepest** floor must be its best prize because it is the doorway to the next band —
  `auction_floor` sat unrun until raised above `broodpens`, and that one change opened all of band 4.
  **Result:** all 20 floors unlock and are reached; the guild's preferred destination is now depth 20.
  Gold 671k -> 732k, rank-5 escorts 8 -> 10, buildings 17. 18-20 companions per seed, 4 of 10 clear
  Founder's Due, zero missed payments, 52 tests green, no assertion touched.
  **Next iteration should know:** band 5 (21-25) is all that remains; difficulty should run ~88-104
  against the `max_floor_difficulty` ceiling of 120, and the ramp must continue past eggs 3 /
  relics 12. `tower_core` already exists as a magic egg source (`day_cycle/eggs.rs:7`) and is the
  thread band 5 should pay off.

- **2026-08-02 — phase 1 COMPLETE, band 5 authored (depths 21-25). The tower is 25 floors.**
  **The Long Descent** (21, d88, a day of stair cut in one piece, and something counts you down it),
  **Chorus Vault** (22, d92, a hall that answers in a voice that has had longer to think),
  **The Hollow Works** (23, d96, chapel-sized sockets where whatever drove the machinery used to sit),
  **The Threshold** (24, d100, one door carrying the guild's own mark, cut from the inside and older
  than the guild; wants a rank-4 Golemkin Warden), and **The Tower Core** (25, d104, warm, lit, in
  use, and still sending). Five events. **The band needed no tuning passes** — the monotonic ramp and
  best-prize-at-the-bottom rules from band 4 worked first time, which is the sign those rules are
  real rather than curve-fitted.
  **`tower_core` is paid off.** `day_cycle/eggs.rs` has always stamped the guild's founding egg with
  `source_floor_id: "tower_core"`, a place that did not exist, and `engine/validation.rs` carried an
  explicit exemption so that dangling id would validate. Depth 25 is now that room, the id resolves
  like any other floor, and **the exemption is deleted** — the opening egg retroactively came from
  the bottom of the tower.
  **Result:** 25 floors, all unlocked and reached; difficulty 20 -> 104. Gold ~730k, ten rank-5
  escorts, buildings 17, 18-20 companions per seed, 4 of 10 clear Founder's Due, zero missed
  payments. 52 tests green, no assertion touched across bands 4 and 5.
  **Phase 1 is done. Next is phase 2 (density)**, and the thin axes are unchanged since day one:
  **species 8** (deep bands still draw from the same eight — nothing lives only at depth 20+),
  **missions 4** (the GDD names six; deep floors want rescue and sealed-extraction stances),
  **mutations 3**, **relics still ids not objects** (~30 `relic_drop_ids` now exist across the tower
  and not one has a name, description or a patron who wants it — that is the single largest content
  gap the tower has opened up), **guild rooms 4**, **contracts 12**, **patron tiers 3**.

- **2026-08-02 — phase 2 opens: relics become objects.** 28 `relic_drop_ids` existed across the tower
  as bare strings — unvalidated, unnamed, and invisible: a relic was a number going up. Added
  `assets/data/relics.json` (id, name, description, discovery_note) covering all 28, `RelicData` /
  `RelicCatalogData` in the schema, and load-time validation **in both directions** — a floor may not
  name a relic that does not exist, and a relic no floor drops fails the load as prose nobody can
  reach. `day_cycle/relics.rs` reports the find in the day's event log, which the journal already
  renders, so no new UI was needed.
  **One real trap, caught by the harness.** The first version picked the relic with `gen_range`,
  which consumes the seeded RNG and shifted every downstream roll: a 365-day campaign went from 17
  buildings to 8 on a cosmetic change. It now rotates on `resolved_day` instead. **Flavour output
  must never draw from the simulation's RNG** — the numbers are identical to the previous commit
  except `final_event_log_entries`, which is the whole point.
  **Also deduplicated four copies of `test_game_data()`** (bootstrap, debt, guest/tests,
  view_models/decisions) onto the shared `crate::data::test_game_data`. They were straight copies
  and every one of them broke when the catalog was added, which is exactly the recurring tax the
  shared fixture exists to stop. ~45 lines gone.
  **Result:** 56 tests (was 52), balance byte-identical, publish green. Assets are now 27 files.
  **Next iteration should know:** the other half of "relics as objects" is a **patron who wants one** —
  contracts can reward relics but nothing ever *asks* for a named one, so the collection has no
  demand behind it. After that the thin axes are **species 8** (nothing lives only at depth 20+),
  **missions 4** for 25 floors, **mutations 3**, **guild rooms 4**, **patron tiers 3**.

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
