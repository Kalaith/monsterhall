//! Running a campaign and turning it into a `SimulationReport`.
//!
//! Split out of `scenarios.rs` when that file crossed the 800-line limit; the
//! assertions about what a report must contain stayed behind.

use super::scenarios::seed_simulation;
use super::*;

/// What the repeatable sinks have actually taken out of circulation.
pub(super) fn sink_absorbed(
    data: &GameData,
    game_state: &GameState,
) -> SimulationResourcesSnapshot {
    let mut total = SimulationResourcesSnapshot::default();
    for building_id in &game_state.town.constructed_building_ids {
        let Some(building) = data
            .buildings
            .buildings
            .iter()
            .find(|entry| &entry.id == building_id)
        else {
            continue;
        };
        if !matches!(building.category.as_str(), "project" | "prestige") {
            continue;
        }
        total.gold = total.gold.saturating_add(building.cost.gold);
        total.tower_materials = total
            .tower_materials
            .saturating_add(building.cost.tower_materials);
        total.relics = total.relics.saturating_add(building.cost.relics);
        total.arcane_residue = total
            .arcane_residue
            .saturating_add(building.cost.arcane_residue);
    }
    total
}

/// What they could ever take, every repeatable build bought to its limit.
///
/// The relic figure was 188 for an entire campaign against deep-tower income in
/// the thousands, and nothing in the reports said so.
pub(super) fn sink_capacity(data: &GameData) -> SimulationResourcesSnapshot {
    let mut total = SimulationResourcesSnapshot::default();
    for building in &data.buildings.buildings {
        if !matches!(building.category.as_str(), "project" | "prestige") {
            continue;
        }
        let limit = u32::from(building.build_limit);
        total.gold = total.gold.saturating_add(building.cost.gold * limit);
        total.tower_materials = total
            .tower_materials
            .saturating_add(building.cost.tower_materials * limit);
        total.relics = total.relics.saturating_add(building.cost.relics * limit);
        total.arcane_residue = total
            .arcane_residue
            .saturating_add(building.cost.arcane_residue * limit);
    }
    total
}

pub(super) fn run_simulation_report_with_seed(
    data: &GameData,
    simulation_days: u32,
    rng_seed: u64,
) -> SimulationReport {
    seed_simulation(rng_seed);
    run_simulation_report(data, simulation_days, rng_seed)
}

