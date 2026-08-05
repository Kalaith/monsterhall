//! Validation-policy growth investment and workforce demand.
use super::*;

pub(super) enum GrowthInvestmentKind {
    Hatch,
    Building,
}

pub(super) fn can_make_growth_investment(
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
    // Once a payment has actually been missed, the simulated steward stops
    // calling discretionary purchases "growth" and holds every coin until the
    // balance can be cleared. Without this it resumed buying as soon as the
    // grace calendar looked long, guaranteeing the next miss despite the new
    // terminal campaign state.
    if debt.missed_payment_count > 0 {
        return post_spend_gold >= debt.current_balance_due;
    }
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

    if (debt.days_until_due <= 1 || debt.missed_payment_count > 0)
        && game_state.resources.gold < debt.current_balance_due
    {
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

pub(super) fn has_unfilled_workforce_demand(game_state: &GameState) -> bool {
    game_state.monsters.len() < workforce_demand(game_state)
}

pub(super) fn pending_eggs_cover_workforce_demand(game_state: &GameState) -> bool {
    game_state.monsters.len() + game_state.egg_inventory.len() >= workforce_demand(game_state)
}

pub(super) fn workforce_demand(game_state: &GameState) -> usize {
    let guest_coverage = game_state.live_contract_count();
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
