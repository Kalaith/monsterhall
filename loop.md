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
| Missions | **7** | 8–10 | The GDD's six are all written, plus `sealed_extraction`. **The harness cannot measure a mission** — see the ledger for 2026-08-02; mission work is verified by reading the probe's SCORE table, not by balance movement. |
| Species | **12** | 14–18 | Four now hatch only below depth 16 (`wyrm_registrar`, `gargoyle_stairwarden`, `revenant_chorister`, `salamander_corekeeper`). Bands 1–3 still share the original eight. |
| Mutations | **8** | 10+ | Five chains, terminating in three of the four deep species. `salamander_corekeeper` is the only species that is neither a source nor a target. **Read the mutation warning below before adding a ninth.** |
| Buildings | 12 | 20+ | Buildings are the only species/floor gate today, and **a new one is very hard to land** — see the ledger for 2026-08-02. |
| Guild rooms | 4 | 7–8 | Four rooms for a 20-companion roster. |
| Traits | **13** | 16+ | Traits drive contract fit and role assignment. |
| Contracts | **16** | 25+ | Should reference named floors and deep species. Every species now has exactly one patron who asks for it by name. |
| Events | 67 | — | Healthy. Deep-floor events are the gap, not event count. |
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

**The planner runs one expedition a day at the single best (floor, mission) pair in the whole
tower.** So it is structurally incapable of showing mission variety: whatever wins, wins every day,
and adding a mission changes the balance reports by exactly zero unless it beats that one pair — in
which case it becomes the new monoculture. **Do not tune a mission until the balance numbers move.**
Judge missions by the probe's `SCORE` table, which prints every mission of every unlocked floor
side by side; that is where a stance being strictly dominated is visible.

### A mutation can close the tower — read this before authoring one

Five floors gate on a **named species at a named rank** (`required_roster`: slime at d6, Minotaur
Porter at d11 and d14, Moth Archivist at d18, Golemkin Warden at d24). A mutation **removes** its
source from the roster, and rank climbs with the same deep running that drives corruption — so a
mutation off a gate species races the very companion the gate is waiting for. Authoring
`minotaur_porter -> golemkin_warden` at 30 corruption stranded `broodpens` for a whole campaign and
with it **eleven floors**, because the survey chain is serial.

- **Check `required_roster` against the mutation's source before writing it.** If the source gates a
  floor, the threshold must sit far above where that floor gets run — 90, not 40.
- Supply is the real variable. `slime_companion` has mutated away since the first commit and gates
  d6 harmlessly, because slimes keep hatching. `minotaur_porter` ends a campaign at a roster count
  of **one**; anything that consumes it consumes the only one.
- `long_campaign_simulation_reports_stay_valid` now asserts the 365-day campaign opens **all** 25
  floors. Before that assertion existed this failure was completely silent: gold, roster, buildings
  and debt all stayed inside their bands while half the tower was unreachable.

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
   `expedition_planning_sections.rs`. ~~**Roster lists capped at six.**~~ **Done 2026-08-03** —
   `ui/screens/roster_window.rs` pages the Expedition Desk team panel, the Contract Desk candidate
   panel and the Town Overview roster strip. The last of those was the serious one: it is the only
   route to a companion's profile, and the profile is the only place she can be released, so
   seventeen of twenty companions could never be let go. See the Ledger.
   Still open but **latent, not live**: `town_management.rs:144` takes 10 of the buildings against
   9 core and 4 projects today, and the panel only fits about nine rows — so the tenth already
   draws past the panel edge if a group ever reaches it.
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

