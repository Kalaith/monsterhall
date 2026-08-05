# Monster Hall Balance Remediation Plan

This plan turns the 2026-08-05 whole-game review into an ordered set of
independently testable changes. Each numbered phase is intended to land as its
own commit. The final phase validates the combined campaign through the normal
publisher rather than treating an individual green unit test as proof of
balance.

## 1. Restore a mixed economy

Town work currently pays more than one hundred times as much campaign gold as
contracts, while contract penalties, special events, upkeep, and missed debt
payments become negligible. Rework the income and wage rank curves, make a
fulfilled contract competitive with the room shift it displaces, and scale
pressure costs with the stage of the guild.

Acceptance criteria:

- No single repeatable income source supplies more than 85% of earned gold in
  the representative 365-day campaign.
- A fully fulfilled contract pays at least as well as the same companion's
  ordinary shift, while partial or under-prepared service remains worse.
- Rank and skill growth raise both income and wages without creating an
  order-of-magnitude late-game margin.
- Late events, upkeep, contract penalties, and debt misses remain visible
  against late daily income.

This phase covers GDD §2, §3, §7, and §8.

## 2. Make tower choices carry consequences

Expeditions currently award their exact projected resources even when the
success score is negative. Turn the advertised success score into a real
outcome, with failed runs losing the focused rewards and survey progress while
still consuming preparation and readiness. Separate Safe from Recovery Focused
so neither stance strictly dominates the other.

Acceptance criteria:

- An under-prepared expedition can fail and cannot unlock deeper floors.
- Failure consumes preparation, time, and condition while returning at most a
  small salvage share.
- The planning screen communicates chance and consequences without promising
  rewards that resolution may not deliver.
- Safe offers the best injury protection; Recovery Focused instead reduces the
  condition/instability toll of a completed run.

This phase covers GDD pillar 2 and §4.

## 3. Make debt a real campaign pressure

A missed payment currently adds at most 90 gold and two more days forever,
despite the interface naming debt as the campaign fail condition. Introduce an
authored escalation path and a terminal foreclosure state that the game can
display, persist, and recover from only by starting or loading another
campaign.

Acceptance criteria:

- Repeated missed payments increase rather than reset pressure.
- The authored maximum consecutive misses ends the campaign visibly.
- Saves preserve the failed state and normal day resolution cannot continue it.
- A solvent campaign can still pay early and complete the debt chain.

This phase covers GDD §8 and the MVP success/failure loop.

## 4. Finish companion skill ownership

Navigation, Arcana, and Strength are visible but remain zero in ordinary play.
Give each an authored training route, a gameplay consumer, and at least one
later contract that values it. Preserve the rule that a room only teaches work
it genuinely performs.

Acceptance criteria:

- Every displayed skill is trainable from shipped content.
- Every skill changes at least one player-facing preview or eligibility check.
- The long campaign exercises all ten skills without relying on test-only
  companion construction.

This phase covers GDD §1 and §2.

## 5. Complete the contract desk

`ContractStatus::Declined` has no action, and follow-up contracts are absent
from generation telemetry. Add a deliberate decline flow, retain one-day
outcome visibility, and make every generated request count exactly once.

Acceptance criteria:

- A pending offer can be declined without being misreported as failed.
- Declining releases offer pressure and cannot leave a companion reserved.
- For any report, starting requests plus generated requests equals resolved
  requests plus live ending requests.
- Follow-up chains remain eligible and visible after the correction.

This phase covers GDD §3.

## 6. Make capability progression explicit

Research buildings can currently be purchased in any order if their price is
met. Add authored prerequisites, validation, purchase guards, and clear UI
feedback so major species, patron, and tower capabilities arrive through a
coherent infrastructure chain.

Acceptance criteria:

- Every non-starting capability building has an intentional prerequisite or an
  explicit assertion that it is a root choice.
- Invalid, missing, or cyclic building prerequisites fail data validation.
- The town screen explains which prerequisite blocks a purchase.
- Existing saves remain compatible.

This phase covers GDD pillar 5 and §6.

## 7. Make balance evidence repeatable and representative

The seeded reports share a process-global RNG with parallel tests, so a full
suite can rewrite a fixed-seed report while isolated runs agree. The policy
also uses one-person Balanced expeditions and assigns every worker to the room
with the highest authored base gold, leaving most choices untested.

Acceptance criteria:

- A fixed seed produces byte-identical reports in isolated and full-suite runs.
- The policy evaluates companion-specific room returns, multi-member parties,
  all viable stances, and contract opportunity cost.
- Multi-seed assertions cover economy mix, failures, roster/role variety,
  expedition continuity, and debt outcomes without requiring every seed to
  follow one route.
- Generated reports satisfy their own accounting identities.

## 8. Validate the combined campaign

After all phases land, run formatting, Clippy with warnings denied, the complete
test suite, the explicit balance probe, and `./publish.ps1` with no parameters.
Review the 30-, 90-, 180-, and 365-day reports plus the ten-seed summary. Any
threshold changed in response to those reports must be explained in the commit
that changes it.

The remediation is complete only when the worktree is clean, the publisher
passes, and the final review can answer that the game's major choices are both
mechanically coherent and economically meaningful.
