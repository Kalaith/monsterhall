use super::*;

pub fn effective_population_cap(data: &GameData, game_state: &GameState) -> usize {
    let building_bonus = collect_building_modifiers(data, game_state)
        .population_cap_flat
        .max(0) as u16;
    let raw_cap = data
        .config
        .new_game
        .population_cap
        .saturating_add(building_bonus);
    usize::from(raw_cap.min(data.config.new_game.max_population_cap))
}

pub fn preview_upkeep(data: &GameData, game_state: &GameState) -> UpkeepForecast {
    let mut forecast = upkeep_forecast_for_roster(
        data,
        &game_state.town.constructed_building_ids,
        &game_state.monsters,
        game_state.town.patron_tiers.len(),
    );
    let pressure_multiplier_pct = 100 + upkeep_pressure_pct(game_state);
    if pressure_multiplier_pct > 100 {
        forecast.wage_gold = scale_upkeep(forecast.wage_gold, pressure_multiplier_pct);
        forecast.cleaning_gold = scale_upkeep(forecast.cleaning_gold, pressure_multiplier_pct);
        forecast.maintenance_gold =
            scale_upkeep(forecast.maintenance_gold, pressure_multiplier_pct);
        forecast.total_gold = forecast
            .wage_gold
            .saturating_add(forecast.cleaning_gold)
            .saturating_add(forecast.maintenance_gold);
        forecast.next_companion_total_gold =
            scale_upkeep(forecast.next_companion_total_gold, pressure_multiplier_pct);
        forecast.next_companion_delta_gold = forecast
            .next_companion_total_gold
            .saturating_sub(forecast.total_gold);
        forecast.next_building_total_gold =
            scale_upkeep(forecast.next_building_total_gold, pressure_multiplier_pct);
        forecast.next_building_delta_gold = forecast
            .next_building_total_gold
            .saturating_sub(forecast.total_gold);
    }
    forecast
}

pub fn preview_guild_job(
    data: &GameData,
    game_state: &GameState,
    monster: &CompanionState,
    room_id: &str,
) -> Result<GuildJobPreview, String> {
    let building_bonus = collect_building_modifiers(data, game_state);
    preview_guild_job_for_town(data, &game_state.town, &building_bonus, monster, room_id)
}

pub fn preview_expedition_plan(
    data: &GameData,
    game_state: &GameState,
    floor_id: &str,
    mission_id: &str,
    priority: &ExpeditionPriority,
) -> Result<ExpeditionPlanPreview, String> {
    let floor = data
        .floors
        .floors
        .iter()
        .find(|entry| entry.id == floor_id)
        .ok_or_else(|| format!("Unknown floor id '{floor_id}'."))?;
    floor_roster_gate_report(data, game_state, floor)?;
    let mission = data
        .missions
        .missions
        .iter()
        .find(|entry| entry.id == mission_id)
        .ok_or_else(|| format!("Unknown mission id '{mission_id}'."))?;
    let assigned_monsters = game_state
        .monsters
        .iter()
        .filter(|monster| {
            matches!(&monster.current_job, CompanionJobState::OnExpedition { .. })
                && game_state
                    .active_expedition
                    .as_ref()
                    .map(|expedition| {
                        expedition
                            .assigned_monster_ids
                            .iter()
                            .any(|assigned_id| assigned_id == &monster.id)
                    })
                    .unwrap_or(true)
        })
        .collect::<Vec<_>>();

    Ok(calculate_expedition_plan(
        data,
        game_state,
        floor,
        mission,
        priority,
        &assigned_monsters,
    ))
}

/// Extra injury exposure a stance buys. Shared so the planning preview and day
/// resolution cannot drift apart on what "Aggressive" costs.
pub(super) fn priority_injury_risk(priority: &ExpeditionPriority) -> i32 {
    match priority {
        ExpeditionPriority::Balanced => 0,
        ExpeditionPriority::Aggressive => 8,
        ExpeditionPriority::Safe => -10,
        ExpeditionPriority::RecoveryFocused => -14,
        ExpeditionPriority::Curiosity => 5,
    }
}