- **2026-08-02 — phase 2, four species that only live at the bottom of the tower.** Ten floors of
  endgame were drawing from the same eight species available at depth 3, so the roster you finished
  with said nothing about how deep you got. Added **Wyrm Registrar** (16–18, keeps the sealed
  archive's catalogue; performer), **Gargoyle Stairwarden** (19–21, cut from the understair and
  counts what goes down; delver), **Revenant Chorister** (22–24, answers in a voice that has had
  longer to think; corruption adept) and **Salamander Corekeeper** (24–25, turns the clutches in the
  warm room; hatchery specialist) — four species, four distinct inferred roles, three traits
  (`meticulous`, `stonebound`, `long_patience`), four name pools, four patrons who ask for them by
  name, six events, and room preferences so they earn where they belong.
  **The gate is depth, not gold, and that was the whole fight.** My first draft gated them behind two
  new buildings. Three findings, all worth keeping:
  (1) **The harness's building gate has one early window and then shuts.**
  `can_make_growth_investment` values a building against `conservative_daily_gold_income`, which caps
  at 8 income units — so once the debt milestones get large, no one-off building is affordable again
  until ~day 320. `species_archive` (day 60) and `tower_route_cartography` (day 70) squeeze through
  that window; **a new one-off building essentially cannot.** Any future slice that wants to add a
  building must solve this first.
  (2) **A cheap new building can close the tower.** The policy buys whatever is affordable today
  rather than saving for the plan, so a 2,350-gold vault held the guild's gold under cartography's
  3,100 and slid it from day 72 to day 321 — which never opened the survey chain, and **the bottom
  fourteen floors were never entered all campaign** with every assertion still green. A strict
  "one-off blocks everything behind it" ordering fixed it and measured *better* than today's baseline
  (gold 932k, ranks [0,0,2,5,13], all 25 floors) — **not shipped this iteration**, because it is a
  separate concern from species; it is written up under Deferred as a ready-made slice.
  (3) So the four species were attached to the two deep-tower buildings that already exist and are
  already bought — the species archive and the cartography office, both of which read as exactly the
  institutions that would let you keep such a thing. The real gate is that their eggs only exist on
  floors 16+.
  **One bug the content exposed, one stale cap.** Contracts were offered whenever their species was
  *unlocked*, which was fine when every species hatched days after its unlock; a deep species stays
  unlocked and absent for most of a campaign, so the desk filled with unfillable work — rejections
  1,279 → 2,327. `request_template_available` now asks whether the hall actually has one, which took
  rejections to **324**, a quarter of the pre-existing baseline. And contract
  `minimum_quality_rank` was still validated against the pre-escort-economy 1–3 ladder; it now derives
  the ceiling from `egg_quality_rank_thresholds` like the floor roster requirements already did.
  **Result:** all four species reach the roster, all 25 floors still reached, **rank-5 escorts 10 →
  14**, gold 730k → 846k, hatches 33 → 37, buildings 17 → 16, 4 of 10 seeds clear Founder's Due,
  zero missed payments. 56 tests green, no assertion touched. New load-time validation: a species must
  appear in some floor's egg pool **and** be unlockable, so the next authored species cannot be
  invisible the way these nearly were. `probe_floor_usage` now prints per-species unlock day and
  roster count — **read the unlock day, not just the count**; unlocked on day 324 of 365 is not
  content.
  **Next iteration should know:** the thin axes are now **missions 4** for 25 floors (the GDD names
  six; deep floors want rescue and sealed-extraction stances), **mutations 3** against 12 species,
  **guild rooms 4**, **patron tiers 3**, and relics still have no patron who asks for a *named* one.

- **2026-08-02 — phase 2, three mission stances, and the relic missions start paying again.**
  The GDD names six mission types and only four existed; every floor below depth 6 carried the same
  four, so no floor uniquely permitted anything. Added **Rescue Retrieval** (go in after one specific
  thing that is still alive: focus eggs, +3 egg grade, dear and dangerous, prefers `comfort`),
  **Contract Fulfilment** (an order in hand — a lot number, a shelfmark: focus relics, +14 success,
  −6 injury, poor at everything it was not sent for, prefers `performer`), and **Sealed Extraction**
  (cut a ward, take what is inside, close it again: the largest relic payout in the game and by far
  the worst injury, prefers `corruption_adept`). Each sits on six thematically-right floors — things
  the tower keeps alive, places somebody wrote an inventory, and doors shut on purpose — so a floor's
  mission list finally says something about the floor. Six events.
  **The bug the content exposed is the real content here.** `success_score` gates the reward payout
  *and* carries the mission's own `success_bonus_pct`, so a stance that is deliberately riskier stops
  paying the thing it exists to fetch. Below depth 17 **Relic Recovery yielded no relics at all** —
  the Egg Hunt's +20 success walked off with the floor's entire relic pile while the dedicated relic
  mission came back with nothing. New `mission_focus_reward_relief_pct` (17) takes a further slice of
  difficulty off *one* bar when the mission was chosen to look for exactly that reward. At the Tower
  Core relic yields now read egg_hunt 20 < relic_raid 22 < sealed_extraction 23, with injury climbing
  140 → 152 → 174 to match: a real decision where there was previously a dominant answer.
  **Two shape lessons.** (a) A *flat* relief was the first attempt and it closed the tower: shallow
  floors were already near their bars, so a safe depth-5 errand started paying three relics for
  almost no injury and the guild never went deep again. The relief has to scale with difficulty,
  exactly like `reward_threshold_depth_relief_pct`, because the gap it closes is a depth effect.
  (b) A relic mission should not be success-negative at all — cutting a ward is slow and dangerous,
  not *unlikely to find the thing*, so `sealed_extraction` pays for itself in injury (+26) rather
  than in success, which is both better fiction and the only way it clears its own bar.
  **One UI bug the content exposed.** The mission buttons were a single row splitting the panel by
  count with a 58px floor — which silently overflowed at five missions and drew the last buttons
  across the priority panel. `MissionGrid` in `expedition_planning_sections.rs` wraps them and sizes
  the panel from the result; four missions still render exactly as before. 4 unit tests, including
  one that asserts no button leaves the panel at any count from 1 to 8.
  **Result:** every numeric field in all four balance reports is **byte-identical** to the previous
  commit — only the sampled event-log strings moved. That is the expected outcome and is written up
  under "How the planner values a floor" above: the planner runs one expedition a day at one globally
  best pair, so it cannot see mission variety at all. 60 tests (was 56), no assertion touched, publish
  green. New load-time validation: an unknown `reward_focus` is now a startup error rather than a
  mission that silently stops being about anything.
  **Next iteration should know:** the thin axes are **mutations 3** against 12 species (deep floors
  push `corruption_pressure` up to 8 and almost nothing comes of it — this is the most obvious gap
  left), **guild rooms 4** for a 20-companion roster, **patron tiers 3** with upkeep bands already
  referencing a fourth, and relics still have no patron who asks for one by name.

