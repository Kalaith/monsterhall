# Balance Follow-Up Tuning

`docs/BALANCE_PLAN.md` is considered complete as the baseline pass. This document tracks future tuning ideas, investigations, and polish work that should be evaluated against the current 30/90/180/365-day campaign reports.

## Current Baseline

- 30 days should end near the early stable roster target.
- 90 days should show steady growth without exhausting the economy.
- 180 days should clearly exceed the old 9-companion stall point.
- 365 days should reach the 20-companion hard cap while keeping Founder’s Due pressure active.
- Late-game projects should convert surplus relics and residue without blocking normal hatching.
- Patron demand, expeditions, upkeep, debt, special events, and project purchases should all appear in simulation reports.

## Follow-Up Checks

- [x] Run multiple 365-day samples and compare average roster, gold, debt gap, relics, residue, buildings, and missed payments.
- [ ] Decide an acceptable target range for `surplus_summary.debt_gold_gap` at day 365.
- [ ] Decide an acceptable target range for final relic and residue stockpiles after project purchases.
- [ ] Review whether 30-day outcomes are too high when early egg rolls are favorable.
- [ ] Review whether 90-day outcomes are too low when early debt or event rolls are unfavorable.
- [ ] Review whether 180-day building count consistently opens enough population cap before late catch-up hatching.

## 365-Day Multi-Seed Check

The validation harness now runs explicit seeded 365-day samples and writes `tmp_screens/playtests/365_day_multi_seed_simulation_summary.json`. The current seeded pass reaches the 20-companion cap in every sample, clears the debt chain by day 365, and ends with average cash near 10% of scheduled debt. Missed payments remain bounded instead of spiraling.

| Metric | Average | Min | Max |
| --- | ---: | ---: | ---: |
| Companions | 20.0 | 20 | 20 |
| Buildings | 12.8 | 11 | 13 |
| Gold | 2,340.4 | 299 | 4,292 |
| Debt gap | 0.0 | 0 | 0 |
| Relics | 205.9 | 171 | 254 |
| Residue essence | 20,952.2 | 15,436 | 32,574 |
| Missed payments | 0.0 | 0 | 0 |
| Max active requests | 6.0 | 6 | 6 |
| Patron expirations | 3.1 | 1 | 7 |

Scheduled debt is currently 23,050 gold, so the 10% cash target is 2,305 gold by day 365.

Current locked expectations:

- Every 365-day seeded sample reaches the 20-companion hard cap.
- Every 365-day seeded sample clears the debt chain.
- Average day-365 cash stays within the target band around 10% of scheduled debt.
- No seeded sample ends above 5 missed payments.

Raw sample output is written by `cargo test -p monsterhall multi_seed_365_simulation_summary_reports_variance -- --nocapture`.

## UI And Reporting

- [ ] Add a projects-specific status line in Town Management that explains repeatable project count, cost, and purpose.
- [ ] Add a concise project/sink summary to simulation reports.
- [ ] Add visual baselines for any new screens or modal states before relying on screenshot comparisons.
- [ ] Add a lock or unique backup paths to `tmp_screens/play_ui_test.py` so UI scenarios cannot collide when run in parallel.

## Balance Ideas To Consider

- [ ] Tune final debt pressure around average results instead of a single deterministic report.
- [ ] Add more late-game project varieties that spend different surplus mixes.
- [ ] Add patron satisfaction as an explicit state if patron completions/expirations are not enough pressure.
- [ ] Add clearer replacement/release flow for the 20-companion cap.
- [ ] Add a stronger reason to choose non-egg expedition missions after the roster reaches cap.
- [ ] Review whether special events should scale cost from roster, reputation, or project count.

## Regression Commands

Run these after balance changes:

```powershell
cargo test -p monsterhall -- --nocapture
cargo check -p monsterhall
cargo build -p monsterhall
```

Useful UI checks:

```powershell
python tmp_screens\play_ui_test.py --scenario town_management --compare-threshold 2.0
python tmp_screens\play_ui_test.py --scenario expedition_planning --compare-threshold 2.0
python tmp_screens\play_ui_test.py --scenario guild_jobs --compare-threshold 2.0
python tmp_screens\play_ui_test.py --scenario guest_management --compare-threshold 2.0
python tmp_screens\play_ui_test.py --scenario chamber_management --compare-threshold 2.0
python tmp_screens\play_ui_test.py --scenario monster_profile --compare-threshold 2.0
```
