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
            // `preferred_room_ids` is the decorative half of this relation —
            // only the room's `preferred_species_ids` earns
            // `preferred_species_bonus_pct`. Six species used to claim an
            // affinity the room did not grant, which reads as a working bonus
            // and is not one. Either side may be edited; they just have to
            // agree, so the mismatch is caught at load instead of never.
            for room_id in &species.preferred_room_ids {
                let Some(room) = self
                    .guild_rooms
                    .rooms
                    .iter()
                    .find(|room| &room.id == room_id)
                else {
                    continue;
                };
                if !room.preferred_species_ids.contains(&species.id) {
                    return Err(format!(
                        "species '{}' prefers room '{}', but that room does not list it in preferred_species_ids - only the room side grants the bonus.",
                        species.id, room_id
                    ));
                }
            }
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
            if mutation.source_species_id == mutation.target_species_id {
                return Err(format!(
                    "mutation '{}' turns '{}' into itself.",
                    mutation.id, mutation.source_species_id
                ));
            }
        }
        self.validate_mutation_chains()?;
        let reachable = self.reachable_trait_states();
        self.validate_mutation_traits_are_reachable(&reachable)?;
        self.validate_every_trait_is_reachable(&reachable)
    }

    /// Every `(species, traits)` a companion can ever stand in.
    ///
    /// `try_apply_mutation` adds the target species' `starting_traits` *and* the
    /// mutation's `granted_trait_ids` on top of what she already carries, so
    /// traits accumulate along a lineage and a companion's set depends on the
    /// whole route that brought her there, not on her current species.
    fn reachable_trait_states(&self) -> Vec<(String, Vec<String>)> {
        let mut reachable: Vec<(String, Vec<String>)> = self
            .species
            .species
            .iter()
            .map(|species| (species.id.clone(), sorted(&species.starting_traits)))
            .collect();

        // Each mutation adds at most one new state per existing state, and the
        // graph is a strictly ascending tree (checked above), so this settles.
        let mut index = 0;
        while index < reachable.len() {
            let (species_id, traits) = reachable[index].clone();
            index += 1;
            for mutation in &self.mutations.mutations {
                if mutation.source_species_id != species_id
                    || !mutation
                        .required_trait_ids
                        .iter()
                        .all(|trait_id| traits.contains(trait_id))
                {
                    continue;
                }
                let Some(target) = self
                    .species
                    .species
                    .iter()
                    .find(|species| species.id == mutation.target_species_id)
                else {
                    continue;
                };
                let mut next = traits.clone();
                next.extend(target.starting_traits.iter().cloned());
                next.extend(mutation.granted_trait_ids.iter().cloned());
                next.sort();
                next.dedup();
                let state = (target.id.clone(), next);
                if !reachable.contains(&state) {
                    reachable.push(state);
                }
            }
        }

        reachable
    }

    /// A trait no companion can ever hold is content that cannot fire.
    ///
    /// A trait is only ever handed out by a species' `starting_traits` or a
    /// mutation's `granted_trait_ids`, and every mutation here grants the trait
    /// its own target species already starts with — so a trait belongs to a
    /// species or it belongs to nobody. `calming_presence` belonged to nobody
    /// for the game's whole life while five pieces of content paid for it: two
    /// guild rooms listed it as preferred, three guest contracts wanted it, and
    /// `contract_depth_score` read it as a companion who settles a room. All of
    /// that was written, priced, and unreachable, and nothing said so because
    /// every consumer of a trait id checks it against `traits.json`, which had
    /// the trait — the question none of them asked is whether anyone can hold
    /// it.
    fn validate_every_trait_is_reachable(
        &self,
        reachable: &[(String, Vec<String>)],
    ) -> Result<(), String> {
        for trait_data in &self.traits.traits {
            if !reachable
                .iter()
                .any(|(_, traits)| traits.contains(&trait_data.id))
            {
                return Err(format!(
                    "trait '{}' is authored but no species starts with it and no mutation grants it, so no companion can ever hold it.",
                    trait_data.id
                ));
            }
        }
        Ok(())
    }

    /// A mutation may only require traits a companion standing at its source
    /// species could actually be holding.
    ///
    /// Because traits accumulate along a lineage, the requirement is easy to
    /// author against the
    /// wrong set: `golemkin_warden -> gargoyle_stairwarden` needs `commanding`
    /// and `resilient`, and of the two ways to become a golemkin only the
    /// `minotaur_porter` route supplies `resilient` — the common
    /// slime/residue route never does. The mutation is one edit away from being
    /// unreachable with nothing to say so, which is how
    /// `corekeeper_sending_vigil` shipped permanently unfulfillable.
    ///
    /// Checks every mutation's requirement against the lineage states in
    /// [`Self::reachable_trait_states`].
    fn validate_mutation_traits_are_reachable(
        &self,
        reachable: &[(String, Vec<String>)],
    ) -> Result<(), String> {
        for mutation in &self.mutations.mutations {
            let satisfiable = reachable.iter().any(|(species_id, traits)| {
                species_id == &mutation.source_species_id
                    && mutation
                        .required_trait_ids
                        .iter()
                        .all(|trait_id| traits.contains(trait_id))
            });
            if !satisfiable {
                return Err(format!(
                    "mutation '{}' requires {:?}, which no companion who has become '{}' can ever hold.",
                    mutation.id, mutation.required_trait_ids, mutation.source_species_id
                ));
            }
        }
        Ok(())
    }

    /// Corruption only ever climbs, and `try_apply_mutation` runs every day, so
    /// the mutation graph has to be a strictly ascending tree.
    ///
    /// A cycle would flip a companion between two species forever. A step that
    /// costs no more corruption than the step feeding it fires on the same day
    /// as its own source, so the species in between never exists in play — the
    /// chain reads as three forms in the data and shows the player one.
    fn validate_mutation_chains(&self) -> Result<(), String> {
        for mutation in &self.mutations.mutations {
            for feeder in &self.mutations.mutations {
                if feeder.target_species_id != mutation.source_species_id {
                    continue;
                }
                if feeder.source_species_id == mutation.target_species_id {
                    return Err(format!(
                        "mutations '{}' and '{}' form a cycle between '{}' and '{}'.",
                        feeder.id, mutation.id, feeder.source_species_id, feeder.target_species_id
                    ));
                }
                if mutation.minimum_corruption <= feeder.minimum_corruption {
                    return Err(format!(
                        "mutation '{}' needs more corruption than '{}' that feeds it, or '{}' never exists in play.",
                        mutation.id, feeder.id, mutation.source_species_id
                    ));
                }
            }
        }
        Ok(())
    }
}

