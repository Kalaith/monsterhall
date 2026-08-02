use std::collections::HashSet;

use super::config_types::*;
use super::depth_validation::validate_depth_systems;
use super::types::*;

mod config;
mod creatures;
mod events;
mod facilities;
mod ids;
mod story;

/// Ids collected once up front and shared across the validation passes.
struct IdIndex<'a> {
    trait_ids: HashSet<&'a str>,
    debt_milestone_ids: HashSet<&'a str>,
    patron_archetype_ids: HashSet<&'a str>,
    patron_tier_ids: HashSet<&'a str>,
    room_ids: HashSet<&'a str>,
    building_ids: HashSet<&'a str>,
    floor_ids: HashSet<&'a str>,
    species_ids: HashSet<&'a str>,
    mission_ids: HashSet<&'a str>,
    opening_step_ids: HashSet<&'a str>,
}

impl<'a> IdIndex<'a> {
    fn collect(data: &'a GameData) -> Self {
        let trait_ids = collect_ids(data.traits.traits.iter().map(|entry| entry.id.as_str()));
        let debt_milestone_ids = collect_ids(
            data.debt_milestones
                .milestones
                .iter()
                .map(|entry| entry.id.as_str()),
        );
        let patron_archetype_ids = collect_ids(
            data.patron_archetypes
                .archetypes
                .iter()
                .map(|entry| entry.id.as_str()),
        );
        let patron_tier_ids = collect_ids(
            data.patron_tiers
                .patron_tiers
                .iter()
                .map(|entry| entry.id.as_str()),
        );
        let room_ids = collect_ids(data.guild_rooms.rooms.iter().map(|entry| entry.id.as_str()));
        let building_ids = collect_ids(
            data.buildings
                .buildings
                .iter()
                .map(|entry| entry.id.as_str()),
        );
        let floor_ids = collect_ids(data.floors.floors.iter().map(|entry| entry.id.as_str()));
        let species_ids = collect_ids(data.species.species.iter().map(|entry| entry.id.as_str()));
        let mission_ids = collect_ids(data.missions.missions.iter().map(|entry| entry.id.as_str()));
        let opening_step_ids = collect_ids(
            data.story_events
                .opening_steps
                .iter()
                .map(|entry| entry.id.as_str()),
        );
        Self {
            trait_ids,
            debt_milestone_ids,
            patron_archetype_ids,
            patron_tier_ids,
            room_ids,
            building_ids,
            floor_ids,
            species_ids,
            mission_ids,
            opening_step_ids,
        }
    }
}

impl GameData {
    pub(super) fn validate(&self) -> Result<(), String> {
        self.validate_config()?;
        self.validate_unique_id_domains()?;

        let ids = IdIndex::collect(self);

        self.validate_core_content(&ids)?;
        self.validate_debt_and_patrons(&ids)?;
        self.validate_contracts(&ids)?;
        self.validate_monster_names(&ids)?;
        self.validate_species(&ids)?;
        self.validate_mutations(&ids)?;
        self.validate_rooms(&ids)?;
        self.validate_buildings(&ids)?;
        self.validate_floors(&ids)?;
        self.validate_relics()?;
        self.validate_events(&ids)?;
        self.validate_references(&ids)?;
        self.validate_new_game(&ids)?;

        self.ui_text.validate()?;
        validate_depth_systems(self)?;

        Ok(())
    }
}

fn validate_unique_ids<'a, I>(ids: I, domain_name: &str) -> Result<(), String>
where
    I: Iterator<Item = &'a str>,
{
    let mut seen_ids = HashSet::new();

    for id in ids {
        if id.trim().is_empty() {
            return Err(format!("{domain_name} contains an empty id."));
        }

        if !seen_ids.insert(id.to_owned()) {
            return Err(format!("{domain_name} contains duplicate id '{id}'."));
        }
    }

    Ok(())
}

/// Every skill the engine can actually train, score and name.
///
/// The five newer ones were declared on `CompanionSkillState` and plumbed
/// nowhere, so a room that listed one would have trained a value no code
/// incremented and displayed it as "Unknown". They work now; whether any room
/// opts into them is a content decision.
fn is_valid_companion_skill_id(skill_id: &str) -> bool {
    matches!(
        skill_id,
        "scouting"
            | "guarding"
            | "hospitality"
            | "crafting"
            | "charm"
            | "recovery"
            | "bargaining"
            | "navigation"
            | "arcana"
            | "strength"
    )
}

fn companion_skill_progression_is_empty(progression: &CompanionSkillProgressionData) -> bool {
    progression.scouting == 0
        && progression.guarding == 0
        && progression.hospitality == 0
        && progression.crafting == 0
        && progression.charm == 0
        && progression.recovery == 0
        && progression.bargaining == 0
        && progression.navigation == 0
        && progression.arcana == 0
        && progression.strength == 0
}

fn work_history_progression_is_empty(progression: &CompanionWorkHistoryProgressionData) -> bool {
    progression.scouting_runs == 0
        && progression.guard_duties == 0
        && progression.hospitality_jobs == 0
        && progression.craft_jobs == 0
        && progression.contracts_completed == 0
        && progression.recovery_shifts == 0
        && progression.hatchery_assists == 0
}

fn validate_reference_list(
    ids: &[String],
    known_ids: &HashSet<&str>,
    label: &str,
) -> Result<(), String> {
    for id in ids {
        if !known_ids.contains(id.as_str()) {
            return Err(format!("{label} references unknown id '{id}'."));
        }
    }

    Ok(())
}

fn collect_ids<'a, I>(ids: I) -> HashSet<&'a str>
where
    I: Iterator<Item = &'a str>,
{
    ids.collect()
}

fn validate_non_negative_stats(stats: &StatBlockData, label: &str) -> Result<(), String> {
    if stats.power < 0 || stats.charm < 0 || stats.endurance < 0 || stats.instinct < 0 {
        return Err(format!("{label} contains a negative base stat."));
    }

    Ok(())
}
