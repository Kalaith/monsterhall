//! Contract generation and qualification helpers.

use std::collections::HashSet;

use super::day_cycle::{
    apply_guild_job_progression, charm_training_bonus, companion_effectiveness_pct,
    scale_by_effectiveness,
};
use super::{
    active_situation_guest_bonus, apply_monster_relationship_gain, contract_follow_up_request,
    contract_partial_success,
};
use crate::data::{ContractData, GameData};
use crate::state::{
    CompanionJobState, CompanionState, ContractHistoryRequirementState,
    ContractSkillRequirementState, ContractState, ContractStatus, GameState, ResourcesState,
};

const MIN_ACTIVE_CONTRACTS: usize = 3;
const MAX_ACTIVE_CONTRACTS: usize = 6;

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
    game_state
        .active_contracts
        .retain(|request| request.status.is_live());

    let mut candidate_templates = data
        .contracts
        .requests
        .iter()
        .filter(|template| {
            !game_state
                .live_contracts()
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
    let mut next_sequence = game_state.live_contract_count() + 1;
    for template in candidate_templates {
        if game_state.live_contract_count() >= active_request_limit {
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
                "contract_{:03}",
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
        };

        let candidate_reports = request_candidates(data, game_state, &candidate_request);
        let has_eligible_candidate = candidate_reports.iter().any(|(monster, report)| {
            report.is_eligible
                || (meets_guest_hard_gates(data, game_state, &candidate_request, monster)
                    && contract_partial_success(data, game_state, &candidate_request, monster))
        });
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
    MIN_ACTIVE_CONTRACTS
        .saturating_add(game_state.monsters.len() / 5)
        .saturating_add(game_state.town.unlocked_room_ids.len().saturating_sub(1))
        .saturating_add(game_state.town.patron_tiers.len().saturating_sub(1))
        .saturating_add(game_state.town.constructed_building_ids.len() / 4)
        .saturating_add(active_situation_guest_bonus(game_state) as usize)
        .min(MAX_ACTIVE_CONTRACTS)
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

    let base = room_tier
        .saturating_mul(35)
        .saturating_add(tier_fit_bonus)
        .saturating_add(reward_score)
        .saturating_add(special_bonus)
        .saturating_add(pressure_bonus);

    // How often this patron shows up at all. `spawn_weight` is authored on every
    // archetype and validated to be positive, and until now decided nothing —
    // a Tower Scholar at weight 3 crowded onto the board exactly as readily as a
    // Curious Local at 10. Scaling the priority is the deterministic reading of
    // the field: rarer patrons lose ties and drop off a full board first.
    //
    // A weighted random draw was tried instead and rejected. It is the more
    // literal reading of "spawn weight", but the campaign is tuned around a
    // best-first board, and flattening it cost enough income that the guild
    // stopped reaching its final debt milestone at all.
    let spawn_weight = data
        .patron_archetypes
        .archetypes
        .iter()
        .find(|entry| entry.id == template.archetype_id)
        .map(|archetype| archetype.spawn_weight)
        .unwrap_or(DEFAULT_SPAWN_WEIGHT);

    base.saturating_mul(spawn_weight) / DEFAULT_SPAWN_WEIGHT
}

/// The weight a patron carries when the catalogue is silent, and the divisor
/// that keeps a common patron's priority unchanged.
const DEFAULT_SPAWN_WEIGHT: u32 = 10;

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
    if !game_state.active_contracts[request_index].status.is_live() {
        return Err("That contract has already been resolved.".to_owned());
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

    // Taking a booking releases whatever she was rostered for, the same way
    // every other assignment releases her from an expedition.
    //
    // Blocking the reverse order was only half the fix: `assign_monster_to_room`
    // refuses a companion who is already booked, but booking a companion who is
    // already working the hall was still allowed — and `resolve_day` settles the
    // contract first and discards her shift, so the guild-job slot was held by
    // somebody whose work would never happen. Refusing here would be wrong; she
    // is perfectly able to take the contract. It is the slot that is wasted, so
    // the slot goes back.
    if let Some(monster) = game_state
        .monsters
        .iter_mut()
        .find(|monster| monster.id == monster_id)
    {
        if matches!(
            monster.current_job,
            CompanionJobState::GuildJob { .. } | CompanionJobState::Resting
        ) {
            monster.current_job = CompanionJobState::Idle;
        }
    }
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
    if !request.status.is_live() {
        return Err("That contract has already been resolved.".to_owned());
    }
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
    let mut resolved_contracts = Vec::new();
    let charm_training_flat = charm_training_bonus(data, game_state);
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
                        &data
                            .story_events
                            .guest_missing_assigned_companion_event_template,
                        &[("{guest}", request.guest_name.clone())],
                    ));
                    request.status = ContractStatus::Failed;
                    resolved_contracts.push(request);
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
                    request.status = ContractStatus::Failed;
                    resolved_contracts.push(request);
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
                // Contracts were the last place the condition meters were
                // ignored: a companion the guild had run into the ground served
                // a booking exactly as well as a rested one, while the same
                // exhaustion visibly cut her guild-job and expedition output.
                let effectiveness_pct =
                    companion_effectiveness_pct(&data.config.day_cycle, monster);
                let reward_divisor = if partial_success { 2 } else { 1 };
                let scaled = |amount: u32| {
                    scale_by_effectiveness(amount / reward_divisor, effectiveness_pct)
                };
                let gold_reward = scaled(request.reward.gold);
                let residue_reward = scaled(request.reward.arcane_residue);
                game_state.resources.gold += gold_reward;
                game_state.resources.tower_materials += scaled(request.reward.tower_materials);
                game_state.resources.relics += scaled(request.reward.relics);
                game_state.resources.arcane_residue += residue_reward;
                *guild_job_gold += gold_reward;
                *guild_job_arcane_residue += residue_reward;
                monster.fatigue = monster.fatigue.saturating_add(room.stamina_cost);
                monster.stress = monster
                    .stress
                    .saturating_add(if partial_success { 4 } else { 2 });
                let progression_update =
                    apply_guild_job_progression(monster, room, true, charm_training_flat);
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
                        ("{companion}", monster.name.clone()),
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
                        ("{companion}", monster.name.clone()),
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
                request.status = ContractStatus::Completed;
                resolved_contracts.push(request);
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
                request.status = ContractStatus::Failed;
                resolved_contracts.push(request);
            }
            ContractStatus::Pending => remaining_requests.push(request),
            // Swept a day after they resolved, which is what gives the player a
            // turn to read the outcome on the contract desk.
            ContractStatus::Completed | ContractStatus::Failed | ContractStatus::Declined => {}
        }
    }

    remaining_requests.extend(follow_up_requests);
    game_state.active_contracts = remaining_requests;
    game_state.resolved_contracts = resolved_contracts;
    serviced_monster_ids
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
        // A patron asks for what the hall is known to have, not for what the
        // guild has read about. Keying this off the unlock alone was survivable
        // while every species came from a shallow floor and hatched within
        // days; a species that only hatches below the auction floor stays
        // unlocked and absent for most of a campaign, and the desk fills up
        // with work nobody in the hall can take.
        && template.required_species_ids.iter().all(|species_id| {
            game_state
                .monsters
                .iter()
                .any(|monster| &monster.species_id == species_id)
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

mod eligibility;

#[cfg(test)]
mod tests;

pub use eligibility::evaluate_contract_eligibility;
use eligibility::*;
