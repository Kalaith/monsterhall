# TODO — Monsterhall

## Unconnected systems (audit 2026-08-02)

Systems that exist in data/state/UI but never affect the simulation, or vice versa.

### Gameplay-affecting

- ~~Wire fatigue/stress/injury into the simulation.~~ Done and now complete. `engine/day_cycle/condition.rs` turns the three meters into one effectiveness percentage (`config.json` → `day_cycle.condition_effects`), applied to guild-job gold/materials/residue/reputation/prep quality, to each companion's stat contribution to an expedition plan, and — as of this pass — to contract rewards, which were the last place a companion run into the ground served a booking exactly as well as a rested one.
- ~~Cap the condition meters and let idle companions recover.~~ Done. Both were backed out in the first pass because they swung the single-seed report 25 unlocked floors → 11; that was the `final_unlocked_floors` assertion measuring route choice, not a defect in either change, and with it replaced they land untouched. `max_meter` (100) stops a contract worker climbing past 400 and being written off permanently by a bad fortnight, and `idle_fatigue_recovery`/`idle_stress_recovery` stop the meters being a one-way ratchet for anyone the player never explicitly parks — standing around the hall is rest, just poorer rest than the Resting assignment.
- ~~Fix expedition priority/mission selection reaching game state.~~ Done: `open_expedition_planning` (`game/navigation.rs`) now pushes floor/mission/stance into an already-formed `active_expedition`, so what the preview shows is what day resolution runs.
- ~~Apply `TraitData.stat_modifiers`.~~ Done, after three iterations parked. `engine/companion.rs` sums the bonuses on demand rather than baking them in at hatch, because mutations grant traits after a companion exists. Wired into both expedition stat totals and the endurance term in `previews.rs`, the whole guild-job yield chain, `monster_role`/`role_affinity` in `depth.rs`, `monster_depth_role_label`, `replacement_score`, and the profile screen, which shows the trait share as `12 (+2)`. It only became landable once `final_unlocked_floors == 25` was replaced by the stranded-floor check — that assertion, not this change, was what blocked it.
- ~~Consume `charm_training_flat`.~~ Done: it is percentage points on `should_gain_charm`'s per-room odds, so a Nursery Habitat now trains more charm than an empty lot. A room that never teaches charm is still not taught it by architecture — the town bonus improves lessons that already happen rather than inventing them.
- ~~Second half of the skill system.~~ Done. The mechanism landed earlier (all ten skills survive increment/name/score, `recovery_shifts` → recovery and `contracts_completed` → bargaining, two guards against unqualifiable contracts and unteachable rooms). This pass switched two on in `guild_rooms.json`: `packroom_annex` and `reception_hall` teach **recovery**, `nursery_wing` teaches **bargaining**.

  Which rooms was measured, not guessed. Turning bargaining on at `common_room` as well — the starter room, which the guild works most — compounds `guild_job_skill_bonus` enough that the deterministic campaign finishes the Founder's Due before day 365, which the spec forbids. `nursery_wing` alone lands green. That leaves bargaining a specialist's skill taught where contract work is the focus rather than on the room everybody starts with, which is a defensible shape as well as the one the numbers allow.

  A new test enforces the pairing: a room may only teach a skill its own banked work can feed, so a `trained_skill_ids` entry can never again be an empty promise that quietly inflates `guild_job_skill_bonus`. Note the simulation never exercises bargaining, because `best_unlocked_room_id` always takes the highest gold yield and `nursery_wing` (72) loses to `reception_hall` (86) — that is a real player tradeoff rather than a gap, but it does mean the skill is unproven in the long-campaign reports.

  Navigation, arcana and strength still have no work-history category feeding them, so no room may teach them yet — the new test would reject it. They need a gain source first, and `preparation_quality` and depth scoring still read them as zero.

- ~~Align the planning preview's `injury_risk_score` with the actual injury roll.~~ Done. The preview now runs `expedition_safety_score` — resolution's own arithmetic — and reports the margin for whoever is most exposed, so zero means somebody is certain to come home hurt and everything below it is daylight. `risk_label`/`risk_color` re-banded to match, and `expedition_growth_score` updated to the new scale with a stated margin rather than a bare `.max(0)` that the rescale would have silently turned into a no-op.
- ~~Guild-job preview shows `work_history_gains` verbatim while resolution rolls per-room probabilities.~~ Done: the odds were a `match` on room id inside `progression.rs`, so the preview could only quote the ceiling. They now live in `guild_rooms.json` as `work_history_gain_chance_pct`, `roll_work_history_gains` reads them, and the guild-hall card renders `C+1 @12%` instead of a bare `C+1`. The refactor is RNG-identical — every room's categories already rolled in the field order of `CompanionWorkHistoryProgressionData`, and a zero chance skips the roll exactly as a zero ceiling always did, so the simulation reports are byte-for-byte unchanged.
- ~~`hatchery_assists` had no source in the entire game.~~ Fixed: all four guild rooms authored a ceiling of 0, so no shift could ever bank one — which made `corekeeper_sending_vigil` (requires 3) permanently unfulfillable and left the `hatchery_specialist` role in `monster_role` reachable only through the `hatchery_attuned` trait. `nursery_wing` already carried the intended 5% odds, so it now has the ceiling to match. A new test asserts no room authors odds for work it cannot bank. Note the contract needs roughly sixty nursery shifts at 5%; whether that is the intended pace is a balance question.
- ~~`ContractStatus::Completed`/`Failed`/`Declined` are never assigned.~~ Done: resolution now stamps the outcome and moves the contract to a new `GameState.resolved_contracts`, which the desk lists under the live offers so the player can see how yesterday's bookings went. The lingering-status approach was tried first and abandoned — a resolved booking left inside `active_contracts` moves the offer limit, the request-id sequence, `workforce_demand`, the follow-up check and the booking policy, and it shifted the 365-day report by six buildings. A separate list is neutral by construction. `assign_monster_to_contract` and `clear_contract_assignment` now refuse non-live contracts, which is a real guard rather than bookkeeping.
- **The harness no longer pins the simulated guild's route.** `final_unlocked_floors == floors.len()` was replaced by `stranded_floor_ids` (`engine/depth.rs`), which asks the question the old assertion's comment actually described: is any floor locked behind a gate that can never be met — a `required_roster` species neither on the roster nor obtainable from any unlocked floor? That is the recorded bug (a mutation consumed the last Minotaur Porter and closed eleven floors). Where the guild chose to walk is not a defect, and pinning it fired on four separate correct changes while catching nothing. Every remaining parked item should be re-tried against this; trait `stat_modifiers` landed immediately.
- ~~`event_tags` encode intended gating that no code applies.~~ Investigated and resolved differently: the gating **is** applied, just not by the tags. Every one of the 23 tier/late_game-tagged events is already `required_building_ids`-gated (23/23), and the `min_day` bands are tight and monotonic — tier_1 at days 8–20, tier_2 at 20–30, tier_3 at 45–55, late_game at 140–230. You cannot get the archive event without the archive. The tags are a taxonomy describing that gating rather than a second gate, and mapping `tier_N` onto patron tiers was a dead end anyway: only three tiers exist against four tags. So they are now checked documentation — load-time validation rejects a tier/late_game tag on an event nothing gates, and a `late_game` tag on an event that can fire before day 100.
- ~~Patron archetype `spawn_weight` and `tags` never influence contract generation.~~ Done, though not the way it was first written. `spawn_weight` now scales `request_pressure_priority`, so rarer patrons lose ties and fall off a full board first. The literal reading — weighted random draw without replacement — was built twice and rejected on measurement both times: the campaign is tuned around a best-first board, and flattening it cost enough income that the single-seed run stopped reaching its final debt milestone (only `tribute_cart_5` with flat weighting, `broker_compact_6` with the pressure term squared). Variety in the offer board is worth having, but it needs the economy retuned around it, not bolted on. `tags` now carry a guard: an archetype tagged `special` whose contracts are not `is_special` is rejected at load, because that silently costs the story flag and the priority bonus.

### Roster variety — design direction given 2026-08-02

**Stated intent:** a day-365 roster should show variety in creatures. Lower-tier
monsters cannot do higher-tier work (a slime cannot explore the deep floors), but
it should not be all high tier either — high tiers cost more and are less
flexible across roles.

Against that, one of the three mechanics was simply absent:

- ~~High tiers did not cost more.~~ Fixed. `companion_daily_wage` was
  `quality_rank` plus skills — both properties of the *egg a companion hatched
  from*. Mutation rewrites `species_id` mid-campaign and every step up the tree
  raises base stats, so a `gargoyle_stairwarden` at 10/4/10/6 cost exactly what a
  `slime_companion` at 3/2/5/2 cost: climbing the tree was free power, and
  nothing on the ledger said otherwise. The wage now carries a species term read
  from `base_stats` (so it cannot drift from a second authored ranking), divided
  by `day_cycle.species_stat_wage_divisor`. Measured on the 365-day report:
  **3 species → 6, and golemkin 18/20 → 12/20**, with a `revenant_chorister`
  appearing for the first time. A test asserts any species that outclasses
  another costs at least as much, so a future species cannot be authored as free
  power.