- **2026-08-02 — phase 2, corruption starts changing who the guild has.** Deep floors push
  `corruption_pressure` up to 8 and the mutation table had three entries, every one of them
  authored at 8/16/18 — so every mutation the game had fired in the opening weeks and the stat did
  nothing for the remaining eleven months. Five more, forming five chains that terminate in three of
  the four deep species: **Imp Runner → Lamia Routekeeper** (30, has run the passages so often she
  has stopped running them), **Minotaur Porter → Golemkin Warden** (90, the load stops being
  something she carries), **Moth Archivist → Wyrm Registrar** (60, closes the guild's ledger and
  starts a different one in a hand nobody taught her), **Golemkin Warden → Gargoyle Stairwarden**
  (100, the stone stops being armour), **Lamia Routekeeper → Revenant Chorister** (80, answered by
  the tower too often to keep only asking). **Thresholds were read off the campaign, not guessed** —
  the probe now prints every companion's corruption, and a finished roster runs 45 to 199.
  **Two routes into Golemkin Warden, only one that continues.** Gargoyle needs `commanding` *and*
  `resilient`; the residue-slime line grants `commanding` but has no `resilient`, so it stalls at
  Golemkin, and only the Minotaur-descended line walks on to the stair. The deep species now arrive
  two ways — hatched below depth 16, or grown into — and Wyrm Registrar reaches the roster long
  before floor 16 opens.
  **The regression this iteration nearly shipped is the real content here, and it is written up in
  full above.** At the authored 40 corruption, `minotaur_porter_to_golemkin_warden` consumed the
  guild's only Minotaur Porter, `broodpens` (d14) requires one at rank 3, and the serial survey
  chain stranded **eleven floors** — the campaign ended at depth 14 having never seen bands 4 or 5,
  **with every balance assertion green**. Only `probe_floor_usage` showed it. Fixed by moving the
  threshold to 90 (and Gargoyle to 100 to keep the chain ascending), which lets a porter serve the
  floors that name her and turn afterwards.
  **New load-time validation, all three of which the first draft could have broken:** a mutation may
  not turn a species into itself; the graph may not contain a cycle (a companion flipping between two
  forms forever); and a step must cost **strictly more** corruption than the step feeding it, or the
  two fire on the same day and the species in between never exists in play. **New balance
  assertion:** the 365-day campaign must open all 25 floors — verified to fail on the 40-corruption
  version before being kept.
  **Result:** multi-seed gold 274k → 338k average, relics floor 8 → 36, companions 19.5 → 19.6,
  buildings 13.6 → 13.7, zero missed payments; single-seed 365 gold 846k → 880k with all 25 floors
  still reached and ranks unchanged at [0,1,2,3,14]. 60 tests green, no assertion loosened, publish
  green.
  **Next iteration should know:** mutations are 8 against a target of 10+, and the two obvious gaps
  are a Harpy Lookout line that goes somewhere other than the archive, and `salamander_corekeeper`,
  the only species that is neither a source nor a target of anything. Beyond that the thin axes are
  unchanged: **guild rooms 4** for a 20-companion roster, **patron tiers 3** with upkeep bands
  already referencing a fourth, and relics still have no patron who asks for one by name.

