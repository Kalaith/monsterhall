use super::*;
use crate::state::{CompanionState, GameState};

#[derive(Debug, Serialize, Clone)]
pub(super) struct SimulationResourcesSnapshot {
    pub(super) gold: u32,
    pub(super) tower_materials: u32,
    pub(super) eggs: u32,
    pub(super) relics: u32,
    pub(super) arcane_residue: u32,
}

#[derive(Debug, Serialize, Default, Clone)]
pub(super) struct SimulationResourceNetSnapshot {
    pub(super) gold: i64,
    pub(super) tower_materials: i64,
    pub(super) eggs: i64,
    pub(super) relics: i64,
    pub(super) arcane_residue: i64,
}

#[derive(Debug, Serialize, Clone)]
pub(super) struct SimulationSurplusSummary {
    pub(super) starting_resources: SimulationResourcesSnapshot,
    pub(super) ending_resources: SimulationResourcesSnapshot,
    pub(super) net_change: SimulationResourceNetSnapshot,
    pub(super) debt_gold_gap: i64,
    pub(super) upkeep_shortfall_gold: u32,
    pub(super) expedition_prep_shortfall: u32,
}

#[derive(Debug, Serialize)]
pub(super) struct SimulationDebtSnapshot {
    pub(super) active_milestone_id: String,
    pub(super) current_balance_due: u32,
    pub(super) days_until_due: u32,
    pub(super) missed_payment_count: u32,
    pub(super) status_message: String,
}

#[derive(Debug, Serialize, Default, Clone)]
pub(super) struct GuestTierMetrics {
    pub(super) tier: u32,
    pub(super) active: usize,
    pub(super) pending: usize,
    pub(super) accepted: usize,
    pub(super) generated: usize,
    pub(super) completed: usize,
    pub(super) expired: usize,
}

#[derive(Debug, Serialize, Default, Clone)]
pub(super) struct GuestPressureMetrics {
    pub(super) active: usize,
    pub(super) pending: usize,
    pub(super) accepted: usize,
    pub(super) generated: usize,
    pub(super) completed: usize,
    pub(super) expired: usize,
    pub(super) rejected: usize,
    pub(super) next_deadline_days: Option<u32>,
    pub(super) by_tier: Vec<GuestTierMetrics>,
}

#[derive(Debug, Serialize, Default, Clone)]
pub(super) struct UpkeepForecastSnapshot {
    pub(super) food_gold: u32,
    pub(super) cleaning_gold: u32,
    pub(super) maintenance_gold: u32,
    pub(super) total_gold: u32,
    pub(super) active_band_min_girls: u32,
    pub(super) active_band_min_patron_tiers: u32,
    pub(super) next_girl_total_gold: u32,
    pub(super) next_girl_delta_gold: u32,
    pub(super) next_building_total_gold: u32,
    pub(super) next_building_delta_gold: u32,
}

#[derive(Debug, Serialize, Default, Clone)]
pub(super) struct ExpeditionOpportunityMetrics {
    pub(super) attempted: bool,
    pub(super) mission_id: Option<String>,
    pub(super) reward_focus: Option<String>,
    pub(super) egg_focused: bool,
    pub(super) prep_gold: u32,
    pub(super) prep_materials: u32,
    pub(super) prep_arcane_residue: u32,
    pub(super) prep_shortfall: u32,
    pub(super) unavailable_girls: usize,
    pub(super) accepted_guest_locks: usize,
    pub(super) pending_guest_pressure: usize,
    pub(super) missed_guest_deadlines: usize,
}

#[derive(Debug, Serialize)]
pub(super) struct SimulationMilestoneSnapshot {
    pub(super) day: u32,
    pub(super) girls: usize,
    pub(super) population_cap: usize,
    pub(super) town_job_limit: u8,
    pub(super) buildings: usize,
    pub(super) rooms: usize,
    pub(super) patron_tiers: usize,
    pub(super) reputation_proxy: usize,
    pub(super) active_contracts: usize,
    pub(super) resources: SimulationResourcesSnapshot,
    pub(super) debt: Option<SimulationDebtSnapshot>,
    pub(super) upkeep_forecast: UpkeepForecastSnapshot,
}