- **Low tiers cannot do high-tier work** — already true: floors gate on
  `required_roster` species and depth scoring reads stats.
- ~~High tiers are less flexible across roles.~~ Done. `role_affinity` was a
  hardcoded flat 12 for a matching role, 4 for `versatile`, 0 otherwise — so a
  gargoyle was exactly as flexible as a slime and strictly stronger, and there
  was never a reason to keep a low tier once a high one existed. Off-role now
  costs a penalty that scales with the species' base-stat total, from nothing at
  `flexibility_stat_floor` to `off_role_penalty_max` at
  `flexibility_stat_ceiling` (new `day_cycle.role_affinity` block — the numbers
  were hardcoded, which was a data-driven-design violation as well). `versatile`
  is deliberately exempt: being flexible is what that role *is*.

  Built the other way round first — crediting *low* tiers off-role rather than
  penalising high ones — and reverted it on measurement: inflating a weak
  companion's apparent off-role value made the policy staff her onto work she was
  bad at, costing the deterministic campaign a building tier and **11 missed debt
  payments**. Penalising the strong leaves every existing assignment intact and
  all 79 tests green.

- ~~Half the tower's eggs funnelled down one lineage.~~ Rebalanced. Following
  every `egg_species_entries` weight to the species it eventually mutates into:

  | Lineage ends as | Before | After |
  | --- | --- | --- |
  | `gargoyle_stairwarden` (via golemkin) | **50.2%** | 32.9% |
  | `wyrm_registrar` | 28.6% | 31.7% |
  | `revenant_chorister` | 18.7% | 26.4% |
  | `salamander_corekeeper` | **2.6%** | 8.9% |

  The egg tables looked varied per floor — `golemkin_warden` and `moth_archivist`
  each appear on 13 of 25 floors at identical weight — so the skew is invisible
  unless the lineages are followed through. `residue_slime` is the second most
  common egg in the tower and converts to golemkin at 16 corruption, which is
  what stacks the deck. `salamander_corekeeper` at 2.6% also made
  `corekeeper_sending_vigil` close to unfulfillable for a player who *does*
  explore. Weights only — no species moved between floors, because the depth
  tiering is deliberate and correct (`slime_companion` appears on depths 1–2
  only, and nothing below stat total 19 appears from depth 3 down).

  **Caveat worth stating: the simulation cannot confirm this helps.** The
  composition reports are byte-identical before and after, because the simulated
  guild stops running expeditions at day 76 and hatches 21 companions in a whole
  campaign, so it barely draws from the egg tables at all. This is a fix to the
  content distribution a *player* sees, validated by arithmetic rather than by
  the harness.

Two levers were tried against the funnel and **rejected on measurement**, which
is worth recording so they are not retried blind:

- Raising the entry thresholds (`slime -> residue` 8→40, `residue -> golemkin`
  16→70) produced **77 missed debt payments**. Keeping the roster low-tier that
  long starves the economy — the mutation upgrade is currently load-bearing for
  debt service.
- A moderate version (20/38) still failed the building-chain assertion (8 against
  9 required) and did not improve composition, because delaying the funnel does
  not break it: `golemkin_warden` is *terminal* for the slime lineage, so
  everything still arrives there, just later.

**The structural reason is not the thresholds.** The mutation tree already has
four distinct endpoints — `gargoyle_stairwarden`, `revenant_chorister`,
`wyrm_registrar`, `salamander_corekeeper`. Variety is designed in. The roster
goes monoculture because the guild almost only ever hatches slimes, and every
other lineage begins at a species whose eggs come from deeper floors. The
simulated guild stops running expeditions at day 76 on the 365-day seed
(`expedition_days_after_day_90 = 0`), and `egg_reward_days_after_day_90` is **0
on all three seeds** — even the 180-day run, which does 90 expedition days after
day 90 and gets no eggs from any of them, because `expedition_growth_score`
drops egg value to 15 against relics at 70 once `pending_eggs_cover_workforce_demand`.

So roster variety is gated on the same question as the survey chain below: what
makes a well-resourced guild keep going down the tower instead of settling into
contract work. Fixing egg supply is likely worth more to variety than any
further mutation tuning.

### Open design question: mutation collapses the roster onto one species

Measured, not guessed. Every simulation report now carries
`final_species_counts` and `final_corruption_max`, because the newest gameplay
system — corruption rewriting a companion's species mid-campaign — was the one
thing the reports did not track at all. What they show:

| Day | Roster | Composition | Corruption max |
| --- | --- | --- | --- |
| 90 | 14 | **golemkin_warden 12**, residue_slime 1, slime_companion 1 | 169 |
| 180 | 19 | **golemkin_warden 17**, harpy_lookout 1, moth_archivist 1 | 457 |
| 365 | 20 | **golemkin_warden 18**, minotaur_porter 1, wyrm_registrar 1 | 494 |

`final_role_diversity` has read **1** on the 365-day report all along — the
report was already saying the roster had collapsed to a single role and nothing
asserted on it.

Two mechanical causes, both visible in the data:

- **The entry thresholds are trivial against the corruption a campaign
  reaches.** `slime -> residue_slime` at 8 and `residue_slime -> golemkin` at 16,
  against a corruption max of 169 by day 90. Every slime becomes a golemkin
  almost immediately.
- **Golemkin is terminal for that route.** Its only exit,
  `golemkin_warden -> gargoyle_stairwarden` at 100, requires `commanding` *and*
  `resilient`. The slime/residue lineage accumulates
  `{stretchy, eager, corruption_tuned, commanding}` — never `resilient`. Only
  the `minotaur_porter` route supplies it, and that mutation needs 90 corruption
  of its own.

Consequences worth weighing: ten unlocked species hold nobody at day 365, so the
ten contracts requiring a specific species are unfulfillable in practice even
though every one passes the static reachability check. Corruption above 100 does
nothing at all — the highest threshold in the catalogue — so roughly 400 points
of its range are inert, the same runaway that `max_meter` was added to stop for
fatigue and stress.

**This is a balance decision, not a bug fix, so it is left for a deliberate
call.** The options measurement cannot choose between: raise the early
thresholds so the first two steps are a mid-campaign event rather than a
formality; give golemkin an exit its own lineage can satisfy; give corruption a
relief mechanism (it currently has *three* writers, all `saturating_add`, and no
reduction path anywhere in the codebase — resting recovers fatigue, stress and
injury but not this); or accept that the tower turns everyone to stone and say so
in the spec.

What was fixed this pass is the measurement, not the balance: the collapse is
now in every report instead of only visible by running the probe by hand.

### Found by review, not by the audit (2026-08-03, thirteenth pass)

Followed the twelfth pass's note to the other monotonic meters. `bond`,
`reputation`, `corruption`, `skills` and `work_history` are **all** strictly
monotonic — not one of them is ever decremented anywhere in the game. The
threshold sweep over them turned up no second latch worth the name, but it
walked straight into the shape this ledger has now recorded six times, and this
time with a root cause rather than another instance.

- ~~The authored text catalogue named five skills; the game has ten.~~ Fixed.
  `ui_text.common` carried five flat `skill_label_*` keys, so **every screen
  that reached for the authored vocabulary — the correct place to reach — could
  only name half the game**, and screens that needed all ten had to hardcode
  English in Rust (`format_skill_name`). That is why this keeps recurring: the
  right instinct produced the wrong list.

  Live consequences, all on screens a player uses to decide something:
  - `packroom_annex` and `reception_hall` train **recovery**, `nursery_wing`
    trains **bargaining**. The guild hall's room card mapped both to
    `unknown_label`, so it read *"Trains Crafting, Charm, Unknown"*.
  - `companion_skill_summary` — the roster strip and the contract desk's
    candidate line — printed the five original skills including their zeros and
    none of the five added later, so a companion could train recovery for a
    season and her line never changed.
  - The **profile screen**, the one place a player opens to see what a companion
    has learned, drew five hardcoded English chips (`Scout`, `Guard`, `Hosp.`,
    `Craft`, `Charm`).
  - `guest_skill_requirement_label` and `opening_skill_gain_label` covered the
    same five, with letter codes (`K+`, `O+`, `V+`) left over from the premise
    this game was reskinned from.

  The catalogue is now a **list** — `common.skills`, one `{id, label, code}` per
  skill — and the engine has `SKILL_IDS` plus `companion_skill_value` /
  `required_skill_value` / `progression_skill_value`, so a site iterates instead
  of hand-listing. Hand-listing is what produced five every single time.

  **Three guards, each verified by planting.** Every skill the engine knows has
  authored text, and no authored entry names a skill it does not (planted:
  dropping recovery and bargaining from the list). `SKILL_IDS` reaches every
  field of both skill structs, via distinct powers of two so an eleventh field
  added without its id leaves a sum this cannot reach. And the profile band —
  two fixed rows, now four columns — holds every skill any room can teach plus
  bond, so a room authored to teach an eighth skill fails the test instead of
  the panel silently dropping a chip (planted: a room teaching eight).

