//! Validation-policy guest booking decisions.
use super::policy_eggs::monster_service_score;
use super::policy_jobs::{best_growth_expedition_plan, best_unlocked_room_id};
use super::*;

pub(super) fn assign_guest_bookings(data: &GameData, game_state: &mut GameState) {
    let pending_request_ids = game_state
        .live_contracts()
        .filter(|request| request.assigned_monster_id.is_none())
        .map(|request| request.request_id.clone())
        .collect::<Vec<_>>();

    for request_id in pending_request_ids {
        let Some(request) = game_state
            .active_contracts
            .iter()
            .find(|entry| entry.request_id == request_id)
            .cloned()
        else {
            continue;
        };

        if should_defer_guest_for_growth(data, game_state, &request) {
            continue;
        }

        let mut monster_ids = game_state
            .monsters
            .iter()
            .map(|monster| monster.id.clone())
            .collect::<Vec<_>>();
        monster_ids.sort_by_key(|monster_id| {
            let Some(monster) = game_state
                .monsters
                .iter()
                .find(|monster| monster.id == *monster_id)
            else {
                return (u32::MAX, std::cmp::Reverse(0));
            };
            (
                guest_assignment_opportunity_cost(data, game_state, monster),
                std::cmp::Reverse(monster_service_score(monster)),
            )
        });

        for monster_id in monster_ids {
            let Some(monster) = game_state
                .monsters
                .iter()
                .find(|entry| entry.id == monster_id)
            else {
                continue;
            };

            let report =
                crate::engine::evaluate_contract_eligibility(data, game_state, &request, monster);
            if report.is_eligible
                && assign_monster_to_contract(data, game_state, &request_id, &monster_id).is_ok()
            {
                break;
            }
        }
    }
}

pub(super) fn should_defer_guest_for_growth(
    data: &GameData,
    game_state: &GameState,
    request: &crate::state::ContractState,
) -> bool {
    // A one-companion guild that loses its first egg runs must keep its only
    // worker available for another attempt and for recovery. Booking her into
    // the contract chain runs fatigue to the cap and turns two unlucky rolls
    // into a permanent recruitment deadlock.
    if game_state.monsters.len() < 5 && game_state.egg_inventory.is_empty() {
        return true;
    }
    if request.deadline_day <= game_state.current_day {
        return false;
    }

    let reserved_guest_monster_ids = accepted_guest_monster_ids(game_state);
    let fit_workers = game_state
        .monsters
        .iter()
        .filter(|monster| monster.injury == 0 && monster.fatigue < 34 && monster.stress < 20)
        .count();
    if fit_workers > reserved_guest_monster_ids.len() + 1 {
        return false;
    }

    best_growth_expedition_plan(data, game_state, &reserved_guest_monster_ids).is_some()
}

pub(super) fn guest_assignment_opportunity_cost(
    data: &GameData,
    game_state: &GameState,
    monster: &crate::state::CompanionState,
) -> u32 {
    best_unlocked_room_id(data, game_state, monster)
        .and_then(|room_id| {
            crate::engine::preview_guild_job(data, game_state, monster, &room_id).ok()
        })
        .map(|preview| preview.projected_gold)
        .unwrap_or(0)
}

pub(super) fn accepted_guest_monster_ids(
    game_state: &GameState,
) -> std::collections::HashSet<String> {
    game_state
        .active_contracts
        .iter()
        .filter(|request| matches!(request.status, crate::state::ContractStatus::Accepted))
        .filter_map(|request| request.assigned_monster_id.clone())
        .collect()
}

pub(super) fn accepted_guest_booking_count(game_state: &GameState) -> usize {
    game_state
        .active_contracts
        .iter()
        .filter(|request| matches!(request.status, crate::state::ContractStatus::Accepted))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_staffing_preserves_the_worker_with_the_better_room_shift() {
        let data = crate::data::test_game_data();
        let low_opportunity = crate::state::CompanionState {
            id: "monster_low".to_owned(),
            species_id: "slime_companion".to_owned(),
            name: "Mira".to_owned(),
            quality_rank: 1,
            ..crate::state::CompanionState::default()
        };
        let high_opportunity = crate::state::CompanionState {
            id: "monster_high".to_owned(),
            species_id: "slime_companion".to_owned(),
            name: "Tess".to_owned(),
            quality_rank: 3,
            stats: crate::data::StatBlockData {
                power: 12,
                charm: 12,
                endurance: 12,
                instinct: 12,
            },
            skills: crate::state::CompanionSkillState {
                hospitality: 20,
                charm: 20,
                ..crate::state::CompanionSkillState::default()
            },
            ..crate::state::CompanionState::default()
        };
        let mut monsters = vec![low_opportunity.clone(), high_opportunity.clone()];
        for index in 0..3 {
            monsters.push(crate::state::CompanionState {
                id: format!("monster_untrained_{index}"),
                species_id: "slime_companion".to_owned(),
                name: format!("Untrained {index}"),
                quality_rank: 0,
                ..crate::state::CompanionState::default()
            });
        }
        let mut game_state = crate::engine::create_new_game_state(&data);
        game_state.current_day = 4;
        game_state.town.unlocked_room_ids = vec!["common_room".to_owned()];
        game_state.active_contracts = vec![crate::state::ContractState {
            request_id: "contract_001".to_owned(),
            requested_room_id: "common_room".to_owned(),
            minimum_quality_rank: 1,
            deadline_day: 4,
            status: crate::state::ContractStatus::Pending,
            ..crate::state::ContractState::default()
        }];
        game_state.monsters = monsters;
        assert!(
            guest_assignment_opportunity_cost(&data, &game_state, &high_opportunity)
                > guest_assignment_opportunity_cost(&data, &game_state, &low_opportunity),
            "the fixture should give the stronger worker a better alternative shift"
        );

        assign_guest_bookings(&data, &mut game_state);

        assert_eq!(
            game_state.active_contracts[0]
                .assigned_monster_id
                .as_deref(),
            Some("monster_low")
        );
    }
}