#[derive(Debug, Serialize)]
pub(super) struct SimulationDayReport {
    pub(super) day: u32,
    pub(super) girls: usize,
    pub(super) population_cap: usize,
    pub(super) eggs_in_inventory: usize,
    pub(super) buildings: usize,
    pub(super) rooms: usize,
    pub(super) patron_tiers: usize,
    pub(super) active_contracts: usize,
    pub(super) average_bond: f64,
    pub(super) average_reputation: f64,
    pub(super) graded_eggs: usize,
    pub(super) role_diversity: usize,
    pub(super) town_projects: usize,
    pub(super) active_situations: usize,
    pub(super) guild_job_workers_assigned: usize,
    pub(super) expedition_members_assigned: usize,
    pub(super) guest_bookings_assigned: usize,
    pub(super) hatches: usize,
    pub(super) buildings_purchased: usize,
    pub(super) guild_job_gold: u32,
    pub(super) guild_job_arcane_residue: u32,
    pub(super) expedition_prep_gold: u32,
    pub(super) expedition_prep_materials: u32,
    pub(super) expedition_prep_arcane_residue: u32,
    pub(super) expedition_prep_shortfall: u32,
    pub(super) upkeep_food_gold: u32,
    pub(super) upkeep_cleaning_gold: u32,
    pub(super) upkeep_maintenance_gold: u32,
    pub(super) upkeep_gold: u32,
    pub(super) upkeep_shortfall: u32,
    pub(super) upkeep_forecast: UpkeepForecastSnapshot,
    pub(super) special_event_gold_delta: i32,
    pub(super) special_event_count: u32,
    pub(super) expedition_materials: u32,
    pub(super) expedition_arcane_residue: u32,
    pub(super) expedition_eggs: u32,
    pub(super) expedition_relics: u32,
    pub(super) expedition_opportunity: ExpeditionOpportunityMetrics,
    pub(super) guest_completions: usize,
    pub(super) guest_expirations: usize,
    pub(super) guest_pressure: GuestPressureMetrics,
    pub(super) resources: SimulationResourcesSnapshot,
    pub(super) debt: Option<SimulationDebtSnapshot>,
    pub(super) day_summary: Vec<String>,
}

#[derive(Debug, Serialize)]
pub(super) struct SimulationReport {
    pub(super) rng_seed: u64,
    pub(super) simulation_days: u32,
    pub(super) starting_day: u32,
    pub(super) ending_day: u32,
    pub(super) opening_event_log_entries: usize,
    pub(super) final_event_log_entries: usize,
    pub(super) final_girls: usize,
    pub(super) final_buildings: usize,
    pub(super) final_active_contracts: usize,
    pub(super) final_average_bond: f64,
    pub(super) final_average_reputation: f64,
    pub(super) final_graded_eggs: usize,
    pub(super) final_role_diversity: usize,
    pub(super) final_town_projects: usize,
    pub(super) total_hatches: usize,
    pub(super) total_buildings_purchased: usize,
    pub(super) total_guest_completions: usize,
    pub(super) total_guest_expirations: usize,
    pub(super) total_contracts_generated: usize,
    pub(super) total_contracts_rejected: usize,
    pub(super) total_guild_job_gold: u32,
    pub(super) total_expedition_prep_gold: u32,
    pub(super) total_expedition_prep_materials: u32,
    pub(super) total_expedition_prep_arcane_residue: u32,
    pub(super) total_expedition_prep_shortfall: u32,
    pub(super) total_upkeep_food_gold: u32,
    pub(super) total_upkeep_cleaning_gold: u32,
    pub(super) total_upkeep_maintenance_gold: u32,
    pub(super) total_upkeep_gold: u32,
    pub(super) total_upkeep_shortfall: u32,
    pub(super) total_special_event_gold_delta: i32,
    pub(super) total_special_event_count: u32,
    pub(super) total_expedition_days: u32,
    pub(super) total_egg_focused_expedition_days: u32,
    pub(super) expedition_days_after_day_90: u32,
    pub(super) egg_reward_days: u32,
    pub(super) egg_reward_days_after_day_90: u32,
    pub(super) last_expedition_day: Option<u32>,
    pub(super) total_expedition_eggs: u32,
    pub(super) final_resources: SimulationResourcesSnapshot,
    pub(super) final_debt: Option<SimulationDebtSnapshot>,
    pub(super) final_upkeep_forecast: UpkeepForecastSnapshot,
    pub(super) surplus_summary: SimulationSurplusSummary,
    pub(super) milestone_snapshots: Vec<SimulationMilestoneSnapshot>,
    pub(super) per_day: Vec<SimulationDayReport>,
}

