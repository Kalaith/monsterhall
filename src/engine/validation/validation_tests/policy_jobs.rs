//! Validation-policy daily job and expedition assignment.
use super::policy_growth::{
    can_spare_worker_for_growth, can_survive_debt_after_growth_assignment,
    pending_eggs_cover_workforce_demand, workforce_demand,
};
use super::policy_guests::accepted_guest_monster_ids;
use super::*;
use crate::engine::preview_guild_job;

pub(super) fn assign_daily_jobs(data: &GameData, game_state: &mut GameState) -> (usize, usize) {
    let guild_job_limit = usize::from(game_state.town.town_job_limit);
    let reserved_guest_monster_ids = accepted_guest_monster_ids(game_state);
    let mut guild_job_workers = 0usize;
    let mut expedition_members = 0usize;

    let monster_ids = game_state
        .monsters
        .iter()
        .map(|monster| monster.id.clone())
        .collect::<Vec<_>>();

    if let Some(plan) = best_growth_expedition_plan(data, game_state, &reserved_guest_monster_ids) {
        configure_expedition_plan(game_state, &plan.floor_id, &plan.mission_id, plan.priority);
        for monster_id in plan.monster_ids {
            if assign_monster_to_expedition(data, game_state, &monster_id, &plan.floor_id).is_ok() {
                expedition_members += 1;
            }
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
            if let Some(room_id) = best_unlocked_room_id(data, game_state, monster) {
                if assign_monster_to_room(game_state, &monster_id, &room_id).is_ok() {
                    guild_job_workers += 1;
                    continue;
                }
            }
        }
    }

    (guild_job_workers, expedition_members)
}

pub(super) fn best_unlocked_room_id(
    data: &GameData,
    game_state: &GameState,
    monster: &crate::state::CompanionState,
) -> Option<String> {
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
        .max_by_key(|room| {
            let untrained_curriculum = room
                .trained_skill_ids
                .iter()
                .filter(|skill_id| {
                    game_state.monsters.iter().all(|entry| {
                        crate::engine::companion_skill_value(&entry.skills, skill_id) == 0
                    })
                })
                .count();
            let ready_gold = preview_guild_job(data, game_state, monster, &room.id)
                .map(|preview| preview.projected_gold)
                .unwrap_or_default();
            (untrained_curriculum > 0, untrained_curriculum, ready_gold)
        })
        .map(|room| room.id.clone())
}

#[derive(Debug, Clone)]
pub(super) struct ExpeditionPolicyPlan {
    pub(super) monster_ids: Vec<String>,
    pub(super) floor_id: String,
    pub(super) mission_id: String,
    pub(super) priority: ExpeditionPriority,
    score: i32,
}

pub(super) fn best_growth_expedition_plan(
    data: &GameData,
    game_state: &GameState,
    reserved_guest_monster_ids: &std::collections::HashSet<String>,
) -> Option<ExpeditionPolicyPlan> {
    let mut best_plan = None::<ExpeditionPolicyPlan>;
    let reserve_egg_run = should_reserve_egg_expedition(game_state);
    let mut available_monster_ids = game_state
        .monsters
        .iter()
        .filter(|monster| {
            !reserved_guest_monster_ids.contains(&monster.id)
                && monster.injury == 0
                && monster.fatigue < 34
                && monster.stress < 20
        })
        .map(|monster| monster.id.clone())
        .collect::<Vec<_>>();
    available_monster_ids.sort();
    let max_party_size = usize::from(game_state.town.party_size).min(available_monster_ids.len());
    if max_party_size == 0
        || (!reserve_egg_run
            && !can_spare_worker_for_growth(game_state, reserved_guest_monster_ids.len()))
    {
        return None;
    }
    let mut simulated_state = game_state.clone();

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
            if reserve_egg_run && mission.reward_focus != "eggs" {
                continue;
            }
            let mut mission_monster_ids = available_monster_ids.clone();
            mission_monster_ids.sort_by(|left_id, right_id| {
                let score = |monster_id: &String| {
                    game_state
                        .monsters
                        .iter()
                        .find(|monster| &monster.id == monster_id)
                        .map(|monster| {
                            expedition_candidate_score(data, game_state, mission, monster)
                        })
                        .unwrap_or(i32::MIN)
                };
                score(right_id)
                    .cmp(&score(left_id))
                    .then_with(|| left_id.cmp(right_id))
            });
            for priority in expedition_priority_options() {
                let mut party = Vec::<String>::new();
                for monster_id in mission_monster_ids.iter().take(max_party_size) {
                    party.push(monster_id.clone());
                    let Some(preview) = preview_expedition_party(
                        data,
                        &mut simulated_state,
                        &party,
                        &floor.id,
                        mission_id,
                        &priority,
                    ) else {
                        break;
                    };
                    let score = expedition_growth_score(game_state, &preview)
                        - expedition_party_opportunity_cost(data, game_state, &party)
                        + expedition_stance_value(data, game_state, &party, &priority);

                    if !reserve_egg_run
                        && (!can_survive_debt_after_growth_assignment(
                            game_state,
                            party.len() as u32,
                        ) || available_monster_ids.len() <= party.len())
                    {
                        continue;
                    }
                    if best_plan.as_ref().is_none_or(|best| score > best.score) {
                        best_plan = Some(ExpeditionPolicyPlan {
                            monster_ids: party.clone(),
                            floor_id: floor.id.clone(),
                            mission_id: mission_id.clone(),
                            priority: priority.clone(),
                            score,
                        });
                    }
                }
            }
        }
    }

    best_plan
}

