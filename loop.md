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
| **Tower floors** | **11** | **25** | Bands 1 and 2 authored, depths 1–10, plus Gilded Kennels at 11. Difficulty 20→48. All eleven are reachable and all but Gilded Kennels are actually run by the planner. |
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
| Relic drops | 2 declared, now **gate relic yield** | named objects | `relic_drop_ids` decides whether a floor can pay a relic. The ids are still not objects with names/descriptions/patrons — that is phase 2. |

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
