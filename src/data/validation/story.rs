//! Opening steps, debt milestones, patrons, contracts, and name pools.
use super::IdIndex;
use super::{
    companion_skill_progression_is_empty, validate_reference_list,
    work_history_progression_is_empty,
};
use crate::data::types::*;
use crate::data::validation_helpers::validate_story_event_text_fields;

impl GameData {
    pub(super) fn validate_core_content(&self, ids: &IdIndex<'_>) -> Result<(), String> {
        let IdIndex {
            debt_milestone_ids,
            opening_step_ids,
            ..
        } = ids;
        if self.debt_milestones.milestones.is_empty() {
            return Err("debt_milestones.json must define at least one milestone.".to_owned());
        }
        if self.patron_archetypes.archetypes.is_empty() {
            return Err("patron archetype catalog must define at least one archetype.".to_owned());
        }
        if self.contracts.requests.is_empty() {
            return Err("contract catalog must define at least one request.".to_owned());
        }

        if !debt_milestone_ids.contains(self.debt_milestones.first_milestone_id.as_str()) {
            return Err(format!(
                "debt_milestones.json first_milestone_id '{}' was not found in milestones.",
                self.debt_milestones.first_milestone_id
            ));
        }

        for required_step_id in [
            "camp",
            "discovery",
            "incubation",
            "hatch",
            "build_room",
            "first_client",
        ] {
            if !opening_step_ids.contains(required_step_id) {
                return Err(format!(
                    "story_events.json must define opening step '{}'.",
                    required_step_id
                ));
            }
        }

        for opening_step in &self.story_events.opening_steps {
            if opening_step.title.trim().is_empty() {
                return Err(format!(
                    "opening step '{}' must contain a title.",
                    opening_step.id
                ));
            }
            if opening_step.body_lines.is_empty() {
                return Err(format!(
                    "opening step '{}' must contain at least one body line.",
                    opening_step.id
                ));
            }
            if opening_step.primary_action_label.trim().is_empty() {
                return Err(format!(
                    "opening step '{}' must contain a primary action label.",
                    opening_step.id
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_debt_and_patrons(&self, ids: &IdIndex<'_>) -> Result<(), String> {
        let IdIndex {
            debt_milestone_ids, ..
        } = ids;
        for milestone in &self.debt_milestones.milestones {
            if milestone.name.trim().is_empty() {
                return Err(format!(
                    "debt milestone '{}' must contain a name.",
                    milestone.id
                ));
            }
            if milestone.description.trim().is_empty() {
                return Err(format!(
                    "debt milestone '{}' must contain a description.",
                    milestone.id
                ));
            }
            if milestone.amount_due == 0 {
                return Err(format!(
                    "debt milestone '{}' must define a positive amount_due.",
                    milestone.id
                ));
            }
            if milestone.days_allowed == 0 {
                return Err(format!(
                    "debt milestone '{}' must define a positive days_allowed.",
                    milestone.id
                ));
            }
            if let Some(next_milestone_id) = &milestone.next_milestone_id {
                if next_milestone_id == &milestone.id {
                    return Err(format!(
                        "debt milestone '{}' cannot point to itself as next_milestone_id.",
                        milestone.id
                    ));
                }
                if !debt_milestone_ids.contains(next_milestone_id.as_str()) {
                    return Err(format!(
                        "debt milestone '{}' references unknown next_milestone_id '{}'.",
                        milestone.id, next_milestone_id
                    ));
                }
            }
        }

        for archetype in &self.patron_archetypes.archetypes {
            if archetype.name.trim().is_empty() {
                return Err(format!(
                    "guest archetype '{}' must contain a name.",
                    archetype.id
                ));
            }
            if archetype.description.trim().is_empty() {
                return Err(format!(
                    "guest archetype '{}' must contain a description.",
                    archetype.id
                ));
            }
            // `tags` are prose today, but a `special` archetype whose contracts
            // are not `is_special` is a content mistake that reads as working:
            // the special-guest story flag never fires and the offer never gets
            // its priority bonus.
            if archetype.tags.iter().any(|tag| tag == "special") {
                let has_special_contract = self
                    .contracts
                    .requests
                    .iter()
                    .any(|request| request.archetype_id == archetype.id && request.is_special);
                if !has_special_contract {
                    return Err(format!(
                        "guest archetype '{}' is tagged special but none of its contracts set is_special.",
                        archetype.id
                    ));
                }
            }
            if archetype.spawn_weight == 0 {
                return Err(format!(
                    "guest archetype '{}' must define a positive spawn_weight.",
                    archetype.id
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_contracts(&self, ids: &IdIndex<'_>) -> Result<(), String> {
        let IdIndex {
            patron_archetype_ids,
            room_ids,
            species_ids,
            ..
        } = ids;
        for request in &self.contracts.requests {
            if request.name.trim().is_empty() {
                return Err(format!("contract '{}' must contain a name.", request.id));
            }
            if request.description.trim().is_empty() {
                return Err(format!(
                    "contract '{}' must contain a description.",
                    request.id
                ));
            }
            if !patron_archetype_ids.contains(request.archetype_id.as_str()) {
                return Err(format!(
                    "contract '{}' references unknown archetype '{}'.",
                    request.id, request.archetype_id
                ));
            }
            if !room_ids.contains(request.requested_room_id.as_str()) {
                return Err(format!(
                    "contract '{}' references unknown room '{}'.",
                    request.id, request.requested_room_id
                ));
            }
            validate_reference_list(
                &request.required_species_ids,
                species_ids,
                &format!("contract '{}'.required_species_ids", request.id),
            )?;
            let max_rank = self.config.day_cycle.egg_quality_rank_thresholds.len() as u8 + 1;
            if !(1..=max_rank).contains(&request.minimum_quality_rank) {
                return Err(format!(
                    "contract '{}'.minimum_quality_rank must be between 1 and {max_rank}.",
                    request.id
                ));
            }
            if request.deadline_days == 0 {
                return Err(format!(
                    "contract '{}' must define a positive deadline_days.",
                    request.id
                ));
            }
            // A contract may only ask for a skill some guild room actually
            // teaches. Nothing trains recovery, bargaining, navigation, arcana
            // or strength today, so asking for one would be a booking no
            // companion could ever qualify for — the same shape of dead content
            // as a room advertising odds for work it cannot bank.
            for (skill_id, threshold) in [
                ("scouting", request.required_skill_thresholds.scouting),
                ("guarding", request.required_skill_thresholds.guarding),
                ("hospitality", request.required_skill_thresholds.hospitality),
                ("crafting", request.required_skill_thresholds.crafting),
                ("charm", request.required_skill_thresholds.charm),
                ("recovery", request.required_skill_thresholds.recovery),
                ("bargaining", request.required_skill_thresholds.bargaining),
                ("navigation", request.required_skill_thresholds.navigation),
                ("arcana", request.required_skill_thresholds.arcana),
                ("strength", request.required_skill_thresholds.strength),
            ] {
                if threshold == 0 {
                    continue;
                }
                if !self
                    .guild_rooms
                    .rooms
                    .iter()
                    .any(|room| room.trained_skill_ids.iter().any(|id| id == skill_id))
                {
                    return Err(format!(
                        "contract '{}' requires {skill_id} {threshold}, but no guild room trains {skill_id} - no companion could ever qualify.",
                        request.id
                    ));
                }
            }
            if companion_skill_progression_is_empty(&request.required_skill_thresholds)
                && work_history_progression_is_empty(&request.required_work_history_thresholds)
                && request.required_species_ids.is_empty()
                && request.minimum_quality_rank <= 1
            {
                return Err(format!(
                    "contract '{}' must define at least one real requirement.",
                    request.id
                ));
            }
        }

        if self.story_events.first_client_skill_gains.hospitality == 0
            && self.story_events.first_client_skill_gains.scouting == 0
            && self.story_events.first_client_skill_gains.charm == 0
        {
            return Err(
                "story_events.json must define meaningful first_client_skill_gains.".to_owned(),
            );
        }

        if self
            .story_events
            .first_client_work_history_gains
            .hospitality_jobs
            == 0
        {
            return Err(
                "story_events.json must define first_client_work_history_gains.hospitality_jobs."
                    .to_owned(),
            );
        }
        validate_story_event_text_fields(&self.story_events)?;
        Ok(())
    }

    pub(super) fn validate_monster_names(&self, ids: &IdIndex<'_>) -> Result<(), String> {
        let IdIndex { species_ids, .. } = ids;
        for name_pool in &self.monster_names.name_pools {
            validate_reference_list(
                &name_pool.species_ids,
                species_ids,
                &format!("monster name pool '{}'.species_ids", name_pool.id),
            )?;
            if name_pool.names.is_empty() {
                return Err(format!(
                    "monster name pool '{}' must contain at least one name.",
                    name_pool.id
                ));
            }
            for name in &name_pool.names {
                if name.trim().is_empty() {
                    return Err(format!(
                        "monster name pool '{}' contains a blank name.",
                        name_pool.id
                    ));
                }
            }
        }
        Ok(())
    }
}