- ~~The room card said the common room trains one skill when it trains three.~~
  Fixed, and it was pre-existing rather than fallout. The badge is 168px in a
  row that exactly fills its 432, with a hard 20-character cap, so
  *"Scouting, Hospitality, Charm"* was cut to **"Trains Scouting,"** — which does
  not look truncated, it looks like a room that teaches one thing. Choosing a
  room is the entire decision that screen exists for. It now uses the compact
  codes the vocabulary authors for exactly this and reads "Trains Sc/Ho/Ch",
  complete. Verified in a capture rather than by reasoning about pixels.

- **Balance byte-identical** — the only line that moved in any report is
  `content_version`. This pass changed what the game tells the player, not what
  it does.

### Found by review, not by the audit (2026-08-03, twelfth pass)

Swept the remaining axes the reachability heuristic had not been pointed at —
relics, events, missions. Event gates came back clean: all eighteen
species/trait-gated events name a combination some companion can reach, checked
against `reachable_trait_states()`. The mission axis did not, and the reason was
temporal rather than static.

- ~~Every companion became a `corruption_adept` around day 10 and stayed one for
  the rest of the campaign.~~ Fixed. `monster_role` is an ordered ladder, and
  its first rung was `monster.corruption >= 10`. **Corruption is only ever
  `saturating_add`ed — there is no path in the game that reduces it.** The
  reception hall adds 1 an occupied shift and the packroom 2, so a working
  companion crossed 10 inside a fortnight of a 365-day run and could never read
  as anything else again, whatever her stats, skills, traits or bond said.

  Everything below that rung was therefore dead in play from about day 10:
  `hatchery_specialist`, `performer`, `delver`, `comfort` and — the expensive
  one — **`versatile`**, which is the entire mechanism the roster-variety slice
  added so that low-tier companions stay worth keeping. `versatile_bonus: 4` was
  authored, measured, and unreachable. Of the seven missions, only
  `scout_route` and `sealed_extraction` prefer `corruption_adept`; on the other
  five every companion took `-off_role_penalty`, scaled by species stat total,
  so the high tiers paid the most for a role nobody could avoid.

  **The fix is not a bigger number.** Any threshold on a monotonic meter is a
  latch and the only question is which day it shuts. Corruption already reaches
  the role system by the route that can express change rather than accumulation
  — mutation, which rewrites the species and carries `corruption_tuned` along
  most of its branches, and which deliberately does *not* on others
  (`imp_runner → lamia_routekeeper` produces a changed companion who is not an
  adept). So `corruption_adept_minimum` is `Option<u32>` and the catalogue omits
  it, the same idiom the upkeep band established for distinguishing "no axis"
  from zero.

  **All six rungs are authorable now** (`day_cycle.role_thresholds`); they were
  literals inside the classifier, which put the game's answer to *what is this
  companion for* out of the authors' reach. The role ladder moved to
  `engine/depth/roles.rs` — `depth.rs` was at 855 lines and the standards
  require the restructure to happen in the pass that touches it.

  **Two guards, both verified by planting `10` back.** A unit test asserts a
  companion keeps her role at corruption 8, 10, 16, 100, 488 and `u32::MAX`
  (this is the property that was violated, and it is temporal, so no static
  check could have caught it). Load-time validation rejects a threshold at or
  below the *last* mutation: while a companion still has any mutation ahead of
  her the tower has not finished with her, and a raw meter reading must not
  overrule the mechanism that is supposed to be doing this. A third test — every
  role a mission prefers is one some companion reads as — was added alongside,
  and immediately caught a bad fixture rather than a bad game: companion stats
  live on `CompanionState`, written from the species at hatch, so a fixture
  leaving them zero is testing a companion who does not exist.

- **Measured: `final_role_diversity` 1 → 3.** The day-365 roster shows three
  distinct roles where it showed one, which is the stated roster-variety intent
  reaching the report for the first time. The economy barely moves — ten-seed
  gold 1.26M → 1.32M (+4.3%), buildings 31.5 → 31.9 — because the dominant
  golemkin line carries `corruption_tuned` as a starting trait and is *correctly*
  adept; what changed is that the porter, the imp and the routekeeper get their
  own roles back.

- **The species monoculture is still the ceiling on this.** Role diversity 3
  against a roster of 20 is bounded by species diversity, which remains the
  parked mutation item. What this pass removed was a defect that would have
  defeated that fix too: with the latch in place, fixing species variety would
  have produced twenty different creatures all reading as the same role.

### Found by review, not by the audit (2026-08-03, eleventh pass)

The tenth pass's heuristic — *a number that cannot be reached* — kept pointing at
thresholds. Turned one notch, it points at **content**: not a number nobody can
reach, but a thing nobody can hold.

- ~~`calming_presence` was authored, priced, rewarded in five places, and no
  companion could ever have it.~~ Fixed. A trait reaches a companion exactly two
  ways — a species' `starting_traits`, or a mutation's `granted_trait_ids` — and
  every mutation here grants the trait its own target species already starts
  with, so a trait belongs to a species or it belongs to nobody. This one
  belonged to nobody for the game's whole life (`git log -S` finds no species or
  mutation ever holding it, under this name or its pre-rename `submissive`),
  while five pieces of content paid for it: `common_room` and `nursery_wing`
  listed it as preferred, three guest contracts wanted it
  (`repeat_client_confidence`, `quiet_room_return`, `veiled_suite_audience`),
  and `contract_depth_score` read it as the companion who settles a room. It is
  the joint fourth most-wanted trait in the game by consumer count.

  Nothing said so because every consumer checks its trait ids against
  `traits.json`, which had the trait. The question none of them asked is whether
  anyone can hold it.

  **Where it belongs was structural, not taste**: `slime_companion` was the only
  species carrying one trait where the other eleven carry two, and
  `calming_presence` was the only trait carrying no species. One hole, one
  orphan, and the trait's own text — *"thrives under clear direction"* — is the
  biddable starter. It reaches the late roster by inheritance rather than by
  species: no slime survives to day 365, but the dominant golemkin line descends
  from one, and traits accumulate along a lineage.

  **Guard**: the mutation-reachability walk already computed every
  `(species, traits)` state a companion can stand in — it just never asked
  whether each authored trait appears in one. It is now
  `reachable_trait_states()`, shared by the existing mutation check and two new
  tests: every authored trait is holdable, and every trait a room or contract
  *prefers* is holdable, since a preference naming a trait nobody has is a bonus
  that never lands. Verified by planting the regression; both fire, naming the
  trait and the room.

- **Balance deliberately re-based, and this one is large.** Five content hooks
  that never fired now fire, and the trait's stat block (charm +2, +12% guild
  income, −1 stress, −2 expedition) was authored against a trait nobody had, so
  it had never been balanced against play. Ten-seed day-365 means:
  **gold 855k → 1.26M (+47%), buildings 18.4 → 31.5 (+71%), worst-seed gold
  109k → 530k, and the worst debt gap +778k → 0** — every seed now clears its
  milestones. Single-seed `final_corruption_max` **112 → 488**, which does not
  make the parked mutation/corruption item worse so much as show what it already
  was at four times the volume.

  **Whether the trait's numbers should now be retuned is a design call, not a
  bug**, and it is left to the player-facing owner rather than made here: the
  fix is that the trait exists at all. The measurement above is what a retune
  would be arguing with.

### Found by review, not by the audit (2026-08-03, tenth pass)

The ledger's own "thin axes" note turned out to name a live unconnected system
rather than a content wish: **patron tiers 3, with upkeep bands already
referencing a fourth**.

