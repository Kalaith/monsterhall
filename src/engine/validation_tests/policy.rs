use super::*;

pub(super) fn play_opening_sequence(data: &GameData, game_state: &mut GameState) {
    advance_opening_step(data, game_state).expect("camp step should advance");
    advance_opening_step(data, game_state).expect("discovery step should advance");
    advance_opening_step(data, game_state).expect("incubation step should advance");
    advance_opening_step(data, game_state).expect("hatch step should advance");
    build_first_room(data, game_state).expect("first room should build");
    resolve_first_client(data, game_state).expect("first client should resolve");
    initialize_first_debt(data, game_state).expect("first debt should initialize");
    refresh_contracts(data, game_state).expect("contracts should refresh");
}

pub(super) fn run_daily_policy(data: &GameData, game_state: &mut GameState) -> DailyPolicyMetrics {
    let mut metrics = DailyPolicyMetrics::default();
    metrics.hatches += hatch_affordable_eggs(data, game_state);
    metrics.buildings_purchased += purchase_priority_buildings(data, game_state);
    assign_guest_bookings(data, game_state);
    metrics.guest_bookings_assigned = accepted_guest_booking_count(game_state);
    let (guild_job_workers, expedition_members) = assign_daily_jobs(data, game_state);
    metrics.guild_job_workers_assigned = guild_job_workers;
    metrics.expedition_members_assigned = expedition_members;
    if let Some(expedition) = &game_state.active_expedition {
        metrics.expedition_mission_id = Some(expedition.mission_id.clone());
        metrics.expedition_reward_focus = data
            .missions
            .missions
            .iter()
            .find(|mission| mission.id == expedition.mission_id)
            .map(|mission| mission.reward_focus.clone());
    }
    metrics
}

pub(super) fn purchase_priority_buildings(data: &GameData, game_state: &mut GameState) -> usize {
    let build_order = [
        "slime_pool",
        "aftercare_lounge",
        "healing_hot_springs",
        "residue_alchemy_bench",
        "silk_rope_forge",
        "hatchery_scrying_pool",
        "warm_love_nest",
        "monster_kink_archive",
        "tower_route_cartography",
        "relic_residue_condenser",
        "luxury_room_renovation",
        "prestige_hospitality_wing",
    ];
    let mut purchased = 0usize;

    for building_id in build_order {
        let Some(building) = data
            .buildings
            .buildings
            .iter()
            .find(|entry| entry.id == building_id)
        else {
            continue;
        };

        if game_state
            .town
            .constructed_building_ids
            .iter()
            .filter(|id| *id == building_id)
            .count()
            >= usize::from(building.build_limit)
        {
            continue;
        }

        if is_late_game_sink_building(building)
            && game_state.monsters.len() < day_cycle::effective_population_cap(data, game_state)
        {
            continue;
        }
        if is_late_game_sink_building(building)
            && !can_spend_on_late_game_sink(data, game_state, building.cost.gold)
        {
            continue;
        }

        let added_income_units = projected_building_growth_units(building);
        if !can_make_growth_investment(
            game_state,
            building.cost.gold,
            added_income_units,
            GrowthInvestmentKind::Building,
        ) {
            continue;
        }

        if purchase_building(data, game_state, building_id).is_ok() {
            purchased += 1;
        }
    }

    purchased
}

pub(super) fn is_late_game_sink_building(building: &crate::data::BuildingData) -> bool {
    matches!(building.category.as_str(), "prestige" | "project")
        && building.unlocks.room_ids.is_empty()
        && building.unlocks.floor_ids.is_empty()
        && building.unlocks.species_ids.is_empty()
        && building.unlocks.patron_tiers.is_empty()
}

pub(super) fn can_spend_on_late_game_sink(
    data: &GameData,
    game_state: &GameState,
    gold_cost: u32,
) -> bool {
    let scheduled_debt_reserve = total_scheduled_debt_for_validation(data) / 10;
    let Some(debt) = game_state.debt.as_ref() else {
        return game_state.resources.gold.saturating_sub(gold_cost) >= scheduled_debt_reserve;
    };
    if debt.active_milestone_id == "founders_due_7" {
        return game_state.resources.gold.saturating_sub(gold_cost)
            >= debt
                .current_balance_due
                .saturating_add(scheduled_debt_reserve);
    }
    if debt.days_until_due > 21 {
        return true;
    }
    if gold_cost <= 750
        && game_state.resources.gold >= debt.current_balance_due.saturating_mul(4).saturating_div(5)
    {
        return true;
    }
    game_state.resources.gold.saturating_sub(gold_cost) >= debt.current_balance_due
}

