use super::*;

pub fn resolve_day(data: &GameData, game_state: &mut GameState) -> DayResolutionSummary {
    let resolved_day = game_state.current_day;
    let mut pending_egg_rewards: Vec<(String, Vec<EggSpeciesEntryData>, u32, u32)> = Vec::new();
    let mut summary = DayResolutionSummary {
        resolved_day,
        guild_job_gold: 0,
        guild_job_arcane_residue: 0,
        expedition_prep_gold: 0,
        expedition_prep_materials: 0,
        expedition_prep_arcane_residue: 0,
        expedition_prep_shortfall: 0,
        expedition_materials: 0,
        expedition_arcane_residue: 0,
        expedition_eggs: 0,
        expedition_relics: 0,
        upkeep_food_gold: 0,
        upkeep_cleaning_gold: 0,
        upkeep_maintenance_gold: 0,
        upkeep_gold: 0,
        upkeep_shortfall: 0,
        special_event_gold_delta: 0,
        special_event_count: 0,
        contracts_generated: 0,
        contracts_rejected: 0,
        special_event_lines: Vec::new(),
        debt_updates: Vec::new(),
        contract_updates: Vec::new(),
        event_lines: Vec::new(),
        roster_updates: Vec::new(),
    };

    summary
        .debt_updates
        .extend(tick_town_situations(game_state));
    let building_bonus = collect_building_modifiers(data, game_state);
    let active_expedition = game_state.active_expedition.clone();
    let guest_serviced_monster_ids = resolve_contracts(
        data,
        game_state,
        &mut summary.guild_job_gold,
        &mut summary.guild_job_arcane_residue,
        &mut summary.contract_updates,
        &mut summary.event_lines,
        &mut summary.roster_updates,
    );

    if let Some(expedition) = &active_expedition {
        if let Some(mission) = data
            .missions
            .missions
            .iter()
            .find(|entry| entry.id == expedition.mission_id)
        {
            apply_expedition_prep_cost(game_state, &mut summary, mission);
        }
    }
    let active_expedition_depth_profile = active_expedition.as_ref().and_then(|expedition| {
        let floor = data
            .floors
            .floors
            .iter()
            .find(|entry| entry.id == expedition.floor_id)?;
        let mission = data
            .missions
            .missions
            .iter()
            .find(|entry| entry.id == expedition.mission_id)?;
        let party = game_state
            .monsters
            .iter()
            .filter(|monster| {
                expedition
                    .assigned_monster_ids
                    .iter()
                    .any(|id| id == &monster.id)
            })
            .collect::<Vec<_>>();
        Some(expedition_depth_profile(
            data,
            game_state,
            floor,
            mission,
            &expedition.priority,
            &party,
        ))
    });

    for monster in &mut game_state.monsters {
        if guest_serviced_monster_ids.contains(&monster.id) {
            continue;
        }
        match monster.current_job.clone() {
            CompanionJobState::GuildJob { room_id } => {
                let Ok(preview) = preview_guild_job_for_town(
                    data,
                    &game_state.town,
                    &building_bonus,
                    monster,
                    &room_id,
                ) else {
                    summary.event_lines.push(format!(
                        "{} could not work because room assignment '{}' is invalid.",
                        monster.name, room_id
                    ));
                    continue;
                };
                let Some(room) = data
                    .guild_rooms
                    .rooms
                    .iter()
                    .find(|entry| entry.id == room_id)
                else {
                    summary.event_lines.push(format!(
                        "{} could not work because room '{}' no longer exists.",
                        monster.name, room_id
                    ));
                    continue;
                };
                let trait_modifier = collect_trait_modifiers(data, monster);
                let depth_profile = room_depth_profile_for_town(
                    &game_state.town.constructed_building_ids,
                    game_state.town.completed_project_ids.len() as u32,
                    room,
                );
                let gold_gain = preview.projected_gold;
                let residue_gain = preview.projected_arcane_residue;
                let materials_gain = preview.projected_materials;

                game_state.resources.gold += gold_gain;
                game_state.resources.arcane_residue += residue_gain;
                game_state.resources.tower_materials = game_state
                    .resources
                    .tower_materials
                    .saturating_add(materials_gain);
                summary.guild_job_gold += gold_gain;
                summary.guild_job_arcane_residue += residue_gain;

                monster.fatigue = monster.fatigue.saturating_add(
                    ((data.config.day_cycle.guild_job_fatigue + room.stamina_cost) as i32
                        + depth_profile.fatigue_delta)
                        .max(0) as u32,
                );
                let stress_delta = (data.config.day_cycle.guild_job_stress as i32
                    + trait_modifier.stress_change_flat)
                    .saturating_add(depth_profile.stress_delta)
                    .max(0) as u32;
                monster.stress = monster.stress.saturating_add(stress_delta);
                monster.injury = monster.injury.saturating_sub(
                    data.config.day_cycle.base_injury_recovery
                        + building_bonus.injury_recovery_flat.max(0) as u32,
                );
                monster.fatigue = monster.fatigue.saturating_sub(preview.recovery_bonus);
                monster.stress = monster.stress.saturating_sub(preview.recovery_bonus);
                monster.reputation = monster
                    .reputation
                    .saturating_add(preview.projected_reputation);
                let progression_update = apply_guild_job_progression(monster, room, false);
                let corruption_gain = guild_job_instability_gain(room, monster);
                if corruption_gain > 0 {
                    monster.corruption = monster.corruption.saturating_add(corruption_gain);
                    summary.roster_updates.push(format!(
                        "{} absorbed {} instability from the {}.",
                        monster.name, corruption_gain, room.name
                    ));
                    if let Some(event_text) = select_event_text(data, "corruption", "gain", monster)
                    {
                        summary.event_lines.push(event_text);
                    }
                }
                if depth_profile.corruption_pressure > 0 {
                    monster.corruption = monster
                        .corruption
                        .saturating_add(depth_profile.corruption_pressure);
                }
                apply_monster_relationship_gain(data, monster, None, 1, 0);
                monster.current_job = CompanionJobState::Idle;

                let room_niche_label = display_room_niche(&depth_profile.niche);
                let room_profile_label = if depth_profile.upgrade_tier > 0 {
                    format!("{} tier {}", room_niche_label, depth_profile.upgrade_tier)
                } else {
                    room_niche_label.to_owned()
                };
                summary.roster_updates.push(format!(
                    "{} worked the {} as a {} job for {} gold, {} materials, {} arcane residue, and {} prep quality.",
                    monster.name,
                    room.name,
                    room_profile_label,
                    gold_gain,
                    materials_gain,
                    residue_gain,
                    preview.preparation_quality
                ));
                if let Some(progression_update) = progression_update {
                    summary.roster_updates.push(progression_update);
                }
                if let Some(mutation_text) = try_apply_mutation(data, monster) {
                    summary.roster_updates.push(mutation_text);
                }

                if let Some(event_text) =
                    select_event_text(data, "Guild Jobs", "shift_resolution", monster)
                {
                    summary.event_lines.push(event_text);
                }
            }
            CompanionJobState::Resting => {
                monster.fatigue = monster
                    .fatigue
                    .saturating_sub(data.config.day_cycle.resting_fatigue_recovery);
                monster.stress = monster.stress.saturating_sub(
                    data.config.day_cycle.resting_stress_recovery
                        + building_bonus.stress_recovery_flat.max(0) as u32,
                );
                monster.injury = monster.injury.saturating_sub(
                    data.config.day_cycle.base_injury_recovery
                        + building_bonus.injury_recovery_flat.max(0) as u32,
                );
                monster.current_job = CompanionJobState::Idle;

                summary.roster_updates.push(format!(
                    "{} rested and came back steadier for tomorrow.",
                    monster.name
                ));
                if let Some(mutation_text) = try_apply_mutation(data, monster) {
                    summary.roster_updates.push(mutation_text);
                }
            }
            CompanionJobState::OnExpedition { .. } => {
                if let Some(expedition) = &active_expedition {
                    let Some(floor) = data
                        .floors
                        .floors
                        .iter()
                        .find(|entry| entry.id == expedition.floor_id)
                    else {
                        summary.event_lines.push(format!(
                            "{} could not complete the expedition because floor '{}' no longer exists.",
                            monster.name, expedition.floor_id
                        ));
                        continue;
                    };
                    let trait_modifier = collect_trait_modifiers(data, monster);
                    let Some(mission) = data
                        .missions
                        .missions
                        .iter()
                        .find(|entry| entry.id == expedition.mission_id)
                    else {
                        summary.event_lines.push(format!(
                            "{} could not complete the expedition because mission '{}' no longer exists.",
                            monster.name, expedition.mission_id
                        ));
                        continue;
                    };
                    let priority_bonus = match expedition.priority {
                        ExpeditionPriority::Balanced => 0,
                        ExpeditionPriority::Aggressive => 6,
                        ExpeditionPriority::Safe => -4,
                        ExpeditionPriority::RecoveryFocused => -1,
                        ExpeditionPriority::Curiosity => -2,
                    };
                    let priority_injury_risk = match expedition.priority {
                        ExpeditionPriority::Balanced => 0,
                        ExpeditionPriority::Aggressive => 8,
                        ExpeditionPriority::Safe => -10,
                        ExpeditionPriority::RecoveryFocused => -14,
                        ExpeditionPriority::Curiosity => 5,
                    };
                    let depth_profile = active_expedition_depth_profile.clone().unwrap_or_default();
                    let total_success = data.config.day_cycle.base_expedition_success
                        + monster.stats.power * 4
                        + monster.stats.instinct * 2
                        + building_bonus.expedition_success_pct
                        + trait_modifier.expedition_success_pct
                        + mission.success_bonus_pct
                        + priority_bonus
                        + depth_profile.success_bonus
                        - floor.difficulty as i32;
                    let reward_bonus = (total_success.max(0) as u32
                        / data.config.day_cycle.expedition_reward_success_divisor)
                        .max(1);
                    let material_gain = (floor.baseline_rewards.tower_materials
                        + (monster.stats.power.max(0) as u32
                            * data.config.day_cycle.expedition_power_materials_multiplier)
                        + reward_bonus)
                        * mission.materials_multiplier_pct
                        * depth_profile.material_multiplier_pct
                        / 10_000;
                    let residue_gain = (floor.baseline_rewards.arcane_residue
                        + (monster.stats.instinct.max(0) as u32
                            * data.config.day_cycle.expedition_instinct_residue_multiplier))
                        * mission.residue_multiplier_pct
                        * depth_profile.residue_multiplier_pct
                        / 10_000;
                    let egg_discovery_score = total_success + building_bonus.egg_discovery_flat;
                    let egg_gain = if egg_discovery_score
                        >= data.config.day_cycle.expedition_egg_reward_threshold
                    {
                        floor.baseline_rewards.eggs
                            + mission.egg_bonus_flat
                            + depth_profile.egg_bonus
                    } else {
                        0
                    };
                    let relic_gain = if total_success
                        >= data.config.day_cycle.expedition_relic_reward_threshold
                    {
                        floor.baseline_rewards.relics
                            + mission.relic_bonus_flat
                            + depth_profile.relic_bonus
                    } else {
                        0
                    };

                    game_state.resources.tower_materials += material_gain;
                    game_state.resources.arcane_residue += residue_gain;
                    pending_egg_rewards.push((
                        floor.id.clone(),
                        floor.egg_species_entries.clone(),
                        egg_gain,
                        depth_profile.egg_grade_score,
                    ));
                    game_state.resources.relics += relic_gain;

                    summary.expedition_materials += material_gain;
                    summary.expedition_arcane_residue += residue_gain;
                    summary.expedition_eggs += egg_gain;
                    summary.expedition_relics += relic_gain;

                    monster.fatigue = monster
                        .fatigue
                        .saturating_add(data.config.day_cycle.expedition_fatigue);
                    monster.stress = monster.stress.saturating_add(
                        data.config.day_cycle.expedition_stress
                            + trait_modifier.stress_change_flat.max(0) as u32,
                    );
                    let safety_score = total_success
                        + (monster.stats.endurance * 4)
                        + ((monster.stats.endurance.max(0) as u32
                            / data.config.day_cycle.expedition_endurance_safety_divisor)
                            as i32)
                        - trait_modifier.injury_risk_pct
                        - mission.injury_risk_pct
                        - depth_profile.injury_risk_delta
                        - priority_injury_risk;
                    if safety_score < data.config.day_cycle.expedition_injury_threshold {
                        monster.injury = monster.injury.saturating_add(6);
                        summary.roster_updates.push(format!(
                            "{} returned from {} banged up.",
                            monster.name, floor.name
                        ));
                    } else {
                        summary.roster_updates.push(format!(
                            "{} returned from {} with fresh loot.",
                            monster.name, floor.name
                        ));
                    }
                    let corruption_gain = expedition_corruption_gain(floor, mission, monster)
                        .saturating_add(depth_profile.corruption_pressure);
                    if corruption_gain > 0 {
                        monster.corruption = monster.corruption.saturating_add(corruption_gain);
                        summary.roster_updates.push(format!(
                            "{} came back carrying {} instability from {}.",
                            monster.name, corruption_gain, floor.name
                        ));
                        if let Some(event_text) =
                            select_event_text(data, "corruption", "gain", monster)
                        {
                            summary.event_lines.push(event_text);
                        }
                    }

                    monster.current_job = CompanionJobState::Idle;
                    apply_monster_relationship_gain(data, monster, None, 1, 1);
                    if let Some(mutation_text) = try_apply_mutation(data, monster) {
                        summary.roster_updates.push(mutation_text);
                    }

                    if let Some(event_text) =
                        select_event_text(data, "expedition", "expedition_resolution", monster)
                    {
                        summary.event_lines.push(event_text);
                    }
                }
            }
            CompanionJobState::Idle => {
                monster.injury = monster
                    .injury
                    .saturating_sub(data.config.day_cycle.base_injury_recovery);
                if let Some(mutation_text) = try_apply_mutation(data, monster) {
                    summary.roster_updates.push(mutation_text);
                }
            }
        }
    }

    for (floor_id, egg_species_entries, egg_gain, grade_score) in pending_egg_rewards {
        add_floor_egg_rewards(
            game_state,
            &floor_id,
            &egg_species_entries,
            egg_gain,
            grade_score,
        );
    }
    game_state.active_expedition = None;
    apply_daily_upkeep(data, game_state, &mut summary);
    apply_special_day_event(data, game_state, &mut summary);
    resolve_debt_cycle(
        data,
        game_state,
        &mut summary.debt_updates,
        &mut summary.event_lines,
        &mut summary.roster_updates,
    );
    game_state.current_day += 1;
    if let Ok(guest_refresh) = refresh_contracts(data, game_state) {
        summary.contracts_generated = guest_refresh.generated;
        summary.contracts_rejected = guest_refresh.rejected;
    }
    if let Some(event_text) = select_town_event_text(data, &game_state.monsters) {
        summary.event_lines.push(event_text);
    }
    game_state.event_log.extend(summary.event_lines.clone());
    summary
}

