//! How worn down a companion is, expressed as the share of her work that
//! actually lands.
//!
//! Fatigue, stress and injury are written by every job, expedition, rest and
//! debt shock. This module is the one place that turns those meters back into
//! consequence, so previews and day resolution can never disagree about what a
//! tired roster is worth.

use super::*;

/// Percentage of full output this companion delivers today.
///
/// 100 means rested. Damage past each allowance costs effectiveness at its own
/// rate — injury bites hardest and starts immediately, fatigue is the gentlest
/// and the easiest to sleep off.
pub(crate) fn companion_effectiveness_pct(
    config: &DayCycleConfigData,
    monster: &CompanionState,
) -> u32 {
    let effects = &config.condition_effects;
    let penalty = penalty_pct(
        monster.fatigue,
        effects.fatigue_allowance,
        effects.fatigue_penalty_pct_per_ten,
    ) + penalty_pct(
        monster.stress,
        effects.stress_allowance,
        effects.stress_penalty_pct_per_ten,
    ) + penalty_pct(
        monster.injury,
        effects.injury_allowance,
        effects.injury_penalty_pct_per_ten,
    );

    let floor = effects.min_effectiveness_pct.min(100);
    100u32.saturating_sub(penalty).max(floor)
}

/// Average effectiveness across an expedition party, so a preview can quote one
/// number for the group. An empty party is nominally fresh.
pub(crate) fn party_effectiveness_pct(
    config: &DayCycleConfigData,
    monsters: &[&CompanionState],
) -> u32 {
    if monsters.is_empty() {
        return 100;
    }

    let total: u32 = monsters
        .iter()
        .map(|monster| companion_effectiveness_pct(config, monster))
        .sum();
    total / monsters.len() as u32
}

/// Scale a yield by an effectiveness percentage without overflowing on the
/// large gold figures the rank ladder produces.
pub(crate) fn scale_by_effectiveness(value: u32, effectiveness_pct: u32) -> u32 {
    u32::try_from(u64::from(value) * u64::from(effectiveness_pct) / 100).unwrap_or(u32::MAX)
}

/// Signed variant for reputation and score contributions, which can be negative.
pub(crate) fn scale_signed_by_effectiveness(value: i32, effectiveness_pct: u32) -> i32 {
    i32::try_from(i64::from(value) * i64::from(effectiveness_pct) / 100).unwrap_or(value)
}

/// Holds every condition meter under its ceiling.
///
/// Fatigue and stress used to climb without limit because only the Resting
/// assignment took them down. Now that the meters are read, an unbounded one
/// means a companion is written off permanently by a bad fortnight.
pub(crate) fn clamp_condition_meters(config: &DayCycleConfigData, monster: &mut CompanionState) {
    let ceiling = config.condition_effects.max_meter;
    monster.fatigue = monster.fatigue.min(ceiling);
    monster.stress = monster.stress.min(ceiling);
    monster.injury = monster.injury.min(ceiling);
}

fn penalty_pct(value: u32, allowance: u32, penalty_per_ten: u32) -> u32 {
    value.saturating_sub(allowance) * penalty_per_ten / 10
}
