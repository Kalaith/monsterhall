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
