//! What a companion is actually worth right now.
//!
//! Her `stats` are what her species and her hatch rolled. Traits are the rest of
//! the story: thirteen of them in `traits.json` author stat bonuses that nothing
//! read, so a Stonebound porter and a plain one hauled exactly the same. Traits
//! arrive after creation too — mutations grant them — so the bonus is summed on
//! demand rather than baked in at hatch.

use crate::data::{GameData, StatBlockData};
use crate::state::CompanionState;

/// The companion's base stats plus every bonus her traits carry.
///
/// This is the figure the simulation and the profile screen should both use;
/// `monster.stats` on its own is the pre-trait baseline.
pub fn effective_stats(data: &GameData, monster: &CompanionState) -> StatBlockData {
    let bonus = trait_stat_bonus(data, monster);
    StatBlockData {
        power: monster.stats.power + bonus.power,
        charm: monster.stats.charm + bonus.charm,
        endurance: monster.stats.endurance + bonus.endurance,
        instinct: monster.stats.instinct + bonus.instinct,
    }
}

/// Only the trait half, for a screen that wants to show what the traits are
/// adding rather than the total.
pub fn trait_stat_bonus(data: &GameData, monster: &CompanionState) -> StatBlockData {
    let mut bonus = StatBlockData::default();

    for trait_id in &monster.trait_ids {
        let Some(trait_data) = data
            .traits
            .traits
            .iter()
            .find(|entry| entry.id == *trait_id)
        else {
            continue;
        };
        bonus.power += trait_data.stat_modifiers.power;
        bonus.charm += trait_data.stat_modifiers.charm;
        bonus.endurance += trait_data.stat_modifiers.endurance;
        bonus.instinct += trait_data.stat_modifiers.instinct;
    }

    bonus
}

/// The species a companion is *right now*.
///
/// Mutation rewrites `species_id` mid-campaign, so this is looked up per call
/// rather than cached at hatch. Both the wage and the role-flexibility term read
/// it; they must not grow separate copies of the lookup.
pub fn species_of<'a>(
    data: &'a GameData,
    monster: &CompanionState,
) -> Option<&'a crate::data::SpeciesData> {
    data.species
        .species
        .iter()
        .find(|species| species.id == monster.species_id)
}

/// A species' total base stats — the game's existing measure of how capable it
/// is, reused as its tier so no second authored ranking can drift from it.
pub fn species_stat_total(species: &crate::data::SpeciesData) -> u32 {
    (species.base_stats.power
        + species.base_stats.charm
        + species.base_stats.endurance
        + species.base_stats.instinct)
        .max(0) as u32
}

/// Everything a companion has been taught, across all ten skills.
///
/// The single sum, because it had already been written out longhand three times
/// and every copy was authored against the five skills that existed at the time.
/// `companion_daily_wage` was fixed once; `replacement_score` on the hatchery
/// screen and `monster_service_score` in the validation policy were still
/// counting five, and both of those decide **which companion gets released** —
/// so a companion who trained recovery or bargaining read as more expendable
/// than an identical one who had learned nothing, while costing more to keep.
pub fn companion_skill_total(skills: &crate::state::CompanionSkillState) -> u32 {
    skills.scouting
        + skills.guarding
        + skills.hospitality
        + skills.crafting
        + skills.charm
        + skills.recovery
        + skills.bargaining
        + skills.navigation
        + skills.arcana
        + skills.strength
}

/// Share of full output this companion delivers today, 100 when rested.
///
/// The engine's own answer, exposed so screens stop inventing their own. The
/// profile screen called a companion "hurt" at `fatigue >= 3` — a threshold from
/// before the condition system existed. A single guild shift adds ten fatigue
/// and four stress, and the allowances are thirty and twenty, so from her first
/// day of work every companion read as hurt while delivering exactly 100%, and
/// the screen advised resting her for a day at no benefit.
pub fn companion_effectiveness(data: &GameData, monster: &CompanionState) -> u32 {
    crate::engine::day_cycle::companion_effectiveness_pct(&data.config.day_cycle, monster)
}
