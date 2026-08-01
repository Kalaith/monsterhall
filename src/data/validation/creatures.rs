//! Species and mutation validation.
use super::IdIndex;
use super::{validate_non_negative_stats, validate_reference_list};
use crate::data::types::*;

impl GameData {
    pub(super) fn validate_species(&self, ids: &IdIndex<'_>) -> Result<(), String> {
        let IdIndex {
            trait_ids,
            room_ids,
            ..
        } = ids;
        for species in &self.species.species {
            validate_non_negative_stats(&species.base_stats, &format!("species '{}'", species.id))?;
            validate_reference_list(
                &species.starting_traits,
                trait_ids,
                &format!("species '{}'.starting_traits", species.id),
            )?;
            validate_reference_list(
                &species.preferred_room_ids,
                room_ids,
                &format!("species '{}'.preferred_room_ids", species.id),
            )?;
            if !self
                .monster_names
                .name_pools
                .iter()
                .any(|pool| pool.species_ids.iter().any(|id| id == &species.id))
            {
                return Err(format!(
                    "species '{}' must have at least one monster name pool.",
                    species.id
                ));
            }
            // Both halves of "the player can actually end up with one of
            // these". A species needs an egg somewhere in the tower to hatch
            // from, and something that unlocks the right to hatch it; miss
            // either and the entry is prose nobody will ever read in play.
            if !self.floors.floors.iter().any(|floor| {
                floor
                    .egg_species_entries
                    .iter()
                    .any(|entry| entry.species_id == species.id)
            }) {
                return Err(format!(
                    "species '{}' is on no floor's egg_species_entries, so no egg can ever produce it.",
                    species.id
                ));
            }
            let starts_unlocked = self
                .config
                .new_game
                .starting_species_ids
                .iter()
                .any(|id| id == &species.id)
                || self
                    .config
                    .new_game
                    .starter_monsters
                    .iter()
                    .any(|starter| starter.species_id == species.id);
            let building_unlocks = self
                .buildings
                .buildings
                .iter()
                .any(|building| building.unlocks.species_ids.contains(&species.id));
            if !starts_unlocked && !building_unlocks {
                return Err(format!(
                    "species '{}' is unlocked by no building and is not a starting species, so it can never be hatched.",
                    species.id
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_mutations(&self, ids: &IdIndex<'_>) -> Result<(), String> {
        let IdIndex {
            trait_ids,
            species_ids,
            ..
        } = ids;
        for mutation in &self.mutations.mutations {
            if !species_ids.contains(mutation.source_species_id.as_str()) {
                return Err(format!(
                    "mutation '{}' references unknown source species '{}'.",
                    mutation.id, mutation.source_species_id
                ));
            }
            if !species_ids.contains(mutation.target_species_id.as_str()) {
                return Err(format!(
                    "mutation '{}' references unknown target species '{}'.",
                    mutation.id, mutation.target_species_id
                ));
            }
            validate_reference_list(
                &mutation.required_trait_ids,
                trait_ids,
                &format!("mutation '{}'.required_trait_ids", mutation.id),
            )?;
            validate_reference_list(
                &mutation.granted_trait_ids,
                trait_ids,
                &format!("mutation '{}'.granted_trait_ids", mutation.id),
            )?;
            if mutation.event_text.trim().is_empty() {
                return Err(format!(
                    "mutation '{}' must contain event_text.",
                    mutation.id
                ));
            }
        }
        Ok(())
    }
}
