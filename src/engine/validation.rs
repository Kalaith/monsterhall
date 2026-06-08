//! Validation helpers for runtime state loaded from save data.

use std::collections::HashSet;

use crate::data::GameData;
use crate::state::{CompanionJobState, EggState, GameState};

pub fn validate_game_state_references(
    data: &GameData,
    game_state: &GameState,
) -> Result<(), String> {
    let building_ids = collect_ids(
        data.buildings
            .buildings
            .iter()
            .map(|entry| entry.id.as_str()),
    );
    let room_ids = collect_ids(data.guild_rooms.rooms.iter().map(|entry| entry.id.as_str()));
    let floor_ids = collect_ids(data.floors.floors.iter().map(|entry| entry.id.as_str()));
    let species_ids = collect_ids(data.species.species.iter().map(|entry| entry.id.as_str()));
    let mission_ids = collect_ids(data.missions.missions.iter().map(|entry| entry.id.as_str()));
    let trait_ids = collect_ids(data.traits.traits.iter().map(|entry| entry.id.as_str()));
    let client_tier_ids = collect_ids(
        data.patron_tiers
            .patron_tiers
            .iter()
            .map(|entry| entry.id.as_str()),
    );
    let event_ids = collect_ids(data.events.events.iter().map(|entry| entry.id.as_str()));
    let monster_ids = collect_ids(game_state.monsters.iter().map(|entry| entry.id.as_str()));

    validate_reference_list(
        &game_state.town.constructed_building_ids,
        &building_ids,
        "save town.constructed_building_ids",
    )?;
    validate_reference_list(
        &game_state.town.unlocked_room_ids,
        &room_ids,
        "save town.unlocked_room_ids",
    )?;
    validate_reference_list(
        &game_state.town.unlocked_floor_ids,
        &floor_ids,
        "save town.unlocked_floor_ids",
    )?;
    validate_reference_list(
        &game_state.town.unlocked_species_ids,
        &species_ids,
        "save town.unlocked_species_ids",
    )?;
    validate_reference_list(
        &game_state.town.patron_tiers,
        &client_tier_ids,
        "save town.patron_tiers",
    )?;

    for situation in &game_state.town.active_situations {
        if !event_ids.contains(situation.event_id.as_str()) {
            return Err(format!(
                "active situation '{}' references unknown event '{}'.",
                situation.label, situation.event_id
            ));
        }
    }

    for monster in &game_state.monsters {
        if !species_ids.contains(monster.species_id.as_str()) {
            return Err(format!(
                "monster '{}' references unknown species '{}'.",
                monster.name, monster.species_id
            ));
        }
        validate_reference_list(
            &monster.trait_ids,
            &trait_ids,
            &format!("save monster '{}'.trait_ids", monster.id),
        )?;

        match &monster.current_job {
            CompanionJobState::GuildJob { room_id } => {
                if !room_ids.contains(room_id.as_str()) {
                    return Err(format!(
                        "monster '{}' is assigned to unknown room '{}'.",
                        monster.name, room_id
                    ));
                }
            }
            CompanionJobState::OnExpedition { expedition_id: _ } => {
                if game_state.active_expedition.is_none() {
                    return Err(format!(
                        "monster '{}' is marked on expedition but no active expedition exists.",
                        monster.name
                    ));
                }
            }
            CompanionJobState::Idle | CompanionJobState::Resting => {}
        }
    }

    for egg in &game_state.egg_inventory {
        validate_egg_references(&species_ids, &floor_ids, egg)?;
    }

    for request in &game_state.active_contracts {
        if !room_ids.contains(request.requested_room_id.as_str()) {
            return Err(format!(
                "contract '{}' references unknown room '{}'.",
                request.request_id, request.requested_room_id
            ));
        }
        validate_reference_list(
            &request.required_species_ids,
            &species_ids,
            &format!(
                "save contract '{}'.required_species_ids",
                request.request_id
            ),
        )?;
        if let Some(monster_id) = &request.assigned_monster_id {
            if !monster_ids.contains(monster_id.as_str()) {
                return Err(format!(
                    "contract '{}' references missing assigned monster '{}'.",
                    request.request_id, monster_id
                ));
            }
        }
    }

    if let Some(expedition) = &game_state.active_expedition {
        if !floor_ids.contains(expedition.floor_id.as_str()) {
            return Err(format!(
                "active expedition references unknown floor '{}'.",
                expedition.floor_id
            ));
        }
        if !mission_ids.contains(expedition.mission_id.as_str()) {
            return Err(format!(
                "active expedition references unknown mission '{}'.",
                expedition.mission_id
            ));
        }
        validate_reference_list(
            &expedition.assigned_monster_ids,
            &monster_ids,
            "save active_expedition.assigned_monster_ids",
        )?;
    }

    Ok(())
}

fn validate_egg_references(
    species_ids: &HashSet<&str>,
    floor_ids: &HashSet<&str>,
    egg: &EggState,
) -> Result<(), String> {
    if egg.source_floor_id != "tower_core" && !floor_ids.contains(egg.source_floor_id.as_str()) {
        return Err(format!(
            "egg '{}' references unknown source floor '{}'.",
            egg.id, egg.source_floor_id
        ));
    }

    validate_reference_list(
        &egg.possible_species_ids,
        species_ids,
        &format!("save egg '{}'.possible_species_ids", egg.id),
    )?;

    if let Some(selected_species_id) = &egg.selected_species_id {
        if !species_ids.contains(selected_species_id.as_str()) {
            return Err(format!(
                "egg '{}' references unknown selected species '{}'.",
                egg.id, selected_species_id
            ));
        }
        if !egg
            .possible_species_ids
            .iter()
            .any(|entry| entry == selected_species_id)
        {
            return Err(format!(
                "egg '{}' selected species '{}' is not in possible_species_ids.",
                egg.id, selected_species_id
            ));
        }
    }

    Ok(())
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

#[cfg(test)]
#[path = "validation_tests/mod.rs"]
mod validation_tests;