/// How safely one companion comes back from a run. Compared against
/// `expedition_injury_threshold`; below it she is hurt.
///
/// This is the single formula for that decision — the planning preview and day
/// resolution both call it.
pub(super) fn expedition_safety_score(
    data: &GameData,
    monster: &CompanionState,
    mission: &crate::data::MissionData,
    depth_injury_risk_delta: i32,
    priority_injury_risk: i32,
    total_success: i32,
) -> i32 {
    let day_cycle = &data.config.day_cycle;
    let trait_modifier = collect_trait_modifiers(data, monster);
    // A worn-down companion braces for less of the hit than her endurance says.
    let effective_endurance = scale_by_effectiveness(
        effective_stats(data, monster).endurance.max(0) as u32,
        companion_effectiveness_pct(day_cycle, monster),
    );

    total_success
        + (effective_endurance as i32 * 4)
        + (effective_endurance / day_cycle.expedition_endurance_safety_divisor.max(1)) as i32
        - trait_modifier.injury_risk_pct
        - mission.injury_risk_pct
        - depth_injury_risk_delta
        - priority_injury_risk
}

pub(crate) fn calculate_expedition_plan(
    data: &GameData,
    game_state: &GameState,
    floor: &crate::data::TowerFloorData,
    mission: &crate::data::MissionData,
    priority: &ExpeditionPriority,
    assigned_monsters: &[&CompanionState],
) -> ExpeditionPlanPreview {
    let building_bonus = collect_building_modifiers(data, game_state);
    let day_cycle = &data.config.day_cycle;

    // A party carries only what its condition lets it carry. Each companion's
    // trait-adjusted stats are weighted by how worn down she is before anything is totalled,
    // so a battered party both succeeds less and — through the endurance term
    // in the safety score — gets hurt more.
    let condition_weighted = |monster: &CompanionState, stat: i32| -> u32 {
        scale_by_effectiveness(
            stat.max(0) as u32,
            companion_effectiveness_pct(day_cycle, monster),
        )
    };

    let total_power = assigned_monsters
        .iter()
        .map(|monster| condition_weighted(monster, effective_stats(data, monster).power))
        .sum::<u32>();
    let total_instinct = assigned_monsters
        .iter()
        .map(|monster| condition_weighted(monster, effective_stats(data, monster).instinct))
        .sum::<u32>();
    let party_effectiveness_pct = party_effectiveness_pct(day_cycle, assigned_monsters);
    let total_trait_success = assigned_monsters
        .iter()
        .map(|monster| collect_trait_modifiers(data, monster).expedition_success_pct)
        .sum::<i32>();

    let priority_bonus = match priority {
        ExpeditionPriority::Balanced => 0,
        ExpeditionPriority::Aggressive => 6,
        ExpeditionPriority::Safe => -4,
        ExpeditionPriority::RecoveryFocused => -1,
        ExpeditionPriority::Curiosity => -2,
    };
    let priority_residue_bonus_pct = match priority {
        ExpeditionPriority::Balanced => 100,
        ExpeditionPriority::Aggressive => 100,
        ExpeditionPriority::Safe => 90,
        ExpeditionPriority::RecoveryFocused => 95,
        ExpeditionPriority::Curiosity => 125,
    };
    let priority_material_bonus_pct = match priority {
        ExpeditionPriority::Balanced => 100,
        ExpeditionPriority::Aggressive => 110,
        ExpeditionPriority::Safe => 90,
        ExpeditionPriority::RecoveryFocused => 95,
        ExpeditionPriority::Curiosity => 85,
    };
    let priority_injury_risk = priority_injury_risk(priority);
    let depth_profile = expedition_depth_profile(
        data,
        game_state,
        floor,
        mission,
        priority,
        assigned_monsters,
    );

    let success_score = data.config.day_cycle.base_expedition_success
        + total_power as i32 * 4
        + total_instinct as i32 * 2
        + mission.success_bonus_pct
        + priority_bonus
        + building_bonus.expedition_success_pct
        + total_trait_success
        + depth_profile.success_bonus
        - floor.difficulty as i32;
    let reward_bonus = (success_score.max(0) as u32
        / data.config.day_cycle.expedition_reward_success_divisor)
        .max(1);
    let projected_materials = (floor.baseline_rewards.tower_materials
        + total_power * data.config.day_cycle.expedition_power_materials_multiplier
        + reward_bonus)
        * mission.materials_multiplier_pct
        * priority_material_bonus_pct
        * depth_profile.material_multiplier_pct
        / 1_000_000;
    let projected_arcane_residue = (floor.baseline_rewards.arcane_residue
        + total_instinct * data.config.day_cycle.expedition_instinct_residue_multiplier)
        * mission.residue_multiplier_pct
        * priority_residue_bonus_pct
        * depth_profile.residue_multiplier_pct
        / 1_000_000;
    // Depth is already priced into `success_score`; charging it again at the
    // reward bar would mean the deepest floors never produce eggs or relics,
    // and better companions are the only thing the tower is really for.
    let depth_relief =
        floor.difficulty as i32 * data.config.day_cycle.reward_threshold_depth_relief_pct / 100;
    // A party that went down looking for one thing in particular clears that
    // thing's bar more easily. Without this a riskier stance is charged for its
    // own risk twice, and the mission named after a reward is the worst way to
    // come back with one.
    let focus_relief =
        floor.difficulty as i32 * data.config.day_cycle.mission_focus_reward_relief_pct / 100;
    let egg_focus_relief = if mission.reward_focus == "eggs" {
        focus_relief
    } else {
        0
    };
    let relic_focus_relief = if mission.reward_focus == "relics" {
        focus_relief
    } else {
        0
    };
    let egg_threshold =
        data.config.day_cycle.expedition_egg_reward_threshold - depth_relief - egg_focus_relief;
    let relic_threshold =
        data.config.day_cycle.expedition_relic_reward_threshold - depth_relief - relic_focus_relief;

    let egg_discovery_score = success_score + building_bonus.egg_discovery_flat;
    let projected_eggs = if egg_discovery_score >= egg_threshold {
        floor.baseline_rewards.eggs + mission.egg_bonus_flat + depth_profile.egg_bonus
    } else {
        0
    };
    let projected_relics = if success_score >= relic_threshold {
        floor.baseline_rewards.relics + mission.relic_bonus_flat + depth_profile.relic_bonus
    } else {
        0
    };
    // Day resolution decides injuries per companion, by comparing a safety
    // score against a threshold. Quoting a different party-wide formula here
    // meant the planning screen could promise safety while the sim maimed
    // somebody, so the preview now runs resolution's own arithmetic and
    // reports the margin for whoever is most exposed. Above zero, somebody
    // comes home hurt.
    // Day resolution decides injuries per companion by comparing a safety score
    // against a threshold, so the preview runs resolution's own arithmetic and
    // reports the margin for whoever is most exposed. Quoting a different
    // party-wide formula meant the planning screen could promise safety while
    // the sim maimed somebody. Above zero, someone comes home hurt.
    // `None` when nobody is assigned: there is no companion to be hurt, and any
    // number here is a fabrication. It was `i32::MIN / 2`, which the planning
    // screen printed verbatim as "Injury Risk -1073741824" every time the screen
    // was opened before a party was picked.
    let injury_risk_score = assigned_monsters
        .iter()
        .map(|monster| {
            day_cycle.expedition_injury_threshold
                - expedition_safety_score(
                    data,
                    monster,
                    mission,
                    depth_profile.injury_risk_delta,
                    priority_injury_risk,
                    success_score,
                )
        })
        .max();

    ExpeditionPlanPreview {
        success_score,
        projected_materials,
        projected_arcane_residue,
        projected_eggs,
        projected_relics,
        injury_risk_score,
        party_effectiveness_pct,
    }
}