#[derive(Debug, Serialize)]
pub(super) struct MultiSeedSimulationSample {
    pub(super) sample: usize,
    pub(super) rng_seed: u64,
    pub(super) girls: usize,
    pub(super) buildings: usize,
    pub(super) gold: u32,
    pub(super) debt_gap: i64,
    pub(super) relics: u32,
    pub(super) arcane_residue: u32,
    pub(super) missed_payments: u32,
    pub(super) debt_milestone_id: Option<String>,
    pub(super) debt_status: String,
    pub(super) max_active_requests: usize,
    pub(super) expirations: usize,
}

#[derive(Debug, Serialize)]
pub(super) struct NumericRangeSummary<T> {
    pub(super) average: f64,
    pub(super) min: T,
    pub(super) max: T,
}

#[derive(Debug, Serialize)]
pub(super) struct MultiSeedSimulationSummary {
    pub(super) simulation_days: u32,
    pub(super) samples: Vec<MultiSeedSimulationSample>,
    pub(super) girls: NumericRangeSummary<usize>,
    pub(super) buildings: NumericRangeSummary<usize>,
    pub(super) gold: NumericRangeSummary<u32>,
    pub(super) debt_gap: NumericRangeSummary<i64>,
    pub(super) relics: NumericRangeSummary<u32>,
    pub(super) arcane_residue: NumericRangeSummary<u32>,
    pub(super) missed_payments: NumericRangeSummary<u32>,
    pub(super) max_active_requests: NumericRangeSummary<usize>,
    pub(super) expirations: NumericRangeSummary<usize>,
}

#[derive(Debug, Default)]
pub(super) struct DailyPolicyMetrics {
    pub(super) hatches: usize,
    pub(super) buildings_purchased: usize,
    pub(super) guest_bookings_assigned: usize,
    pub(super) guild_job_workers_assigned: usize,
    pub(super) expedition_members_assigned: usize,
    pub(super) expedition_mission_id: Option<String>,
    pub(super) expedition_reward_focus: Option<String>,
}

#[derive(Debug, Clone)]
pub(super) struct GuestRequestStartSnapshot {
    pub(super) request_id: String,
    pub(super) status: ContractStatus,
    pub(super) tier: u32,
    pub(super) deadline_day: u32,
}

pub(super) fn average_bond(game_state: &GameState) -> f64 {
    if game_state.monsters.is_empty() {
        0.0
    } else {
        game_state
            .monsters
            .iter()
            .map(|monster| monster.bond as f64)
            .sum::<f64>()
            / game_state.monsters.len() as f64
    }
}

pub(super) fn average_reputation(game_state: &GameState) -> f64 {
    if game_state.monsters.is_empty() {
        0.0
    } else {
        game_state
            .monsters
            .iter()
            .map(|monster| monster.reputation as f64)
            .sum::<f64>()
            / game_state.monsters.len() as f64
    }
}

pub(super) fn graded_egg_count(game_state: &GameState) -> usize {
    game_state
        .egg_inventory
        .iter()
        .filter(|egg| egg.grade_score > 1)
        .count()
}

pub(super) fn role_diversity(game_state: &GameState) -> usize {
    let mut roles = game_state
        .monsters
        .iter()
        .map(monster_validation_role)
        .collect::<Vec<_>>();
    roles.sort_unstable();
    roles.dedup();
    roles.len()
}

