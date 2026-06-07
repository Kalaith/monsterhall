use super::*;

pub fn assign_monster_to_room(
    game_state: &mut GameState,
    monster_id: &str,
    room_id: &str,
) -> Result<(), String> {
    let worker_count = game_state
        .monsters
        .iter()
        .filter(|monster| matches!(monster.current_job, CompanionJobState::GuildJob { .. }))
        .count();

    let current_job = game_state
        .monsters
        .iter()
        .find(|monster| monster.id == monster_id)
        .map(|monster| monster.current_job.clone())
        .ok_or_else(|| format!("Unknown monster id '{monster_id}'."))?;

    let was_already_worker = matches!(current_job, CompanionJobState::GuildJob { .. });
    if !was_already_worker && worker_count >= usize::from(game_state.town.town_job_limit) {
        return Err("No more guild job slots are available.".to_owned());
    }

    let monster = find_monster_mut(game_state, monster_id)?;
    monster.current_job = CompanionJobState::GuildJob {
        room_id: room_id.to_owned(),
    };
    remove_monster_from_expedition(game_state, monster_id);
    Ok(())
}

pub fn assign_monster_to_rest(game_state: &mut GameState, monster_id: &str) -> Result<(), String> {
    let monster = find_monster_mut(game_state, monster_id)?;
    monster.current_job = CompanionJobState::Resting;
    remove_monster_from_expedition(game_state, monster_id);
    Ok(())
}

pub fn assign_monster_to_idle(game_state: &mut GameState, monster_id: &str) -> Result<(), String> {
    let monster = find_monster_mut(game_state, monster_id)?;
    monster.current_job = CompanionJobState::Idle;
    remove_monster_from_expedition(game_state, monster_id);
    Ok(())
}

pub fn assign_monster_to_expedition(
    data: &GameData,
    game_state: &mut GameState,
    monster_id: &str,
    floor_id: &str,
) -> Result<(), String> {
    let floor = data
        .floors
        .floors
        .iter()
        .find(|floor| floor.id == floor_id)
        .ok_or_else(|| format!("Unknown floor id '{floor_id}'."))?;
    floor_roster_gate_report(data, game_state, floor)?;

    let active_count = game_state
        .active_expedition
        .as_ref()
        .map(|expedition| expedition.assigned_monster_ids.len())
        .unwrap_or(0);

    let current_job = game_state
        .monsters
        .iter()
        .find(|monster| monster.id == monster_id)
        .map(|monster| monster.current_job.clone())
        .ok_or_else(|| format!("Unknown monster id '{monster_id}'."))?;

    let was_already_assigned = matches!(current_job, CompanionJobState::OnExpedition { .. });
    if !was_already_assigned && active_count >= usize::from(game_state.town.party_size) {
        return Err("No more expedition slots are available.".to_owned());
    }

    ensure_active_expedition(game_state, floor_id);

    let monster = find_monster_mut(game_state, monster_id)?;
    monster.current_job = CompanionJobState::OnExpedition {
        expedition_id: "expedition_001".to_owned(),
    };

    if let Some(expedition) = &mut game_state.active_expedition {
        if !expedition
            .assigned_monster_ids
            .iter()
            .any(|id| id == monster_id)
        {
            expedition.assigned_monster_ids.push(monster_id.to_owned());
        }
        expedition.floor_id = floor_id.to_owned();
    }

    Ok(())
}

pub fn configure_expedition_plan(
    game_state: &mut GameState,
    floor_id: &str,
    mission_id: &str,
    priority: ExpeditionPriority,
) {
    ensure_active_expedition(game_state, floor_id);
    if let Some(expedition) = &mut game_state.active_expedition {
        expedition.floor_id = floor_id.to_owned();
        expedition.mission_id = mission_id.to_owned();
        expedition.priority = priority;
    }
}

