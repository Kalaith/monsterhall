//! What the guild pays each day.
//!
//! Upkeep scales on two axes at once: headcount feeds companions, and standing
//! buildings need maintaining. Both are then multiplied by whichever upkeep band
//! the guild has grown into, so expansion raises the floor under every day.

use super::*;

pub(super) fn upkeep_forecast_for_counts(
    data: &GameData,
    constructed_building_ids: &[String],
    companion_count: usize,
    patron_tier_count: usize,
    extra_building_id: Option<&str>,
) -> UpkeepForecast {
    let raw_food_gold = companion_count
        .saturating_mul(data.config.day_cycle.companion_food_gold_per_day as usize)
        as u32;
    let building_upkeep_gold = constructed_building_ids
        .iter()
        .map(String::as_str)
        .chain(extra_building_id)
        .filter_map(|building_id| {
            data.buildings
                .buildings
                .iter()
                .find(|building| building.id == building_id)
        })
        .map(|building| building_maintenance_gold(data, building))
        .sum::<u32>();
    let raw_cleaning_gold = if building_upkeep_gold == 0 {
        0
    } else {
        (building_upkeep_gold / 4).max(1)
    };
    let raw_maintenance_gold = building_upkeep_gold.saturating_sub(raw_cleaning_gold);
    let upkeep_band = active_upkeep_band(data, companion_count, patron_tier_count);
    let food_gold = scale_upkeep(raw_food_gold, upkeep_band.food_multiplier_pct);
    let cleaning_gold = scale_upkeep(raw_cleaning_gold, upkeep_band.cleaning_multiplier_pct);
    let maintenance_gold =
        scale_upkeep(raw_maintenance_gold, upkeep_band.maintenance_multiplier_pct);
    let total_gold = food_gold
        .saturating_add(cleaning_gold)
        .saturating_add(maintenance_gold);
    let next_companion_total_gold = upkeep_forecast_total_for_counts(
        data,
        constructed_building_ids,
        companion_count + 1,
        patron_tier_count,
        None,
    );
    let next_building_delta_gold = data
        .buildings
        .buildings
        .iter()
        .filter(|building| {
            constructed_building_ids
                .iter()
                .filter(|id| *id == &building.id)
                .count()
                < usize::from(building.build_limit)
        })
        .min_by_key(|building| building.cost.gold)
        .map(|building| {
            upkeep_forecast_total_for_counts(
                data,
                constructed_building_ids,
                companion_count,
                patron_tier_count,
                Some(building.id.as_str()),
            )
            .saturating_sub(total_gold)
        })
        .unwrap_or(0);
    let next_building_total_gold = total_gold.saturating_add(next_building_delta_gold);

    UpkeepForecast {
        food_gold,
        cleaning_gold,
        maintenance_gold,
        total_gold,
        active_band_min_companions: upkeep_band.min_companions,
        active_band_min_patron_tiers: upkeep_band.min_patron_tiers,
        next_companion_total_gold,
        next_companion_delta_gold: next_companion_total_gold.saturating_sub(total_gold),
        next_building_total_gold,
        next_building_delta_gold,
    }
}

pub(super) fn upkeep_forecast_total_for_counts(
    data: &GameData,
    constructed_building_ids: &[String],
    companion_count: usize,
    patron_tier_count: usize,
    extra_building_id: Option<&str>,
) -> u32 {
    let raw_food_gold = companion_count
        .saturating_mul(data.config.day_cycle.companion_food_gold_per_day as usize)
        as u32;
    let building_upkeep_gold = constructed_building_ids
        .iter()
        .map(String::as_str)
        .chain(extra_building_id)
        .filter_map(|building_id| {
            data.buildings
                .buildings
                .iter()
                .find(|building| building.id == building_id)
        })
        .map(|building| building_maintenance_gold(data, building))
        .sum::<u32>();
    let raw_cleaning_gold = if building_upkeep_gold == 0 {
        0
    } else {
        (building_upkeep_gold / 4).max(1)
    };
    let raw_maintenance_gold = building_upkeep_gold.saturating_sub(raw_cleaning_gold);
    let upkeep_band = active_upkeep_band(data, companion_count, patron_tier_count);

    scale_upkeep(raw_food_gold, upkeep_band.food_multiplier_pct)
        .saturating_add(scale_upkeep(
            raw_cleaning_gold,
            upkeep_band.cleaning_multiplier_pct,
        ))
        .saturating_add(scale_upkeep(
            raw_maintenance_gold,
            upkeep_band.maintenance_multiplier_pct,
        ))
}

pub(super) fn active_upkeep_band(
    data: &GameData,
    companion_count: usize,
    patron_tier_count: usize,
) -> crate::data::UpkeepBandData {
    data.config
        .day_cycle
        .upkeep_bands
        .iter()
        .filter(|band| {
            companion_count >= band.min_companions as usize
                || patron_tier_count >= band.min_patron_tiers as usize
        })
        .max_by_key(|band| band.min_companions.max(band.min_patron_tiers))
        .cloned()
        .unwrap_or(crate::data::UpkeepBandData {
            min_companions: 0,
            min_patron_tiers: 0,
            food_multiplier_pct: 100,
            cleaning_multiplier_pct: 100,
            maintenance_multiplier_pct: 100,
        })
}

pub(super) fn scale_upkeep(value: u32, multiplier_pct: u32) -> u32 {
    if value == 0 {
        0
    } else {
        value.saturating_mul(multiplier_pct).div_ceil(100)
    }
}

pub(super) fn building_maintenance_gold(
    data: &GameData,
    building: &crate::data::BuildingData,
) -> u32 {
    let divisor = if matches!(building.category.as_str(), "project" | "prestige") {
        data.config
            .day_cycle
            .building_maintenance_cost_divisor
            .saturating_mul(4)
            .max(1)
    } else {
        data.config.day_cycle.building_maintenance_cost_divisor
    };

    (building.cost.gold / divisor).max(1)
}