- **2026-08-03 — five gameplay defects, and the harness was one of them.** Not a
  content axis: a review pass over `TODO.md`'s gameplay-affecting section, which
  had nothing left in it but decisions parked for the user. Five landed, and the
  first one matters more than the other four together.
  **The star ladder still said three in two places.** Two passes ago the hatchery
  UI's hardcoded `egg_quality_rank` was fixed; nobody swept for other copies.
  `convert_egg`'s Refine path refused every rank-3 egg against a ceiling of 5 —
  so the only way to *make* a good egg rather than find one could not reach the
  two ranks that earn 7x and 10x — and `policy_eggs.rs` carried a private
  `egg_quality_rank_for_policy` capped at `_ => 3`, feeding
  `replacement_plan_for_egg`. **The balance harness was reading every egg above
  grade 10 as a three** and declining to replace any rank-3-or-better companion
  with one. Delegating it moved multi-seed 365 gold **684k → 1.07M**, buildings
  22.3 → 24.2, companions 18.7 → 19.2, expedition days 59.2 → 72.2, debt gap
  −1.31M → −0.98M; single-seed floors 16 → 19, hatches 33 → 43. Zero missed
  payments, no assertion touched. **Every balance figure in `TODO.md` and this
  ledger predating today was measured through that lens.**
  **The other four moved nothing and all four were player-facing.** The
  preparation-quality formula existed twice and only the preview scaled it by
  condition, so the guild-hall card promised that resting someone before a
  booking would help while the desk scored her as fresh. The contract desk
  printed **"Kiss Count"** and **"Birth Count"** as refusal reasons — the retired
  premise's vocabulary, surviving as string arguments rather than content ids,
  which is exactly why the rename pass missed them; the guild-hall badge codes
  `K`/`O`/`V`/`A`/`C`/`M`/`B` were the same initials, one letter wide. Charm
  odds were the last room table hardcoded as a `match` on room id (a new room got
  its charm training from whether it named a required building), and
  `guild_job_instability_gain` named `packroom_annex` outright. Both are authored
  data now, at the exact values the matches produced, so the RNG stream and every
  seeded report are byte-identical.
  **One latent trap worth remembering:** `validate_role_or_niche` checked room
  niches and companion roles against the *union* of both closed sets. A mission
  authored `preferred_role: "performance"` — a room niche — passes validation and
  matches no companion, so `role_affinity` charges the **entire party** the
  off-role penalty instead of rewarding anyone. Split in two, and
  `COMPANION_ROLES` is now tested by sweeping `monster_role`'s branches rather
  than by eye. Shipped content was already correct.
  **Result:** 87 tests (was 79), fmt and clippy clean, publish green. Two new
  config blocks (`egg_sale_gold_by_rank`, `egg_dissolve_residue_by_rank`,
  `egg_dissolve_relic_minimum_rank`) and three new room fields. Both new guards
  verified by planting the bug and watching them fire.
  **Next iteration should know:** (a) re-measure before trusting any parked
  balance decision in `TODO.md` — "a competent guild finishes the campaign early"
  in particular is now measured against a guild with 57% more gold, and the
  survey-term question may have a different answer; (b) `monster_service_score`
  in `policy_eggs.rs` still counts five of the ten skills, the same shape as the
  wage bug, and it decides which companion the policy releases; (c) the thin
  content axes are unchanged — **guild rooms 4** for a 20-companion roster,
  **patron tiers 3** with upkeep bands already referencing a fourth, and relics
  still have no patron who asks for one by name.

- **2026-08-03 — the roster lists, and the last of phase 0.** Blocker 1's leftover
  had been open since the first iteration and turned out to be the biggest live
  gameplay defect in the game. Both assignment panels drew
  `game_state.monsters.iter().take(6)` against a population cap of **20**: six was
  never a layout measurement, it was the roster size when those panels were
  written. **Fourteen companions could not be sent on an expedition or offered to
  a contract at all**, and nothing on either screen said they existed.
  **The Town Overview strip was worse.** `OpenMonsterProfile` has exactly one call
  site — that strip, capped at three — and `ReleaseMonster` exists **only** on the
  profile screen. So a guild at its cap could only ever release one of the first
  three companions in roster order, and hatching at cap *requires* releasing or
  replacing. The late game was gated on a button seventeen companions did not have.
  New `ui/screens/roster_window.rs` pages all three grids. The floor list stayed
  stateless by deriving its page from the selected floor; that does not transfer,
  because a roster panel has no selection to follow — so the page lives in phase
  state beside `inventory_scroll` (transient, never saved) and survives a phase
  rebuild, so assigning somebody does not throw the player back to page one.
  Deliberately **unsorted**: ordering by availability would put the useful cards
  first, but assignments change as the player works, so cards would reshuffle
  under the cursor between clicks. `town_overview_sections.rs` crossed 800 lines
  as a result and the roster panel was extracted to `town_overview_roster.rs`.
  **The harness could not photograph any of this**, because `seed_capture_scene`
  starts a fresh campaign with one companion — the panels only misbehave when the
  guild is crowded. A `_full` scene suffix now fills the roster to the cap first;
  `ui_town_full.png`, `ui_expedition_full.png`, `ui_contracts_full.png` are the
  new baselines, and all three show a working pager.
  **Also: a third and fourth copy of the skill sum.** `companion_daily_wage` was
  fixed two passes ago; the hatchery's `replacement_score` — which picks the
  companion the game *recommends you sacrifice* — was still counting five of ten,
  so training recovery or bargaining made a companion cheaper to discard and more
  expensive to keep at once. One `engine::companion_skill_total` now.
  **One thing measured and deliberately not taken.** `monster_service_score` in
  `policy_eggs.rs` has the same incompleteness and picks who the *simulated* guild
  releases. Completing it moved multi-seed gold **1.07M → 851k** and buildings
  **24.2 → 17.3** with every assertion green. It is a policy heuristic standing in
  for player judgement rather than a formula the game defines, and taking it would
  re-base every parked balance question for the second time in two passes — so it
  is written into the function's own doc comment and `TODO.md` for a deliberate
  call rather than shipped.
  **Result:** 94 tests (was 87), balance byte-identical, fmt and clippy clean,
  publish green. **Phase 0 is now fully closed.**
  **Next iteration should know:** (a) `town_management.rs:144` still takes 10 of
  the buildings — latent, not live, since core holds 9 and projects 4, but the
  panel only fits ~9 rows so the tenth already overflows; (b) the Contract Desk
  detail column's two known overlaps are still there (list rows wider than their
  panel, thumbnail caption reaching into the text column) and the crowded capture
  shows the caption now colliding with the room name; (c) the thin content axes
  are unchanged — **guild rooms 4**, **patron tiers 3**, and relics still have no
  patron who asks for one by name.