fn sorted(traits: &[String]) -> Vec<String> {
    let mut sorted = traits.to_vec();
    sorted.sort();
    sorted.dedup();
    sorted
}

#[cfg(test)]
mod tests {
    use crate::data::test_game_data;

    /// `calming_presence` was authored with a description, stat block and icon,
    /// listed as preferred by `common_room` and `nursery_wing`, wanted by three
    /// guest contracts, and read by `contract_depth_score` — and no species
    /// started with it and no mutation granted it, so no companion ever had it.
    #[test]
    fn every_authored_trait_is_one_some_companion_can_hold() {
        let data = test_game_data();
        let reachable = data.reachable_trait_states();

        for trait_data in &data.traits.traits {
            assert!(
                reachable
                    .iter()
                    .any(|(_, traits)| traits.contains(&trait_data.id)),
                "trait '{}' is authored but unreachable: no species starts with it and no mutation grants it.",
                trait_data.id
            );
        }
    }

    /// The trait a room or contract prefers is the one it pays a bonus for, so
    /// a preference naming a trait nobody holds is a bonus that never lands.
    #[test]
    fn every_preferred_trait_is_one_some_companion_can_hold() {
        let data = test_game_data();
        let reachable = data.reachable_trait_states();
        let holdable = |trait_id: &String| {
            reachable
                .iter()
                .any(|(_, traits)| traits.contains(trait_id))
        };

        for room in &data.guild_rooms.rooms {
            for trait_id in &room.preferred_trait_ids {
                assert!(
                    holdable(trait_id),
                    "room '{}' prefers '{trait_id}', which no companion can hold.",
                    room.id
                );
            }
        }

        for request in &data.contracts.requests {
            for trait_id in &request.preferred_trait_ids {
                assert!(
                    holdable(trait_id),
                    "contract '{}' prefers '{trait_id}', which no companion can hold.",
                    request.id
                );
            }
        }
    }
}