pub(super) fn total_scheduled_debt_for_validation(data: &GameData) -> u32 {
    data.debt_milestones
        .milestones
        .iter()
        .map(|milestone| milestone.amount_due)
        .sum()
}

pub(super) fn hatch_affordable_eggs(data: &GameData, game_state: &mut GameState) -> usize {
    let mut hatch_count = 0usize;
    loop {
        let mut hatched_any = false;
        let egg_ids = game_state
            .egg_inventory
            .iter()
            .map(|egg| egg.id.clone())
            .collect::<Vec<_>>();

        for egg_id in egg_ids {
            let Some(egg) = game_state
                .egg_inventory
                .iter()
                .find(|entry| entry.id == egg_id)
                .cloned()
            else {
                continue;
            };
            let Some(species_id) =
                cheapest_unlocked_species_option(data, game_state, &egg.possible_species_ids)
                    .map(str::to_owned)
            else {
                continue;
            };
            let Some(species) = data
                .species
                .species
                .iter()
                .find(|entry| entry.id == species_id)
            else {
                continue;
            };

            if game_state.monsters.len() >= day_cycle::effective_population_cap(data, game_state) {
                if let Some((replacement_id, replacement_species_id)) =
                    replacement_plan_for_egg(data, game_state, &egg)
                {
                    let Some(replacement_species) = data
                        .species
                        .species
                        .iter()
                        .find(|entry| entry.id == replacement_species_id)
                    else {
                        continue;
                    };
                    if can_make_growth_investment(
                        game_state,
                        replacement_species.hatching_cost.gold,
                        0,
                        GrowthInvestmentKind::Hatch,
                    ) && replace_monster_with_selected_egg(
                        data,
                        game_state,
                        &egg_id,
                        Some(&replacement_species_id),
                        &replacement_id,
                    )
                    .is_ok()
                    {
                        hatched_any = true;
                        hatch_count += 1;
                    }
                } else if convert_egg(data, game_state, &egg_id, EggConversionKind::Refine)
                    .or_else(|_| convert_egg(data, game_state, &egg_id, EggConversionKind::Sell))
                    .is_ok()
                {
                    hatched_any = true;
                }
                continue;
            }

            if !has_unfilled_workforce_demand(game_state) {
                continue;
            }

            if !hatch_pacing_allows(game_state, hatch_count) {
                continue;
            }

            if !can_make_growth_investment(
                game_state,
                species.hatching_cost.gold,
                1,
                GrowthInvestmentKind::Hatch,
            ) {
                continue;
            }

            if hatch_selected_egg(data, game_state, &egg_id, Some(&species_id)).is_ok() {
                hatched_any = true;
                hatch_count += 1;
            }
        }

        if !hatched_any {
            break;
        }
    }
    hatch_count
}

fn replacement_plan_for_egg(
    data: &GameData,
    game_state: &GameState,
    egg: &crate::state::EggState,
) -> Option<(String, String)> {
    let new_quality = egg_quality_rank_for_policy(egg.grade_score);
    egg.possible_species_ids
        .iter()
        .filter(|species_id| {
            game_state
                .town
                .unlocked_species_ids
                .iter()
                .any(|unlocked_id| unlocked_id == *species_id)
        })
        .filter_map(|species_id| {
            let replacement =
                replacement_candidate_for_species(game_state, species_id, new_quality)?;
            let species = data
                .species
                .species
                .iter()
                .find(|entry| entry.id == *species_id)?;
            let cost_score = species.hatching_cost.gold
                + species.hatching_cost.tower_materials
                + species.hatching_cost.arcane_residue
                + species.hatching_cost.relics * 100;
            Some((
                replacement.id.clone(),
                species_id.clone(),
                species_count(game_state, species_id),
                replacement.quality_rank,
                cost_score,
            ))
        })
        .min_by_key(|(_, _, count, old_quality, cost)| (*count, *old_quality, *cost))
        .map(|(replacement_id, species_id, _, _, _)| (replacement_id, species_id))
}