pub(super) fn apply_daily_upkeep(
    data: &GameData,
    game_state: &mut GameState,
    summary: &mut DayResolutionSummary,
) {
    let forecast = preview_upkeep(data, game_state);
    let total_upkeep = forecast.total_gold;
    if total_upkeep == 0 {
        return;
    }

    let paid = game_state.resources.gold.min(total_upkeep);
    game_state.resources.gold = game_state.resources.gold.saturating_sub(paid);
    let shortfall = total_upkeep.saturating_sub(paid);
    summary.upkeep_food_gold = forecast.food_gold;
    summary.upkeep_cleaning_gold = forecast.cleaning_gold;
    summary.upkeep_maintenance_gold = forecast.maintenance_gold;
    summary.upkeep_gold = paid;
    summary.upkeep_shortfall = shortfall;

    if shortfall == 0 {
        summary.debt_updates.push(format!(
            "Paid {} gold in upkeep: {} wages, {} supplies, {} repairs.",
            paid, forecast.food_gold, forecast.cleaning_gold, forecast.maintenance_gold
        ));
        return;
    }

    let stress_penalty = 1 + shortfall / 75;
    for monster in &mut game_state.monsters {
        monster.stress = monster.stress.saturating_add(stress_penalty);
    }
    summary.debt_updates.push(format!(
        "Upkeep cost {} gold, but only {} was paid. Shortfall {} stressed the roster by {}.",
        total_upkeep, paid, shortfall, stress_penalty
    ));
}