fn monster_validation_role(monster: &CompanionState) -> &'static str {
    if monster.corruption >= 10 || monster.trait_ids.iter().any(|id| id == "corruption_tuned") {
        "corruption"
    } else if monster.work_history.hatchery_assists > 0
        || monster.trait_ids.iter().any(|id| id == "hatchery_attuned")
    {
        "hatchery"
    } else if monster.skills.charm >= 2 || monster.stats.charm >= monster.stats.power + 2 {
        "performer"
    } else if monster.stats.power >= monster.stats.charm + 2 {
        "delver"
    } else if monster.bond >= 8 || monster.trait_ids.iter().any(|id| id == "submissive") {
        "comfort"
    } else {
        "versatile"
    }
}

pub(super) fn max_active_contracts(report: &SimulationReport) -> usize {
    report
        .per_day
        .iter()
        .map(|day| day.active_contracts)
        .max()
        .unwrap_or(0)
}

pub(super) fn summarize_usize<I>(values: I) -> NumericRangeSummary<usize>
where
    I: Iterator<Item = usize>,
{
    let values = values.collect::<Vec<_>>();
    NumericRangeSummary {
        average: values.iter().sum::<usize>() as f64 / values.len() as f64,
        min: values.iter().copied().min().unwrap_or(0),
        max: values.iter().copied().max().unwrap_or(0),
    }
}

pub(super) fn summarize_u32<I>(values: I) -> NumericRangeSummary<u32>
where
    I: Iterator<Item = u32>,
{
    let values = values.collect::<Vec<_>>();
    NumericRangeSummary {
        average: values.iter().sum::<u32>() as f64 / values.len() as f64,
        min: values.iter().copied().min().unwrap_or(0),
        max: values.iter().copied().max().unwrap_or(0),
    }
}

pub(super) fn summarize_i64<I>(values: I) -> NumericRangeSummary<i64>
where
    I: Iterator<Item = i64>,
{
    let values = values.collect::<Vec<_>>();
    NumericRangeSummary {
        average: values.iter().sum::<i64>() as f64 / values.len() as f64,
        min: values.iter().copied().min().unwrap_or(0),
        max: values.iter().copied().max().unwrap_or(0),
    }
}

pub(super) fn build_day_report(
    data: &GameData,
    game_state: &GameState,
    summary: &crate::state::DayResolutionSummary,
    policy_metrics: &DailyPolicyMetrics,
    request_start: &[GuestRequestStartSnapshot],
    guest_completions: usize,
    guest_expirations: usize,
) -> SimulationDayReport {
    let mut day_summary = Vec::new();
    day_summary.extend(summary.debt_updates.iter().cloned());
    day_summary.extend(summary.contract_updates.iter().cloned());
    day_summary.extend(summary.roster_updates.iter().cloned());
    day_summary.extend(summary.special_event_lines.iter().cloned());
    day_summary.extend(summary.event_lines.iter().cloned());

    SimulationDayReport {
        day: summary.resolved_day,
        girls: game_state.monsters.len(),
        population_cap: day_cycle::effective_population_cap(data, game_state),
        eggs_in_inventory: game_state.egg_inventory.len(),
        buildings: game_state.town.constructed_building_ids.len(),
        rooms: game_state.town.unlocked_room_ids.len(),
        patron_tiers: game_state.town.patron_tiers.len(),
        active_contracts: game_state.active_contracts.len(),
        average_bond: average_bond(game_state),
        average_reputation: average_reputation(game_state),
        graded_eggs: graded_egg_count(game_state),
        role_diversity: role_diversity(game_state),
        town_projects: game_state.town.completed_project_ids.len(),
        active_situations: game_state.town.active_situations.len(),
        guild_job_workers_assigned: policy_metrics.guild_job_workers_assigned,
        expedition_members_assigned: policy_metrics.expedition_members_assigned,
        guest_bookings_assigned: policy_metrics.guest_bookings_assigned,
        hatches: policy_metrics.hatches,
        buildings_purchased: policy_metrics.buildings_purchased,
        guild_job_gold: summary.guild_job_gold,
        guild_job_arcane_residue: summary.guild_job_arcane_residue,
        expedition_prep_gold: summary.expedition_prep_gold,
        expedition_prep_materials: summary.expedition_prep_materials,
        expedition_prep_arcane_residue: summary.expedition_prep_arcane_residue,
        expedition_prep_shortfall: summary.expedition_prep_shortfall,
        upkeep_food_gold: summary.upkeep_food_gold,
        upkeep_cleaning_gold: summary.upkeep_cleaning_gold,
        upkeep_maintenance_gold: summary.upkeep_maintenance_gold,
        upkeep_gold: summary.upkeep_gold,
        upkeep_shortfall: summary.upkeep_shortfall,
        upkeep_forecast: upkeep_forecast_snapshot(data, game_state),
        special_event_gold_delta: summary.special_event_gold_delta,
        special_event_count: summary.special_event_count,
        expedition_materials: summary.expedition_materials,
        expedition_arcane_residue: summary.expedition_arcane_residue,
        expedition_eggs: summary.expedition_eggs,
        expedition_relics: summary.expedition_relics,
        expedition_opportunity: expedition_opportunity_metrics(
            policy_metrics,
            request_start,
            summary.resolved_day,
            summary,
        ),
        guest_completions,
        guest_expirations,
        guest_pressure: guest_pressure_metrics(data, game_state, request_start, summary),
        resources: resources_snapshot(game_state),
        debt: debt_snapshot(game_state),
        day_summary,
    }
}

