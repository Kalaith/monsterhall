# TODO — Monsterhall

## Unconnected systems (audit 2026-08-02)

Systems that exist in data/state/UI but never affect the simulation, or vice versa.

### Gameplay-affecting

- ~~Wire fatigue/stress/injury into the simulation.~~ Done: `engine/day_cycle/condition.rs` turns the three meters into a single effectiveness percentage (`config.json` → `day_cycle.condition_effects`), applied to guild-job gold/materials/residue/reputation/prep quality and to each companion's stat contribution to an expedition plan. It is deliberately tuned so a roster rested on the simulation policy's schedule pays nothing — the 365-day report is byte-identical to before — and only a neglected roster loses output. Remaining work: contract/guest rewards (`engine/guest.rs`) still pay in full regardless of condition, which is the last place the meters are ignored.
- Cap the condition meters and/or let idle companions recover. Both were tried and backed out: fatigue/stress are one-way for anyone never given the explicit Resting job, so a contract worker climbs past 400 and (uncapped) can never rest back under the policy's re-entry gate. Either change reshuffles who is available on which day and swings the single-seed 365-day report hard (25 unlocked floors → 11). Needs doing against the multi-seed harness, not the deterministic report.
- ~~Fix expedition priority/mission selection reaching game state.~~ Done: `open_expedition_planning` (`game/navigation.rs`) now pushes floor/mission/stance into an already-formed `active_expedition`, so what the preview shows is what day resolution runs.
- Apply `TraitData.stat_modifiers`. Thirteen traits in `traits.json` author stat bonuses; the field's only reference in code is its declaration (`data/types.rs:637`). **Built and reverted — blocked on the balance decision below, not on the code.** The implementation is an `engine/companion.rs` with `effective_stats(data, monster)` = base + summed trait bonuses, called on demand rather than baked in at hatch (mutations grant traits after creation, `progression.rs:357`). Call sites: the two stat totals and the endurance term in `previews.rs`, `monster_role`/`role_affinity` in `depth.rs` (needs `data` threaded through three call sites), `monster_depth_role_label`, `replacement_score`, and the profile screen, which should show the trait share as `12 (+2)`. Every trait carries +1 to +5, so it is a straight buff and it moves the deterministic 365-day report.
- Consume `charm_training_flat`. Six buildings advertise it in their tooltip and the building aggregate sums it (`day_cycle/modifiers.rs:21`), but no code reads the sum.
- Finish the second half of the skill system. `increment_skill` only trains scouting/guarding/hospitality/crafting/charm; recovery, bargaining, navigation, arcana, strength are never incremented anywhere, and contract skill requirements on those five are never checked (`guest.rs` `append_skill_requirement_reasons`). Navigation/arcana are read in previews and depth scoring but are permanently zero.
- Align the planning preview's `injury_risk_score` with the actual injury roll — `previews.rs` and `resolution.rs` use different formulas that can disagree in sign, so the number the player plans against is not the number the sim uses. Half-done: resolution's side is now the shared `expedition_safety_score` in `previews.rs` (one formula, one priority-risk table, evaluated before the day's fatigue toll so it matches what a preview would see). The preview still reports the old party-risk figure, because swapping it for `threshold - safety` changes the *scale* of a number `expedition_growth_score` (`validation_tests/policy_jobs.rs`) steers the whole long-campaign simulation on — a sweep of re-tuned weights moved the 365-day tower between 5 and 25 unlocked floors with no stable optimum. Do the swap together with recalibrating that policy against the multi-seed harness, and re-band `risk_label`/`risk_color` in `expedition_planning.rs` (zero becomes "certain injury", not "high").
- ~~Guild-job preview shows `work_history_gains` verbatim while resolution rolls per-room probabilities.~~ Done: the odds were a `match` on room id inside `progression.rs`, so the preview could only quote the ceiling. They now live in `guild_rooms.json` as `work_history_gain_chance_pct`, `roll_work_history_gains` reads them, and the guild-hall card renders `C+1 @12%` instead of a bare `C+1`. The refactor is RNG-identical — every room's categories already rolled in the field order of `CompanionWorkHistoryProgressionData`, and a zero chance skips the roll exactly as a zero ceiling always did, so the simulation reports are byte-for-byte unchanged.
- ~~`hatchery_assists` had no source in the entire game.~~ Fixed: all four guild rooms authored a ceiling of 0, so no shift could ever bank one — which made `corekeeper_sending_vigil` (requires 3) permanently unfulfillable and left the `hatchery_specialist` role in `monster_role` reachable only through the `hatchery_attuned` trait. `nursery_wing` already carried the intended 5% odds, so it now has the ceiling to match. A new test asserts no room authors odds for work it cannot bank. Note the contract needs roughly sixty nursery shifts at 5%; whether that is the intended pace is a balance question.
- ~~`ContractStatus::Completed`/`Failed`/`Declined` are never assigned.~~ Done: resolution now stamps the outcome and moves the contract to a new `GameState.resolved_contracts`, which the desk lists under the live offers so the player can see how yesterday's bookings went. The lingering-status approach was tried first and abandoned — a resolved booking left inside `active_contracts` moves the offer limit, the request-id sequence, `workforce_demand`, the follow-up check and the booking policy, and it shifted the 365-day report by six buildings. A separate list is neutral by construction. `assign_monster_to_contract` and `clear_contract_assignment` now refuse non-live contracts, which is a real guard rather than bookkeeping.
- `event_tags` in `events.json` (tier_1–tier_4, late_game, crisis, …) encode intended gating that no code applies; event selection filters only on category/phase/required ids/min_day/chance/weight.
- Patron archetype `spawn_weight` and `tags` never influence contract generation — offers are taken in pressure-priority order only.

### UI and feedback

- ~~Status messages are computed then discarded on five screens; Ctrl+S saves with zero visible confirmation.~~ Done: all five now render `status_message` (Monster Profile had no draw path for it at all; Town Management and Monster Profile yield the slot to an error when there is one). `persist_game_state` already called `apply_phase_status("Campaign saved")`, so Ctrl+S became visible everywhere for free.
- ~~No way back to the Main Menu from a running campaign.~~ Done: the settings modal is the one panel reachable from every screen (Escape), so `main_menu_button` is drawn there and `ReturnToMainMenu` saves before switching phase — there is no confirmation prompt in front of it, so it must not be able to lose a day.
- Roughly 100 `ui_text.json` fields are orphaned — screens were rewritten with hardcoded English (whole Town Overview debt/contract-pressure/roster blocks, Guild Hall and Expedition Planning label sets). Decide whether to re-wire the text catalog or delete the dead keys.
- `config.json`'s `ui` block (`target_width`/`target_height`/`town_panels`) is loaded and validated but read by nothing — the town panel set is hardcoded.
- ~~`draw_condition_badges` has zero call sites.~~ Done: it draws on the Town Overview roster card, which is where the Rest button lives — the decision to rest someone was being made with no sight of her fatigue, and since the condition wiring landed those numbers cost real output. Gated on card height so the compact layout is unaffected.
- 17 `UiIcon` entries (assignment, mission-type, and status icons) have atlas rects but no draw path. `ui_icon_atlas.json` and `backdrops.json` are not read by code — icon rects are re-derived in `art_helpers.rs`, leaving two sources of truth.

### Write-only state (wire it or delete it)

- Egg preparation metadata (`preparation_focus`, `loyalty_imprinted`, `secrecy_locked`) is set by Refine and never read; a refined egg differs from raw only by `grade_score`.
- ~~`DayResolutionSummary` has ten write-only fields that `day_results.rs` never shows.~~ Done: the Town Jobs panel now breaks operating costs into wages/cleaning/maintenance and reports special-event count and gold; the Expedition panel reports prep spend and prep shortfall; the Contracts panel reports offers received and turned away. New keys in `ui_text.json` rather than more hardcoded English.
- `DebtState.status_message` / `last_resolution`, `ContractState.partial_progress`, `ExpeditionState.started_day`, and story flags `tower_hole_discovered` / `first_egg_created` are written and never read.
- ~~`SpeciesData.preferred_room_ids` is the dead half of a relation and nothing validates the two agree.~~ Done: load-time validation now rejects a species that claims a room the room does not claim back. It found six mismatches — `slime_companion`/`packroom_annex`, `minotaur_porter`/`reception_hall`, `wyrm_registrar`/`nursery_wing`, `gargoyle_stairwarden`/`common_room`, `revenant_chorister`/`packroom_annex`, `salamander_corekeeper`/`packroom_annex` — each reading as a working affinity bonus that the engine never granted. Reconciled by trimming the species side, because only the room side is live and adding to the room side would have changed the simulation. **If the authoring intent was the other direction, adding those six to the rooms' `preferred_species_ids` is a balance change worth making deliberately** — each is worth `preferred_species_bonus_pct` (10%) on that room's guild-job success.
- `SpeciesData.portrait_key` and `species_portrait_key_by_id()` have no callers — portraits are drawn procedurally.

## Balance

### Blocker: the late game has no teeth once the tower actually opens (found 2026-08-02)

This is now the gate on several other items, so it comes first.

`expedition_growth_score` (`validation_tests/policy_jobs.rs`) scores a run by its
immediate haul — eggs, relics, materials, residue, success — and has **no term for
survey progress**. The survey chain is serial, so the simulated guild wanders off
it the moment anything changes which floor looks richest, and every floor beneath
the stalled link goes unrun for the rest of the campaign. That is why
`final_unlocked_floors == 25` is knife-edge: three separate, correct gameplay
changes (condition wiring, the honest injury preview, trait stat bonuses) each
collapsed it to 10–15 floors without touching the tower's own rules.

Adding the missing term fixes it — gate on "does another locked floor still name
this one in `requires_surveyed_floor_ids`", worth `mission.survey_value * 90`
against an egg at 120–180. The tower then opens fully and the guild buys 3 town
projects, where it previously bought none. **But it also shows the late game is
hollow:**

- All 10 multi-seed samples clear the Founder's Due, with 1.6M–2.5M gold spare.
  `multi_seed_365_simulation_summary_reports_variance` asserts clearing must *not*
  be guaranteed, and it fails.
- All 10 samples land on exactly 19 buildings. The baseline spread was 8–20, so
  the variance that assertion is named for came from the bot failing, not from
  the seeds.
- Total relic sink capacity is **188 relics** — every `project`/`prestige`
  building in `buildings.json` bought to its full build limit. Deep-tower income
  runs to thousands, so the existing `relics < 260` assertion measures how deep
  the guild got, not whether surplus was converted. Replace it with
  `final_town_projects > 0` (residue keeps a stockpile bound; its sink capacity
  is 252,900 and is genuinely adequate).

Verified independent of any gameplay change: with all `stat_modifiers` zeroed in
`traits.json`, the survey term alone still fails both debt assertions. So this is
a pre-existing hole that the bot's incompetence was hiding, not a regression.

Doing this properly is one pass: land the survey term, rebalance late-game debt
pressure and relic sinks against the multi-seed harness, then land trait
`stat_modifiers` on top. Piecemeal does not work — trait stats alone strand the
survey chain, and the survey term alone unmasks the debt hole.

- Decide an acceptable target range for day-365 `surplus_summary.debt_gold_gap`.
- Decide an acceptable target range for final relic and residue stockpiles after project purchases.
- Review whether 30-day outcomes run too high when early egg rolls are favourable.
- Review whether 90-day outcomes run too low when early debt or event rolls are unfavourable.
- Review whether the 180-day building count reliably opens enough population cap before late catch-up hatching.
- Tune final debt pressure against averaged multi-seed results rather than one deterministic report. See the blocker above for the measured gap: 1.6M–2.5M gold of slack on every seed.
- Add late-game project varieties that spend different surplus mixes. Concretely: relic sinks cap at 188 across the whole game, against multi-thousand deep-tower income.
- Consider patron satisfaction as explicit state if completions and expirations are not enough pressure.
- Give the 20-companion cap a clearer replacement/release flow, and a reason to run non-egg missions once capped.
- Review whether special-event cost should scale from roster, reputation, or project count.

## UI and reporting

- Add a projects status line to Town Management explaining repeatable project count, cost, and purpose.
- Add a concise project/sink summary to simulation reports.
- Capture visual baselines for new screens and modal states before relying on screenshot comparison.

## Art

- Replace the remaining procedural placeholder art with content-specific key art; only backdrops and one icon atlas exist today.
- Author character portraits, room/building/floor thumbnails, story CGs, patron art, and egg art per `docs/UI_STYLE_GUIDE.md` and `docs/UI_THEME_SHEET.md`.
