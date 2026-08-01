//! Room, building, and floor validation.
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
            room_ids,
            floor_ids,
            patron_tier_ids,
            species_ids,
            ..
        } = ids;
        for building in &self.buildings.buildings {
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