fn display_room_niche(niche: &str) -> &str {
    match niche {
        "comfort" => "hospitality",
        "performance" => "reception",
        "hatchery" => "hatchery care",
        "corruption" => "hazard support",
        other => other,
    }
}

pub(super) fn apply_expedition_prep_cost(
    game_state: &mut GameState,
    summary: &mut DayResolutionSummary,
    mission: &crate::data::MissionData,
) {
    let cost = &mission.prep_cost;
    let gold_paid = game_state.resources.gold.min(cost.gold);
    let materials_paid = game_state
        .resources
        .tower_materials
        .min(cost.tower_materials);
    let residue_paid = game_state.resources.arcane_residue.min(cost.arcane_residue);

    game_state.resources.gold = game_state.resources.gold.saturating_sub(gold_paid);
    game_state.resources.tower_materials = game_state
        .resources
        .tower_materials
        .saturating_sub(materials_paid);
    game_state.resources.arcane_residue = game_state
        .resources
        .arcane_residue
        .saturating_sub(residue_paid);

    summary.expedition_prep_gold = gold_paid;
    summary.expedition_prep_materials = materials_paid;
    summary.expedition_prep_arcane_residue = residue_paid;
    summary.expedition_prep_shortfall = cost
        .gold
        .saturating_sub(gold_paid)
        .saturating_add(cost.tower_materials.saturating_sub(materials_paid))
        .saturating_add(cost.arcane_residue.saturating_sub(residue_paid));

    if gold_paid > 0 || materials_paid > 0 || residue_paid > 0 {
        summary.event_lines.push(format!(
            "Prepared {} for {} gold, {} materials, and {} arcane residue.",
            mission.name, gold_paid, materials_paid, residue_paid
        ));
    }

    if summary.expedition_prep_shortfall > 0 {
        let stress_penalty = 1 + summary.expedition_prep_shortfall / 10;
        for monster in &mut game_state.monsters {
            if matches!(monster.current_job, CompanionJobState::OnExpedition { .. }) {
                monster.stress = monster.stress.saturating_add(stress_penalty);
            }
        }
        summary.roster_updates.push(format!(
            "Thin expedition supplies added {} stress to the tower team.",
            stress_penalty
        ));
    }
}

