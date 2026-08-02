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
