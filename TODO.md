# TODO — Monsterhall

## Unconnected systems (audit 2026-08-02)

Systems that exist in data/state/UI but never affect the simulation, or vice versa.

### Gameplay-affecting

- ~~Wire fatigue/stress/injury into the simulation.~~ Done: `engine/day_cycle/condition.rs` turns the three meters into a single effectiveness percentage (`config.json` → `day_cycle.condition_effects`), applied to guild-job gold/materials/residue/reputation/prep quality and to each companion's stat contribution to an expedition plan. It is deliberately tuned so a roster rested on the simulation policy's schedule pays nothing — the 365-day report is byte-identical to before — and only a neglected roster loses output. Remaining work: contract/guest rewards (`engine/guest.rs`) still pay in full regardless of condition, which is the last place the meters are ignored.
- Cap the condition meters and/or let idle companions recover. Both were tried and backed out: fatigue/stress are one-way for anyone never given the explicit Resting job, so a contract worker climbs past 400 and (uncapped) can never rest back under the policy's re-entry gate. Either change reshuffles who is available on which day and swings the single-seed 365-day report hard (25 unlocked floors → 11). Needs doing against the multi-seed harness, not the deterministic report.
- ~~Fix expedition priority/mission selection reaching game state.~~ Done: `open_expedition_planning` (`game/navigation.rs`) now pushes floor/mission/stance into an already-formed `active_expedition`, so what the preview shows is what day resolution runs.
- Apply `TraitData.stat_modifiers`. Thirteen traits in `traits.json` author stat bonuses; the field's only reference in code is its declaration (`data/types.rs:637`).
- Consume `charm_training_flat`. Six buildings advertise it in their tooltip and the building aggregate sums it (`day_cycle/modifiers.rs:21`), but no code reads the sum.
- Finish the second half of the skill system. `increment_skill` only trains scouting/guarding/hospitality/crafting/charm; recovery, bargaining, navigation, arcana, strength are never incremented anywhere, and contract skill requirements on those five are never checked (`guest.rs` `append_skill_requirement_reasons`). Navigation/arcana are read in previews and depth scoring but are permanently zero.
- Align the planning preview's `injury_risk_score` with the actual injury roll — `previews.rs` and `resolution.rs` use different formulas that can disagree in sign, so the number the player plans against is not the number the sim uses. Half-done: resolution's side is now the shared `expedition_safety_score` in `previews.rs` (one formula, one priority-risk table, evaluated before the day's fatigue toll so it matches what a preview would see). The preview still reports the old party-risk figure, because swapping it for `threshold - safety` changes the *scale* of a number `expedition_growth_score` (`validation_tests/policy_jobs.rs`) steers the whole long-campaign simulation on — a sweep of re-tuned weights moved the 365-day tower between 5 and 25 unlocked floors with no stable optimum. Do the swap together with recalibrating that policy against the multi-seed harness, and re-band `risk_label`/`risk_color` in `expedition_planning.rs` (zero becomes "certain injury", not "high").
- Guild-job preview shows `work_history_gains` verbatim while resolution rolls per-room probabilities (12–70%) — gains look guaranteed but are coin flips.
- `ContractStatus::Completed`/`Failed`/`Declined` are never assigned; resolved contracts are just dropped from `active_contracts`, and the contract desk has labels/colors for states that can never appear.
- `event_tags` in `events.json` (tier_1–tier_4, late_game, crisis, …) encode intended gating that no code applies; event selection filters only on category/phase/required ids/min_day/chance/weight.
- Patron archetype `spawn_weight` and `tags` never influence contract generation — offers are taken in pressure-priority order only.

### UI and feedback

- Status messages are computed then discarded on Town Overview, Town Management, Hatchery, Expedition Planning, and Monster Profile (`let _status_message = …`); only Contract Desk, Guild Hall, and Loading render them. Ctrl+S saves with zero visible confirmation (its message only shows inside the closed settings modal).
- No way back to the Main Menu from a running campaign; `main_menu_button` text exists in data but no control was built.
- Roughly 100 `ui_text.json` fields are orphaned — screens were rewritten with hardcoded English (whole Town Overview debt/contract-pressure/roster blocks, Guild Hall and Expedition Planning label sets). Decide whether to re-wire the text catalog or delete the dead keys.
- `config.json`'s `ui` block (`target_width`/`target_height`/`town_panels`) is loaded and validated but read by nothing — the town panel set is hardcoded.
- 17 `UiIcon` entries (assignment, mission-type, and status icons) have atlas rects but no draw path; `draw_condition_badges` (fatigue/stress/injury/corruption strip) has zero call sites. `ui_icon_atlas.json` and `backdrops.json` are not read by code — icon rects are re-derived in `art_helpers.rs`, leaving two sources of truth.

### Write-only state (wire it or delete it)

- Egg preparation metadata (`preparation_focus`, `loyalty_imprinted`, `secrecy_locked`) is set by Refine and never read; a refined egg differs from raw only by `grade_score`.
- `DayResolutionSummary` has ten write-only fields (prep cost breakdown, upkeep wage/cleaning/maintenance split, special-event totals, contracts generated/rejected) that `day_results.rs` never shows.
- `DebtState.status_message` / `last_resolution`, `ContractState.partial_progress`, `ExpeditionState.started_day`, and story flags `tower_hole_discovered` / `first_egg_created` are written and never read.
- `SpeciesData.preferred_room_ids` is the dead half of a relation (the room-side `preferred_species_ids` is the live one) and nothing validates the two agree.
- `SpeciesData.portrait_key` and `species_portrait_key_by_id()` have no callers — portraits are drawn procedurally.

## Balance

- Decide an acceptable target range for day-365 `surplus_summary.debt_gold_gap`.
- Decide an acceptable target range for final relic and residue stockpiles after project purchases.
- Review whether 30-day outcomes run too high when early egg rolls are favourable.
- Review whether 90-day outcomes run too low when early debt or event rolls are unfavourable.
- Review whether the 180-day building count reliably opens enough population cap before late catch-up hatching.
- Tune final debt pressure against averaged multi-seed results rather than one deterministic report.
- Add late-game project varieties that spend different surplus mixes.
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