pub(super) fn request_start_snapshot(
    data: &GameData,
    game_state: &GameState,
) -> Vec<GuestRequestStartSnapshot> {
    game_state
        .active_contracts
        .iter()
        .map(|request| GuestRequestStartSnapshot {
            request_id: request.request_id.clone(),
            status: request.status.clone(),
            tier: contract_tier(data, request),
            deadline_day: request.deadline_day,
        })
        .collect()
}

pub(super) fn guest_pressure_metrics(
    data: &GameData,
    game_state: &GameState,
    request_start: &[GuestRequestStartSnapshot],
    summary: &crate::state::DayResolutionSummary,
) -> GuestPressureMetrics {
    let generated = summary.contracts_generated;
    let completed = request_start
        .iter()
        .filter(|start| {
            matches!(start.status, ContractStatus::Accepted)
                && !game_state
                    .active_contracts
                    .iter()
                    .any(|request| request.request_id == start.request_id)
        })
        .count();
    let expired = request_start
        .iter()
        .filter(|start| {
            matches!(start.status, ContractStatus::Pending)
                && !game_state
                    .active_contracts
                    .iter()
                    .any(|request| request.request_id == start.request_id)
        })
        .count();
    let pending = game_state
        .active_contracts
        .iter()
        .filter(|request| matches!(request.status, ContractStatus::Pending))
        .count();
    let accepted = game_state
        .active_contracts
        .iter()
        .filter(|request| matches!(request.status, ContractStatus::Accepted))
        .count();
    let next_deadline_days = game_state
        .active_contracts
        .iter()
        .map(|request| request.deadline_day.saturating_sub(game_state.current_day))
        .min();

    GuestPressureMetrics {
        active: game_state.active_contracts.len(),
        pending,
        accepted,
        generated,
        completed,
        expired,
        rejected: summary.contracts_rejected,
        next_deadline_days,
        by_tier: guest_pressure_by_tier(data, game_state, request_start, summary),
    }
}