- **2026-08-03 — the fourth roster panel, and the geometry nobody was measuring
  against.** Last pass fixed three panels that hid companions behind a `.take(6)`.
  The Guild Jobs screen had the same six-companion assumption and failed the
  *other* way: `draw_worker_cards` clamped its panel to `.min(330.0)` while the
  loop below drew a card for **every** worker. A full guild's Available column ran
  nine rows past its own frame, through the footer and off the bottom of the
  window — companions 13–20 drawn where nothing can be clicked. 330px only ever
  held three rows, so it already overflowed at six. It derives its capacity from
  the space to the footer now and pages with the same `RosterWindow`.
  **Then the same shape twice more, in geometry rather than counts.** The Hatchery
  drew a hardcoded four egg rows into a panel that grew to hold eight, so half the
  column sat empty and the player scrolled twice as far as needed. And both
  mouse-wheel handlers carried their own stale copies of the panels they scroll:
  the Hatchery's claimed the **full screen width** (hovering the detail panel
  scrolled the egg list) over a fixed `230..666` band that missed the panel's top
  and most of its bottom; the Journal's was 720px tall against a log panel that
  follows the window — 60px too tall at 1080p, 420px too tall at 720p, so the
  wheel scrolled the log from over the footer. Both read the screen's own layout
  now, and the row count is one constant instead of one per file.
  **And the last recorded overlap, fixed at its source.** `draw_text_center`
  centred text without ever measuring it against the box width, so a caption wider
  than its box spilled out of *both* sides. Every thumbnail caption in the game
  goes through it. On the Contract Desk the guest name reached far enough right to
  print through the room name, reward, penalty and deadline — the one screen whose
  job is showing a contract's requirements. Captions ellipsise to fit; anything
  that already fitted is untouched.
  **Result:** 94 tests, balance byte-identical (no engine arithmetic moved), fmt
  and clippy clean, publish green. `_full` captures now crowd the egg inventory as
  well as the roster, because every list fixed across these two passes is one that
  only misbehaves when it is full.
  **Next iteration should know:** (a) this closes the panel-capacity class — every
  list that renders game entities now derives its capacity from the panel it draws
  into, except `town_management.rs:144`, which still takes 10 against 9 core and 4
  project buildings and so cannot bite yet; (b) the contract list rows still draw
  a few pixels wider than their panel, which is genuinely cosmetic; (c) the thin
  content axes are untouched and are the honest next slice — **guild rooms 4** for
  a 20-companion roster, **patron tiers 3** with upkeep bands already referencing
  a fourth, and relics still have no patron who asks for one by name.

