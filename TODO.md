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

### UI and feedback

- ~~Status messages are computed then discarded on five screens; Ctrl+S saves with zero visible confirmation.~~ Done: all five now render `status_message` (Monster Profile had no draw path for it at all; Town Management and Monster Profile yield the slot to an error when there is one). `persist_game_state` already called `apply_phase_status("Campaign saved")`, so Ctrl+S became visible everywhere for free.
- ~~No way back to the Main Menu from a running campaign.~~ Done: the settings modal is the one panel reachable from every screen (Escape), so `main_menu_button` is drawn there and `ReturnToMainMenu` saves before switching phase — there is no confirmation prompt in front of it, so it must not be able to lose a day.
- Roughly 100 `ui_text.json` fields are orphaned — screens were rewritten with hardcoded English (whole Town Overview debt/contract-pressure/roster blocks, Guild Hall and Expedition Planning label sets). Decide whether to re-wire the text catalog or delete the dead keys.
- `config.json`'s `ui` block (`target_width`/`target_height`/`town_panels`) is loaded and validated but read by nothing — the town panel set is hardcoded.
- ~~`draw_condition_badges` has zero call sites.~~ Done: it draws on the Town Overview roster card, which is where the Rest button lives — the decision to rest someone was being made with no sight of her fatigue, and since the condition wiring landed those numbers cost real output. Gated on card height so the compact layout is unaffected.
- 17 `UiIcon` entries (assignment, mission-type, and status icons) have atlas rects but no draw path. `ui_icon_atlas.json` and `backdrops.json` are not read by code — icon rects are re-derived in `art_helpers.rs`, leaving two sources of truth.

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
- Give the 20-companion cap a clearer replacement/release flow, and a reason to run non-egg missions once capped.
- Review whether special-event cost should scale from roster, reputation, or project count.

## UI and reporting

- ~~Add a projects status line to Town Management explaining repeatable project count, cost, and purpose.~~ Done. A build limit of forty with no unlocks reads as pointless until something says the thing exists to convert surplus, and nothing did — `reliquary_vault` shipped invisible. The line reports how many repeatable builds stand against how many could, and what they have absorbed.
- ~~Add a concise project/sink summary to simulation reports.~~ Done: `sink_absorbed` and `sink_capacity` on every report. This is the pair that would have made the relic ceiling obvious immediately — capacity now reads 4,188 relics against the 188 it was, with 142 absorbed and 6 left standing on the deterministic seed. Gold is the one worth watching: capacity 516,500 against 446,624 still banked, so the sinks are roughly adequate and the guild simply is not spending, which is the reserve behaviour described in the Balance section rather than a missing sink.
- Capture visual baselines for new screens and modal states before relying on screenshot comparison.

## Art

- Replace the remaining procedural placeholder art with content-specific key art; only backdrops and one icon atlas exist today.
- Author character portraits, room/building/floor thumbnails, story CGs, patron art, and egg art per `docs/UI_STYLE_GUIDE.md` and `docs/UI_THEME_SHEET.md`.