pub fn purchase_building(
    data: &GameData,
    game_state: &mut GameState,
    building_id: &str,
) -> Result<String, String> {
    let building = data
        .buildings
        .buildings
        .iter()
        .find(|entry| entry.id == building_id)
        .ok_or_else(|| format!("Unknown building id '{building_id}'."))?;

    if game_state
        .town
        .constructed_building_ids
        .iter()
        .filter(|id| *id == building_id)
        .count()
        >= usize::from(building.build_limit)
    {
        return Err(format!("{} is already at its build limit.", building.name));
    }

    if !can_afford(&game_state.resources, &building.cost) {
        return Err(format!("Not enough resources to build {}.", building.name));
    }

    spend_resources(&mut game_state.resources, &building.cost);
    game_state
        .town
        .constructed_building_ids
        .push(building_id.to_owned());

    unlock_building_content(&mut game_state.town, building);
    complete_town_project_if_needed(data, game_state, building_id);
    game_state
        .event_log
        .push(format!("Constructed {}", building.name));

    Ok(format!("{} is now part of the keep.", building.name))
}

pub fn hatch_species(
    data: &GameData,
    game_state: &mut GameState,
    species_id: &str,
) -> Result<String, String> {
    let species = data
        .species
        .species
        .iter()
        .find(|entry| entry.id == species_id)
        .ok_or_else(|| format!("Unknown species id '{species_id}'."))?;

    if !game_state
        .town
        .unlocked_species_ids
        .iter()
        .any(|id| id == species_id)
    {
        return Err(format!("{} is not unlocked yet.", species.name));
    }

    let egg_index = if let Some(egg_index) = find_ready_egg_index(game_state, species_id) {
        egg_index
    } else if let Some(egg_index) = find_raw_egg_index(game_state, species_id) {
        prepare_raw_egg_for_species(game_state, egg_index, species_id)?;
        egg_index
    } else {
        return Err(format!(
            "No hatchery egg is available to hatch {}.",
            species.name
        ));
    };
    hatch_species_from_ready_egg(data, game_state, species, egg_index, None)
}

pub fn hatch_selected_egg(
    data: &GameData,
    game_state: &mut GameState,
    egg_id: &str,
    species_id_override: Option<&str>,
) -> Result<String, String> {
    let egg_index = game_state
        .egg_inventory
        .iter()
        .position(|egg| egg.id == egg_id)
        .ok_or_else(|| format!("Unknown egg id '{egg_id}'."))?;
    if game_state.egg_inventory[egg_index].incubation_state == EggIncubationState::Raw {
        let species_id = species_id_override.ok_or_else(|| {
            "A raw egg needs a hatchery outcome selected before it can hatch.".to_owned()
        })?;
        prepare_raw_egg_for_species(game_state, egg_index, species_id)?;
    }
    let egg = &game_state.egg_inventory[egg_index];
    let species_id = egg
        .selected_species_id
        .clone()
        .ok_or_else(|| "This egg has not been prepared for a species outcome.".to_owned())?;
    let species = data
        .species
        .species
        .iter()
        .find(|entry| entry.id == species_id)
        .ok_or_else(|| format!("Unknown species id '{}'.", species_id))?;
    hatch_species_from_ready_egg(data, game_state, species, egg_index, None)
}

pub fn replace_monster_with_selected_egg(
    data: &GameData,
    game_state: &mut GameState,
    egg_id: &str,
    species_id_override: Option<&str>,
    replacement_monster_id: &str,
) -> Result<String, String> {
    let egg_index = game_state
        .egg_inventory
        .iter()
        .position(|egg| egg.id == egg_id)
        .ok_or_else(|| format!("Unknown egg id '{egg_id}'."))?;
    if game_state.egg_inventory[egg_index].incubation_state == EggIncubationState::Raw {
        let species_id = species_id_override.ok_or_else(|| {
            "A raw egg needs a hatchery outcome selected before it can replace a roster slot."
                .to_owned()
        })?;
        prepare_raw_egg_for_species(game_state, egg_index, species_id)?;
    }
    let egg = &game_state.egg_inventory[egg_index];
    let species_id = egg
        .selected_species_id
        .clone()
        .ok_or_else(|| "This egg has not been prepared for a species outcome.".to_owned())?;
    let species = data
        .species
        .species
        .iter()
        .find(|entry| entry.id == species_id)
        .ok_or_else(|| format!("Unknown species id '{}'.", species_id))?;
    hatch_species_from_ready_egg(
        data,
        game_state,
        species,
        egg_index,
        Some(replacement_monster_id),
    )
}