fn expedition_candidate_score(
    data: &GameData,
    game_state: &GameState,
    mission: &crate::data::MissionData,
    monster: &crate::state::CompanionState,
) -> i32 {
    let stats = crate::engine::effective_stats(data, monster);
    let role_fit = i32::from(
        mission
            .preferred_role
            .as_deref()
            .is_some_and(|role| crate::engine::monster_role(data, monster) == role),
    ) * 20;
    let capability = stats.power * 4
        + stats.instinct * 2
        + monster.skills.scouting as i32
        + monster.skills.guarding as i32
        + monster.skills.navigation as i32
        + monster.skills.arcana as i32
        + monster.skills.strength as i32;
    capability + role_fit
        - i32::try_from(guest_room_alternative_gold(data, game_state, monster) / 2)
            .unwrap_or(i32::MAX)
}

fn guest_room_alternative_gold(
    data: &GameData,
    game_state: &GameState,
    monster: &crate::state::CompanionState,
) -> u32 {
    best_unlocked_room_id(data, game_state, monster)
        .and_then(|room_id| preview_guild_job(data, game_state, monster, &room_id).ok())
        .map(|preview| preview.projected_gold)
        .unwrap_or(0)
}

fn preview_expedition_party(
    data: &GameData,
    simulated_state: &mut GameState,
    monster_ids: &[String],
    floor_id: &str,
    mission_id: &str,
    priority: &ExpeditionPriority,
) -> Option<crate::engine::day_cycle::ExpeditionPlanPreview> {
    simulated_state.active_expedition = None;
    for simulated_monster in &mut simulated_state.monsters {
        simulated_monster.current_job = crate::state::CompanionJobState::Idle;
    }
    configure_expedition_plan(simulated_state, floor_id, mission_id, priority.clone());
    for monster_id in monster_ids {
        assign_monster_to_expedition(data, simulated_state, monster_id, floor_id).ok()?;
    }
    preview_expedition_plan(data, simulated_state, floor_id, mission_id, priority).ok()
}

fn expedition_party_opportunity_cost(
    data: &GameData,
    game_state: &GameState,
    monster_ids: &[String],
) -> i32 {
    let foregone_gold = monster_ids
        .iter()
        .filter_map(|monster_id| {
            game_state
                .monsters
                .iter()
                .find(|monster| &monster.id == monster_id)
        })
        .map(|monster| guest_room_alternative_gold(data, game_state, monster))
        .sum::<u32>();
    i32::try_from(foregone_gold / 2).unwrap_or(i32::MAX)
}

fn expedition_stance_value(
    data: &GameData,
    game_state: &GameState,
    monster_ids: &[String],
    priority: &ExpeditionPriority,
) -> i32 {
    match priority {
        ExpeditionPriority::RecoveryFocused => {
            let raw_condition_cost = data
                .config
                .day_cycle
                .expedition_fatigue
                .saturating_add(data.config.day_cycle.expedition_stress);
            let saved_per_companion = raw_condition_cost.saturating_mul(
                100u32.saturating_sub(data.config.day_cycle.recovery_focused_condition_cost_pct),
            ) / 100;
            let existing_condition = monster_ids
                .iter()
                .filter_map(|monster_id| {
                    game_state
                        .monsters
                        .iter()
                        .find(|monster| &monster.id == monster_id)
                })
                .map(|monster| monster.fatigue.saturating_add(monster.stress))
                .sum::<u32>()
                / 4;
            i32::try_from(
                saved_per_companion
                    .saturating_mul(monster_ids.len() as u32)
                    .saturating_add(existing_condition),
            )
            .unwrap_or(i32::MAX)
        }
        ExpeditionPriority::Balanced => 2,
        ExpeditionPriority::Aggressive
        | ExpeditionPriority::Safe
        | ExpeditionPriority::Curiosity => 0,
    }
}

pub(super) fn expedition_priority_options() -> [ExpeditionPriority; 5] {
    [
        ExpeditionPriority::Balanced,
        ExpeditionPriority::Aggressive,
        ExpeditionPriority::Safe,
        ExpeditionPriority::RecoveryFocused,
        ExpeditionPriority::Curiosity,
    ]
}

pub(super) fn expedition_priority_id(priority: &ExpeditionPriority) -> &'static str {
    match priority {
        ExpeditionPriority::Balanced => "balanced",
        ExpeditionPriority::Aggressive => "aggressive",
        ExpeditionPriority::Safe => "safe",
        ExpeditionPriority::RecoveryFocused => "recovery_focused",
        ExpeditionPriority::Curiosity => "curiosity",
    }
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
    let residue_value = if game_state.resources.arcane_residue < 1_000 {
        3
    } else {
        1
    };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expedition_policy_considers_every_player_stance() {
        let stance_ids = expedition_priority_options()
            .iter()
            .map(expedition_priority_id)
            .collect::<Vec<_>>();

        assert_eq!(
            stance_ids,
            [
                "balanced",
                "aggressive",
                "safe",
                "recovery_focused",
                "curiosity"
            ]
        );
    }
}