pub(super) fn preview_guild_job_for_town(
    data: &GameData,
    town: &PlayerTownState,
    building_bonus: &BuildingAggregate,
    monster: &CompanionState,
    room_id: &str,
) -> Result<GuildJobPreview, String> {
    let room = data
        .guild_rooms
        .rooms
        .iter()
        .find(|entry| entry.id == room_id)
        .ok_or_else(|| format!("Unknown room id '{room_id}'."))?;
    let trait_modifier = collect_trait_modifiers(data, monster);
    let room_trait_bonus = room
        .preferred_trait_ids
        .iter()
        .filter(|trait_id| monster.trait_ids.contains(trait_id))
        .count() as i32
        * data.config.day_cycle.preferred_trait_bonus_pct;
    let room_species_bonus = if room.preferred_species_ids.contains(&monster.species_id) {
        data.config.day_cycle.preferred_species_bonus_pct
    } else {
        0
    };
    let patron_tier = active_patron_tier_for_room(data, town, room, monster.quality_rank)?;
    let skill_bonus = guild_job_skill_bonus(monster, room);
    let depth_profile = room_depth_profile_for_town(
        &town.constructed_building_ids,
        town.completed_project_ids.len() as u32,
        room,
    );

    let stats = effective_stats(data, monster);

    // `guild_income_pct` is deliberately absent from this Score. It is authored
    // as a percentage, named as one and shown as one, so it multiplies what the
    // shift returns rather than adding a quarter-gold per point to a score that
    // is not a percentage of anything.
    //
    // Its sibling `expedition_success_pct` stays a score term and is right to:
    // an expedition's success score *is* a percentage, so a point there is a
    // point of success chance. Guild jobs have no such scale.
    let success_score = data.config.day_cycle.base_guild_job_success
        + stats.charm * 3
        + skill_bonus
        + room_trait_bonus
        + room_species_bonus
        + depth_profile.success_bonus;

    let base_gold = room.base_gold_yield
        + (stats.charm.max(0) as u32 * data.config.day_cycle.worker_charm_gold_multiplier)
        + (success_score.max(0) as u32 / 4);
    let base_residue = room.base_residue_yield
        + (stats.instinct.max(0) as u32 * data.config.day_cycle.worker_instinct_residue_multiplier)
        + (success_score.max(0) as u32 / 12);
    let base_materials = room
        .base_materials_yield
        .saturating_add(stats.power.max(0) as u32 / 4)
        .saturating_add(monster.skills.crafting / 2);
    // The engine's own figure, condition included, rather than a second copy of
    // the same arithmetic — this preview and `town_preparation_quality` are
    // quoting and scoring the exact same shift.
    let preparation_quality =
        crate::engine::depth::companion_preparation_quality(data, room, monster);

    // An adventuring party pays for the calibre of escort it gets. Below what
    // the tier expects they still hire, but not at full rate.
    let day_cycle = &data.config.day_cycle;
    let escort_rate_pct = if monster.quality_rank >= patron_tier.minimum_quality_rank {
        100
    } else {
        day_cycle.understrength_income_pct
    };

    // What the guild's buildings and this companion's traits add to the shift,
    // as the percentage both are authored as. Floored at zero so a companion
    // whose traits all sour cannot invert the payment.
    let income_multiplier_pct =
        (100 + building_bonus.guild_income_pct + trait_modifier.guild_income_pct).max(0) as u64;

    // Six percentage multipliers stacked on a fee overflows u32 once the rank
    // curve reaches the top of the ladder, so the escort fee is computed wide.
    let escort_fee_gold = u64::from(base_gold)
        * u64::from(patron_tier.income_multiplier_pct)
        * u64::from(depth_profile.gold_multiplier_pct)
        * u64::from(quality_income_multiplier_pct(
            day_cycle,
            monster.quality_rank,
        ))
        * u64::from(escort_rate_pct)
        * income_multiplier_pct
        / 10_000_000_000;

    // A companion run ragged escorts worse parties for less coin. Applied here,
    // in the preview both the planning screen and day resolution read, so the
    // number the player is quoted is the number the guild is paid.
    let effectiveness_pct = companion_effectiveness_pct(day_cycle, monster);
    // The same multiplier rides the residue a shift brings back. The term it
    // replaces lived in `success_score`, which fed gold *and* residue, so
    // applying it to coin alone would have cut the guild's residue income by a
    // sixth — a balance change smuggled in under a labelling fix.
    let base_residue_income = u32::try_from(
        u64::from(base_residue)
            * u64::from(patron_tier.residue_multiplier_pct)
            * u64::from(depth_profile.residue_multiplier_pct)
            * income_multiplier_pct
            / 1_000_000,
    )
    .unwrap_or(u32::MAX);

    Ok(GuildJobPreview {
        success_score,
        projected_gold: scale_by_effectiveness(
            u32::try_from(escort_fee_gold).unwrap_or(u32::MAX),
            effectiveness_pct,
        ),
        projected_arcane_residue: scale_by_effectiveness(base_residue_income, effectiveness_pct),
        projected_materials: scale_by_effectiveness(base_materials, effectiveness_pct),
        projected_reputation: scale_signed_by_effectiveness(
            room.reputation_yield + success_score.max(0) / 40,
            effectiveness_pct,
        ),
        preparation_quality,
        recovery_bonus: room.recovery_bonus,
        effectiveness_pct,
    })
}