pub(super) fn upkeep_forecast_for_counts(
    data: &GameData,
    constructed_building_ids: &[String],
    girl_count: usize,
    client_tier_count: usize,
    extra_building_id: Option<&str>,
) -> UpkeepForecast {
    let raw_food_gold =
        girl_count.saturating_mul(data.config.day_cycle.girl_food_gold_per_day as usize) as u32;
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
    let upkeep_band = active_upkeep_band(data, girl_count, client_tier_count);
    let food_gold = scale_upkeep(raw_food_gold, upkeep_band.food_multiplier_pct);
    let cleaning_gold = scale_upkeep(raw_cleaning_gold, upkeep_band.cleaning_multiplier_pct);
    let maintenance_gold =
        scale_upkeep(raw_maintenance_gold, upkeep_band.maintenance_multiplier_pct);
    let total_gold = food_gold
        .saturating_add(cleaning_gold)
        .saturating_add(maintenance_gold);
    let next_girl_total_gold = upkeep_forecast_total_for_counts(
        data,
        constructed_building_ids,
        girl_count + 1,
        client_tier_count,
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
                girl_count,
                client_tier_count,
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
        active_band_min_girls: upkeep_band.min_girls,
        active_band_min_patron_tiers: upkeep_band.min_patron_tiers,
        next_girl_total_gold,
        next_girl_delta_gold: next_girl_total_gold.saturating_sub(total_gold),
        next_building_total_gold,
        next_building_delta_gold,
    }
}

