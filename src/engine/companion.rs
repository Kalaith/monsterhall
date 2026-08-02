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
    SKILL_IDS
        .iter()
        .map(|skill_id| companion_skill_value(skills, skill_id))
        .sum()
}

/// The ten skills, in the order a screen should list them.
///
/// There was no such list. Every site that needed all ten wrote them out by
/// hand, and the count kept coming out five — the number of skills the game
/// shipped with — in `companion_daily_wage`, `replacement_score`,
/// `monster_service_score`, the contract desk's gap badges, and both of the
/// label builders in `view_models`. Iterating this is how a site stops being
/// able to miss one.
pub const SKILL_IDS: [&str; 10] = [
    "scouting",
    "guarding",
    "hospitality",
    "crafting",
    "charm",
    "recovery",
    "bargaining",
    "navigation",
    "arcana",
    "strength",
];

/// One skill off a companion, by id. Unknown ids read as zero.
pub fn companion_skill_value(skills: &crate::state::CompanionSkillState, skill_id: &str) -> u32 {
    match skill_id {
        "scouting" => skills.scouting,
        "guarding" => skills.guarding,
        "hospitality" => skills.hospitality,
        "crafting" => skills.crafting,
        "charm" => skills.charm,
        "recovery" => skills.recovery,
        "bargaining" => skills.bargaining,
        "navigation" => skills.navigation,
        "arcana" => skills.arcana,
        "strength" => skills.strength,
        _ => 0,
    }
}

/// The same lookup against an authored skill gain.
pub fn progression_skill_value(
    skills: &crate::data::CompanionSkillProgressionData,
    skill_id: &str,
) -> u32 {
    match skill_id {
        "scouting" => skills.scouting,
        "guarding" => skills.guarding,
        "hospitality" => skills.hospitality,
        "crafting" => skills.crafting,
        "charm" => skills.charm,
        "recovery" => skills.recovery,
        "bargaining" => skills.bargaining,
        "navigation" => skills.navigation,
        "arcana" => skills.arcana,
        "strength" => skills.strength,
        _ => 0,
    }
}

/// The same lookup against a contract's required thresholds.
pub fn required_skill_value(
    skills: &crate::state::ContractSkillRequirementState,
    skill_id: &str,
) -> u32 {
    match skill_id {
        "scouting" => skills.scouting,
        "guarding" => skills.guarding,
        "hospitality" => skills.hospitality,
        "crafting" => skills.crafting,
        "charm" => skills.charm,
        "recovery" => skills.recovery,
        "bargaining" => skills.bargaining,
        "navigation" => skills.navigation,
        "arcana" => skills.arcana,
        "strength" => skills.strength,
        _ => 0,
    }
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

#[cfg(test)]
mod tests {
    use super::{companion_skill_value, required_skill_value, SKILL_IDS};
    use crate::state::{CompanionSkillState, ContractSkillRequirementState};

    /// [`SKILL_IDS`] has to stay the whole list, not most of it.
    ///
    /// Every site that needed all ten skills used to write them out by hand and
    /// kept coming out five. The list fixes that only while it is complete, so
    /// this fills each field with a distinct value and checks the list can see
    /// every one of them: adding an eleventh skill without adding its id here
    /// leaves a number this sum cannot reach.
    #[test]
    fn the_skill_list_reaches_every_field_of_a_companions_skills() {
        let skills = CompanionSkillState {
            scouting: 1,
            guarding: 2,
            hospitality: 4,
            crafting: 8,
            charm: 16,
            recovery: 32,
            bargaining: 64,
            navigation: 128,
            arcana: 256,
            strength: 512,
        };
        let seen: u32 = SKILL_IDS
            .iter()
            .map(|skill_id| companion_skill_value(&skills, skill_id))
            .sum();
        assert_eq!(
            seen, 1023,
            "SKILL_IDS does not name every field on CompanionSkillState."
        );
    }

    /// The same, for the requirement side the contract desk reads.
    #[test]
    fn the_skill_list_reaches_every_field_of_a_contracts_requirement() {
        let required = ContractSkillRequirementState {
            scouting: 1,
            guarding: 2,
            hospitality: 4,
            crafting: 8,
            charm: 16,
            recovery: 32,
            bargaining: 64,
            navigation: 128,
            arcana: 256,
            strength: 512,
        };
        let seen: u32 = SKILL_IDS
            .iter()
            .map(|skill_id| required_skill_value(&required, skill_id))
            .sum();
        assert_eq!(
            seen, 1023,
            "SKILL_IDS does not name every field on ContractSkillRequirementState."
        );
    }
}