pub(super) fn guild_job_skill_bonus(
    monster: &CompanionState,
    room: &crate::data::GuildRoomData,
) -> i32 {
    room.trained_skill_ids
        .iter()
        .enumerate()
        .map(|(index, skill_id)| {
            let raw_value = companion_skill_value(&monster.skills, skill_id) as i32;
            if index == 0 {
                raw_value * 2
            } else {
                raw_value
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::data::test_game_data;
    use crate::state::{CompanionState, GameState, PlayerTownState};

    /// `guild_income_pct` has to behave like the percentage its name, its
    /// authored values and the building card all claim.
    ///
    /// It spent most of this game's life as a term inside `success_score`, which
    /// becomes gold at a quarter per point — so a building advertised at "+4%"
    /// paid about 1.3% of a shift. This drives a real preview twice, changing
    /// nothing but one building's percentage, and checks the fee moves by that
    /// percentage rather than by a quarter-gold a point.
    #[test]
    fn a_guild_income_percent_moves_the_fee_by_that_percent() {
        let mut data = test_game_data();
        let building_id = data.buildings.buildings[0].id.clone();
        let room_id = data.guild_rooms.rooms[0].id.clone();
        let species_id = data.species.species[0].id.clone();

        let game_state = GameState {
            current_day: 1,
            town: PlayerTownState {
                constructed_building_ids: vec![building_id.clone()],
                unlocked_room_ids: vec![room_id.clone()],
                unlocked_species_ids: vec![species_id.clone()],
                patron_tiers: vec![data.patron_tiers.patron_tiers[0].id.clone()],
                party_size: 3,
                town_job_limit: 2,
                ..PlayerTownState::default()
            },
            monsters: vec![CompanionState {
                id: "m1".to_owned(),
                name: "Fixture".to_owned(),
                species_id: species_id.clone(),
                quality_rank: 1,
                stats: data.species.species[0].base_stats.clone(),
                ..CompanionState::default()
            }],
            ..GameState::default()
        };
        let monster = game_state.monsters[0].clone();

        for building in &mut data.buildings.buildings {
            if building.id == building_id {
                building.passive_modifiers.guild_income_pct = 0;
            }
        }
        let baseline = super::preview_guild_job(&data, &game_state, &monster, &room_id)
            .expect("the fixture room accepts the fixture companion");

        for building in &mut data.buildings.buildings {
            if building.id == building_id {
                building.passive_modifiers.guild_income_pct = 50;
            }
        }
        let boosted = super::preview_guild_job(&data, &game_state, &monster, &room_id)
            .expect("the fixture room still accepts her");

        assert_eq!(
            boosted.success_score, baseline.success_score,
            "guild income is a percentage of the fee, not a term in the job score"
        );
        let expected = baseline.projected_gold * 3 / 2;
        assert!(
            boosted.projected_gold.abs_diff(expected) <= 2,
            "+50% guild income should pay about {expected} against a baseline of {}, but paid {}",
            baseline.projected_gold,
            boosted.projected_gold
        );
    }
}