- ~~The top upkeep band asked for four patron tiers against a catalogue holding
  three, and the Town Overview printed that number at the player.~~ Fixed.
  `active_upkeep_band` selects with `count >= threshold` on two axes joined by
  OR, so band 3's tier axis could never fire — the band ran on `min_companions:
  16` alone, with nothing saying so. Worse, the Town Overview's upkeep line read
  *"Band 16 companions / 4 patron tiers"*: a threshold the game cannot reach,
  shown to the player as though planning around it were possible.

  The fix had to say what the band actually does, and **`0` could not say it** —
  that is the `party_size` trap again, because `count >= 0` is always true, so a
  zero would have pinned the guild to the top band's 180% wages from day one.
  `min_patron_tiers` is `Option<u32>` now: the top band omits it, which is
  behaviour-identical (`companions >= 16 || tiers >= 4` and `companions >= 16`
  agree at every reachable tier count), and the Town Overview drops the clause
  rather than printing a number.

  **Load-time validation rejects both mistakes**: a tier threshold above the
  authored tier count, and a zero on either axis — with the error naming the
  distinction, since `None` and `0` read the same to an author and behave as
  opposites. Verified by planting each.

- **A sweep of every authored threshold against what the catalogue can supply
  found nothing else.** Band companion counts sit under the population cap, every
  patron tier is both served by a room and unlocked by a building, and every
  tier's `minimum_quality_rank` is inside the star ladder.

**Balance byte-identical.** The only line that moved in any report is
`active_band_min_patron_tiers`, 4 → 0, which is the reported threshold changing
from a lie to "no axis" — not one economic number shifted, because the band
selection was already behaving this way.

**The content question is unchanged and still open.** Authoring a fourth patron
tier would make that axis live and raise upkeep earlier for a guild that unlocks
it; that is a balance change needing a room to serve it and a building to unlock
it, and `loop.md` records that a new one-off building is very hard to land. What
this pass fixed is the game telling the player a tier that does not exist is
worth reaching.

### Found by review, not by the audit (2026-08-03, ninth pass)

**This pass found no defect.** After eight passes that each found several, that is
itself the result worth recording — but it was not found by giving up early. The
last untried surface was the one the seventh pass's lesson pointed at: *what does
the harness never run?* The answer was **the action layer**. Every test in this
repo, and the whole balance harness, calls engine functions directly; nothing
exercised `apply_action`, the phase machine, or the transitions between them.

- **Sixty days driven through the actions a player actually sends: clean.** The
  new smoke test staffs the hall, books the desk and sends a party down each day
  before ending it — so the assignment rules added in passes five and six, and
  their refusals, are on the path too. The day advances every cycle, nothing
  wedges, and the campaign still passes `validate_game_state_references` at the
  end. No stuck state, no phase that fails to return, no action that errors where
  a player would expect it to work.

- ~~The opening chapter could not be driven through the action layer at all.~~
  Fixed, and this was the one real finding. `apply_action` called `get_time()`
  inline to stamp a hatch reveal, which panics without a macroquad window — so
  the sequence **every new player hits first** was untestable through the path
  they take. Time is sampled once per frame in `update` now and the action layer
  reads the stored value, which makes it a pure function of state; the reveal's
  own animation still reads real time when it *draws*, which is where wall-clock
  belongs.

  The opening now plays out through `ContinueOpening` / `BuildOpeningRoom` /
  `ResolveOpeningClient` in a test, checking the dispatch and the phase
  transitions rather than only the engine arithmetic that two journal tests
  already covered. That matters because the opening is linear with no way to
  earn: a step the player cannot take is a permanent soft-lock on every new
  campaign, and until now nothing exercised the route they actually use to take
  it.

110 tests (was 108). Balance byte-identical, no save file written by the run.

### Found by review, not by the audit (2026-08-03, eighth pass)

Three systematic sweeps came back clean before the one that did not, and the
clean results are worth recording so they are not re-run blind:

- **Every `UiAction` variant is emitted by some screen** — no feature is
  unreachable, which was the shape behind the profile-screen bug.
- **Every public engine function is exercised by a test** except
  `debt_intro_status`, a pure string formatter.
- **The opening chapter is affordable end to end** — starting resources 180/60/18
  cover the slime's hatch (25g/3r) and then the first room (40g/20m). It is a
  linear phase with no way to earn, so an unaffordable step would soft-lock every
  new campaign permanently; it is already guarded, incidentally, by two journal
  tests that play the opening out and `.expect()` each step.

- ~~The contract desk's gap badges named five of ten skills and three of seven
  work histories.~~ Fixed, and this is the **fifth** copy of the same five-of-ten
  omission — after the wage formula, the hatchery's `replacement_score` and the
  policy's `monster_service_score`. `blocked_candidate_summary` checked
  scouting/guarding/hospitality/crafting/charm and three history categories, and
  the caller only falls back to the engine's *complete* reason list when the
  summary comes back **empty**. So a candidate blocked by both a covered
  requirement and an uncovered one showed only the covered half: the card reads
  "Charm 1/2", the player trains charm, comes back, and she is still blocked with
  nothing saying why until that first gap closes. All ten skills and all seven
  categories now, with labels taken from the engine's own `format_skill_name` and
  `work_history_label` rather than a third naming, so a badge cannot call a
  requirement one thing while the refusal reason calls it another. Two tests,
  verified by planting.

- **Player-visible copy is hardcoded in Rust on six screens, and `ui_text.json`
  is not the source of truth it appears to be.** Measured while chasing the
  above: roughly forty capitalised literals go straight to draw calls — every
  Expedition Desk metric tile ("Success", "Injury Risk", "Party Condition",
  "Materials", "Eggs", "Residue", "Relics"), the Town Management group tabs
  ("Core", "Projects"), the egg conversion buttons ("Sell Egg", "Refine"), the
  profile screen's "Today" and "Danger Zone", and essentially the whole Hatch
  Reveal screen.

  `tests/ui_text_catalog.rs` enforces that every *key* is read and cannot see the
  inverse, so the catalogue looks authoritative and is not. This is the same
  complaint the dead-key deletion was justified with — "an author edits the
  wording and sees no change" — pointing the other way. **Recorded rather than
  done**: it is a forty-string migration across six screens with real
  mistyped-key risk and no player-visible benefit today, which is a poor trade
  against a pass asked to focus on gameplay. It wants its own slice.

### Found by review, not by the audit (2026-08-03, seventh pass)

This pass swept the **save path**, which six passes of screen-and-engine hunting
had never touched. `#[serde(default)]` is on every saved struct — correctly, so
old saves keep loading — and that is exactly how a save arrives holding a value
that is structurally valid and functionally fatal. It is the display-settings bug
again, one layer down, and this time it kills the game rather than a highlight.

- ~~A save missing `party_size` and `town_job_limit` loaded fine and killed both
  of the game's verbs.~~ Fixed. Both gates read `count >= limit`, so a defaulted
  **zero does not mean "no limit"** — it means *nobody may ever be sent on an
  expedition or given a guild-room shift again*. Measured on a real
  `save_version: 9` payload: the save parses, `validate_game_state_references`
  **passes it**, and then `assign_monster_to_room` and
  `assign_monster_to_expedition` both refuse forever. The campaign is left able
  to end days and nothing else, with no error to explain it.

  New `engine::reconcile_game_state_after_load` restores the configured baseline
  when either reads zero, and `continue_game` calls it *before* validating —
  after would be useless, because the reference check is what waves the broken
  save through. Zero is never legitimate for either, and anything non-zero is
  left exactly as found: `town_job_limit` grows past its baseline through
  `passive_modifiers.town_job_limit_flat`, and clamping it back would demolish
  every worker-limit building the player bought. A test covers that too.

- ~~The same trap one level down: a companion loaded at rank zero.~~ Fixed. Rank
  0 is not a rank the game can produce — `egg_quality_rank` never returns below 1
  and a new companion is created at 1 — but it is what a save predating the field
  deserializes to, and it is quietly ruinous for that companion for the rest of
  the campaign. `evaluate_contract_eligibility` compares
  `rank < minimum.max(1)`, so she fails **every** contract including one that
  asks for nothing; the same comparison in `floor_roster_gate_report` means she
  satisfies no floor's roster gate; and `active_patron_tier_for_room` finds no
  tier she qualifies for, so every shift she works is paid at the understrength
  rate. Repaired in the same pass over the loaded state.

- **There was no save round-trip test at all.** Added: a campaign is run twelve
  days, saved, loaded and compared field for field. It is lossless today — this
  is the guard that would notice a future state field arriving without its serde
  wiring, which is the hard constraint `loop.md` names and nothing was checking.

Both repairs verified by planting the regression. Balance byte-identical: the
simulation builds its state rather than loading it, so none of this is on the
harness's path — which is precisely why six passes of balance-measured hunting
never found it.

### Found by review, not by the audit (2026-08-03, sixth pass)

- ~~The double-booking fix only covered one order.~~ Fixed. Last pass made
  `assign_monster_to_room` refuse a companion already booked onto a contract —
  but **booking a companion who was already working the hall was still allowed**,
  and `resolve_day` settles the contract first and discards her shift exactly as
  before. The same bug, reachable by doing the two actions in the other order.

  Refusing here would have been wrong: she is perfectly able to serve the
  contract. It is the *slot* that is wasted. So taking a booking now releases
  whatever she was rostered for — the same way every other assignment already
  releases her from an expedition — and the guild-job slot goes back for somebody
  else. **Zero balance movement**, as predicted: the policy books contracts
  before it staffs rooms, so only a human reaches this order.

- ~~A companion changing species was announced once, in a clipped panel, and
  never written down.~~ Fixed. Mutation text went only to `roster_updates`, which
  lives on the Day Results screen inside a 140px box — about **seven lines**
  against a twenty-companion roster — and is the one narrative list that is
  *never* extended into `event_log`. So the announcement was usually clipped away
  and then lost: the player would find a different species on the roster with
  nothing anywhere to say when or why, for the single system the whole corruption
  mechanic exists to drive. Mutations now go to `event_lines`, which reaches both
  the Day Results event panel and the journal, where it can be scrolled back to.

  **Every scalar in every report is unchanged except `final_event_log_entries`**
  (+23 to +39 across the seeds) — the mutations were always happening, they are
  just written down now.

- **The authored-data audit that opened this file re-ran clean.** Every key in
  every `assets/data/*.json` is consumed. The five the sweep flagged as read only
  inside `src/data` are all deliberate: `max_floor_difficulty`, `event_tags` and
  `preferred_room_ids` are load-time validation rules, and
  `keyboard_shortcuts_visible` / `primary_mode` are *constraints* — the validator
  actively rejects a config that turns shortcuts on-screen or picks a non-mouse
  primary mode. No unconnected data remains.

  Worth knowing as a design fact rather than a defect: the game deliberately ships
  its keyboard shortcuts undiscoverable. Enter ends the day, Ctrl+S saves, Escape
  opens settings, and nothing on screen says so — enforced, not forgotten.

### Found by review, not by the audit (2026-08-03, fifth pass)