pub(super) fn upkeep_forecast_total_for_counts(
    data: &GameData,
    constructed_building_ids: &[String],
    girl_count: usize,
    client_tier_count: usize,
    extra_building_id: Option<&str>,
) -> u32 {
    let raw_food_gold =
        girl_count.saturating_mul(data.config.day_cycle.girl_food_gold_per_day as usize) as u32;
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
    let upkeep_band = active_upkeep_band(data, girl_count, client_tier_count);

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
    girl_count: usize,
    client_tier_count: usize,
) -> crate::data::UpkeepBandData {
    data.config
        .day_cycle
        .upkeep_bands
        .iter()
        .filter(|band| {
            girl_count >= band.min_girls as usize
                || client_tier_count >= band.min_patron_tiers as usize
        })
        .max_by_key(|band| band.min_girls.max(band.min_patron_tiers))
        .cloned()
        .unwrap_or(crate::data::UpkeepBandData {
            min_girls: 0,
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

pub(super) fn ensure_active_expedition(game_state: &mut GameState, floor_id: &str) {
    if game_state.active_expedition.is_none() {
        game_state.active_expedition = Some(ExpeditionState {
            expedition_id: "expedition_001".to_owned(),
            floor_id: floor_id.to_owned(),
            mission_id: "resource_run".to_owned(),
            priority: ExpeditionPriority::Balanced,
            assigned_monster_ids: Vec::new(),
            started_day: game_state.current_day,
        });
    }
}

pub(super) fn apply_special_day_event(
    data: &GameData,
    game_state: &mut GameState,
    summary: &mut DayResolutionSummary,
) {
    let Some(event) = select_special_event(data, game_state) else {
        return;
    };

    let mut gold_delta = 0i32;
    if let Some(cost) = &event.cost {
        let gold_paid = game_state.resources.gold.min(cost.gold);
        game_state.resources.gold = game_state.resources.gold.saturating_sub(gold_paid);
        game_state.resources.tower_materials = game_state
            .resources
            .tower_materials
            .saturating_sub(cost.tower_materials);
        game_state.resources.eggs = game_state.resources.eggs.saturating_sub(cost.eggs);
        game_state.resources.relics = game_state.resources.relics.saturating_sub(cost.relics);
        game_state.resources.arcane_residue = game_state
            .resources
            .arcane_residue
            .saturating_sub(cost.arcane_residue);
        gold_delta -= gold_paid as i32;
    }

    if let Some(reward) = &event.reward {
        game_state.resources.gold = game_state.resources.gold.saturating_add(reward.gold);
        game_state.resources.tower_materials = game_state
            .resources
            .tower_materials
            .saturating_add(reward.tower_materials);
        game_state.resources.eggs = game_state.resources.eggs.saturating_add(reward.eggs);
        game_state.resources.relics = game_state.resources.relics.saturating_add(reward.relics);
        game_state.resources.arcane_residue = game_state
            .resources
            .arcane_residue
            .saturating_add(reward.arcane_residue);
        gold_delta += reward.gold as i32;
    }

    summary.special_event_count += 1;
    summary.special_event_gold_delta += gold_delta;
    summary.special_event_lines.push(event.text.clone());
    if let Some(situation_line) = start_town_situation_from_event(game_state, event) {
        summary.special_event_lines.push(situation_line);
    }

    if gold_delta > 0 {
        let line = format!("Special event windfall: +{} gold.", gold_delta);
        summary.debt_updates.push(line.clone());
        summary.special_event_lines.push(line);
    } else if gold_delta < 0 {
        let line = format!("Special event expense: {} gold.", gold_delta);
        summary.debt_updates.push(line.clone());
        summary.special_event_lines.push(line);
    }
}