pub(super) fn run_simulation_report(
    data: &GameData,
    simulation_days: u32,
    rng_seed: u64,
) -> SimulationReport {
    let mut game_state = create_new_game_state(data);
    play_opening_sequence(data, &mut game_state);

    let starting_log_len = game_state.event_log.len();
    let starting_day = game_state.current_day;
    let starting_resources = resources_snapshot(&game_state);
    let mut per_day = Vec::new();
    let mut total_hatches = 0usize;
    let mut total_buildings_purchased = 0usize;
    let mut total_guest_completions = 0usize;
    let mut total_guest_expirations = 0usize;
    let mut total_contracts_generated = 0usize;
    let mut total_contracts_rejected = 0usize;
    let mut total_guild_job_gold = 0u32;
    let mut total_contract_gold = 0u32;
    let mut total_expedition_prep_gold = 0u32;
    let mut total_expedition_prep_materials = 0u32;
    let mut total_expedition_prep_arcane_residue = 0u32;
    let mut total_expedition_prep_shortfall = 0u32;
    let mut total_upkeep_wage_gold = 0u32;
    let mut total_upkeep_cleaning_gold = 0u32;
    let mut total_upkeep_maintenance_gold = 0u32;
    let mut total_upkeep_gold = 0u32;
    let mut total_upkeep_shortfall = 0u32;
    let mut total_special_event_gold_delta = 0i32;
    let mut total_special_event_count = 0u32;
    let mut total_expedition_days = 0u32;
    let mut total_egg_focused_expedition_days = 0u32;
    let mut expedition_days_after_day_90 = 0u32;
    let mut egg_reward_days = 0u32;
    let mut egg_reward_days_after_day_90 = 0u32;
    let mut last_expedition_day = None;
    let mut total_expedition_eggs = 0u32;
    let mut total_expedition_successes = 0u32;
    let mut total_expedition_failures = 0u32;
    let mut milestone_snapshots = Vec::new();

    for _ in 0..simulation_days {
        let policy_metrics = run_daily_policy(data, &mut game_state);
        let request_start = request_start_snapshot(data, &game_state);
        let resolved_day = game_state.current_day;
        let summary = resolve_day(data, &mut game_state);

        assert_eq!(summary.resolved_day, resolved_day);
        assert_eq!(
            game_state.current_day,
            resolved_day + 1,
            "simulation campaign stopped: {:?}; gold {}, debt {:?}",
            game_state.campaign_failure,
            game_state.resources.gold,
            game_state
                .debt
                .as_ref()
                .map(|debt| (debt.current_balance_due, debt.days_until_due))
        );
        assert_eq!(
            game_state.resources.eggs as usize,
            game_state.egg_inventory.len()
        );
        validate_game_state_references(data, &game_state)
            .expect("simulated day should preserve valid references");

        total_hatches += policy_metrics.hatches;
        total_buildings_purchased += policy_metrics.buildings_purchased;
        let guest_completions = count_guest_completions(&summary);
        let guest_expirations = count_guest_expirations(&summary);
        let guest_pressure = guest_pressure_metrics(data, &game_state, &request_start, &summary);
        total_guest_completions += guest_completions;
        total_guest_expirations += guest_expirations;
        total_contracts_generated += guest_pressure.generated;
        total_contracts_rejected += guest_pressure.rejected;
        total_guild_job_gold += summary.guild_job_gold;
        total_contract_gold += summary.contract_gold;
        total_expedition_prep_gold += summary.expedition_prep_gold;
        total_expedition_prep_materials += summary.expedition_prep_materials;
        total_expedition_prep_arcane_residue += summary.expedition_prep_arcane_residue;
        total_expedition_prep_shortfall += summary.expedition_prep_shortfall;
        total_upkeep_wage_gold += summary.upkeep_wage_gold;
        total_upkeep_cleaning_gold += summary.upkeep_cleaning_gold;
        total_upkeep_maintenance_gold += summary.upkeep_maintenance_gold;
        total_upkeep_gold += summary.upkeep_gold;
        total_upkeep_shortfall += summary.upkeep_shortfall;
        total_special_event_gold_delta += summary.special_event_gold_delta;
        total_special_event_count += summary.special_event_count;
        if policy_metrics.expedition_members_assigned > 0 {
            total_expedition_days += 1;
            if policy_metrics.expedition_reward_focus.as_deref() == Some("eggs") {
                total_egg_focused_expedition_days += 1;
            }
            last_expedition_day = Some(summary.resolved_day);
            if summary.resolved_day > 90 {
                expedition_days_after_day_90 += 1;
            }
        }
        if summary.expedition_eggs > 0 {
            egg_reward_days += 1;
            if summary.resolved_day > 90 {
                egg_reward_days_after_day_90 += 1;
            }
        }
        total_expedition_eggs += summary.expedition_eggs;
        total_expedition_successes += summary.expedition_successes;
        total_expedition_failures += summary.expedition_failures;
        if [30, 90, 180, 365].contains(&summary.resolved_day) {
            milestone_snapshots.push(milestone_snapshot(data, &game_state, summary.resolved_day));
        }
        per_day.push(build_day_report(
            data,
            &game_state,
            &summary,
            &policy_metrics,
            &request_start,
            guest_completions,
            guest_expirations,
        ));
        if game_state.campaign_failure.is_some() {
            break;
        }
    }

    SimulationReport {
        content_version: data.config.content_version.clone(),
        rng_seed,
        simulation_days,
        starting_day,
        ending_day: game_state.current_day,
        opening_event_log_entries: starting_log_len,
        final_event_log_entries: game_state.event_log.len(),
        final_roster_size: game_state.monsters.len(),
        final_buildings: game_state.town.constructed_building_ids.len(),
        final_unlocked_floors: game_state.town.unlocked_floor_ids.len(),
        final_stranded_floor_ids: super::super::super::depth::stranded_floor_ids(data, &game_state),
        final_active_contracts: game_state.active_contracts.len(),
        final_average_bond: average_bond(&game_state),
        final_average_reputation: average_reputation(&game_state),
        final_graded_eggs: graded_egg_count(&game_state),
        final_role_diversity: role_diversity(data, &game_state),
        final_species_counts: species_counts(&game_state),
        final_corruption_max: corruption_max(&game_state),
        final_town_projects: game_state.town.completed_project_ids.len(),
        sink_absorbed: sink_absorbed(data, &game_state),
        sink_capacity: sink_capacity(data),
        total_hatches,
        total_buildings_purchased,
        total_guest_completions,
        total_guest_expirations,
        total_contracts_generated,
        total_contracts_rejected,
        total_guild_job_gold,
        total_contract_gold,
        total_expedition_prep_gold,
        total_expedition_prep_materials,
        total_expedition_prep_arcane_residue,
        total_expedition_prep_shortfall,
        total_upkeep_wage_gold,
        total_upkeep_cleaning_gold,
        total_upkeep_maintenance_gold,
        total_upkeep_gold,
        total_upkeep_shortfall,
        total_special_event_gold_delta,
        total_special_event_count,
        total_expedition_days,
        total_egg_focused_expedition_days,
        expedition_days_after_day_90,
        egg_reward_days,
        egg_reward_days_after_day_90,
        last_expedition_day,
        total_expedition_eggs,
        total_expedition_successes,
        total_expedition_failures,
        final_resources: resources_snapshot(&game_state),
        final_debt: debt_snapshot(&game_state),
        final_campaign_failure: game_state.campaign_failure.clone(),
        final_upkeep_forecast: upkeep_forecast_snapshot(data, &game_state),
        surplus_summary: surplus_summary(
            starting_resources,
            &game_state,
            total_upkeep_shortfall,
            total_expedition_prep_shortfall,
        ),
        milestone_snapshots,
        per_day,
    }
}