- ~~A companion could be booked for two jobs on the same day, and one of them was
  silently thrown away.~~ Fixed, and this one moves the balance baseline — read
  the measurement below before trusting a pre-fifth-pass figure.

  `resolve_contracts` runs before the job loop and adds everyone it serviced to a
  skip set, so a companion who was both **accepted onto a contract** and
  **rostered to a guild room** did the contract and had her shift discarded.
  Nothing stopped the double booking: `evaluate_contract_eligibility` rejects a
  companion who is `OnExpedition` but says nothing about a guild job, and
  `assign_monster_to_room` never looked at contracts at all. Two screens lied
  about it — the Guild Hall kept quoting her projected gold, and the Expedition
  Desk kept counting her stats into the party preview — for work the day cycle
  would never run. With `town_job_limit` at **2**, burning one slot on a
  discarded shift is half the hall's income for that day.

  **The simulation was doing it too.** `assign_daily_jobs` computes
  `reserved_guest_monster_ids`, applies it to expedition selection, and then
  never checks it in the guild-job loop — so the policy reserved contract workers
  away from the tower and handed them to the hall anyway. The engine refuses the
  assignment now (`is_booked_for_contract`), and the policy check was added
  alongside so the intent is visible rather than learned from an error; the
  policy line is documentation, verified by measuring identical numbers with and
  without it.

  **Measured.** Direct effect on the deterministic seed:
  `total_guild_job_gold` **1,813,165 → 1,834,424** (+21k — the recovered slot,
  exactly the predicted direction). The multi-seed aggregate moved
  **1.07M → 855k gold** and **24.2 → 18.4 buildings**, but that is *chaotic
  divergence, not a cost*: per seed it goes both ways (1.78M→1.93M, 951k→958k and
  103k→109k up; 1.60M→713k, 1.22M→609k and 923k→226k down). Changing which
  companion works which day reshuffles the whole campaign path from day one.
  Zero missed payments, no assertion touched, 100 tests green.

- ~~The two assignment screens still offered buttons that could only error.~~
  Followed through. A booked companion now shows **"On Contract"** in place of
  her job state, greys out, and loses both her Assign and her Rest button on the
  Guild Hall and the Expedition Desk. Rest was as futile as a shift — day
  resolution skips her entirely, so her fatigue never came down either. The
  `_full` capture scenes book the first companion so the state is
  photographable; `ui_guildhall_full.png` is the baseline.

### Found by review, not by the audit (2026-08-03, fourth pass)

The panel-capacity class is closed, so this pass swept the two surfaces that
**advise** the player. Both were giving advice the engine contradicts.

- ~~The profile screen called a companion "hurt" after one day's work.~~ Fixed,
  and this is the sharpest of the frozen-threshold bugs so far because it cost
  the player *days*. `monster_role_summary` read
  `injury > 0 || stress >= 3 || fatigue >= 3` — written before the condition
  system existed and never revisited against it. One guild shift adds **10
  fatigue and 4 stress**; the allowances are **30 and 20**. So from her very
  first shift every companion showed "hurt" with "best next use: rest", while
  `companion_effectiveness_pct` — the function that actually decides her output —
  still returned exactly **100**. The player was being told to spend a rest day
  recovering nothing, permanently, for the whole roster. Readiness now asks the
  engine: she needs rest when her condition is genuinely costing output and not a
  day before. New `engine::companion_effectiveness` is that one answer, exposed
  so screens stop inventing their own.

  The same function also carried a **partial fourth copy of the role
  classifier** — it re-tested `power >= charm + 2` and a couple of skill
  thresholds to pick a recommendation, so it could disagree with the role printed
  in the same sentence. It maps over `monster_role` now.

- ~~"Today's Priority" hid the debt window behind any egg, and told a full guild
  to grow its roster.~~ Fixed, two orderings in one branch chain.
  `daily_priority_summary` ranked eggs **above** the debt warning, so a single
  egg in the inventory meant the debt panel never appeared — and the debt copy's
  own words are *"favour reliable guild work and contract fulfilment over
  speculative tower work"*, which is exactly the call it was being prevented from
  making. Missing a payment costs gold and stresses the whole roster; an egg
  keeps. Debt outranks eggs now.

  And the eggs branch reads "grow the roster before the day ends", which at the
  population cap is the one thing hatching cannot do — the egg needs a companion
  released first. The guild fills its cap by mid-campaign, so the panel stuck on
  impossible advice for the entire late game and never mentioned contracts or
  growth again. It is gated on being below the cap. Both verified by planting the
  regression.

- ~~The expedition injury amount was the last balance number in the day cycle's
  Rust.~~ Moved to `config.json` as `expedition_injury_amount`. Every other side
  of that exchange was already authored — `base_injury_recovery`,
  `injury_allowance`, `injury_penalty_pct_per_ten`, `expedition_injury_threshold`
  — so how hard a bad run hits was the one term nobody could tune. Authored at
  the same 6, so nothing moves.

### Found by review, not by the audit (2026-08-03, third pass)

- ~~The Guild Jobs worker column drew off the bottom of the screen.~~ Fixed, and
  it was the third assignment surface with the same six-companion assumption —
  but failing the other way. `draw_worker_cards` clamped its panel to
  `.min(330.0)` while the loop below drew a card for **every** worker, so a full
  guild's Available column ran nine rows past its own frame, straight through the
  footer and off the bottom of the window. The capture shows companions 13–20
  drawn where nothing can be clicked. 330px was only ever enough for three rows,
  so this already overflowed at six workers. The column now derives its row
  capacity from the space between itself and the footer and pages the rest with
  the same `RosterWindow` the other three panels use. Assigned Here never pages —
  `town_job_limit` caps it at two.
- ~~The Hatchery drew four egg rows into a panel that holds eight.~~ Fixed. The
  row count was a hardcoded `4` from when the panel had a fixed height; the panel
  now fills the screen, so at 1080p half the inventory column was empty and the
  player scrolled twice as far as the panel needed. `egg_rows_visible()` derives
  it from `content_h`.
- ~~Both mouse-wheel hit-boxes were stale copies of geometry the screens
  compute.~~ Fixed. The Hatchery handler claimed the **full screen width**, so
  hovering the detail panel scrolled the egg list, and a fixed `230..666` band
  that missed the panel's top and most of its bottom. The Journal handler carried
  a fixed 720px height against a log panel that follows the window — 60px too
  tall at 1080p and 420px too tall at 720p, so the wheel scrolled the log from
  over the footer. Both now read the screen's own layout, and the visible-row
  count is a single constant rather than one per file.
- ~~A thumbnail caption could print through the panel beside it.~~ Fixed at the
  shared helper. `draw_text_center` centred text without ever measuring it
  against the box width, so a caption wider than its box spilled out of **both**
  sides. Every thumbnail caption in the game goes through it. The Contract Desk
  showed the cost: the guest name under the silhouette reached far enough right
  to print through the room name, reward, penalty and deadline in the detail
  column — on the one screen whose whole job is showing a contract's
  requirements, and the last of the overlaps recorded as "left" from the
  2026-08-02 capture pass. Captions are ellipsised to fit now; anything that
  already fitted is unchanged.
- **The capture harness now crowds the egg inventory as well as the roster.**
  `_full` fills both, because every list fixed in the last two passes is one that
  only misbehaves once it is full.

### Found by review, not by the audit (2026-08-03, second pass)

- ~~Fourteen of twenty companions could not be assigned to anything.~~ Fixed, and
  this was the largest live gameplay defect left in the game. The Expedition Desk
  team panel and the Contract Desk candidate panel both drew
  `game_state.monsters.iter().take(6)` against a population cap of **20**. Six was
  never a layout measurement — it was the roster size when those panels were
  written. Once the guild filled up, fourteen companions could not be sent on an
  expedition or offered to a contract at all, and nothing on either screen said
  they existed. New `ui/screens/roster_window.rs` pages both grids, with the page
  carried in phase state beside `inventory_scroll` (transient, never saved) and
  preserved across a phase rebuild so assigning somebody does not throw the player
  back to page one. Deliberately unsorted: ordering by availability would put the
  useful cards first, but assignments change as the player works, so the cards
  would reshuffle under the cursor between clicks.
- ~~Seventeen of twenty companions had no profile screen, and so could never be
  released.~~ Fixed, and it is the same cap with worse consequences.
  `OpenMonsterProfile` had **exactly one call site** — the Town Overview roster
  strip, capped at `.min(3)` — and `ReleaseMonster` exists **only** on the profile
  screen. So a guild at its population cap could only ever release one of the
  first three companions in roster order, which is the wall the whole late game is
  gated on: hatching at cap requires releasing or replacing. The strip pages now
  too, taking the pager out of the card height rather than a row of its own since
  it is a single row. `town_overview_sections.rs` crossed the 800-line limit as a
  result, so the roster panel was extracted to `town_overview_roster.rs` — the
  natural seam, since that panel is the guild's roster view.
- **The capture harness could only ever photograph a one-companion guild**, so
  the state all three defects live in was unphotographable. `seed_capture_scene`
  now accepts a `_full` scene suffix that fills the roster to the population cap
  first; `ui_town_full.png`, `ui_expedition_full.png` and `ui_contracts_full.png`
  are the baselines that actually show the pagers.
