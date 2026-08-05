//! Validation-policy daily job and expedition assignment.
use super::policy_growth::{
    can_spare_worker_for_growth, can_survive_debt_after_growth_assignment,
    pending_eggs_cover_workforce_demand, workforce_demand,
};
use super::policy_guests::accepted_guest_monster_ids;
use super::*;

pub(super) fn assign_daily_jobs(data: &GameData, game_state: &mut GameState) -> (usize, usize) {
    let room_id = best_unlocked_room_id(data, game_state);
    let guild_job_limit = usize::from(game_state.town.town_job_limit);
    let reserved_guest_monster_ids = accepted_guest_monster_ids(game_state);
    let mut guild_job_workers = 0usize;
    let mut expedition_members = 0usize;

    let monster_ids = game_state
        .monsters
        .iter()
        .map(|monster| monster.id.clone())
        .collect::<Vec<_>>();

    if let Some((monster_id, floor_id, mission_id)) =
        best_growth_expedition_assignment(data, game_state, &reserved_guest_monster_ids)
    {
        configure_expedition_plan(
            game_state,
            &floor_id,
            &mission_id,
            ExpeditionPriority::Balanced,
        );
        if assign_monster_to_expedition(data, game_state, &monster_id, &floor_id).is_ok() {
            expedition_members += 1;
        }
    }

    for monster_id in monster_ids {
        let Some(monster) = game_state
            .monsters
            .iter()
            .find(|entry| entry.id == monster_id)
        else {
            continue;
        };

        if monster.injury > 0 || monster.fatigue >= 34 || monster.stress >= 20 {
            let _ = assign_monster_to_rest(game_state, &monster_id);
            continue;
        }

        if matches!(
            monster.current_job,
            crate::state::CompanionJobState::OnExpedition { .. }
        ) {
            continue;
        }

        // The reservation this function already computes, finally honoured here
        // too. It was applied to expedition selection and not to guild jobs, so
        // the policy handed contract-booked companions a room shift that
        // `resolve_day` then discarded — burning one of only two guild-job slots
        // on work that never happened. The engine refuses the assignment now, so
        // this is the policy saying why rather than learning it from an error.
        if reserved_guest_monster_ids.contains(&monster_id) {
            continue;
        }

        if guild_job_workers < guild_job_limit {
            if let Some(room_id) = &room_id {
                if assign_monster_to_room(game_state, &monster_id, room_id).is_ok() {
                    guild_job_workers += 1;
                    continue;
                }
            }
        }
    }

    (guild_job_workers, expedition_members)
}

pub(super) fn best_unlocked_room_id(data: &GameData, game_state: &GameState) -> Option<String> {
    data.guild_rooms
        .rooms
        .iter()
        .filter(|room| {
            game_state
                .town
                .unlocked_room_ids
                .iter()
                .any(|room_id| room_id == &room.id)
        })
        .max_by_key(|room| room.base_gold_yield)
        .map(|room| room.id.clone())
}

pub(super) fn best_growth_expedition_assignment(
    data: &GameData,
    game_state: &GameState,
    reserved_guest_monster_ids: &std::collections::HashSet<String>,
) -> Option<(String, String, String)> {
    let mut best_assignment: Option<(String, String, String, i32)> = None;
    let should_reserve_egg_expedition = should_reserve_egg_expedition(game_state);

    for monster in &game_state.monsters {
        if reserved_guest_monster_ids.contains(&monster.id)
            || monster.injury > 0
            || monster.fatigue >= 34
            || monster.stress >= 20
        {
            continue;
        }

        let mut simulated_state = game_state.clone();
        for simulated_monster in &mut simulated_state.monsters {
            simulated_monster.current_job = crate::state::CompanionJobState::Idle;
        }

        for floor in data.floors.floors.iter().filter(|floor| {
            game_state
                .town
                .unlocked_floor_ids
                .iter()
                .any(|floor_id| floor_id == &floor.id)
        }) {
            for mission_id in &floor.mission_ids {
                let Some(mission) = data
                    .missions
                    .missions
                    .iter()
                    .find(|entry| entry.id == *mission_id)
                else {
                    continue;
                };
                if should_reserve_egg_expedition && mission.reward_focus != "eggs" {
                    continue;
                }
                configure_expedition_plan(
                    &mut simulated_state,
                    &floor.id,
                    mission_id,
                    ExpeditionPriority::Balanced,
                );
                if assign_monster_to_expedition(data, &mut simulated_state, &monster.id, &floor.id)
                    .is_err()
                {
                    continue;
                }
                let Ok(preview) = preview_expedition_plan(
                    data,
                    &simulated_state,
                    &floor.id,
                    mission_id,
                    &ExpeditionPriority::Balanced,
                ) else {
                    continue;
                };
                let score = expedition_growth_score(game_state, &preview);
                if !should_reserve_egg_expedition
                    && !can_spare_worker_for_growth(game_state, reserved_guest_monster_ids.len())
                {
                    continue;
                }
                if !should_reserve_egg_expedition
                    && !can_survive_debt_after_growth_assignment(game_state, 1)
                {
                    continue;
                }
                if best_assignment
                    .as_ref()
                    .is_none_or(|(_, _, _, best_score)| score > *best_score)
                {
                    best_assignment = Some((
                        monster.id.clone(),
                        floor.id.clone(),
                        mission_id.clone(),
                        score,
                    ));
                }
            }
        }
    }

    best_assignment.map(|(monster_id, floor_id, mission_id, _)| (monster_id, floor_id, mission_id))
}

/// How much daylight above the injury threshold the simulated guild insists on
/// before it stops discounting a run.
const INJURY_MARGIN_WATCHED: i32 = 20;

pub(super) fn expedition_growth_score(
    game_state: &GameState,
    preview: &crate::engine::day_cycle::ExpeditionPlanPreview,
) -> i32 {
    let egg_value = if pending_eggs_cover_workforce_demand(game_state) {
        15
    } else if game_state.egg_inventory.is_empty() {
        180
    } else {
        120
    };
    let relic_value = 70;
    let material_value = 2;
    let residue_value = 1;
    // `injury_risk_score` is now the margin past the injury threshold for the
    // most exposed companion: at or above zero somebody is certain to come home
    // hurt, and everything below it is daylight. A guild worth simulating starts
    // paying attention a little before the line rather than after it.
    // No party assigned means no risk to price. The policy always scores a
    // one-companion party, so this is a guard rather than a live branch.
    let injury_penalty = preview
        .injury_risk_score
        .map_or(0, |score| (score + INJURY_MARGIN_WATCHED).max(0) * 2);

    preview.projected_eggs as i32 * egg_value
        + preview.projected_relics as i32 * relic_value
        + preview.projected_materials as i32 * material_value
        + preview.projected_arcane_residue as i32 * residue_value
        + preview.success_score.max(0)
        - injury_penalty
}

pub(super) fn should_reserve_egg_expedition(game_state: &GameState) -> bool {
    game_state
        .monsters
        .len()
        .saturating_add(game_state.egg_inventory.len())
        < workforce_demand(game_state).saturating_add(2)
}
