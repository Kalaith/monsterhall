//! Validation-policy guest booking decisions.
use super::policy_eggs::monster_service_score;
use super::policy_jobs::best_growth_expedition_assignment;
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
            game_state
                .monsters
                .iter()
                .find(|monster| monster.id == *monster_id)
                .map(monster_service_score)
                .unwrap_or_default()
        });
        monster_ids.reverse();

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

    best_growth_expedition_assignment(data, game_state, &reserved_guest_monster_ids).is_some()
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

pub(super) fn count_guest_completions(summary: &crate::state::DayResolutionSummary) -> usize {
    summary
        .contract_updates
        .iter()
        .filter(|line| line.contains(" completed: "))
        .count()
}

pub(super) fn count_guest_expirations(summary: &crate::state::DayResolutionSummary) -> usize {
    summary
        .contract_updates
        .iter()
        .filter(|line| line.contains(" expired: "))
        .count()
}