- ~~A third and fourth copy of "what a companion's training is worth", both
  counting five of the ten skills.~~ Half fixed, half deliberately parked.
  `companion_daily_wage` was fixed for this two passes ago; the sum was written
  out longhand in two more places and both were still counting five. **The
  hatchery's `replacement_score`** — which picks the companion the game
  *recommends you sacrifice* at capacity — is fixed: training recovery or
  bargaining made a companion cheaper to throw away and more expensive to keep at
  the same time. There is one `engine::companion_skill_total` now, used by the
  wage and the recommendation, and a test asserts every trainable skill counts.

  **The fourth copy is a balance decision, left for a deliberate call.**
  `monster_service_score` in `policy_eggs.rs` picks who the *simulated* guild
  releases, and counts five of ten skills plus five of seven work-history
  categories. Completing it was built and measured: multi-seed 365-day gold
  **1.07M → 851k** and buildings **24.2 → 17.3**, companions unchanged at 19.2,
  zero missed payments, every assertion still green. Unlike the rank bug last
  pass, this is a policy heuristic standing in for player judgement rather than a
  formula the game itself defines — and taking it would re-base every parked
  balance question here for the second time in two passes. The measurement is
  recorded in the function's own doc comment so it is not re-derived blind.

### Found by review, not by the audit (2026-08-03)

- ~~The star ladder went 1–5 and three places still said 3.~~ Fixed, and this
  one was expensive. Two passes ago the hatchery UI's hardcoded `egg_quality_rank`
  was found and delegated; the sweep did not look for *other* copies of the
  number 3. Two survived. **The refinery** (`convert_egg`) refused every rank-3
  egg with "already at the current quality ceiling" against a ceiling of 5, and
  produced a literal grade of 3-or-5 — so refining, the only way to *make* a
  better egg rather than find one, could not reach ranks 4 and 5, the two that
  earn 7x and 10x. The ceiling is `max_quality_rank` now and the output grade is
  read off `egg_quality_rank_thresholds`, so it climbs exactly one rung whatever
  the ladder says. **And the validation harness** carried a private
  `egg_quality_rank_for_policy` capped at `_ => 3` — the same shape as the third
  copy of the role classifier found in the same place last pass. It fed
  `replacement_plan_for_egg`, so the simulated guild saw every egg above grade 10
  as a three and declined to replace any companion of rank 3 or better with one.
  **That is the yardstick every balance judgement in this file rests on**, and it
  was systematically undervaluing the tower's best output. Delegating it moved
  the multi-seed 365-day numbers hard: gold 684k → **1.07M**, buildings 22.3 →
  24.2, companions 18.7 → 19.2, expedition days 59.2 → 72.2, debt gap −1.31M →
  −0.98M, single-seed floors 16 → 19, hatches 33 → 43. Missed payments stay at 0
  and no assertion was touched. Sell and dissolve payouts moved from a `match` in
  Rust into `config.json`, and load-time validation now rejects any rank-indexed
  curve shorter than the ladder, plus a non-ascending threshold list.
  **Read this before trusting any pre-2026-08-03 balance figure in this file.**
- ~~The preparation-quality formula existed twice and the copies disagreed.~~
  Fixed. `preview_guild_job_for_town` scaled a companion's contribution by her
  condition; `town_preparation_quality` — the figure `contract_depth_score`
  actually scores bookings against — recomputed the same five skill terms and did
  not. So the guild-hall card told the player that resting someone before a
  demanding booking would help, and the desk scored her as fresh either way. One
  `companion_preparation_quality` now, condition included, called by both. Zero
  balance movement: the simulated guild keeps its hall workers rested, so the
  divergence only ever misled a human.
- ~~"Kiss Count" and "Birth Count" were printed on the contract desk.~~ Fixed.
  The refusal reason for a booking short of scouting work read "Kiss Count", and
  for hatchery work "Birth Count" — the vocabulary of the premise this game was
  reskinned from, surviving as string *arguments* rather than content ids, which
  is why the rename pass and every id validation walked past them. The seven
  categories are one table now, and the badge codes on the guild-hall screen
  (`K`, `O`, `V`, `A`, `C`, `M`, `B` — same initials, one letter wide, which is
  how they went unread) are two-letter codes naming the work. A test rejects any
  label containing the retired words.
- ~~Charm training odds were the last room table hardcoded in Rust.~~ Fixed. A
  `match` on room id gave `reception_hall` 65/80% and everything else its odds
  from whether it happened to name a required building — so a newly authored room
  got its charm training from its build requirements. Now
  `charm_training_chance_pct` / `charm_training_booking_chance_pct` in
  `guild_rooms.json`, authored to the exact values the match produced, so the RNG
  stream and every seeded report are unmoved. The guild-hall room badge shows
  them (`Ch @25/45%`), which it could not do while they were not data. A test
  asserts a room advertising charm teaches it and vice versa.
- ~~Room instability keyed on a hardcoded id, and the niche vocabulary was one
  closed set doing two jobs.~~ Fixed. `guild_job_instability_gain` named
  `packroom_annex` outright and gave every other tier-3 room 1 by side effect, so
  a room's exposure to the tower followed its price rather than its purpose;
  it is `shift_instability_gain` in the room data now, authored to the same
  values. And `validate_role_or_niche` checked both room niches and companion
  roles against the union of both vocabularies — so a room authored `delver`
  passes and silently takes the generic gold bias, and worse, a mission authored
  `performance` passes and matches **nobody**, charging the whole party the
  off-role penalty instead of rewarding anyone. Split into two sets, each tested
  against the code that consumes it: `COMPANION_ROLES` is checked by sweeping
  `monster_role`'s branches, so a role the validator accepts must be one a
  companion can actually hold. All shipped content was already correct; this is
  the trap closed, not a live bug. Verified by planting both.

### Found by review, not by the audit (2026-08-02)

- ~~A third copy of the role classifier, in the validation harness.~~ Fixed.
  Two passes ago `monster_depth_role_label` was folded into `engine::monster_role`
  and a sweep declared the shape gone; that sweep compared `src/ui` against
  `src/engine` and this copy lives *inside* `src/engine/validation`, so it never
  appeared. It had already drifted in the way that matters: the engine scores on
  `effective_stats`, which includes trait `stat_modifiers`, and
  `monster_validation_role` read the raw `monster.stats` those modifiers adjust.
  So the harness computed `final_role_diversity` — the metric every balance
  judgement here rests on — with arithmetic the game does not use. It delegates
  now.
- ~~A mutation could require traits no companion reaching it can hold.~~ Guarded.
  `try_apply_mutation` adds the target species' `starting_traits` *and* the
  mutation's `granted_trait_ids`, so a companion's traits depend on her whole
  lineage rather than her current species — which makes the requirement easy to
  author against the wrong set. `validate_mutation_traits_are_reachable` walks
  the lineage graph from every hatchable species and rejects a requirement no
  reachable trait set satisfies. It passes today, but only just:
  `golemkin_warden -> gargoyle_stairwarden` is reachable solely through the
  `minotaur_porter` route, and deleting or retargeting that one entry would
  silently strand it. Verified by planting exactly that edit and watching the
  guard name the mutation and the two traits.


- ~~Saved display settings were loaded without any validation.~~ Fixed. The
  default path in `load_or_default_settings` looks `default_resolution_id` up in
  `available_resolutions` and falls back to the first entry if it is missing —
  careful work — and `config.rs` validates that id at load time too. The *saved*
  path returned the deserialized struct verbatim. `AppSettings` is
  `#[serde(default)]`, so a settings file written before a field existed comes
  back with `resolution_width: 0, resolution_height: 0`, and
  `apply_display_settings` hands those straight to `request_new_screen_size`. A
  resolution dropped from the list in a later patch is quieter: the window still
  opens, but no button on the settings screen is highlighted, so the player
  cannot see which mode is live. `reconcile_resolution_against` now looks the
  saved id up in the list and takes the dimensions from there, or falls back to
  the configured default when the id is gone. The same constraint, enforced on
  one path and not its sibling — the shape this project keeps producing.
- ~~`companion_daily_wage` counted five of the ten skills.~~ Fixed. The wage
  formula summed scouting/guarding/hospitality/crafting/charm and ignored
  recovery, bargaining, navigation, arcana and strength — written against the
  five skills that existed when it was authored and never revisited when the
  others became trainable. Since recovery and bargaining now feed
  `guild_job_skill_bonus` exactly like the original five, training them made a
  companion strictly better at no cost. A test now asserts every trainable skill
  raises the wage, so the formula cannot fall behind the skill list again.
- ~~The upkeep preview was quoted before training and charged after.~~ Fixed.
  Wages scale with skills, companions train during `resolve_day` (line 231), and
  upkeep was charged afterwards (line 462) — so the guild paid for lessons
  learned the same morning while Town Overview had already quoted the pre-
  training figure. The forecast is now taken at the top of `resolve_day` and
  passed down, so the number the player plans against is the number charged.
  Same class as the `injury_risk_score` divergence, found by looking for more of
  it.

