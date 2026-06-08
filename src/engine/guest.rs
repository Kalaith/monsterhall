//! Contract generation and qualification helpers.

use std::collections::HashSet;

use super::day_cycle::apply_guild_job_progression;
use super::{
    active_situation_guest_bonus, apply_monster_relationship_gain, contract_depth_score,
    contract_follow_up_request, contract_partial_success,
};
use crate::data::{ContractData, GameData};
use crate::state::{
    CompanionJobState, CompanionState, ContractHistoryRequirementState,
    ContractSkillRequirementState, ContractState, ContractStatus, GameState, ResourcesState,
};

const MIN_ACTIVE_GUEST_REQUESTS: usize = 3;
const MAX_ACTIVE_GUEST_REQUESTS: usize = 6;

fn story_text(template: &str, replacements: &[(&str, String)]) -> String {
    let mut output = template.to_owned();
    for (token, value) in replacements {
        output = output.replace(token, value);
    }
    output
}

#[derive(Debug, Clone)]
pub struct ContractEligibilityReport {
    pub is_eligible: bool,
    pub failure_reasons: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ContractRefreshReport {
    pub generated: usize,
    pub rejected: usize,
}

pub fn refresh_contracts(
    data: &GameData,
    game_state: &mut GameState,
) -> Result<ContractRefreshReport, String> {
    game_state.active_contracts.retain(|request| {
        matches!(
            request.status,
            ContractStatus::Pending | ContractStatus::Accepted
        )
    });

    let mut candidate_templates = data
        .contracts
        .requests
        .iter()
        .filter(|template| {
            !game_state
                .active_contracts
                .iter()
                .any(|request| request.template_id == template.id)
                && request_template_available(data, game_state, template)
        })
        .cloned()
        .collect::<Vec<_>>();
    candidate_templates
        .sort_by_key(|template| request_pressure_priority(data, game_state, template));
    candidate_templates.reverse();

    let active_request_limit = active_contract_limit(game_state);
    let mut report = ContractRefreshReport::default();
    let mut next_sequence = game_state.active_contracts.len() + 1;
    for template in candidate_templates {
        if game_state.active_contracts.len() >= active_request_limit {
            break;
        }
        let archetype = data
            .patron_archetypes
            .archetypes
            .iter()
            .find(|entry| entry.id == template.archetype_id)
            .ok_or_else(|| {
                format!(
                    "Contract '{}' references missing archetype '{}'.",
                    template.id, template.archetype_id
                )
            })?;

        if template.is_special && !game_state.story_progress.first_special_guest_seen {
            game_state.story_progress.first_special_guest_seen = true;
        }

        let candidate_request = ContractState {
            request_id: format!(
                "guest_request_{:03}",
                game_state.current_day as usize * 10 + next_sequence
            ),
            template_id: template.id.clone(),
            category: template.category.clone(),
            patron_tier_id: template.patron_tier_id.clone(),
            guest_name: story_text(
                &data.story_events.guest_name_template,
                &[("{archetype}", archetype.name.clone())],
            ),
            archetype_id: template.archetype_id.clone(),
            requested_room_id: template.requested_room_id.clone(),
            required_species_ids: template.required_species_ids.clone(),
            minimum_quality_rank: template.minimum_quality_rank,
            required_skill_thresholds: ContractSkillRequirementState {
                scouting: template.required_skill_thresholds.scouting,
                guarding: template.required_skill_thresholds.guarding,
                hospitality: template.required_skill_thresholds.hospitality,
                crafting: template.required_skill_thresholds.crafting,
                charm: template.required_skill_thresholds.charm,
                recovery: template.required_skill_thresholds.recovery,
                bargaining: template.required_skill_thresholds.bargaining,
                navigation: template.required_skill_thresholds.navigation,
                arcana: template.required_skill_thresholds.arcana,
                strength: template.required_skill_thresholds.strength,
            },
            required_work_history_thresholds: ContractHistoryRequirementState {
                scouting_runs: template.required_work_history_thresholds.scouting_runs,
                guard_duties: template.required_work_history_thresholds.guard_duties,
                hospitality_jobs: template.required_work_history_thresholds.hospitality_jobs,
                craft_jobs: template.required_work_history_thresholds.craft_jobs,
                contracts_completed: template
                    .required_work_history_thresholds
                    .contracts_completed,
                recovery_shifts: template.required_work_history_thresholds.recovery_shifts,
                hatchery_assists: template.required_work_history_thresholds.hatchery_assists,
            },
            reward: ResourcesState {
                gold: template.reward.gold,
                tower_materials: template.reward.tower_materials,
                eggs: template.reward.eggs,
                relics: template.reward.relics,
                arcane_residue: template.reward.arcane_residue,
            },
            penalty_gold: template.penalty_gold,
            deadline_day: game_state.current_day
                + scaled_guest_deadline_days(data, game_state, &template),
            preparation_quality_required: template.preparation_quality_required,
            preparation_quality_bonus: template.preparation_quality_bonus,
            status: ContractStatus::Pending,
            assigned_monster_id: None,
            chain_depth: 0,
            partial_progress: 0,
        };

        let candidate_reports = request_candidates(data, game_state, &candidate_request);
        let has_eligible_candidate = candidate_reports.iter().any(|(monster, report)| {
            report.is_eligible
                || (meets_guest_hard_gates(data, game_state, &candidate_request, monster)
                    && contract_partial_success(data, game_state, &candidate_request, monster))
        });
        let _total_failure_reasons = candidate_reports
            .iter()
            .map(|(_, report)| report.failure_reasons.len())
            .sum::<usize>();

        if !has_eligible_candidate {
            report.rejected += 1;
            continue;
        }

        game_state.active_contracts.push(candidate_request);
        report.generated += 1;
        next_sequence += 1;
    }

    Ok(report)
}

fn active_contract_limit(game_state: &GameState) -> usize {
    MIN_ACTIVE_GUEST_REQUESTS
        .saturating_add(game_state.monsters.len() / 5)
        .saturating_add(game_state.town.unlocked_room_ids.len().saturating_sub(1))
        .saturating_add(game_state.town.patron_tiers.len().saturating_sub(1))
        .saturating_add(game_state.town.constructed_building_ids.len() / 4)
        .saturating_add(active_situation_guest_bonus(game_state) as usize)
        .min(MAX_ACTIVE_GUEST_REQUESTS)
}

fn scaled_guest_deadline_days(
    data: &GameData,
    game_state: &GameState,
    template: &ContractData,
) -> u32 {
    let room_tier = request_room_tier(data, &template.requested_room_id);
    let reputation_pressure = game_state.town.patron_tiers.len().saturating_sub(1) as u32;
    let roster_pressure = (game_state.monsters.len() / 8) as u32;
    let tier_pressure = room_tier.saturating_sub(1);
    let pressure = reputation_pressure
        .saturating_add(roster_pressure)
        .saturating_add(tier_pressure);
    let reduction = pressure.min(3);
    template.deadline_days.saturating_sub(reduction).max(2)
}

fn request_pressure_priority(
    data: &GameData,
    game_state: &GameState,
    template: &ContractData,
) -> u32 {
    let room_tier = request_room_tier(data, &template.requested_room_id);
    let target_tier = game_state
        .town
        .unlocked_room_ids
        .iter()
        .map(|room_id| request_room_tier(data, room_id))
        .max()
        .unwrap_or(1);
    let tier_fit_bonus = 40u32.saturating_sub(room_tier.abs_diff(target_tier) * 12);
    let reward_score = template.reward.gold / 4 + template.reward.arcane_residue;
    let special_bonus = if template.is_special { 18 } else { 0 };
    let pressure_bonus = active_contract_limit(game_state) as u32 * 3;

    room_tier
        .saturating_mul(35)
        .saturating_add(tier_fit_bonus)
        .saturating_add(reward_score)
        .saturating_add(special_bonus)
        .saturating_add(pressure_bonus)
}

fn request_room_tier(data: &GameData, room_id: &str) -> u32 {
    data.guild_rooms
        .rooms
        .iter()
        .find(|room| room.id == room_id)
        .map(|room| room.service_tier as u32)
        .unwrap_or(1)
}

pub fn assign_monster_to_contract(
    data: &GameData,
    game_state: &mut GameState,
    request_id: &str,
    monster_id: &str,
) -> Result<(), String> {
    let request_index = game_state
        .active_contracts
        .iter()
        .position(|request| request.request_id == request_id)
        .ok_or_else(|| format!("Unknown contract id '{request_id}'."))?;
    let monster = game_state
        .monsters
        .iter()
        .find(|monster| monster.id == monster_id)
        .ok_or_else(|| format!("Unknown monster id '{monster_id}'."))?;
    let report = evaluate_contract_eligibility(
        data,
        game_state,
        &game_state.active_contracts[request_index],
        monster,
    );
    let depth_score = contract_depth_score(
        data,
        game_state,
        &game_state.active_contracts[request_index],
        monster,
    );
    let partial_success = !report.is_eligible
        && meets_guest_hard_gates(
            data,
            game_state,
            &game_state.active_contracts[request_index],
            monster,
        )
        && contract_partial_success(
            data,
            game_state,
            &game_state.active_contracts[request_index],
            monster,
        );
    if !report.is_eligible && !partial_success {
        return Err(report.failure_reasons.join(" "));
    }
    if game_state.active_contracts.iter().any(|request| {
        request.request_id != request_id
            && request.assigned_monster_id.as_deref() == Some(monster_id)
            && matches!(request.status, ContractStatus::Accepted)
    }) {
        return Err("That companion is already assigned to another contract.".to_owned());
    }

    let request = &mut game_state.active_contracts[request_index];
    request.assigned_monster_id = Some(monster_id.to_owned());
    request.status = ContractStatus::Accepted;
    request.partial_progress = depth_score;
    Ok(())
}

pub fn clear_contract_assignment(
    game_state: &mut GameState,
    request_id: &str,
) -> Result<(), String> {
    let request = game_state
        .active_contracts
        .iter_mut()
        .find(|request| request.request_id == request_id)
        .ok_or_else(|| format!("Unknown contract id '{request_id}'."))?;
    request.assigned_monster_id = None;
    request.status = ContractStatus::Pending;
    Ok(())
}

pub fn resolve_contracts(
    data: &GameData,
    game_state: &mut GameState,
    guild_job_gold: &mut u32,
    guild_job_arcane_residue: &mut u32,
    contract_updates: &mut Vec<String>,
    event_lines: &mut Vec<String>,
    roster_updates: &mut Vec<String>,
) -> HashSet<String> {
    let resolved_day = game_state.current_day;
    let mut serviced_monster_ids = HashSet::new();
    let mut remaining_requests = Vec::new();
    let mut follow_up_requests = Vec::new();
    let requests = std::mem::take(&mut game_state.active_contracts);

    for mut request in requests {
        match request.status {
            ContractStatus::Accepted => {
                let Some(monster_id) = request.assigned_monster_id.clone() else {
                    request.status = ContractStatus::Pending;
                    remaining_requests.push(request);
                    continue;
                };

                let Some(monster_index) = game_state
                    .monsters
                    .iter()
                    .position(|monster| monster.id == monster_id)
                else {
                    event_lines.push(story_text(
                        &data.story_events.guest_missing_assigned_girl_event_template,
                        &[("{guest}", request.guest_name.clone())],
                    ));
                    continue;
                };

                let report = evaluate_contract_eligibility(
                    data,
                    game_state,
                    &request,
                    &game_state.monsters[monster_index],
                );
                let partial_success = !report.is_eligible
                    && meets_guest_hard_gates(
                        data,
                        game_state,
                        &request,
                        &game_state.monsters[monster_index],
                    )
                    && contract_partial_success(
                        data,
                        game_state,
                        &request,
                        &game_state.monsters[monster_index],
                    );
                if !report.is_eligible && !partial_success {
                    event_lines.push(story_text(
                        &data.story_events.guest_failed_event_template,
                        &[
                            ("{guest}", request.guest_name.clone()),
                            ("{reason}", report.failure_reasons.join(" ")),
                        ],
                    ));
                    game_state.resources.gold = game_state
                        .resources
                        .gold
                        .saturating_sub(request.penalty_gold);
                    continue;
                }

                let Some(room) = data
                    .guild_rooms
                    .rooms
                    .iter()
                    .find(|room| room.id == request.requested_room_id)
                else {
                    remaining_requests.push(request);
                    continue;
                };

                let monster = &mut game_state.monsters[monster_index];
                let reward_divisor = if partial_success { 2 } else { 1 };
                let gold_reward = request.reward.gold / reward_divisor;
                let residue_reward = request.reward.arcane_residue / reward_divisor;
                game_state.resources.gold += gold_reward;
                game_state.resources.tower_materials +=
                    request.reward.tower_materials / reward_divisor;
                game_state.resources.relics += request.reward.relics / reward_divisor;
                game_state.resources.arcane_residue += residue_reward;
                *guild_job_gold += gold_reward;
                *guild_job_arcane_residue += residue_reward;
                monster.fatigue = monster.fatigue.saturating_add(room.stamina_cost);
                monster.stress = monster
                    .stress
                    .saturating_add(if partial_success { 4 } else { 2 });
                let progression_update = apply_guild_job_progression(monster, room, true);
                let relationship_request = if partial_success {
                    None
                } else {
                    Some(&request)
                };
                apply_monster_relationship_gain(
                    data,
                    monster,
                    relationship_request,
                    if partial_success { 1 } else { 2 },
                    if partial_success { 0 } else { 1 },
                );
                monster.current_job = CompanionJobState::Idle;
                serviced_monster_ids.insert(monster.id.clone());

                event_lines.push(story_text(
                    &data.story_events.guest_satisfied_event_template,
                    &[
                        ("{guest}", request.guest_name.clone()),
                        ("{girl}", monster.name.clone()),
                        ("{room}", room.name.clone()),
                    ],
                ));
                contract_updates.push(story_text(
                    &data.story_events.guest_completed_update_template,
                    &[
                        ("{guest}", request.guest_name.clone()),
                        ("{gold}", gold_reward.to_string()),
                        ("{residue}", residue_reward.to_string()),
                    ],
                ));
                roster_updates.push(story_text(
                    &data.story_events.guest_completed_roster_template,
                    &[
                        ("{girl}", monster.name.clone()),
                        ("{gold}", gold_reward.to_string()),
                    ],
                ));
                if partial_success {
                    contract_updates.push(format!(
                        "{} accepted a partial fulfillment; reputation holds, but the booking paid less.",
                        request.guest_name
                    ));
                }
                if let Some(progression_update) = progression_update {
                    roster_updates.push(progression_update);
                }
                if !partial_success {
                    if let Some(follow_up) = contract_follow_up_request(data, game_state, &request)
                    {
                        follow_up_requests.push(follow_up);
                    }
                }
            }
            ContractStatus::Pending if request.deadline_day <= resolved_day => {
                game_state.resources.gold = game_state
                    .resources
                    .gold
                    .saturating_sub(request.penalty_gold);
                contract_updates.push(story_text(
                    &data.story_events.guest_expired_update_template,
                    &[
                        ("{guest}", request.guest_name.clone()),
                        ("{gold}", request.penalty_gold.to_string()),
                    ],
                ));
                event_lines.push(story_text(
                    &data.story_events.guest_expired_event_template,
                    &[
                        ("{guest}", request.guest_name.clone()),
                        ("{gold}", request.penalty_gold.to_string()),
                    ],
                ));
            }
            ContractStatus::Pending => remaining_requests.push(request),
            ContractStatus::Completed | ContractStatus::Failed | ContractStatus::Declined => {}
        }
    }

    remaining_requests.extend(follow_up_requests);
    game_state.active_contracts = remaining_requests;
    serviced_monster_ids
}

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

fn meets_guest_hard_gates(
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

fn request_candidates<'a>(
    data: &GameData,
    game_state: &'a GameState,
    request: &ContractState,
) -> Vec<(&'a CompanionState, ContractEligibilityReport)> {
    game_state
        .monsters
        .iter()
        .map(|monster| {
            (
                monster,
                evaluate_contract_eligibility(data, game_state, request, monster),
            )
        })
        .collect()
}

fn request_template_available(
    data: &GameData,
    game_state: &GameState,
    template: &crate::data::ContractData,
) -> bool {
    (!template.is_special || game_state.current_day >= 3)
        && game_state
            .town
            .unlocked_room_ids
            .iter()
            .any(|room_id| room_id == &template.requested_room_id)
        && template.required_species_ids.iter().all(|species_id| {
            game_state
                .town
                .unlocked_species_ids
                .iter()
                .any(|entry| entry == species_id)
        })
        && template
            .patron_tier_id
            .as_ref()
            .map(|tier_id| {
                game_state
                    .town
                    .patron_tiers
                    .iter()
                    .any(|entry| entry == tier_id)
            })
            .unwrap_or(true)
        && data
            .patron_archetypes
            .archetypes
            .iter()
            .any(|archetype| archetype.id == template.archetype_id)
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

fn room_name_by_id(data: &GameData, room_id: &str) -> String {
    data.guild_rooms
        .rooms
        .iter()
        .find(|room| room.id == room_id)
        .map(|room| room.name.clone())
        .unwrap_or_else(|| room_id.to_owned())
}

fn species_name_by_id(data: &GameData, species_id: &str) -> String {
    data.species
        .species
        .iter()
        .find(|species| species.id == species_id)
        .map(|species| species.name.clone())
        .unwrap_or_else(|| species_id.to_owned())
}

#[cfg(test)]
mod tests;