pub(super) fn guest_pressure_by_tier(
    data: &GameData,
    game_state: &GameState,
    request_start: &[GuestRequestStartSnapshot],
    summary: &crate::state::DayResolutionSummary,
) -> Vec<GuestTierMetrics> {
    let mut tiers = Vec::<GuestTierMetrics>::new();
    for request in &game_state.active_contracts {
        let tier = contract_tier(data, request);
        let metrics = tier_metrics_mut(&mut tiers, tier);
        metrics.active += 1;
        match request.status {
            ContractStatus::Pending => metrics.pending += 1,
            ContractStatus::Accepted => metrics.accepted += 1,
            ContractStatus::Completed | ContractStatus::Failed | ContractStatus::Declined => {}
        }
        if !request_start
            .iter()
            .any(|start| start.request_id == request.request_id)
        {
            metrics.generated += 1;
        }
    }
    for start in request_start {
        if game_state
            .active_contracts
            .iter()
            .any(|request| request.request_id == start.request_id)
        {
            continue;
        }
        let metrics = tier_metrics_mut(&mut tiers, start.tier);
        match start.status {
            ContractStatus::Accepted => metrics.completed += 1,
            ContractStatus::Pending => metrics.expired += 1,
            ContractStatus::Completed | ContractStatus::Failed | ContractStatus::Declined => {}
        }
    }
    let tier_generated = tiers.iter().map(|metrics| metrics.generated).sum::<usize>();
    if summary.contracts_generated > tier_generated {
        let metrics = tier_metrics_mut(&mut tiers, 0);
        metrics.generated += summary.contracts_generated - tier_generated;
    }
    tiers.sort_by_key(|metrics| metrics.tier);
    tiers
}

pub(super) fn tier_metrics_mut(
    metrics: &mut Vec<GuestTierMetrics>,
    tier: u32,
) -> &mut GuestTierMetrics {
    if let Some(index) = metrics.iter().position(|entry| entry.tier == tier) {
        return &mut metrics[index];
    }
    metrics.push(GuestTierMetrics {
        tier,
        ..GuestTierMetrics::default()
    });
    metrics
        .last_mut()
        .expect("tier metrics should contain inserted value")
}

pub(super) fn contract_tier(data: &GameData, request: &crate::state::ContractState) -> u32 {
    data.guild_rooms
        .rooms
        .iter()
        .find(|room| room.id == request.requested_room_id)
        .map(|room| room.service_tier)
        .unwrap_or(0)
        .into()
}

pub(super) fn upkeep_forecast_snapshot(
    data: &GameData,
    game_state: &GameState,
) -> UpkeepForecastSnapshot {
    let forecast = day_cycle::preview_upkeep(data, game_state);
    UpkeepForecastSnapshot {
        food_gold: forecast.food_gold,
        cleaning_gold: forecast.cleaning_gold,
        maintenance_gold: forecast.maintenance_gold,
        total_gold: forecast.total_gold,
        active_band_min_girls: forecast.active_band_min_girls,
        active_band_min_patron_tiers: forecast.active_band_min_patron_tiers,
        next_girl_total_gold: forecast.next_girl_total_gold,
        next_girl_delta_gold: forecast.next_girl_delta_gold,
        next_building_total_gold: forecast.next_building_total_gold,
        next_building_delta_gold: forecast.next_building_delta_gold,
    }
}

pub(super) fn expedition_opportunity_metrics(
    policy_metrics: &DailyPolicyMetrics,
    request_start: &[GuestRequestStartSnapshot],
    resolved_day: u32,
    summary: &crate::state::DayResolutionSummary,
) -> ExpeditionOpportunityMetrics {
    let accepted_guest_locks = request_start
        .iter()
        .filter(|request| matches!(request.status, ContractStatus::Accepted))
        .count();
    let pending_guest_pressure = request_start
        .iter()
        .filter(|request| matches!(request.status, ContractStatus::Pending))
        .count();
    let missed_guest_deadlines = request_start
        .iter()
        .filter(|request| {
            matches!(request.status, ContractStatus::Pending)
                && request.deadline_day <= resolved_day
        })
        .count();

    ExpeditionOpportunityMetrics {
        attempted: policy_metrics.expedition_members_assigned > 0,
        mission_id: policy_metrics.expedition_mission_id.clone(),
        reward_focus: policy_metrics.expedition_reward_focus.clone(),
        egg_focused: policy_metrics.expedition_reward_focus.as_deref() == Some("eggs"),
        prep_gold: summary.expedition_prep_gold,
        prep_materials: summary.expedition_prep_materials,
        prep_arcane_residue: summary.expedition_prep_arcane_residue,
        prep_shortfall: summary.expedition_prep_shortfall,
        unavailable_girls: accepted_guest_locks
            .saturating_add(policy_metrics.expedition_members_assigned),
        accepted_guest_locks,
        pending_guest_pressure,
        missed_guest_deadlines,
    }
}