- ~~The hatchery displayed the wrong star rating on every good egg.~~ Fixed.
  There were two `egg_quality_rank` functions: the engine's reads
  `egg_quality_rank_thresholds` from config (`[3, 5, 10, 17]` → ranks 1–5), and
  the hatchery screen carried a hardcoded copy that capped at three. So every
  egg at grade 10 or better was shown worse than it was, and a grade-17 egg —
  which hatches a rank-5 companion earning **ten times** a rank-1 against
  `quality_income_multipliers_pct` — was displayed as a three. The at-cap
  replacement suggestion was computed against the same wrong number, so it could
  decline to recommend swapping a rank-4 companion for a rank-5 egg. The UI
  delegates to the engine now, and a test asserts every authored rank is
  reachable and the top threshold hatches the top rank.

- ~~Two independent copies of the companion role classifier.~~ Fixed before it
  bit. `engine::monster_role` and `view_models::monster_depth_role_label` carried
  byte-identical branching — corruption, hatchery history, charm-vs-power,
  bond — differing only in the label strings they returned. They agreed only
  because both happened to get updated when `effective_stats` was wired in; miss
  one and the profile screen calls a companion a performer while `role_affinity`
  scores her a delver and quietly withholds the mission role bonus. The label is
  now a pure mapping over the engine's role, and a test asserts no engine role
  falls through to the generalist label.

  A systematic sweep for this shape (function names defined in both `src/ui` and
  `src/engine`) turns up nothing else: `room_name_by_id` and `species_name_by_id`
  are trivial id lookups, and `egg_quality_rank` is already a delegate.

- ~~The partial-success fallback carried a stricter gate than full success.~~
  Fixed. `contract_partial_success` required `town_preparation_quality >=
  preparation_quality_required`, while the full-success path never checked
  preparation quality at all. So a guild below the preparation bar was paid in
  full if the companion was otherwise eligible, and paid *nothing* if she also
  missed any other check — the fallback was harder to reach than the thing it
  falls back from. Preparation quality no longer gates the partial path, so an
  under-prepared guild can still scrape a half payment.

- **`preparation_quality_required` is authored on 13 of 16 contracts, displayed
  on the contract desk, and enforced nowhere.** Requirements run 2–6. Guild rooms
  contribute `preparation_quality_bonus` of 1–3 each and only `town_job_limit`
  (2) companions can be on guild jobs at once, so the figure is genuinely
  contested — companions on contracts and expeditions do not contribute to it.

  Enforcing it was built and reverted, with numbers. Adding it to
  `evaluate_contract_eligibility` is wrong on its own: the policy books contracts
  *before* staffing guild jobs (`policy.rs:22` then `:24`), and a player does the
  same, so preparation quality reads 0 at booking time and every demanding
  contract is refused outright. Treating a shortfall as partial pay instead is
  coherent — full pay for meeting the bar, half for delivering under-prepared —
  but it costs the deterministic campaign a whole debt milestone
  (`broker_compact_6` with 34 days left, against reaching `founders_due_7`),
  because most of the guild's income is contract work.

  **That is a real balance decision, not a bug fix**: either the requirement is
  meant to bite and the economy needs headroom for it, or the numbers want
  lowering, or the field should come off the desk. Left for a deliberate call.

- **`hazard_risk_modifier_pct` is added flat, not as a percentage** — and the
  same is true of `guild_income_pct`, `expedition_success_pct`, `injury_risk_pct`
  and `success_bonus_pct`, which are all summed into raw scores rather than
  applied as percentages. Swept all 22 `*_pct` fields; the convention is
  consistent across the whole scoring system, so this is a naming problem rather
  than a defect. Worth knowing because it is an authoring trap: on a depth-1
  floor with two hazard tags the raw hazard is 8, so `hazard_risk_modifier_pct:
  12` is +150%, while on a deep floor it is +20%. Renaming would need serde
  aliases on five fields for no behavioural gain, so it is recorded rather than
  done.

### UI and feedback

- ~~Status messages are computed then discarded on five screens; Ctrl+S saves with zero visible confirmation.~~ Done: all five now render `status_message` (Monster Profile had no draw path for it at all; Town Management and Monster Profile yield the slot to an error when there is one). `persist_game_state` already called `apply_phase_status("Campaign saved")`, so Ctrl+S became visible everywhere for free.
- ~~No way back to the Main Menu from a running campaign.~~ Done: the settings modal is the one panel reachable from every screen (Escape), so `main_menu_button` is drawn there and `ReturnToMainMenu` saves before switching phase — there is no confirmation prompt in front of it, so it must not be able to lose a day.
- ~~Roughly 100 `ui_text.json` fields are orphaned.~~ Measured at **110 dead out of 366** — nearly a third of the catalogue — and deleted. Re-wiring was the other option and it is the wrong one for these: they are leftovers from screens that were rewritten, and `LoadingUiText` was unusable by construction (the loading screen runs before the catalogue it would read has loaded). A dead key is worse than no key, because an author edits the wording, sees no change, and cannot tell whether the screen is hardcoded or they typed the wrong name. `tests/ui_text_catalog.rs` now fails on any field no screen reads, so the catalogue cannot drift back — verified by planting a dead key and watching it fire.
- ~~`config.json`'s `ui` block is loaded and validated but read by nothing.~~ Deleted. `town_panels` named four panels (`Campaign`, `Resources`, `Catalog Snapshot`, `Roster Preview`) that the rewritten Town Overview does not use even as strings, and `target_width`/`target_height` described a fixed design resolution the responsive layout never consults — stale metadata for a screen that no longer exists in that shape. `content_version` was the other unread config field; rather than delete it, reports now carry it, so every playtest JSON in `tmp_screens/playtests/` names the content catalogue that produced it.
- ~~`draw_condition_badges` has zero call sites.~~ Done: it draws on the Town Overview roster card, which is where the Rest button lives — the decision to rest someone was being made with no sight of her fatigue, and since the condition wiring landed those numbers cost real output. Gated on card height so the compact layout is unaffected.
- ~~`ui_icon_atlas.json` is not read by code — icon rects are re-derived in `art_helpers.rs`, leaving two sources of truth.~~ Resolved as checked documentation, the same way the text catalogue was. The JSON declares an explicit rect per icon; `icon_source` ignores it and counts out an eight-column grid of `ICON_CELL` squares from the icon's position in the enum. Both are right today by coincidence of authoring order — repack the sheet, update the JSON, and nothing on screen moves. `tests/icon_atlas_layout.rs` now fails if the file's own rects stop matching the grid the code counts, verified by planting a moved rect. A second test asserts `icon_index` gives every icon a distinct, contiguous slot, since a duplicated arm in that 43-way match would draw one icon in another's place silently.

  The 17 unused `UiIcon` entries are left as-is: they are art that exists ahead of the screens that will use it, which is the right order to build in.

### Write-only state (wire it or delete it)

- ~~Egg preparation metadata is set by Refine and never read.~~ Resolved by splitting it: `secrecy_locked` was written `true` at all three creation sites and never `false` and never read, and `loyalty_imprinted` was exactly `incubation_state == ReadyToHatch` (the only writer of that state sets both) — both deleted. `preparation_focus` was the one field carrying information no other field could give: a refined egg sat in the inventory looking exactly like a wild find, with nothing saying two eggs had been spent to make it. It now has named constants instead of scattered magic strings and renders on the inventory card as "Tower find" / "Lineage set" / "Refined lineage".
- ~~`DayResolutionSummary` has ten write-only fields that `day_results.rs` never shows.~~ Done: the Town Jobs panel now breaks operating costs into wages/cleaning/maintenance and reports special-event count and gold; the Expedition panel reports prep spend and prep shortfall; the Contracts panel reports offers received and turned away. New keys in `ui_text.json` rather than more hardcoded English.
- ~~`DebtState.status_message` / `last_resolution`, `ContractState.partial_progress`, `ExpeditionState.started_day`, story flags `tower_hole_discovered` / `first_egg_created` are written and never read.~~ Split the same way. The two debt fields were worth showing — the engine writes a narrative status line on every resolution and records whether the last payment landed on time, late or not at all, and the Town Overview showed neither, so the number the whole campaign is built around arrived with no account of how it got there. Both now render under the resource strip, in `DANGER` when the last payment was missed. The other four were deleted: `partial_progress` stored a depth score nothing read, `started_day` can never differ from the resolving day because an expedition is resolved the day it is assigned, and both story flags are strictly derivable from `opening_step` (set exactly when it advances past `Camp` and `Discovery`).
- ~~`SpeciesData.preferred_room_ids` is the dead half of a relation and nothing validates the two agree.~~ Done: load-time validation now rejects a species that claims a room the room does not claim back. It found six mismatches — `slime_companion`/`packroom_annex`, `minotaur_porter`/`reception_hall`, `wyrm_registrar`/`nursery_wing`, `gargoyle_stairwarden`/`common_room`, `revenant_chorister`/`packroom_annex`, `salamander_corekeeper`/`packroom_annex` — each reading as a working affinity bonus that the engine never granted. Reconciled by trimming the species side, because only the room side is live and adding to the room side would have changed the simulation. **If the authoring intent was the other direction, adding those six to the rooms' `preferred_species_ids` is a balance change worth making deliberately** — each is worth `preferred_species_bonus_pct` (10%) on that room's guild-job success.
- `SpeciesData.portrait_key` and `species_portrait_key_by_id()` have no callers — portraits are drawn procedurally.

