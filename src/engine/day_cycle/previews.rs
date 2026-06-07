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
    let mut forecast = upkeep_forecast_for_counts(
        data,
        &game_state.town.constructed_building_ids,
        game_state.monsters.len(),
        game_state.town.patron_tiers.len(),
        None,
    );
    let pressure_multiplier_pct = 100 + upkeep_pressure_pct(game_state);
    if pressure_multiplier_pct > 100 {
        forecast.food_gold = scale_upkeep(forecast.food_gold, pressure_multiplier_pct);
        forecast.cleaning_gold = scale_upkeep(forecast.cleaning_gold, pressure_multiplier_pct);
        forecast.maintenance_gold =
            scale_upkeep(forecast.maintenance_gold, pressure_multiplier_pct);
        forecast.total_gold = forecast
            .food_gold
            .saturating_add(forecast.cleaning_gold)
            .saturating_add(forecast.maintenance_gold);
        forecast.next_girl_total_gold =
            scale_upkeep(forecast.next_girl_total_gold, pressure_multiplier_pct);
        forecast.next_girl_delta_gold = forecast
            .next_girl_total_gold
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
    let building_bonus = collect_building_modifiers(data, game_state);

    let assigned_monsters = game_state
        .monsters
        .iter()
        .filter(|monster| matches!(&monster.current_job, CompanionJobState::OnExpedition { .. }))
        .collect::<Vec<_>>();

    let total_power = assigned_monsters
        .iter()
        .map(|monster| monster.stats.power.max(0) as u32)
        .sum::<u32>();
    let total_instinct = assigned_monsters
        .iter()
        .map(|monster| monster.stats.instinct.max(0) as u32)
        .sum::<u32>();
    let total_endurance = assigned_monsters
        .iter()
        .map(|monster| monster.stats.endurance.max(0) as u32)
        .sum::<u32>();
    let total_trait_success = assigned_monsters
        .iter()
        .map(|monster| collect_trait_modifiers(data, monster).expedition_success_pct)
        .sum::<i32>();
    let total_trait_risk = assigned_monsters
        .iter()
        .map(|monster| collect_trait_modifiers(data, monster).injury_risk_pct)
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
    let priority_injury_risk = match priority {
        ExpeditionPriority::Balanced => 0,
        ExpeditionPriority::Aggressive => 8,
        ExpeditionPriority::Safe => -10,
        ExpeditionPriority::RecoveryFocused => -14,
        ExpeditionPriority::Curiosity => 5,
    };
    let depth_profile = expedition_depth_profile(
        data,
        game_state,
        floor,
        mission,
        priority,
        &assigned_monsters,
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
    let egg_discovery_score = success_score + building_bonus.egg_discovery_flat;
    let projected_eggs =
        if egg_discovery_score >= data.config.day_cycle.expedition_egg_reward_threshold {
            floor.baseline_rewards.eggs + mission.egg_bonus_flat + depth_profile.egg_bonus
        } else {
            0
        };
    let projected_relics =
        if success_score >= data.config.day_cycle.expedition_relic_reward_threshold {
            floor.baseline_rewards.relics + mission.relic_bonus_flat + depth_profile.relic_bonus
        } else {
            0
        };
    let injury_risk_score = floor.difficulty as i32
        + mission.injury_risk_pct
        + priority_injury_risk
        + total_trait_risk
        + depth_profile.injury_risk_delta
        - total_endurance as i32 * 3;

    Ok(ExpeditionPlanPreview {
        success_score,
        projected_materials,
        projected_arcane_residue,
        projected_eggs,
        projected_relics,
        injury_risk_score,
    })
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
    let client_tier = active_client_tier_for_room(data, town, room)?;
    let skill_bonus = guild_job_skill_bonus(monster, room);
    let depth_profile = room_depth_profile_for_town(
        &town.constructed_building_ids,
        town.completed_project_ids.len() as u32,
        room,
    );

    let success_score = data.config.day_cycle.base_guild_job_success
        + monster.stats.charm * 3
        + skill_bonus
        + room_trait_bonus
        + room_species_bonus
        + building_bonus.guild_income_pct
        + trait_modifier.guild_income_pct
        + depth_profile.success_bonus;

    let base_gold = room.base_gold_yield
        + (monster.stats.charm.max(0) as u32 * data.config.day_cycle.worker_charm_gold_multiplier)
        + (success_score.max(0) as u32 / 4);
    let base_residue = room.base_residue_yield
        + (monster.stats.instinct.max(0) as u32
            * data.config.day_cycle.worker_instinct_residue_multiplier)
        + (success_score.max(0) as u32 / 12);
    let base_materials = room
        .base_materials_yield
        .saturating_add(monster.stats.power.max(0) as u32 / 4)
        .saturating_add(monster.skills.crafting / 2);
    let preparation_quality = room
        .preparation_quality_bonus
        .saturating_add(monster.skills.scouting / 2)
        .saturating_add(monster.skills.guarding / 2)
        .saturating_add(monster.skills.hospitality / 3)
        .saturating_add(monster.skills.navigation / 2)
        .saturating_add(monster.skills.arcana / 2);

    Ok(GuildJobPreview {
        success_score,
        projected_gold: base_gold
            * client_tier.income_multiplier_pct
            * depth_profile.gold_multiplier_pct
            * quality_income_multiplier_pct(monster.quality_rank)
            / 1_000_000,
        projected_arcane_residue: base_residue
            * client_tier.residue_multiplier_pct
            * depth_profile.residue_multiplier_pct
            / 10_000,
        projected_materials: base_materials,
        projected_reputation: room.reputation_yield + success_score.max(0) / 40,
        preparation_quality,
        recovery_bonus: room.recovery_bonus,
        projected_work_history_gains: CompanionWorkHistoryState {
            scouting_runs: room.work_history_gains.scouting_runs,
            guard_duties: room.work_history_gains.guard_duties,
            hospitality_jobs: room.work_history_gains.hospitality_jobs,
            craft_jobs: room.work_history_gains.craft_jobs,
            contracts_completed: room.work_history_gains.contracts_completed,
            recovery_shifts: room.work_history_gains.recovery_shifts,
            hatchery_assists: room.work_history_gains.hatchery_assists,
        },
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
