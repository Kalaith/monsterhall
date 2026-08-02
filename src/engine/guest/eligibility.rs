//! Whether a given companion can take a given contract, and why not.
//!
//! Split out of `guest.rs` when that file crossed the 800-line limit. The
//! booking lifecycle - offers, assignment, resolution - stayed behind.

use super::*;

pub fn evaluate_contract_eligibility(
    data: &GameData,
    game_state: &GameState,
    request: &ContractState,
    monster: &CompanionState,
) -> ContractEligibilityReport {
    let mut failure_reasons = Vec::new();

    if !game_state
        .town
        .unlocked_room_ids
        .iter()
        .any(|room_id| room_id == &request.requested_room_id)
    {
        failure_reasons.push(story_text(
            &data.story_events.guest_requires_template,
            &[("{value}", room_name_by_id(data, &request.requested_room_id))],
        ));
    }

    if !request.required_species_ids.is_empty()
        && !request
            .required_species_ids
            .iter()
            .any(|species_id| species_id == &monster.species_id)
    {
        let species_names = request
            .required_species_ids
            .iter()
            .map(|species_id| species_name_by_id(data, species_id))
            .collect::<Vec<_>>()
            .join(" or ");
        failure_reasons.push(story_text(
            &data.story_events.guest_requires_template,
            &[("{value}", species_names)],
        ));
    }

    if monster.quality_rank < request.minimum_quality_rank.max(1) {
        failure_reasons.push(format!(
            "Requires {}-star quality (current {}-star).",
            request.minimum_quality_rank.max(1),
            monster.quality_rank.max(1)
        ));
    }

    if matches!(monster.current_job, CompanionJobState::OnExpedition { .. }) {
        failure_reasons.push(data.story_events.guest_already_on_expedition_reason.clone());
    }

    append_skill_requirement_reasons(data, &mut failure_reasons, request, monster);
    append_history_requirement_reasons(data, &mut failure_reasons, request, monster);

    ContractEligibilityReport {
        is_eligible: failure_reasons.is_empty(),
        failure_reasons,
    }
}

pub(super) fn meets_guest_hard_gates(
    data: &GameData,
    game_state: &GameState,
    request: &ContractState,
    monster: &CompanionState,
) -> bool {
    game_state
        .town
        .unlocked_room_ids
        .iter()
        .any(|room_id| room_id == &request.requested_room_id)
        && (request.required_species_ids.is_empty()
            || request
                .required_species_ids
                .iter()
                .any(|species_id| species_id == &monster.species_id))
        && monster.quality_rank >= request.minimum_quality_rank.max(1)
        && !matches!(monster.current_job, CompanionJobState::OnExpedition { .. })
        && data
            .guild_rooms
            .rooms
            .iter()
            .any(|room| room.id == request.requested_room_id)
}

fn append_skill_requirement_reasons(
    data: &GameData,
    failure_reasons: &mut Vec<String>,
    request: &ContractState,
    monster: &CompanionState,
) {
    check_skill_requirement(
        data,
        failure_reasons,
        "Scouting",
        request.required_skill_thresholds.scouting,
        monster.skills.scouting,
    );
    check_skill_requirement(
        data,
        failure_reasons,
        "Guarding",
        request.required_skill_thresholds.guarding,
        monster.skills.guarding,
    );
    check_skill_requirement(
        data,
        failure_reasons,
        "Hospitality",
        request.required_skill_thresholds.hospitality,
        monster.skills.hospitality,
    );
    check_skill_requirement(
        data,
        failure_reasons,
        "Crafting",
        request.required_skill_thresholds.crafting,
        monster.skills.crafting,
    );
    check_skill_requirement(
        data,
        failure_reasons,
        "Charm",
        request.required_skill_thresholds.charm,
        monster.skills.charm,
    );
    // The other five were declared on the threshold struct and never checked
    // here, so a contract asking for Arcana would have been satisfied by anyone.
    // No shipped contract asks yet, and load-time validation now refuses one
    // that asks for a skill no room teaches.
    check_skill_requirement(
        data,
        failure_reasons,
        "Recovery",
        request.required_skill_thresholds.recovery,
        monster.skills.recovery,
    );
    check_skill_requirement(
        data,
        failure_reasons,
        "Bargaining",
        request.required_skill_thresholds.bargaining,
        monster.skills.bargaining,
    );
    check_skill_requirement(
        data,
        failure_reasons,
        "Navigation",
        request.required_skill_thresholds.navigation,
        monster.skills.navigation,
    );
    check_skill_requirement(
        data,
        failure_reasons,
        "Arcana",
        request.required_skill_thresholds.arcana,
        monster.skills.arcana,
    );
    check_skill_requirement(
        data,
        failure_reasons,
        "Strength",
        request.required_skill_thresholds.strength,
        monster.skills.strength,
    );
}

fn append_history_requirement_reasons(
    data: &GameData,
    failure_reasons: &mut Vec<String>,
    request: &ContractState,
    monster: &CompanionState,
) {
    check_history_requirement(
        data,
        failure_reasons,
        "Kiss Count",
        request.required_work_history_thresholds.scouting_runs,
        monster.work_history.scouting_runs,
    );
    check_history_requirement(
        data,
        failure_reasons,
        "Guarding Count",
        request.required_work_history_thresholds.guard_duties,
        monster.work_history.guard_duties,
    );
    check_history_requirement(
        data,
        failure_reasons,
        "Hospitality Count",
        request.required_work_history_thresholds.hospitality_jobs,
        monster.work_history.hospitality_jobs,
    );
    check_history_requirement(
        data,
        failure_reasons,
        "Crafting Count",
        request.required_work_history_thresholds.craft_jobs,
        monster.work_history.craft_jobs,
    );
    check_history_requirement(
        data,
        failure_reasons,
        "Contracts Completed",
        request.required_work_history_thresholds.contracts_completed,
        monster.work_history.contracts_completed,
    );
    check_history_requirement(
        data,
        failure_reasons,
        "Recovery Shifts",
        request.required_work_history_thresholds.recovery_shifts,
        monster.work_history.recovery_shifts,
    );
    check_history_requirement(
        data,
        failure_reasons,
        "Birth Count",
        request.required_work_history_thresholds.hatchery_assists,
        monster.work_history.hatchery_assists,
    );
}

fn check_skill_requirement(
    data: &GameData,
    failure_reasons: &mut Vec<String>,
    label: &str,
    required: u32,
    current: u32,
) {
    if required > 0 && current < required {
        failure_reasons.push(story_text(
            &data.story_events.guest_requirement_detail_template,
            &[
                ("{label}", label.to_owned()),
                ("{required}", required.to_string()),
                ("{current}", current.to_string()),
            ],
        ));
    }
}

fn check_history_requirement(
    data: &GameData,
    failure_reasons: &mut Vec<String>,
    label: &str,
    required: u32,
    current: u32,
) {
    if required > 0 && current < required {
        failure_reasons.push(story_text(
            &data.story_events.guest_requirement_detail_template,
            &[
                ("{label}", label.to_owned()),
                ("{required}", required.to_string()),
                ("{current}", current.to_string()),
            ],
        ));
    }
}