## Balance

### Open design question: a competent guild finishes the campaign early

Re-measured after the harness fix, because most of what was written here as a
blocker turned out to be the harness rather than the game.

**Two of the three symptoms are gone.** `relics < 260` was unreachable because
total sink capacity was 188 relics across a whole campaign; `reliquary_vault`
(project, build limit 40, 100 relics and 9,000 gold each) fixed that and holds
multi-seed stockpiles in the 15–86 band. And `final_unlocked_floors == 25` was
never measuring a defect at all — it pinned the simulated guild's route, and
replacing it with `stranded_floor_ids` let seven simulation-moving changes land
that had been written off as balance-gated. **The harness validates
simulation-moving changes fine now.**

**What is actually left is one question.** Adding the missing survey-progress
term to `expedition_growth_score` — gate on "does another locked floor still
name this one in `requires_surveyed_floor_ids`", worth `mission.survey_value *
90` — makes the simulated guild finish the survey chain. Measured against the
current build it opens all 25 floors and buys 51 buildings, and **all ten seeds
clear the Founder's Due**, failing both `cleared < 10` and the single-seed
"should leave Founder's Due active for future floors".

Tuning does not reach it, and the reason is mechanical rather than a matter of
finding the right number. Sweeping the due:

| Due | Cleared | Average debt gap |
| --- | --- | --- |
| 2.5M | 10/10 | 0 |
| 3.5M | 9/10 | +9k |
| 4.5M | 0/10 | −325k |
| 6.0M | 0/10 | −1,825k |

`cleared` falls off a cliff, and in the uncleared regime every seed reports the
*identical* gap (−324,682 on all ten at 4.5M). `can_spend_on_late_game_sink`
reserves the balance and the vault absorbs everything above it, so end-of-run
gold is pinned to `balance + reserve` for any guild that can reach it. Income
variance becomes buildings bought, never coin, so `debt_gap` cannot vary between
seeds however much the economy does. Wiring `spawn_weight` as a true random draw
was tried as a variance source and does widen the building spread (19-on-every-
seed → 38–47) but costs enough income that the campaign stops reaching its final
milestone; relaxing the reserve changed nothing (9/10 still cleared).

**So the decision is: what should stop a well-played guild finishing early?**
The options measurement can no longer choose between are a late-game cost that
scales with how well the run went rather than a fixed due, an assertion that
measures something the economy can actually vary (buildings, floors, projects),
or accepting that a good run wins and saying so in the spec. Until one is
chosen the survey term stays out — which now costs only the tower's last
fourteen floors going unwalked by the *simulation*, not the ability to test.

- Decide an acceptable target range for day-365 `surplus_summary.debt_gold_gap`.
- Decide an acceptable target range for final relic and residue stockpiles after project purchases.
- Review whether 30-day outcomes run too high when early egg rolls are favourable.
- Review whether 90-day outcomes run too low when early debt or event rolls are unfavourable.
- Review whether the 180-day building count reliably opens enough population cap before late catch-up hatching.
- Tune final debt pressure against averaged multi-seed results rather than one deterministic report. See the blocker above for the measured gap: 1.6M–2.5M gold of slack on every seed.
- ~~Add late-game project varieties that spend different surplus mixes.~~ Done: `reliquary_vault` is the relic-heavy sink the catalogue lacked. Materials were flagged here as having no dedicated sink; measured, they end at 616 against 34,900 of incidental capacity, so there is no runaway to solve.
- Consider patron satisfaction as explicit state if completions and expirations are not enough pressure.
- ~~Give the 20-companion cap a clearer replacement *reach*.~~ Half done: every
  companion now has a reachable profile, and therefore a release button, which
  she did not before (see the roster paging entries above). The remaining half is
  a genuine *flow* — a screen that compares a pending egg against the roster and
  makes the swap in one step, rather than profile-by-profile — plus a reason to
  run non-egg missions once capped.
- Review whether special-event cost should scale from roster, reputation, or project count.

## UI and reporting

- ~~Add a projects status line to Town Management explaining repeatable project count, cost, and purpose.~~ Done. A build limit of forty with no unlocks reads as pointless until something says the thing exists to convert surplus, and nothing did — `reliquary_vault` shipped invisible. The line reports how many repeatable builds stand against how many could, and what they have absorbed.
- ~~Add a concise project/sink summary to simulation reports.~~ Done: `sink_absorbed` and `sink_capacity` on every report. This is the pair that would have made the relic ceiling obvious immediately — capacity now reads 4,188 relics against the 188 it was, with 142 absorbed and 6 left standing on the deterministic seed. Gold is the one worth watching: capacity 516,500 against 446,624 still banked, so the sinks are roughly adequate and the guild simply is not spending, which is the reserve behaviour described in the Balance section rather than a missing sink.
- ~~Capture visual baselines for new screens and modal states before relying on screenshot comparison.~~ Done, and it was overdue. The capture harness existed but only ever photographed the main menu — `MONSTERHALL_CAPTURE_SCENE` was read and discarded, so roughly ten screens changed over this work had never been looked at. `Game::seed_capture_scene` now drives a fresh campaign through the opening and navigates to a named screen using the same actions a player would, so a scene the harness cannot reach is one the player cannot reach either. Baselines for town, buildings, hatchery, contracts, profile and expedition are in `docs/verification/`.

  Looking found three layout defects that no test could have caught:
  - The projects status line (added two passes ago at `detail_top_y + 194`) drew straight through the town status message added several passes before that at `y = 338`. Two of my own changes, colliding.
  - The condition badges on the roster card took the full card width, so four enormous boxes each held two characters. Capped at 300px.
  - Pre-existing: `built_count_label` rendered as "Bui" — 96px was not enough for the label beside its icon, in both metric rows. Both rows re-flowed.

  A second pass caught more, including a flaw in the harness itself: `seed_capture_scene` only sent `ContinueOpening`, but `BuildRoom` and `FirstClient` each need their own action, so the first round of captures were all taken with the campaign stuck on "Make The Hall Useful". The opening is played out properly now.
  - **Day Results overflowed both summary panels.** The cost breakdown, prep spend and offer counts took Town Jobs and Expedition from four lines to six while the frames stayed 188 tall, so the last line of each was sliced by the panel edge and spilled into the row beneath. The height is a named constant now and the row below derives from it.
  - The Guild Jobs room thumbnail was drawn over its own panel title, and the detail panel ended before its last badge, which the roster row then covered. Both panels grown, thumbnail moved below the title band.
  - **The Contract Desk detail column had three draw calls landing on each other.** The guest name at y=138 is 24px and descends past 150, where the status badge started; the badge is 28 tall and ran into the category line at 180. So the patron tier, preparation quality and room name were all printed through "Pending" and through each other — on the one screen whose whole job is showing a contract's requirements. Respaced as an explicit top-to-bottom column with the intended offsets written down, because the original numbers looked deliberate and were not.

    Two smaller ones on that screen were left, and one of them turned out to
    obscure information after all. ~~The thumbnail caption reaching into the text
    column~~ was fixed on 2026-08-03: `draw_text_center` never measured against
    its box, so the guest name spilled both ways and printed through the room
    name, reward, penalty and deadline once the name was long enough. Captions
    ellipsise to fit now, at the shared helper, so every thumbnail in the game is
    covered. The contract list rows still draw slightly wider than the panel that
    holds them; that one really is cosmetic.
  A fourth pass went back to the images already captured but never opened, and that alone was worth it:
  - **The Expedition Desk printed "Injury Risk -1073741824"** — `i32::MIN / 2`, the empty-party fallback I added a few passes earlier, rendered raw. An empty party is the state the screen opens in every single time, so this was the default view. `injury_risk_score` is `Option<i32>` now: with nobody assigned there is no companion to hurt and therefore no number, and the tile shows an em dash. The sentinel could never have been caught by a test that did not already know to look for it, because every consumer treated it as an ordinary score.
  - The same screen's header ended on a bare `|`. The risk clause was long enough that the status strip truncated it away entirely, and with no party it only restated the "0 assigned" beside it. Dropped when there is no party.
  - "Prep cost: 3g" drew through the bottom of the Success tile: the metric row ends exactly at the panel edge, so there was never room for the caption underneath it. The panel is 12px taller.
  - **The Hatchery status panel's title was struck through by its own egg counter.** A panel title tab occupies the first ~36px; the tile was placed at +20. "Hatchery Status" was unreadable on every visit to the screen.

  Journal and Settings were clean.

  - The worker card's prediction line had grown twice — the condition note and the work-history odds both landed there — and was being cut mid-word. The odds describe the *room*, not the companion, so they moved to the room badge, which was still quoting the bare ceiling: the same "looks guaranteed, is a coin flip" the worker card was fixed for two passes earlier. `GuildJobPreview` lost the two fields that were carrying a per-companion copy of room data.

## Art

- Replace the remaining procedural placeholder art with content-specific key art; only backdrops and one icon atlas exist today.
- Author character portraits, room/building/floor thumbnails, story CGs, patron art, and egg art per `docs/UI_STYLE_GUIDE.md` and `docs/UI_THEME_SHEET.md`.
