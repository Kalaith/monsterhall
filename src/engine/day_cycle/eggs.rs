use super::*;

pub fn create_opening_egg(game_state: &mut GameState, species_id: &str) {
    let egg_id = next_egg_id(game_state);
    game_state.egg_inventory.push(EggState {
        id: egg_id,
        source_floor_id: "tower_core".to_owned(),
        possible_species_ids: vec![species_id.to_owned()],
        selected_species_id: None,
        incubation_state: EggIncubationState::Raw,
        grade_score: 0,
        preparation_focus: None,
        loyalty_imprinted: false,
        secrecy_locked: true,
    });
    sync_egg_resource_count(game_state);
}

pub fn sync_egg_resource_count(game_state: &mut GameState) {
    game_state.resources.eggs = game_state.egg_inventory.len() as u32;
}

#[cfg(test)]
pub fn raw_egg_count_for_species(game_state: &GameState, species_id: &str) -> usize {
    game_state
        .egg_inventory
        .iter()
        .filter(|egg| {
            egg.incubation_state == EggIncubationState::Raw
                && egg.possible_species_ids.iter().any(|id| id == species_id)
        })
        .count()
}

#[cfg(test)]
pub fn ready_egg_count_for_species(game_state: &GameState, species_id: &str) -> usize {
    game_state
        .egg_inventory
        .iter()
        .filter(|egg| {
            egg.incubation_state == EggIncubationState::ReadyToHatch
                && egg
                    .selected_species_id
                    .as_ref()
                    .is_some_and(|selected_id| selected_id == species_id)
        })
        .count()
}

pub(super) fn add_floor_egg_rewards(
    game_state: &mut GameState,
    floor_id: &str,
    egg_species_entries: &[EggSpeciesEntryData],
    egg_gain: u32,
    grade_score: u32,
) {
    for _ in 0..egg_gain {
        let egg_id = next_egg_id(game_state);
        let selected_species_id = choose_weighted_floor_egg_species(egg_species_entries);
        let mut possible_species_ids = vec![selected_species_id];
        for entry in egg_species_entries {
            if game_state
                .town
                .unlocked_species_ids
                .iter()
                .any(|species_id| species_id == &entry.species_id)
                && !possible_species_ids
                    .iter()
                    .any(|species_id| species_id == &entry.species_id)
            {
                possible_species_ids.push(entry.species_id.clone());
            }
        }
        game_state.egg_inventory.push(EggState {
            id: egg_id,
            source_floor_id: floor_id.to_owned(),
            possible_species_ids,
            selected_species_id: None,
            incubation_state: EggIncubationState::Raw,
            grade_score,
            preparation_focus: None,
            loyalty_imprinted: false,
            secrecy_locked: true,
        });
    }
    sync_egg_resource_count(game_state);
}

pub(super) fn choose_weighted_floor_egg_species(
    egg_species_entries: &[EggSpeciesEntryData],
) -> String {
    let total_weight = egg_species_entries
        .iter()
        .map(|entry| entry.weight)
        .sum::<u32>();

    let mut roll = gen_range(0, total_weight as i32) as u32;
    for entry in egg_species_entries {
        if roll < entry.weight {
            return entry.species_id.clone();
        }
        roll -= entry.weight;
    }

    egg_species_entries
        .last()
        .map(|entry| entry.species_id.clone())
        .unwrap_or_else(|| "slime_girl".to_owned())
}

pub(super) fn prepare_raw_egg_for_species(
    game_state: &mut GameState,
    egg_index: usize,
    species_id: &str,
) -> Result<(), String> {
    let egg = &mut game_state.egg_inventory[egg_index];
    if egg.incubation_state != EggIncubationState::Raw {
        return Ok(());
    }
    if !egg.possible_species_ids.iter().any(|id| id == species_id) {
        return Err(format!("Egg {} cannot hatch into {}.", egg.id, species_id));
    }
    egg.selected_species_id = Some(species_id.to_owned());
    egg.incubation_state = EggIncubationState::ReadyToHatch;
    egg.preparation_focus = Some("lineage_control".to_owned());
    egg.loyalty_imprinted = true;
    egg.secrecy_locked = true;
    Ok(())
}

pub(super) fn find_raw_egg_index(game_state: &GameState, species_id: &str) -> Option<usize> {
    game_state.egg_inventory.iter().position(|egg| {
        egg.incubation_state == EggIncubationState::Raw
            && egg.possible_species_ids.iter().any(|id| id == species_id)
    })
}

pub(super) fn find_ready_egg_index(game_state: &GameState, species_id: &str) -> Option<usize> {
    game_state.egg_inventory.iter().position(|egg| {
        egg.incubation_state == EggIncubationState::ReadyToHatch
            && egg
                .selected_species_id
                .as_ref()
                .is_some_and(|selected_id| selected_id == species_id)
    })
}

pub(super) fn remove_monster_from_expedition(game_state: &mut GameState, monster_id: &str) {
    if let Some(expedition) = &mut game_state.active_expedition {
        expedition
            .assigned_monster_ids
            .retain(|id| id != monster_id);
        if expedition.assigned_monster_ids.is_empty() {
            game_state.active_expedition = None;
        }
    }
}

pub(super) fn pick_hatched_monster_name(
    data: &GameData,
    game_state: &GameState,
    species_id: &str,
) -> Result<String, String> {
    let Some(name_pool) = data
        .monster_names
        .name_pools
        .iter()
        .find(|pool| pool.species_ids.iter().any(|id| id == species_id))
    else {
        return Err(format!(
            "No monster name pool is defined for species '{species_id}'."
        ));
    };

    let used_names = game_state
        .monsters
        .iter()
        .map(|monster| monster.name.as_str())
        .collect::<HashSet<_>>();

    let available_names = name_pool
        .names
        .iter()
        .filter(|name| !used_names.contains(name.as_str()))
        .collect::<Vec<_>>();

    let source_names = if available_names.is_empty() {
        name_pool.names.iter().collect::<Vec<_>>()
    } else {
        available_names
    };

    let index = gen_range(0, source_names.len() as i32) as usize;
    Ok(source_names[index].clone())
}