- **2026-08-03 — the two screens that give advice were both giving wrong advice.**
  The panel-capacity class closed last pass, so this one swept the surfaces that
  *tell the player what to do*. Both contradicted the engine underneath them.
  **The profile screen called every companion "hurt" after one day's work.**
  `monster_role_summary` tested `injury > 0 || stress >= 3 || fatigue >= 3`, a
  threshold written before the condition system existed. One guild shift adds
  **10 fatigue and 4 stress** against allowances of **30 and 20** — so from her
  first shift every companion read "hurt, best next use: rest" while
  `companion_effectiveness_pct` still returned exactly **100**. The game was
  telling the player to spend a rest day recovering nothing, for the whole
  roster, forever. It asks the engine now, via a new public
  `engine::companion_effectiveness`. The same function also held a partial fourth
  copy of the role classifier (`power >= charm + 2` plus two skill thresholds),
  which could disagree with the role printed in the same sentence; it maps over
  `monster_role` instead.
  **"Today's Priority" hid the debt window behind any egg.** Eggs ranked above
  the debt warning, so one egg in the inventory meant the debt panel never
  appeared — and the debt copy's own words are "favour reliable guild work and
  contract fulfilment over speculative tower work", the exact call it was being
  prevented from making. And the eggs branch says "grow the roster before the day
  ends", which at the population cap is the one thing hatching cannot do; the
  guild fills its cap by mid-campaign, so the panel stuck on impossible advice
  for the whole late game. Debt outranks eggs, and eggs are gated on being below
  the cap. Both verified by planting the regression and watching the guard fire.
  **Also:** `expedition_injury_amount` moved from a bare `6` in `resolve_day`
  into `config.json`. Every other side of that exchange was already authored, so
  how hard a bad run hits was the one term nobody could tune. Same value, nothing
  moves.
  **Result:** 98 tests (was 94), balance byte-identical, fmt and clippy clean,
  publish green, `content_version` 1.16.0.
  **Next iteration should know:** (a) the frozen-threshold sweep is now clean —
  no raw `fatigue`/`stress`/`injury` comparison survives outside
  `condition.rs`, and no balance literal survives in `day_cycle`; (b) all three advice surfaces have now been
  re-read against the systems they describe — `onboarding_lines` orders eggs
  before debt like the priority panel did, but that is teaching order rather than
  urgency ranking and its copy stays accurate at the population cap, so it was
  deliberately left alone; (c) the thin content axes are still untouched and remain the
  honest next slice — **guild rooms 4** for a 20-companion roster, **patron tiers
  3** with upkeep bands already referencing a fourth, and relics still have no
  patron who asks for one by name.

- **2026-08-03 — a companion could work two jobs a day and one was thrown away.**
  `resolve_contracts` runs before the job loop and skips everyone it serviced, so
  a companion **accepted onto a contract** *and* **rostered to a guild room** did
  the contract and had her shift silently discarded. Nothing stopped it:
  `evaluate_contract_eligibility` rejects `OnExpedition` and says nothing about a
  guild job, and `assign_monster_to_room` never looked at contracts. Both
  assignment screens lied — the Guild Hall quoted her projected gold, the
  Expedition Desk counted her stats into the party preview — for work that would
  never run. With `town_job_limit` at **2**, a burned slot is half the hall's
  income for the day.
  **The harness was doing it too, and had the answer in scope.**
  `assign_daily_jobs` computes `reserved_guest_monster_ids`, honours it for
  expedition selection, and never checks it in the guild-job loop. The engine
  refuses the assignment now (`is_booked_for_contract`); the policy check was
  added beside it so the intent reads on the page rather than arriving as an
  error, and it is pure documentation — identical numbers with and without it.
  **This re-bases the balance baseline, so the measurement matters.** Direct
  effect on the deterministic seed is exactly the predicted direction:
  `total_guild_job_gold` **1,813,165 → 1,834,424**, the recovered slot. The
  multi-seed aggregate moved **1.07M → 855k gold**, **24.2 → 18.4 buildings** —
  but per seed it goes *both ways* (three up, three down hard), which is chaotic
  divergence rather than a cost: changing who works which day reshuffles the
  campaign from day one. Zero missed payments, no assertion touched.
  **Followed through in the UI**, because a rule the player cannot see is still a
  trap: a booked companion shows **"On Contract"**, greys out, and loses both her
  Assign and her Rest button on the Guild Hall and Expedition Desk. Rest was as
  futile as a shift — day resolution skips her, so her fatigue never came down
  either. `_full` capture scenes book the first companion so the state is
  photographable.
  **Result:** 100 tests (was 98), fmt and clippy clean, publish green,
  `content_version` 1.17.0. Guard verified by planting the double booking.
  **Next iteration should know:** (a) **any balance figure in `TODO.md` older
  than this pass was measured with a wasted guild-job slot** — the third re-base
  in five passes, and the last one that should come from a correctness fix, since
  the exclusivity rule is now enforced at the only two places that set it;
  (b) `assign_monster_to_idle` deliberately still works on a booked companion —
  it is the escape hatch, and clearing the booking itself lives on the contract
  desk; (c) the thin content axes are untouched and remain the honest next slice
  — **guild rooms 4** for a 20-companion roster, **patron tiers 3** with upkeep
  bands already referencing a fourth, and relics still have no patron who asks
  for one by name.

