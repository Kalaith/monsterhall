//! Validation-policy building purchase decisions.
use super::policy_growth::{can_make_growth_investment, GrowthInvestmentKind};
use super::*;

pub(super) fn purchase_priority_buildings(data: &GameData, game_state: &mut GameState) -> usize {
    let build_order = [
        "slime_pool",
        "recovery_lounge",
        "recovery_baths",
        "residue_alchemy_bench",
        "forge_corner",
        "hatchery_scrying_pool",
        "nursery_habitat",
        "species_archive",
        "tower_route_cartography",
        "relic_residue_condenser",
        "reliquary_vault",
        "guild_room_renovation",
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

pub(super) fn projected_building_growth_units(building: &crate::data::BuildingData) -> u32 {
    let worker_slots = building.passive_modifiers.town_job_limit_flat.max(0) as u32;
    let population_slots = building.passive_modifiers.population_cap_flat.max(0) as u32;
    let unlock_value = u32::from(
        !building.unlocks.room_ids.is_empty()
            || !building.unlocks.species_ids.is_empty()
            || !building.unlocks.patron_tiers.is_empty()
            // Two, not six: `guild_income_pct` used to be a score term worth a
            // quarter-gold a point and was authored at four to nine. It is a real
            // percentage of the fee now and re-authored at a quarter of those
            // numbers, so a threshold left at six would rate every building in
            // the catalogue as growing the guild by nothing.
            || building.passive_modifiers.guild_income_pct >= 2
            || building.passive_modifiers.egg_discovery_flat > 0,
    );

    worker_slots
        .saturating_add(population_slots.min(1))
        .saturating_add(unlock_value)
}
