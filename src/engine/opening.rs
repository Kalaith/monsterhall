//! Scripted opening chapter logic.

use crate::data::GameData;
use crate::state::{GameState, CompanionJobState, OpeningChapterStep};

use super::{create_opening_egg, hatch_species};

pub fn advance_opening_step(data: &GameData, game_state: &mut GameState) -> Result<(), String> {
    match game_state.story_progress.opening_step {
        OpeningChapterStep::Camp => {
            game_state.story_progress.tower_hole_discovered = true;
            game_state.story_progress.opening_step = OpeningChapterStep::Discovery;
            game_state
                .event_log
                .push(data.story_events.camp_discovery_log.clone());
            Ok(())
        }
        OpeningChapterStep::Discovery => {
            game_state.story_progress.first_egg_created = true;
            create_opening_egg(game_state, "slime_girl");
            game_state.story_progress.opening_step = OpeningChapterStep::Incubation;
            game_state
                .event_log
                .push(data.story_events.discovery_incubation_log.clone());
            Ok(())
        }
        OpeningChapterStep::Incubation => {
            game_state.story_progress.opening_step = OpeningChapterStep::Hatch;
            Ok(())
        }
        OpeningChapterStep::Hatch => {
            hatch_first_slimegirl(data, game_state)?;
            game_state.story_progress.opening_step = OpeningChapterStep::BuildRoom;
            game_state
                .event_log
                .push(data.story_events.hatch_loyalty_log.clone());
            Ok(())
        }
        OpeningChapterStep::FirstClient => resolve_first_client(data, game_state),
        OpeningChapterStep::BuildRoom | OpeningChapterStep::Complete => Ok(()),
    }
}

pub fn build_first_room(data: &GameData, game_state: &mut GameState) -> Result<(), String> {
    let cost = &data.story_events.first_room_cost;
    if game_state.resources.gold < cost.gold
        || game_state.resources.tower_materials < cost.tower_materials
        || game_state.resources.arcane_residue < cost.arcane_residue
    {
        return Err(data
            .story_events
            .build_first_room_not_enough_resources_error
            .clone());
    }

    game_state.resources.gold -= cost.gold;
    game_state.resources.tower_materials -= cost.tower_materials;
    game_state.resources.arcane_residue -= cost.arcane_residue;

    if !game_state
        .town
        .unlocked_room_ids
        .contains(&"vanilla_suite".to_owned())
    {
        game_state
            .town
            .unlocked_room_ids
            .push("vanilla_suite".to_owned());
    }

    game_state.story_progress.first_room_built = true;
    game_state.story_progress.opening_step = OpeningChapterStep::FirstClient;
    game_state
        .event_log
        .push(data.story_events.build_first_room_completion_log.clone());

    Ok(())
}

pub fn resolve_first_client(data: &GameData, game_state: &mut GameState) -> Result<(), String> {
    let first_girl = game_state
        .monsters
        .first_mut()
        .ok_or_else(|| data.story_events.first_client_missing_monster_error.clone())?;
    let reward = &data.story_events.first_client_reward;

    game_state.resources.gold += reward.gold;
    game_state.resources.tower_materials += reward.tower_materials;
    game_state.resources.arcane_residue += reward.arcane_residue;
    first_girl.skills.scouting += data.story_events.first_client_skill_gains.scouting;
    first_girl.skills.guarding += data.story_events.first_client_skill_gains.guarding;
    first_girl.skills.hospitality += data.story_events.first_client_skill_gains.hospitality;
    first_girl.skills.crafting += data.story_events.first_client_skill_gains.crafting;
    first_girl.skills.charm += data.story_events.first_client_skill_gains.charm;
    first_girl.work_history.scouting_runs += data.story_events.first_client_work_history_gains.scouting_runs;
    first_girl.work_history.guard_duties += data.story_events.first_client_work_history_gains.guard_duties;
    first_girl.work_history.hospitality_jobs +=
        data.story_events.first_client_work_history_gains.hospitality_jobs;
    first_girl.work_history.craft_jobs += data.story_events.first_client_work_history_gains.craft_jobs;
    first_girl.work_history.contracts_completed +=
        data.story_events.first_client_work_history_gains.contracts_completed;
    first_girl.work_history.recovery_shifts += data
        .story_events
        .first_client_work_history_gains
        .recovery_shifts;
    first_girl.work_history.hatchery_assists += data.story_events.first_client_work_history_gains.hatchery_assists;
    first_girl.current_job = CompanionJobState::Idle;

    game_state.story_progress.first_client_completed = true;
    game_state.story_progress.opening_step = OpeningChapterStep::Complete;
    game_state.event_log.push(
        data.story_events
            .first_client_completion_log_template
            .replace("{name}", &first_girl.name),
    );

    Ok(())
}

fn hatch_first_slimegirl(data: &GameData, game_state: &mut GameState) -> Result<(), String> {
    if game_state.story_progress.first_slimegirl_hatched {
        return Ok(());
    }

    hatch_species(data, game_state, "slime_girl")?;
    let first_girl = game_state
        .monsters
        .last_mut()
        .ok_or_else(|| data.story_events.hatch_missing_monster_error.clone())?;
    first_girl.id = "monster_001".to_owned();
    first_girl.name = data.story_events.first_hatched_monster_name.clone();
    game_state.story_progress.first_slimegirl_hatched = true;
    game_state
        .event_log
        .push(data.story_events.first_hatch_log.clone());

    Ok(())
}