- **2026-08-03 — the other half of last pass's fix, and the mutation nobody was
  told about.** Last pass stopped a booked companion being rostered to a room.
  It did not stop the reverse: **booking a companion who was already working the
  hall was still allowed**, and `resolve_day` settles the contract first and
  discards her shift exactly as before — the same bug, reachable by doing the two
  actions in the other order. Refusing would have been wrong here; she can serve
  the contract perfectly well, it is the *slot* that is wasted. So taking a
  booking releases whatever she was rostered for, the way every other assignment
  already releases her from an expedition, and the slot goes back. Zero balance
  movement, as predicted — the policy books before it staffs, so only a human
  reaches that order.
  **Then the sharper one.** A companion changing species was announced only in
  `roster_updates`, which lives on the Day Results screen in a 140px box holding
  about **seven lines** against a twenty-companion roster, and is the one
  narrative list never extended into `event_log`. So the announcement was usually
  clipped away and then lost forever: the player finds a different species on the
  roster with nothing anywhere to say when or why — for the single system the
  whole corruption mechanic exists to drive, and the one this file has an open
  design question about. Mutations go to `event_lines` now, which reaches both the
  Day Results event panel and the scrollable journal. Every scalar in every report
  is unchanged except `final_event_log_entries` (+23 to +39): the mutations were
  always happening, they are just written down now.
  **The authored-data audit that opened `TODO.md` re-ran clean.** Every key in
  every `assets/data/*.json` is consumed. The five flagged as read only inside
  `src/data` are deliberate — three are load-time validation rules, and
  `keyboard_shortcuts_visible` / `primary_mode` are *constraints* the validator
  actively enforces. No unconnected data remains anywhere in the catalogue.
  **Result:** 102 tests (was 100), fmt and clippy clean, publish green. Both
  guards verified by planting the regression.
  **Next iteration should know:** (a) the code-defect well is genuinely running
  dry — six passes of sweeps have closed the panel-capacity, frozen-threshold,
  duplicated-formula, advice-surface and double-booking classes, and the
  authored-data audit now comes back empty; (b) the one UI capacity issue left is
  the Day Results `roster_updates` panel, which still clips at ~7 lines, but the
  information that mattered is journalled now and skills are on the profile
  screen, so it is convenience rather than loss; (c) **the honest next slice is
  content** — **guild rooms 4** for a 20-companion roster, **patron tiers 3**
  with upkeep bands already referencing a fourth, and relics with no patron who
  asks for one by name.

- **2026-08-03 — the save path, which six passes had never looked at.** Last
  pass's ledger said the code-defect well was running dry. It was running dry in
  the places that had been swept; the load path had not been one of them, and it
  held the worst bug found in this whole run.
  **A save missing `party_size` and `town_job_limit` loads, passes
  `validate_game_state_references`, and kills both of the game's verbs.** Both
  gates read `count >= limit`, so a defaulted **zero does not mean "no limit"** —
  it means nobody may ever be sent on an expedition or given a guild-room shift
  again. Measured on a real `save_version: 9` payload: parses, validates clean,
  and then every assignment refuses forever with no error that explains why. The
  campaign can end days and nothing else.
  `#[serde(default)]` is on every saved struct, correctly — it is what keeps old
  saves loading — and it is exactly how a save arrives structurally valid and
  functionally dead. **This is the display-settings bug one layer down**: that one
  was repaired at load by `reconcile_resolution_against` and the game state never
  got the same treatment. New `engine::reconcile_game_state_after_load` restores
  the configured baseline when either limit reads zero, called *before*
  validation, since the reference check is what waves the broken save through.
  Anything non-zero is left alone — `town_job_limit` grows past its baseline
  through `town_job_limit_flat`, and clamping would demolish every worker-limit
  building bought.
  **The same trap one level down:** a companion loaded at rank zero, which the
  game can never produce and a pre-field save always yields. She fails *every*
  contract (`rank < minimum.max(1)`, even one asking for nothing), satisfies no
  floor's roster gate, and is paid the understrength rate on every shift for the
  rest of the campaign. Repaired in the same pass.
  **And there was no save round-trip test at all** — added, running a campaign
  twelve days then comparing a save/load field for field. Lossless today; it is
  the guard that would notice a future state field arriving without serde wiring,
  which is a hard constraint in this file that nothing was checking.
  **Result:** 106 tests (was 102), balance byte-identical, fmt and clippy clean,
  publish green. Both repairs verified by planting the regression.
  **Next iteration should know:** (a) the reason six balance-measured passes
  missed this is structural — **the harness builds its state rather than loading
  it**, so nothing on the save path is exercised by any simulation; if more bugs
  of this class exist they will not show up as moved numbers; (b) the remaining
  `#[serde(default)]` fields were checked and their zeros are benign (resources,
  grade scores, empty id lists all mean "poor" or "early", not "disabled");
  (c) the honest next slice is still content — **guild rooms 4** for a
  20-companion roster, **patron tiers 3** with upkeep bands already referencing a
  fourth, and relics with no patron who asks for one by name.

