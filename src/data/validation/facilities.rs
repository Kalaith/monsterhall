//! Room, building, and floor validation.
use std::collections::HashMap;

use super::IdIndex;
use super::{is_valid_companion_skill_id, validate_reference_list};
use crate::data::types::*;

impl GameData {
    pub(super) fn validate_rooms(&self, ids: &IdIndex<'_>) -> Result<(), String> {
        let IdIndex {
            trait_ids,
            building_ids,
            patron_tier_ids,
            species_ids,
            ..
        } = ids;
        for room in &self.guild_rooms.rooms {
            if room.service_summary.trim().is_empty() {
                return Err(format!(
                    "guild room '{}' must define a service_summary.",
                    room.id
                ));
            }
            validate_reference_list(
                &room.required_building_ids,
                building_ids,
                &format!("guild room '{}'.required_building_ids", room.id),
            )?;
            validate_reference_list(
                &room.preferred_trait_ids,
                trait_ids,
                &format!("guild room '{}'.preferred_trait_ids", room.id),
            )?;
            validate_reference_list(
                &room.preferred_species_ids,
                species_ids,
                &format!("guild room '{}'.preferred_species_ids", room.id),
            )?;
            validate_reference_list(
                &room.patron_tiers,
                patron_tier_ids,
                &format!("guild room '{}'.patron_tiers", room.id),
            )?;
            if room.trained_skill_ids.is_empty() {
                return Err(format!(
                    "guild room '{}' must define at least one trained_skill_id.",
                    room.id
                ));
            }
            for skill_id in &room.trained_skill_ids {
                if !is_valid_companion_skill_id(skill_id) {
                    return Err(format!(
                        "guild room '{}' references unknown trained skill '{}'.",
                        room.id, skill_id
                    ));
                }
            }
            if room.base_gold_yield == 0 {
                return Err(format!("guild room '{}' must generate gold.", room.id));
            }
            if room.work_history_gains.scouting_runs == 0
                && room.work_history_gains.guard_duties == 0
                && room.work_history_gains.hospitality_jobs == 0
                && room.work_history_gains.craft_jobs == 0
                && room.work_history_gains.contracts_completed == 0
                && room.work_history_gains.recovery_shifts == 0
                && room.work_history_gains.hatchery_assists == 0
            {
                return Err(format!(
                    "guild room '{}' must define at least one history gain.",
                    room.id
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_buildings(&self, ids: &IdIndex<'_>) -> Result<(), String> {
        let IdIndex {
            building_ids,
            room_ids,
            floor_ids,
            patron_tier_ids,
            species_ids,
            ..
        } = ids;
        for building in &self.buildings.buildings {
            validate_reference_list(
                &building.prerequisite_building_ids,
                building_ids,
                &format!("building '{}'.prerequisite_building_ids", building.id),
            )?;
            if building
                .prerequisite_building_ids
                .iter()
                .any(|required_id| required_id == &building.id)
            {
                return Err(format!("building '{}' cannot require itself.", building.id));
            }
            if building.is_root_choice != building.prerequisite_building_ids.is_empty() {
                return Err(format!(
                    "building '{}' must either be an explicit root choice or name prerequisites.",
                    building.id
                ));
            }
            validate_reference_list(
                &building.unlocks.room_ids,
                room_ids,
                &format!("building '{}'.unlocks.room_ids", building.id),
            )?;
            validate_reference_list(
                &building.unlocks.floor_ids,
                floor_ids,
                &format!("building '{}'.unlocks.floor_ids", building.id),
            )?;
            validate_reference_list(
                &building.unlocks.species_ids,
                species_ids,
                &format!("building '{}'.unlocks.species_ids", building.id),
            )?;
            validate_reference_list(
                &building.unlocks.patron_tiers,
                patron_tier_ids,
                &format!("building '{}'.unlocks.patron_tiers", building.id),
            )?;
            if building.build_limit == 0 {
                return Err(format!(
                    "building '{}' must have a positive build_limit.",
                    building.id
                ));
            }
        }
        validate_building_prerequisite_cycles(&self.buildings.buildings)?;
        Ok(())
    }

    /// A relic no floor drops is a relic that cannot be found, which makes it
    /// prose nobody will ever read.
    pub(super) fn validate_relics(&self) -> Result<(), String> {
        for relic in &self.relics.relics {
            if relic.name.trim().is_empty() || relic.description.trim().is_empty() {
                return Err(format!(
                    "relic '{}' must have a name and description.",
                    relic.id
                ));
            }
            if !self
                .floors
                .floors
                .iter()
                .any(|floor| floor.relic_drop_ids.contains(&relic.id))
            {
                return Err(format!(
                    "relic '{}' is dropped by no floor and can never be found.",
                    relic.id
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_floors(&self, ids: &IdIndex<'_>) -> Result<(), String> {
        let IdIndex {
            building_ids,
            species_ids,
            mission_ids,
            ..
        } = ids;
        for floor in &self.floors.floors {
            validate_reference_list(
                &floor.requires_building_ids,
                building_ids,
                &format!("floor '{}'.requires_building_ids", floor.id),
            )?;
            for requirement in &floor.required_roster {
                if !species_ids.contains(requirement.species_id.as_str()) {
                    return Err(format!(
                        "floor '{}'.required_roster references unknown species '{}'.",
                        floor.id, requirement.species_id
                    ));
                }
                let max_rank = self.config.day_cycle.egg_quality_rank_thresholds.len() as u8 + 1;
                if !(1..=max_rank).contains(&requirement.minimum_quality_rank) {
                    return Err(format!(
                        "floor '{}'.required_roster for species '{}' must require 1 to {max_rank} stars.",
                        floor.id, requirement.species_id
                    ));
                }
            }
            if floor.egg_species_entries.is_empty() {
                return Err(format!(
                    "floor '{}' must define at least one egg_species_entry.",
                    floor.id
                ));
            }
            for egg_entry in &floor.egg_species_entries {
                if !species_ids.contains(egg_entry.species_id.as_str()) {
                    return Err(format!(
                        "floor '{}'.egg_species_entries references unknown species '{}'.",
                        floor.id, egg_entry.species_id
                    ));
                }
                if egg_entry.weight == 0 {
                    return Err(format!(
                        "floor '{}'.egg_species_entries for species '{}' must have positive weight.",
                        floor.id, egg_entry.species_id
                    ));
                }
            }
            if floor.mission_ids.is_empty() {
                return Err(format!(
                    "floor '{}' must list at least one mission type.",
                    floor.id
                ));
            }
            validate_reference_list(
                &floor.mission_ids,
                mission_ids,
                &format!("floor '{}'.mission_ids", floor.id),
            )?;
            if floor.difficulty == 0 {
                return Err(format!(
                    "floor '{}' must have a positive difficulty.",
                    floor.id
                ));
            }
            // `difficulty` is subtracted from expedition success and added to
            // injury risk, so past a point no realistic party can clear a floor
            // and it becomes content the player can see but never beat. Band
            // authoring is expected to spread difficulty across the tower, not
            // extend the early slope.
            let max_difficulty = self.config.day_cycle.max_floor_difficulty;
            if floor.difficulty > max_difficulty {
                return Err(format!(
                    "floor '{}' has difficulty {} above the beatable ceiling of {max_difficulty}.",
                    floor.id, floor.difficulty
                ));
            }
            for relic_id in &floor.relic_drop_ids {
                if !self.relics.relics.iter().any(|relic| &relic.id == relic_id) {
                    return Err(format!(
                        "floor '{}'.relic_drop_ids references unknown relic '{relic_id}'.",
                        floor.id
                    ));
                }
            }
            self.validate_floor_survey_chain(floor)?;
        }
        Ok(())
    }

    /// A survey chain that names a floor which does not exist, names itself, or
    /// asks for zero surveys is a floor that can never open — and the failure
    /// would show up as an unreachable floor rather than an error, so it is
    /// caught at load instead.
    fn validate_floor_survey_chain(
        &self,
        floor: &crate::data::TowerFloorData,
    ) -> Result<(), String> {
        if floor.requires_surveyed_floor_ids.is_empty() {
            return Ok(());
        }
        if floor.required_surveys == 0 {
            return Err(format!(
                "floor '{}' lists a survey chain but requires zero surveys.",
                floor.id
            ));
        }
        for required_id in &floor.requires_surveyed_floor_ids {
            if required_id == &floor.id {
                return Err(format!(
                    "floor '{}' cannot require surveying itself.",
                    floor.id
                ));
            }
            if !self
                .floors
                .floors
                .iter()
                .any(|candidate| &candidate.id == required_id)
            {
                return Err(format!(
                    "floor '{}'.requires_surveyed_floor_ids references unknown floor '{}'.",
                    floor.id, required_id
                ));
            }
        }
        Ok(())
    }
}

fn validate_building_prerequisite_cycles(buildings: &[BuildingData]) -> Result<(), String> {
    let mut visit_states = HashMap::<&str, u8>::new();
    let mut path = Vec::<&str>::new();
    for building in buildings {
        visit_building_prerequisites(&building.id, buildings, &mut visit_states, &mut path)?;
    }
    Ok(())
}

fn visit_building_prerequisites<'a>(
    building_id: &'a str,
    buildings: &'a [BuildingData],
    visit_states: &mut HashMap<&'a str, u8>,
    path: &mut Vec<&'a str>,
) -> Result<(), String> {
    match visit_states.get(building_id) {
        Some(2) => return Ok(()),
        Some(1) => {
            let cycle_start = path
                .iter()
                .position(|entry| *entry == building_id)
                .unwrap_or(0);
            let mut cycle = path[cycle_start..].to_vec();
            cycle.push(building_id);
            return Err(format!(
                "building prerequisites contain a cycle: {}.",
                cycle.join(" -> ")
            ));
        }
        _ => {}
    }

    visit_states.insert(building_id, 1);
    path.push(building_id);
    let building = buildings
        .iter()
        .find(|building| building.id == building_id)
        .expect("prerequisite ids are validated before cycle detection");
    for prerequisite_id in &building.prerequisite_building_ids {
        visit_building_prerequisites(prerequisite_id, buildings, visit_states, path)?;
    }
    path.pop();
    visit_states.insert(building_id, 2);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_shipped_building_is_an_explicit_root_or_names_a_prerequisite() {
        let data = crate::data::test_game_data();

        data.validate()
            .expect("the shipped building tree should be valid");
        for building in &data.buildings.buildings {
            assert_eq!(
                building.is_root_choice,
                building.prerequisite_building_ids.is_empty(),
                "building '{}' has no explicit place in the tree",
                building.id
            );
        }
    }

    #[test]
    fn an_unknown_building_prerequisite_fails_validation() {
        let mut data = crate::data::test_game_data();
        let building = data
            .buildings
            .buildings
            .iter_mut()
            .find(|building| !building.is_root_choice)
            .expect("the catalogue should include a dependent building");
        building.prerequisite_building_ids = vec!["missing_foundation".to_owned()];

        let error = data
            .validate()
            .expect_err("an unknown prerequisite should reject the catalogue");
        assert!(error.contains("missing_foundation"), "{error}");
    }

    #[test]
    fn a_building_prerequisite_cycle_fails_validation() {
        let mut data = crate::data::test_game_data();
        let first_id = data.buildings.buildings[0].id.clone();
        let second_id = data.buildings.buildings[1].id.clone();
        data.buildings.buildings[0].is_root_choice = false;
        data.buildings.buildings[0].prerequisite_building_ids = vec![second_id.clone()];
        data.buildings.buildings[1].is_root_choice = false;
        data.buildings.buildings[1].prerequisite_building_ids = vec![first_id.clone()];

        let error = data
            .validate()
            .expect_err("a prerequisite cycle should reject the catalogue");
        assert!(error.contains("cycle"), "{error}");
        assert!(
            error.contains(&first_id) && error.contains(&second_id),
            "{error}"
        );
    }
}