pub fn convert_egg(
    _data: &GameData,
    game_state: &mut GameState,
    egg_id: &str,
    conversion: EggConversionKind,
) -> Result<String, String> {
    let egg_index = game_state
        .egg_inventory
        .iter()
        .position(|egg| egg.id == egg_id)
        .ok_or_else(|| format!("Unknown egg id '{egg_id}'."))?;
    let egg = game_state.egg_inventory[egg_index].clone();
    let quality_rank = egg_quality_rank(egg.grade_score);

    match conversion {
        EggConversionKind::Sell => {
            let gold = match quality_rank {
                1 => 10,
                2 => 20,
                _ => 50,
            };
            game_state.egg_inventory.remove(egg_index);
            game_state.resources.gold = game_state.resources.gold.saturating_add(gold);
            sync_egg_resource_count(game_state);
            let message = format!("Sold {} for {} gold.", egg.id, gold);
            game_state.event_log.push(message.clone());
            Ok(message)
        }
        EggConversionKind::Dissolve => {
            let residue = match quality_rank {
                1 => 8,
                2 => 18,
                _ => 35,
            };
            let relics = u32::from(quality_rank >= 3);
            game_state.egg_inventory.remove(egg_index);
            game_state.resources.arcane_residue =
                game_state.resources.arcane_residue.saturating_add(residue);
            game_state.resources.relics = game_state.resources.relics.saturating_add(relics);
            sync_egg_resource_count(game_state);
            let message = if relics > 0 {
                format!(
                    "Dissolved {} into {} arcane residue and {} relic.",
                    egg.id, residue, relics
                )
            } else {
                format!("Dissolved {} into {} arcane residue.", egg.id, residue)
            };
            game_state.event_log.push(message.clone());
            Ok(message)
        }
        EggConversionKind::Refine => {
            if quality_rank >= 3 {
                return Err("This egg is already at the current quality ceiling.".to_owned());
            }
            let Some(other_index) = game_state
                .egg_inventory
                .iter()
                .enumerate()
                .find(|(index, candidate)| {
                    *index != egg_index && egg_quality_rank(candidate.grade_score) == quality_rank
                })
                .map(|(index, _)| index)
            else {
                return Err("Refining needs a second egg of the same star quality.".to_owned());
            };
            let other = game_state.egg_inventory[other_index].clone();
            let mut possible_species_ids = egg.possible_species_ids.clone();
            for species_id in &other.possible_species_ids {
                if !possible_species_ids.contains(species_id) {
                    possible_species_ids.push(species_id.clone());
                }
            }
            let refined_grade_score = if quality_rank == 1 { 3 } else { 5 };
            for index in [egg_index, other_index].into_iter().max().into_iter() {
                game_state.egg_inventory.remove(index);
            }
            for index in [egg_index, other_index].into_iter().min().into_iter() {
                game_state.egg_inventory.remove(index);
            }
            let refined_id = next_egg_id(game_state);
            game_state.egg_inventory.push(EggState {
                id: refined_id.clone(),
                source_floor_id: egg.source_floor_id.clone(),
                possible_species_ids,
                selected_species_id: None,
                incubation_state: EggIncubationState::Raw,
                grade_score: refined_grade_score,
                preparation_focus: Some("refined_lineage".to_owned()),
                loyalty_imprinted: false,
                secrecy_locked: true,
            });
            sync_egg_resource_count(game_state);
            let message = format!("Refined {} and {} into {}.", egg.id, other.id, refined_id);
            game_state.event_log.push(message.clone());
            Ok(message)
        }
    }
}