- **2026-08-03 — the fifth copy of five-of-ten, and three sweeps that came back
  clean.** Extending last pass's insight (ask what the harness never runs), three
  systematic sweeps found nothing and are worth recording so nobody repeats them:
  every `UiAction` is emitted by some screen; every public engine function is
  exercised by a test bar `debt_intro_status`; and the opening chapter is
  affordable end to end — starting 180/60/18 covers the hatch (25g/3r) then the
  first room (40g/20m), which matters because the opening is a linear phase with
  no way to earn, so an unaffordable step would permanently soft-lock every new
  campaign. It is already guarded incidentally by two journal tests that play the
  opening out.
  **The one that was not clean:** the contract desk's gap badges checked five of
  ten skills and three of seven work-history categories — the same five-of-ten
  omission as the wage formula, `replacement_score` and `monster_service_score`,
  making this the fifth instance. The caller only falls back to the engine's
  complete reason list when the summary is **empty**, so a candidate blocked by
  both a covered and an uncovered requirement showed only the covered half: the
  card says "Charm 1/2", the player trains charm, comes back, still blocked, and
  nothing says why until that first gap closes. Now all ten and all seven, with
  labels from the engine's `format_skill_name` and `work_history_label` so the
  badge cannot drift from the refusal reason. Verified by planting.
  **Recorded, not done:** roughly forty player-visible strings are hardcoded in
  Rust across six screens — every Expedition Desk metric tile, the Town
  Management group tabs, two egg conversion buttons, and most of Hatch Reveal.
  `tests/ui_text_catalog.rs` enforces that every key is *read* and cannot see the
  inverse, so `ui_text.json` looks authoritative and is not. That is the
  dead-key argument pointing the other way, but it is a forty-string migration
  with mistyped-key risk and no player-visible gain today, so it wants its own
  slice rather than being smuggled into a gameplay pass.
  **Result:** 108 tests (was 106), balance byte-identical, fmt and clippy clean,
  publish green.
  **Next iteration should know:** (a) the five-of-ten shape has now appeared
  **five** times — if a sixth list of skills or work-history categories is ever
  written out longhand, it is almost certainly wrong; prefer
  `engine::companion_skill_total` and the label tables; (b) the code-defect
  sweeps are genuinely saturated — three of four came back empty this pass;
  (c) the honest next slice remains content — **guild rooms 4** for a
  20-companion roster, **patron tiers 3** with upkeep bands already referencing a
  fourth, and relics with no patron who asks for one by name.

- **2026-08-03 — the action layer, and a pass that found nothing.** Eight passes
  each found several defects; this one found none, and that is the result. It was
  not reached by giving up: the seventh pass's lesson was *ask what the harness
  never runs*, and the last surface that answered was `apply_action` itself.
  Every test here and the entire balance harness call engine functions directly;
  nothing exercised the action dispatch, the phase machine, or the transitions
  between them.
  **Sixty days driven through the actions a player actually sends came back
  clean** — staffing the hall, booking the desk and sending a party down each day
  before ending it, so the exclusivity rules from passes five and six and their
  refusals are on the path. The day advances every cycle, nothing wedges, and the
  campaign still validates at the end. That test is the deliverable: the first
  coverage of the route a player takes rather than the functions underneath it.
  **The one real finding was a testability hole in the worst possible place.**
  `apply_action` called `get_time()` inline to stamp a hatch reveal, which panics
  without a macroquad window — so the **opening chapter**, the sequence every new
  player hits first, could not be driven through the action layer at all. Time is
  sampled once per frame in `update` now and the action layer reads the stored
  value; the reveal's animation still reads real time when it draws, which is
  where wall-clock belongs. The opening plays out in a test through
  `ContinueOpening` / `BuildOpeningRoom` / `ResolveOpeningClient`, checking the
  dispatch and phase transitions rather than only the engine arithmetic two
  journal tests already covered. It matters because the opening is linear with no
  way to earn: a step the player cannot take soft-locks every new campaign.
  **Result:** 110 tests (was 108), balance byte-identical, no save file written,
  fmt and clippy clean, publish green.
  **Next iteration should know:** the bug-hunting is done. Nine passes have
  closed the panel-capacity, frozen-threshold, duplicated-formula, advice-surface,
  double-booking, save-path and action-layer classes, the authored-data audit
  comes back empty, and this pass found nothing at all. **Two slices with real
  substance remain, both already scoped:** the `ui_text` migration (~40 hardcoded
  strings across six screens — see `TODO.md`, eighth pass) and content
  (**guild rooms 4** for a 20-companion roster, **patron tiers 3** with upkeep
  bands already referencing a fourth, relics with no patron who asks for one by
  name). A tenth bug-hunting pass is not the best use of the next iteration.

## Deferred (needs a new system or a decision; not for this loop)

- **Make the validation policy's build order a plan rather than a shopping list.** Measured and
  working in this iteration but deliberately left out of it: in `policy_buildings.rs`, let a one-off
  building the guild still wants block the purchases behind it instead of being skipped over
  (repeatable sinks must not block, or the condenser sits in front of the prestige wing forever).
  On today's content that measured gold 730k → 932k, rank-5 escorts 10 → 13 and all 25 floors, with
  no assertion touched. It is a real improvement and it is also the precondition for ever adding
  another building, but it moves every balance number, so it deserves its own iteration.

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
