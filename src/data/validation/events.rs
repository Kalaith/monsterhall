//! Event and cross-domain reference validation.
use super::validate_reference_list;
use super::IdIndex;
use crate::data::types::*;

impl GameData {
    pub(super) fn validate_events(&self, ids: &IdIndex<'_>) -> Result<(), String> {
        let IdIndex {
            trait_ids,
            building_ids,
            species_ids,
            ..
        } = ids;
        for event in &self.events.events {
            validate_reference_list(
                &event.required_trait_ids,
                trait_ids,
                &format!("event '{}'.required_trait_ids", event.id),
            )?;
            validate_reference_list(
                &event.required_species_ids,
                species_ids,
                &format!("event '{}'.required_species_ids", event.id),
            )?;
            validate_reference_list(
                &event.required_building_ids,
                building_ids,
                &format!("event '{}'.required_building_ids", event.id),
            )?;
            if event.text.trim().is_empty() {
                return Err(format!("event '{}' must contain text.", event.id));
            }
            if event.weight == Some(0) {
                return Err(format!(
                    "event '{}' weight must be greater than zero.",
                    event.id
                ));
            }
            if let Some(trigger_chance_pct) = event.trigger_chance_pct {
                if !(1..=100).contains(&trigger_chance_pct) {
                    return Err(format!(
                        "event '{}' trigger_chance_pct must be between 1 and 100.",
                        event.id
                    ));
                }
            }
            if event.reward.as_ref().is_some_and(|reward| reward.eggs > 0)
                || event.cost.as_ref().is_some_and(|cost| cost.eggs > 0)
            {
                return Err(format!(
                    "event '{}' cannot modify eggs directly; egg rewards must create inventory entries.",
                    event.id
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_references(&self, ids: &IdIndex<'_>) -> Result<(), String> {
        let IdIndex {
            building_ids,
            room_ids,
            floor_ids,
            species_ids,
            ..
        } = ids;
        validate_reference_list(
            &self.config.new_game.starting_building_ids,
            building_ids,
            "config.new_game.starting_building_ids",
        )?;
        validate_reference_list(
            &self.config.new_game.starting_room_ids,
            room_ids,
            "config.new_game.starting_room_ids",
        )?;
        validate_reference_list(
            &self.config.new_game.starting_floor_ids,
            floor_ids,
            "config.new_game.starting_floor_ids",
        )?;
        validate_reference_list(
            &self.config.new_game.starting_species_ids,
            species_ids,
            "config.new_game.starting_species_ids",
        )?;

        Ok(())
    }
}
/// `event_tags` are a taxonomy, not a second gate — the progression they name is
/// delivered by `required_building_ids` and `min_day`. That only stays true if
/// the two agree, so a tag claiming lateness on an event nothing gates is a
/// content mistake that reads as working.
pub(super) fn validate_event_tag_gating(data: &GameData) -> Result<(), String> {
    for event in &data.events.events {
        let claims_progression = event
            .event_tags
            .iter()
            .any(|tag| tag.starts_with("tier_") || tag == "late_game");
        if !claims_progression {
            continue;
        }
        if event.required_building_ids.is_empty() {
            return Err(format!(
                "event '{}' is tagged for a progression tier but requires no building - nothing actually gates it.",
                event.id
            ));
        }
        if event.event_tags.iter().any(|tag| tag == "late_game")
            && event.min_day.is_none_or(|min_day| min_day < 100)
        {
            return Err(format!(
                "event '{}' is tagged late_game but can fire before day 100.",
                event.id
            ));
        }
    }

    Ok(())
}
