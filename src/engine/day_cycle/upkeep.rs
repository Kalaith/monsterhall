//! What the guild pays each day.
//!
//! Upkeep runs on two axes at once: the roster draws wages, and standing
//! buildings need maintaining. Both are then multiplied by whichever upkeep band
//! the guild has grown into, so expansion raises the floor under every day.
//!
//! Wages are billed per companion rather than per head. An adventuring party
//! pays more for a strong escort, so a strong escort is paid more in turn — the
//! roster stops being free the moment it gets good, which is the pressure that
//! makes a growing guild a decision rather than a ratchet.

use super::*;

/// Total daily wage bill for a roster.
pub(super) fn roster_wage_bill(data: &GameData, monsters: &[CompanionState]) -> u32 {
    monsters
        .iter()
        .map(|monster| {
            companion_daily_wage(
                &data.config.day_cycle,
                crate::engine::species_of(data, monster),
                monster,
            )
        })
        .fold(0u32, |total, wage| total.saturating_add(wage))
}

/// Wage of one more companion hired at the bottom of the ladder, used for the
/// "what does the next hire cost me" forecast.
fn entry_level_wage(data: &GameData) -> u32 {
    let day_cycle = &data.config.day_cycle;
    day_cycle
        .companion_base_wage_gold
        .saturating_mul(quality_wage_multiplier_pct(day_cycle, 1))
        .div_ceil(100)
}

pub(super) fn upkeep_forecast_for_roster(
    data: &GameData,
    constructed_building_ids: &[String],
    monsters: &[CompanionState],
    patron_tier_count: usize,
) -> UpkeepForecast {
    let companion_count = monsters.len();
    let wage_bill = roster_wage_bill(data, monsters);
    let building_upkeep_gold = building_upkeep_total(data, constructed_building_ids, None);
    let (raw_cleaning_gold, raw_maintenance_gold) = split_building_upkeep(building_upkeep_gold);
    let upkeep_band = active_upkeep_band(data, companion_count, patron_tier_count);
    let wage_gold = scale_upkeep(wage_bill, upkeep_band.wage_multiplier_pct);
    let cleaning_gold = scale_upkeep(raw_cleaning_gold, upkeep_band.cleaning_multiplier_pct);
    let maintenance_gold =
        scale_upkeep(raw_maintenance_gold, upkeep_band.maintenance_multiplier_pct);
    let total_gold = wage_gold
        .saturating_add(cleaning_gold)
        .saturating_add(maintenance_gold);

    let next_companion_total_gold = upkeep_total_for(
        data,
        constructed_building_ids,
        companion_count + 1,
        wage_bill.saturating_add(entry_level_wage(data)),
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
            upkeep_total_for(
                data,
                constructed_building_ids,
                companion_count,
                wage_bill,
                patron_tier_count,
                Some(building.id.as_str()),
            )
            .saturating_sub(total_gold)
        })
        .unwrap_or(0);
    let next_building_total_gold = total_gold.saturating_add(next_building_delta_gold);

    UpkeepForecast {
        wage_gold,
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

fn upkeep_total_for(
    data: &GameData,
    constructed_building_ids: &[String],
    companion_count: usize,
    wage_bill: u32,
    patron_tier_count: usize,
    extra_building_id: Option<&str>,
) -> u32 {
    let building_upkeep_gold =
        building_upkeep_total(data, constructed_building_ids, extra_building_id);
    let (raw_cleaning_gold, raw_maintenance_gold) = split_building_upkeep(building_upkeep_gold);
    let upkeep_band = active_upkeep_band(data, companion_count, patron_tier_count);

    scale_upkeep(wage_bill, upkeep_band.wage_multiplier_pct)
        .saturating_add(scale_upkeep(
            raw_cleaning_gold,
            upkeep_band.cleaning_multiplier_pct,
        ))
        .saturating_add(scale_upkeep(
            raw_maintenance_gold,
            upkeep_band.maintenance_multiplier_pct,
        ))
}

fn building_upkeep_total(
    data: &GameData,
    constructed_building_ids: &[String],
    extra_building_id: Option<&str>,
) -> u32 {
    constructed_building_ids
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
        .sum::<u32>()
}

/// Cleaning takes a quarter of a building's upkeep, repairs take the rest.
fn split_building_upkeep(building_upkeep_gold: u32) -> (u32, u32) {
    let cleaning = if building_upkeep_gold == 0 {
        0
    } else {
        (building_upkeep_gold / 4).max(1)
    };
    (cleaning, building_upkeep_gold.saturating_sub(cleaning))
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
            wage_multiplier_pct: 100,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::test_game_data;

    fn companion(quality_rank: u8, skill_total: u32) -> CompanionState {
        CompanionState {
            id: "monster_001".to_owned(),
            species_id: "slime_companion".to_owned(),
            name: "Test".to_owned(),
            quality_rank,
            stats: Default::default(),
            trait_ids: Vec::new(),
            current_job: CompanionJobState::Idle,
            skills: CompanionSkillState {
                scouting: skill_total,
                ..Default::default()
            },
            work_history: CompanionWorkHistoryState::default(),
            fatigue: 0,
            stress: 0,
            injury: 0,
            corruption: 0,
            bond: 0,
            reputation: 0,
        }
    }

    /// The whole point of the rework: a strong companion is not free.
    #[test]
    fn wages_climb_with_rank() {
        let data = test_game_data();
        let day_cycle = &data.config.day_cycle;

        let entry = companion_daily_wage(day_cycle, None, &companion(1, 0));
        let veteran = companion_daily_wage(day_cycle, None, &companion(5, 0));

        // The shipped curve settles around fourfold. What matters is that the
        // climb is material rather than decorative — a top-rank escort must
        // cost several times what a hatchling does.
        assert!(
            veteran >= entry * 3,
            "a rank-5 escort should cost several times a rank-1: {entry} vs {veteran}"
        );
    }

    #[test]
    fn skills_add_to_a_wage() {
        let data = test_game_data();
        let day_cycle = &data.config.day_cycle;

        let green = companion_daily_wage(day_cycle, None, &companion(2, 0));
        let skilled = companion_daily_wage(day_cycle, None, &companion(2, 20));

        assert!(skilled > green, "accumulated skill should be paid for");
    }

    /// Wages must trail income, or climbing the ladder is pointless.
    #[test]
    fn wages_trail_the_income_they_unlock() {
        let data = test_game_data();
        let day_cycle = &data.config.day_cycle;

        for rank in 1..=max_quality_rank(day_cycle) {
            let income = quality_income_multiplier_pct(day_cycle, rank);
            let wage = quality_wage_multiplier_pct(day_cycle, rank);
            assert!(
                wage <= income,
                "rank {rank} is paid {wage}% while earning {income}%"
            );
        }
    }

    #[test]
    fn the_wage_bill_is_the_sum_of_the_roster() {
        let data = test_game_data();
        let roster = vec![companion(1, 0), companion(3, 0), companion(5, 12)];
        let expected: u32 = roster
            .iter()
            .map(|monster| {
                companion_daily_wage(
                    &data.config.day_cycle,
                    crate::engine::species_of(&data, monster),
                    monster,
                )
            })
            .sum();

        assert_eq!(roster_wage_bill(&data, &roster), expected);
    }
}