fn replacement_candidate_for_species<'a>(
    game_state: &'a GameState,
    species_id: &str,
    new_quality: u8,
) -> Option<&'a crate::state::CompanionState> {
    if let Some(upgrade) = game_state
        .monsters
        .iter()
        .filter(|monster| monster.species_id == species_id && monster.quality_rank < new_quality)
        .min_by_key(|monster| (monster.quality_rank, monster_service_score(monster)))
    {
        return Some(upgrade);
    }

    if species_count(game_state, species_id) == 0 {
        return game_state
            .monsters
            .iter()
            .filter(|monster| species_count(game_state, &monster.species_id) > 1)
            .min_by_key(|monster| (monster.quality_rank, monster_service_score(monster)));
    }

    None
}

fn egg_quality_rank_for_policy(grade_score: u32) -> u8 {
    match grade_score {
        0..=2 => 1,
        3..=4 => 2,
        _ => 3,
    }
}

fn species_count(game_state: &GameState, species_id: &str) -> usize {
    game_state
        .monsters
        .iter()
        .filter(|monster| monster.species_id == species_id)
        .count()
}

pub(super) fn hatch_pacing_allows(game_state: &GameState, hatches_this_policy: usize) -> bool {
    if hatches_this_policy > 0 {
        return false;
    }

    if game_state.monsters.len() < 6 {
        return game_state.current_day % 3 == 0;
    }

    if game_state.monsters.len() >= 12 {
        let late_campaign_cadence = if game_state.current_day >= 240 {
            12
        } else {
            18
        };
        return game_state.current_day % late_campaign_cadence == 0;
    }

    let mid_campaign_cadence = if game_state.current_day <= 30 { 12 } else { 6 };
    game_state.current_day % mid_campaign_cadence == 0
}

