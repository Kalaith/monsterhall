# Monster Hall Balance Review

Reviewed and remediated on 2026-08-05 against content version 1.38.0.

## Verdict

Monster Hall is balanced and mechanically coherent for its current 365-day
campaign scope. Its major systems now reinforce one another instead of running
as parallel or decorative tracks:

- room work, contracts, upkeep, events, and debt share one economy;
- expeditions consume readiness and preparation, can fail, and gate survey
  progression;
- stance, party, role, skill, room, and contract choices have distinct costs;
- every displayed skill has shipped training and a gameplay consumer;
- building order expresses capability progression;
- debt can escalate to a persistent terminal state;
- generated contract, resource, and campaign telemetry is deterministic and
  internally accountable.

This verdict is evidence-based rather than a claim that every play style will
produce the same route or outcome.

## Campaign Evidence

| Horizon | Roster | Buildings | Floors | Room / contract gold | Expeditions (win/loss) | Debt state |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| 30 days | 7 | 7 | 9 | 55.95% / 44.05% | 22 / 1 | Collector Visit active |
| 90 days | 13 | 9 | 11 | 53.16% / 46.84% | 47 / 8 | Interest Crush active |
| 180 days | 17 | 9 | 11 | 59.83% / 40.17% | 112 / 9 | Tribute Cart active |
| 365 days | 20 | 19 | 25 | 81.91% / 18.09% | 90 / 7 | Founder's Due active |

The 365-day roster contains ten species and four inferred roles. Every skill
has a non-zero final total, no floor is stranded, and the final 487,230 gold is
2,012,770 short of Founder's Due. Contract accounting balances exactly at all
four horizons: starting plus generated requests equals resolved plus live
requests (46, 237, 440, and 891 respectively).

Across ten independent 365-day seeds:

- room work averages 83.97% of repeatable earned gold and peaks at 90.07%;
- every campaign records 8-26 failed expeditions and continues tower work
  after day 90;
- every campaign exercises a three-companion party;
- the roster averages three roles, never falls below two, and never falls
  below eight species;
- three campaigns clear the debt chain, seven retain a balance, and none
  foreclose under the representative policy.

The cohort deliberately allows route variance. The economy guard therefore
holds room work below 85% on average and below 95% in any one seed, while the
canonical campaign retains the stricter 85% ceiling.

## Remediation Completed

1. Rebalanced room income, contracts, wages, pressure costs, and late sinks.
2. Added real expedition failure, salvage, survey consequences, and distinct
   Safe versus Recovery Focused tradeoffs.
3. Added escalating missed payments, foreclosure, persistence, and recovery
   presentation.
4. Added training and consumers for Navigation, Arcana, and Strength.
5. Added contract decline, one-day outcome visibility, and exact request
   accounting including follow-ups.
6. Added explicit building roots and prerequisites with validation, purchase
   guards, and UI explanations.
7. Isolated seeded simulation randomness from parallel process-global draws.
8. Replaced the one-worker Balanced-only steward with representative room,
   contract, party, mission, and stance evaluation.
9. Expanded multi-seed evidence and tuned the requested-service contract
   premium from 125% to 135%.

## Validation

- `cargo test`: 161 passed, 0 failed, 1 intentionally ignored probe.
- `cargo test probe_floor_usage -- --ignored --nocapture`: passed; all 25
  floors unlocked.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo fmt --package monsterhall -- --check`: passed.
- source-size, icon-atlas, and UI-text catalog guards: passed.
- `.\publish.ps1`: Windows and WebGL release builds, packages, preview deploy,
  Project Roost record, and catalog refresh all passed.

## Watch Items

These are not current blockers, but the regression reports should keep them
visible as more floors and content are authored:

- all ten representative campaigns reach the current population cap by day
  365, so future content should preserve meaningful timing and roster-quality
  decisions even if the cap remains the mature destination;
- one mutation-heavy seed finishes with two inferred roles despite eight
  species, which is why role and species diversity are measured separately;
- the competent representative steward does not foreclose in ten seeds, while
  the terminal path remains covered directly and seven seeds still carry final
  debt;
- late room work is intentionally the largest income source, so the 85%/95%
  economy guards should move only with an explained design change.