pub(super) fn hatch_species_from_ready_egg(
    data: &GameData,
    game_state: &mut GameState,
    species: &crate::data::SpeciesData,
    egg_index: usize,
    replacement_monster_id: Option<&str>,
) -> Result<String, String> {
    let population_cap = effective_population_cap(data, game_state);
    let at_population_cap = game_state.monsters.len() >= population_cap;
    if at_population_cap && replacement_monster_id.is_none() {
        return Err(format!(
            "The guild is at its population cap of {population_cap}. Release a companion before hatching another."
        ));
    }
    if !at_population_cap && replacement_monster_id.is_some() {
        return Err(
            "Replacement hatching is only available once the guild is at capacity.".to_owned(),
        );
    }
    if let Some(monster_id) = replacement_monster_id {
        if game_state.monsters.len() <= 1 {
            return Err("The guild cannot replace its last companion.".to_owned());
        }
        if !game_state
            .monsters
            .iter()
            .any(|monster| monster.id == monster_id)
        {
            return Err(format!("Unknown replacement monster id '{monster_id}'."));
        }
    }

    let hatch_cost = ResourceAmountData {
        gold: species.hatching_cost.gold,
        tower_materials: species.hatching_cost.tower_materials,
        eggs: 0,
        relics: species.hatching_cost.relics,
        arcane_residue: species.hatching_cost.arcane_residue,
    };
    if !can_afford(&game_state.resources, &hatch_cost) {
        return Err(format!(
            "Not enough resources to finish hatching {}.",
            species.name
        ));
    }

    let monster_id = next_monster_id(game_state);
    let monster_name = pick_hatched_monster_name(data, game_state, &species.id)?;
    let egg_grade_score = game_state.egg_inventory[egg_index].grade_score;
    let quality_rank = egg_quality_rank(egg_grade_score);
    let replaced_monster = replacement_monster_id
        .map(|monster_id| remove_monster_for_roster_change(game_state, monster_id))
        .transpose()?;
    spend_resources(&mut game_state.resources, &hatch_cost);
    game_state.egg_inventory.remove(egg_index);
    sync_egg_resource_count(game_state);
    if !game_state
        .story_progress
        .hatched_species_ids
        .iter()
        .any(|id| id == &species.id)
    {
        game_state
            .story_progress
            .hatched_species_ids
            .push(species.id.clone());
    }
    let mut stats = species.base_stats.clone();
    if egg_grade_score >= 2 {
        stats.charm += 1;
        stats.instinct += 1;
    }
    if egg_grade_score >= 3 {
        stats.power += 1;
        stats.endurance += 1;
    }

    game_state.monsters.push(CompanionState {
        id: monster_id,
        species_id: species.id.clone(),
        name: monster_name.clone(),
        quality_rank,
        stats,
        trait_ids: species.starting_traits.clone(),
        current_job: CompanionJobState::Idle,
        skills: CompanionSkillState::default(),
        work_history: CompanionWorkHistoryState::default(),
        fatigue: 0,
        stress: 0,
        injury: 0,
        corruption: 0,
        bond: 1 + egg_grade_score,
        reputation: egg_grade_score as i32,
    });
    game_state
        .event_log
        .push(format!("The hatchery produced {monster_name}."));
    if let Some(replaced_monster) = replaced_monster {
        game_state.event_log.push(format!(
            "{} replaced {} in the roster.",
            monster_name, replaced_monster.name
        ));
    }

    Ok(format!("{monster_name} joined Monsterhall."))
}

pub fn release_monster(game_state: &mut GameState, monster_id: &str) -> Result<String, String> {
    let monster = remove_monster_for_roster_change(game_state, monster_id)?;

    let message = format!("{} left Monsterhall.", monster.name);
    game_state.event_log.push(message.clone());
    Ok(message)
}

fn remove_monster_for_roster_change(
    game_state: &mut GameState,
    monster_id: &str,
) -> Result<CompanionState, String> {
    if game_state.monsters.len() <= 1 {
        return Err("The guild cannot release its last companion.".to_owned());
    }

    let monster_index = game_state
        .monsters
        .iter()
        .position(|monster| monster.id == monster_id)
        .ok_or_else(|| format!("Unknown monster id '{monster_id}'."))?;
    let monster = game_state.monsters.remove(monster_index);

    if let Some(expedition) = &mut game_state.active_expedition {
        expedition
            .assigned_monster_ids
            .retain(|assigned_id| assigned_id != monster_id);
        if expedition.assigned_monster_ids.is_empty() {
            game_state.active_expedition = None;
        }
    }

    for request in &mut game_state.active_contracts {
        if request.assigned_monster_id.as_deref() == Some(monster_id) {
            request.assigned_monster_id = None;
            request.status = crate::state::ContractStatus::Pending;
        }
    }

    Ok(monster)
}