pub(super) fn assign_guest_bookings(data: &GameData, game_state: &mut GameState) {
    let pending_request_ids = game_state
        .active_contracts
        .iter()
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
                && assign_monster_to_contract(data, game_state, &request_id, &monster_id)
                    .is_ok()
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
                if !can_survive_debt_after_growth_assignment(game_state, 1) {
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

enum GrowthInvestmentKind {
    Hatch,
    Building,
}

fn can_make_growth_investment(
    game_state: &GameState,
    gold_cost: u32,
    added_income_units: u32,
    investment_kind: GrowthInvestmentKind,
) -> bool {
    if gold_cost == 0 {
        return true;
    }

    if game_state.resources.gold < gold_cost {
        return false;
    }

    if matches!(investment_kind, GrowthInvestmentKind::Hatch)
        && game_state.current_day >= 240
        && game_state.egg_inventory.len() >= 3
        && game_state.resources.gold >= gold_cost.saturating_add(200)
    {
        return true;
    }

    let Some(debt) = game_state.debt.as_ref() else {
        return true;
    };

    let post_spend_gold = game_state.resources.gold.saturating_sub(gold_cost);
    let projected_daily_income = match investment_kind {
        GrowthInvestmentKind::Hatch => growth_daily_gold_income(game_state, added_income_units),
        GrowthInvestmentKind::Building => {
            conservative_daily_gold_income(game_state, added_income_units)
        }
    };
    let urgent_buffer = if debt.days_until_due <= 2 { 12 } else { 0 };
    let projected_gold_by_due =
        post_spend_gold.saturating_add(projected_daily_income.saturating_mul(debt.days_until_due));

    projected_gold_by_due >= debt.current_balance_due.saturating_add(urgent_buffer)
}

pub(super) fn projected_building_growth_units(building: &crate::data::BuildingData) -> u32 {
    let worker_slots = building.passive_modifiers.town_job_limit_flat.max(0) as u32;
    let population_slots = building.passive_modifiers.population_cap_flat.max(0) as u32;
    let unlock_value = u32::from(
        !building.unlocks.room_ids.is_empty()
            || !building.unlocks.species_ids.is_empty()
            || !building.unlocks.patron_tiers.is_empty()
            || building.passive_modifiers.guild_income_pct >= 6
            || building.passive_modifiers.egg_discovery_flat > 0,
    );

    worker_slots
        .saturating_add(population_slots.min(1))
        .saturating_add(unlock_value)
}

pub(super) fn conservative_daily_gold_income(
    game_state: &GameState,
    added_income_units: u32,
) -> u32 {
    let active_income_units = game_state.monsters.len() as u32 + added_income_units;
    34u32.saturating_mul(active_income_units.min(8))
}

pub(super) fn growth_daily_gold_income(game_state: &GameState, added_income_units: u32) -> u32 {
    let active_income_units = game_state.monsters.len() as u32 + added_income_units;
    50u32.saturating_mul(active_income_units.min(8))
}

pub(super) fn can_spare_worker_for_growth(
    game_state: &GameState,
    reserved_guest_worker_count: usize,
) -> bool {
    let fit_workers = game_state
        .monsters
        .iter()
        .filter(|monster| monster.injury == 0 && monster.fatigue < 34 && monster.stress < 20)
        .count();

    fit_workers > reserved_guest_worker_count
}

pub(super) fn can_survive_debt_after_growth_assignment(
    game_state: &GameState,
    assigned_expedition_workers: u32,
) -> bool {
    let Some(debt) = game_state.debt.as_ref() else {
        return true;
    };

    if debt.days_until_due <= 1 && game_state.resources.gold < debt.current_balance_due {
        return false;
    }

    let income_units_after_growth_day = game_state
        .monsters
        .len()
        .saturating_sub(assigned_expedition_workers as usize)
        as u32;
    let normal_income_units = game_state.monsters.len() as u32;
    let projected_growth_day_income = 42u32.saturating_mul(income_units_after_growth_day.min(3));
    let projected_followup_income = 42u32.saturating_mul(normal_income_units.min(3));
    let projected_gold_by_due = game_state
        .resources
        .gold
        .saturating_add(projected_growth_day_income)
        .saturating_add(
            projected_followup_income.saturating_mul(debt.days_until_due.saturating_sub(1)),
        );

    projected_gold_by_due >= debt.current_balance_due
}

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
    let injury_penalty = preview.injury_risk_score.max(0) * 2;

    preview.projected_eggs as i32 * egg_value
        + preview.projected_relics as i32 * relic_value
        + preview.projected_materials as i32 * material_value
        + preview.projected_arcane_residue as i32 * residue_value
        + preview.success_score.max(0)
        - injury_penalty
}

pub(super) fn has_unfilled_workforce_demand(game_state: &GameState) -> bool {
    game_state.monsters.len() < workforce_demand(game_state)
}

pub(super) fn pending_eggs_cover_workforce_demand(game_state: &GameState) -> bool {
    game_state.monsters.len() + game_state.egg_inventory.len() >= workforce_demand(game_state)
}

pub(super) fn should_reserve_egg_expedition(game_state: &GameState) -> bool {
    game_state
        .monsters
        .len()
        .saturating_add(game_state.egg_inventory.len())
        < workforce_demand(game_state).saturating_add(2)
}

pub(super) fn workforce_demand(game_state: &GameState) -> usize {
    let guest_coverage = game_state.active_contracts.len();
    let room_coverage = usize::from(game_state.town.town_job_limit);
    let expedition_coverage = if game_state.town.unlocked_floor_ids.is_empty() {
        0
    } else {
        1
    };
    let roster_reserve = game_state
        .town
        .unlocked_room_ids
        .len()
        .saturating_sub(1)
        .saturating_add(game_state.town.patron_tiers.len().saturating_sub(1))
        .saturating_add(game_state.town.constructed_building_ids.len() / 3);

    guest_coverage
        .saturating_add(room_coverage)
        .saturating_add(expedition_coverage)
        .saturating_add(roster_reserve)
        .max(1)
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

pub(super) fn cheapest_unlocked_species_option<'a>(
    data: &'a GameData,
    game_state: &GameState,
    species_ids: &'a [String],
) -> Option<&'a str> {
    species_ids
        .iter()
        .filter(|species_id| {
            game_state
                .town
                .unlocked_species_ids
                .iter()
                .any(|unlocked_id| unlocked_id == *species_id)
        })
        .filter_map(|species_id| {
            data.species
                .species
                .iter()
                .find(|entry| entry.id == *species_id)
                .map(|species| {
                    (
                        species_id.as_str(),
                        species.hatching_cost.gold
                            + species.hatching_cost.tower_materials
                            + species.hatching_cost.arcane_residue
                            + species.hatching_cost.relics * 100,
                    )
                })
        })
        .min_by_key(|(_, score)| *score)
        .map(|(species_id, _)| species_id)
}

pub(super) fn monster_service_score(monster: &crate::state::CompanionState) -> u32 {
    monster.skills.scouting
        + monster.skills.guarding
        + monster.skills.hospitality
        + monster.skills.crafting
        + monster.skills.charm
        + monster.work_history.scouting_runs
        + monster.work_history.guard_duties
        + monster.work_history.hospitality_jobs
        + monster.work_history.craft_jobs
        + monster.work_history.contracts_completed
        + monster.stats.charm.max(0) as u32
}