pub(super) fn milestone_snapshot(
    data: &GameData,
    game_state: &GameState,
    day: u32,
) -> SimulationMilestoneSnapshot {
    SimulationMilestoneSnapshot {
        day,
        girls: game_state.monsters.len(),
        population_cap: day_cycle::effective_population_cap(data, game_state),
        town_job_limit: game_state.town.town_job_limit,
        buildings: game_state.town.constructed_building_ids.len(),
        rooms: game_state.town.unlocked_room_ids.len(),
        patron_tiers: game_state.town.patron_tiers.len(),
        reputation_proxy: game_state.town.patron_tiers.len(),
        active_contracts: game_state.active_contracts.len(),
        resources: resources_snapshot(game_state),
        debt: debt_snapshot(game_state),
        upkeep_forecast: upkeep_forecast_snapshot(data, game_state),
    }
}

pub(super) fn resources_snapshot(game_state: &GameState) -> SimulationResourcesSnapshot {
    SimulationResourcesSnapshot {
        gold: game_state.resources.gold,
        tower_materials: game_state.resources.tower_materials,
        eggs: game_state.resources.eggs,
        relics: game_state.resources.relics,
        arcane_residue: game_state.resources.arcane_residue,
    }
}

pub(super) fn surplus_summary(
    starting_resources: SimulationResourcesSnapshot,
    game_state: &GameState,
    upkeep_shortfall_gold: u32,
    expedition_prep_shortfall: u32,
) -> SimulationSurplusSummary {
    let ending_resources = resources_snapshot(game_state);
    let debt_gold_gap = game_state.debt.as_ref().map_or(0, |debt| {
        i64::from(ending_resources.gold) - i64::from(debt.current_balance_due)
    });

    SimulationSurplusSummary {
        net_change: SimulationResourceNetSnapshot {
            gold: i64::from(ending_resources.gold) - i64::from(starting_resources.gold),
            tower_materials: i64::from(ending_resources.tower_materials)
                - i64::from(starting_resources.tower_materials),
            eggs: i64::from(ending_resources.eggs) - i64::from(starting_resources.eggs),
            relics: i64::from(ending_resources.relics) - i64::from(starting_resources.relics),
            arcane_residue: i64::from(ending_resources.arcane_residue)
                - i64::from(starting_resources.arcane_residue),
        },
        starting_resources,
        ending_resources,
        debt_gold_gap,
        upkeep_shortfall_gold,
        expedition_prep_shortfall,
    }
}

pub(super) fn debt_snapshot(game_state: &GameState) -> Option<SimulationDebtSnapshot> {
    game_state.debt.as_ref().map(|debt| SimulationDebtSnapshot {
        active_milestone_id: debt.active_milestone_id.clone(),
        current_balance_due: debt.current_balance_due,
        days_until_due: debt.days_until_due,
        missed_payment_count: debt.missed_payment_count,
        status_message: debt.status_message.clone(),
    })
}

pub(super) fn write_simulation_report<T>(report: &T) -> PathBuf
where
    T: Serialize,
{
    write_named_simulation_report(report, "thirty_day_simulation_report.json")
}

pub(super) fn write_named_simulation_report<T>(report: &T, file_name: &str) -> PathBuf
where
    T: Serialize,
{
    let report_path = PathBuf::from("tmp_screens")
        .join("playtests")
        .join(file_name);
    let report_dir = report_path
        .parent()
        .expect("simulation report path should have a parent directory");
    fs::create_dir_all(report_dir).expect("simulation report directory should be creatable");
    let payload = serde_json::to_string_pretty(report).expect("simulation report should serialize");
    fs::write(&report_path, payload).expect("simulation report should write");
    report_path
}
