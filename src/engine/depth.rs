use crate::data::{ContractData, EventData, GameData, GuildRoomData, MissionData, TowerFloorData};
use crate::state::{
    CompanionJobState, CompanionState, ContractHistoryRequirementState,
    ContractSkillRequirementState, ContractState, ContractStatus, ExpeditionPriority, GameState,
    ResourcesState, TownSituationState,
};

#[derive(Debug, Clone)]
pub(crate) struct RoomDepthProfile {
    pub(crate) niche: String,
    pub(crate) upgrade_tier: u32,
    pub(crate) success_bonus: i32,
    pub(crate) gold_multiplier_pct: u32,
    pub(crate) residue_multiplier_pct: u32,
    pub(crate) fatigue_delta: i32,
    pub(crate) stress_delta: i32,
    pub(crate) corruption_pressure: u32,
    pub(crate) guest_appeal: u32,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ExpeditionDepthProfile {
    pub(crate) success_bonus: i32,
    pub(crate) material_multiplier_pct: u32,
    pub(crate) residue_multiplier_pct: u32,
    pub(crate) injury_risk_delta: i32,
    pub(crate) egg_bonus: u32,
    pub(crate) relic_bonus: u32,
    pub(crate) egg_grade_score: u32,
    pub(crate) corruption_pressure: u32,
}

pub(crate) fn monster_role(monster: &CompanionState) -> &'static str {
    if monster.corruption >= 10 || monster.trait_ids.iter().any(|id| id == "corruption_tuned") {
        "corruption_adept"
    } else if monster.work_history.hatchery_assists > 0
        || monster.trait_ids.iter().any(|id| id == "hatchery_attuned")
    {
        "hatchery_specialist"
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

pub(crate) fn role_affinity(monster: &CompanionState, role: &str) -> i32 {
    if role.is_empty() {
        return 0;
    }

    if monster_role(monster) == role {
        12
    } else if monster_role(monster) == "versatile" {
        4
    } else {
        0
    }
}

pub(crate) fn room_depth_profile(
    data: &GameData,
    game_state: &GameState,
    room: &GuildRoomData,
) -> RoomDepthProfile {
    room_depth_profile_for_town(
        &game_state.town.constructed_building_ids,
        town_project_count(data, game_state),
        room,
    )
}

pub(crate) fn room_depth_profile_for_town(
    constructed_building_ids: &[String],
    project_count: u32,
    room: &GuildRoomData,
) -> RoomDepthProfile {
    let niche = room
        .strategic_niche
        .as_deref()
        .unwrap_or_else(|| inferred_room_niche(room))
        .to_owned();
    let upgrade_tier = room
        .upgrade_building_ids
        .iter()
        .chain(room.required_building_ids.iter())
        .filter(|building_id| constructed_building_ids.contains(*building_id))
        .count() as u32
        + project_count.min(2);

    let (gold_bias, residue_bias, stress_bias, corruption_bias, guest_appeal) = match niche.as_str()
    {
        "comfort" => (8, 0, -2, 0, 8),
        "performance" => (14, 4, 1, 0, 12),
        "hatchery" => (10, 8, 2, 1, 10),
        "corruption" => (6, 14, 3, 2, 6),
        _ => (6, 6, 0, 0, 4),
    };

    RoomDepthProfile {
        niche,
        upgrade_tier,
        success_bonus: (upgrade_tier as i32 * 4) + guest_appeal as i32 / 2,
        gold_multiplier_pct: (100 + gold_bias + upgrade_tier * 5).max(1),
        residue_multiplier_pct: (100 + residue_bias + upgrade_tier * 4).max(1),
        fatigue_delta: room.fatigue_modifier - upgrade_tier as i32,
        stress_delta: room.stress_modifier + stress_bias - upgrade_tier.min(2) as i32,
        corruption_pressure: room
            .corruption_pressure
            .saturating_add(corruption_bias)
            .saturating_add(upgrade_tier / 2),
        guest_appeal: room.guest_appeal.saturating_add(guest_appeal),
    }
}

pub(crate) fn expedition_depth_profile(
    data: &GameData,
    game_state: &GameState,
    floor: &TowerFloorData,
    mission: &MissionData,
    priority: &ExpeditionPriority,
    party: &[&CompanionState],
) -> ExpeditionDepthProfile {
    let role_bonus = mission
        .preferred_role
        .as_deref()
        .map(|role| {
            party
                .iter()
                .map(|monster| role_affinity(monster, role))
                .sum::<i32>()
        })
        .unwrap_or_else(|| inferred_mission_role_bonus(mission, party));
    let project_bonus = town_project_count(data, game_state) as i32 * 2;
    let hazard_risk = floor.depth as i32 * 2
        + floor.hazard_tags.len() as i32 * 3
        + mission.hazard_risk_modifier_pct;
    let priority_grade_bonus = match priority {
        ExpeditionPriority::Aggressive => 1,
        ExpeditionPriority::Safe => 0,
        ExpeditionPriority::RecoveryFocused => 0,
        ExpeditionPriority::Curiosity => 2,
        ExpeditionPriority::Balanced => 1,
    };
    let reward_focus_bonus = u32::from(mission.reward_focus == "eggs");

    ExpeditionDepthProfile {
        success_bonus: role_bonus + project_bonus - hazard_risk / 3,
        material_multiplier_pct: if mission.reward_focus == "materials" {
            110
        } else {
            100
        },
        residue_multiplier_pct: if mission.reward_focus == "residue" {
            112
        } else {
            100
        },
        injury_risk_delta: hazard_risk - role_bonus / 2,
        egg_bonus: reward_focus_bonus,
        relic_bonus: u32::from(mission.reward_focus == "relics" && floor.depth >= 3),
        egg_grade_score: floor
            .depth
            .saturating_add(floor.egg_grade_bonus)
            .saturating_add(mission.egg_grade_bonus)
            .saturating_add(priority_grade_bonus),
        corruption_pressure: floor.corruption_pressure
            + u32::from(mission.reward_focus == "residue") * 2
            + floor.depth / 2,
    }
}

pub(crate) fn floor_roster_gate_report(
    data: &GameData,
    game_state: &GameState,
    floor: &TowerFloorData,
) -> Result<(), String> {
    let mut missing = Vec::new();
    for requirement in &floor.required_roster {
        let minimum_quality = requirement.minimum_quality_rank.max(1);
        let has_match = game_state.monsters.iter().any(|monster| {
            monster.species_id == requirement.species_id && monster.quality_rank >= minimum_quality
        });
        if !has_match {
            let species_name = data
                .species
                .species
                .iter()
                .find(|species| species.id == requirement.species_id)
                .map(|species| species.name.clone())
                .unwrap_or_else(|| requirement.species_id.clone());
            missing.push(format!("{minimum_quality}-star {species_name}"));
        }
    }

    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{} requires roster coverage: {}.",
            floor.name,
            missing.join(", ")
        ))
    }
}

pub(crate) fn contract_depth_score(
    data: &GameData,
    game_state: &GameState,
    request: &ContractState,
    monster: &CompanionState,
) -> u32 {
    let Some(template) = data
        .contracts
        .requests
        .iter()
        .find(|template| template.id == request.template_id)
    else {
        return 0;
    };
    let trait_score = template
        .preferred_trait_ids
        .iter()
        .filter(|trait_id| monster.trait_ids.contains(*trait_id))
        .count() as u32
        * 12;
    let role_score = template
        .preferred_role
        .as_deref()
        .map(|role| role_affinity(monster, role).max(0) as u32)
        .unwrap_or_default();
    let room_score = data
        .guild_rooms
        .rooms
        .iter()
        .find(|room| room.id == request.requested_room_id)
        .map(|room| room_depth_profile(data, game_state, room).guest_appeal)
        .unwrap_or_default();
    let relationship_score = monster.bond.min(12) + monster.reputation.max(0) as u32;
    trait_score
        + role_score
        + room_score
        + relationship_score
        + town_preparation_quality(data, game_state)
}

pub(crate) fn contract_partial_success(
    data: &GameData,
    game_state: &GameState,
    request: &ContractState,
    monster: &CompanionState,
) -> bool {
    let Some(template) = data
        .contracts
        .requests
        .iter()
        .find(|template| template.id == request.template_id)
    else {
        return false;
    };
    template.partial_success_score > 0
        && town_preparation_quality(data, game_state) >= request.preparation_quality_required
        && contract_depth_score(data, game_state, request, monster)
            >= template.partial_success_score
}

pub(crate) fn contract_follow_up_request(
    data: &GameData,
    game_state: &GameState,
    completed_request: &ContractState,
) -> Option<ContractState> {
    let completed_template = data
        .contracts
        .requests
        .iter()
        .find(|template| template.id == completed_request.template_id)?;
    let follow_up_id = completed_template.follow_up_request_id.as_deref()?;
    if game_state
        .active_contracts
        .iter()
        .any(|request| request.template_id == follow_up_id)
    {
        return None;
    }
    let template = data
        .contracts
        .requests
        .iter()
        .find(|template| template.id == follow_up_id)?;
    Some(request_from_template(
        data,
        game_state,
        template,
        completed_request.chain_depth + 1,
    ))
}

pub(crate) fn town_preparation_quality(data: &GameData, game_state: &GameState) -> u32 {
    let job_quality = game_state
        .monsters
        .iter()
        .filter_map(|monster| match &monster.current_job {
            CompanionJobState::GuildJob { room_id } => data
                .guild_rooms
                .rooms
                .iter()
                .find(|room| &room.id == room_id)
                .map(|room| {
                    room.preparation_quality_bonus
                        .saturating_add(monster.skills.scouting / 2)
                        .saturating_add(monster.skills.guarding / 2)
                        .saturating_add(monster.skills.hospitality / 3)
                        .saturating_add(monster.skills.navigation / 2)
                        .saturating_add(monster.skills.arcana / 2)
                }),
            _ => None,
        })
        .sum::<u32>();
    let accepted_contract_bonus = game_state
        .active_contracts
        .iter()
        .filter(|request| matches!(request.status, ContractStatus::Accepted))
        .map(|request| request.preparation_quality_bonus)
        .sum::<u32>();
    let project_bonus = town_project_count(data, game_state);
    job_quality
        .saturating_add(accepted_contract_bonus)
        .saturating_add(project_bonus)
}

pub(crate) fn apply_monster_relationship_gain(
    data: &GameData,
    monster: &mut CompanionState,
    request: Option<&ContractState>,
    bond_gain: u32,
    reputation_gain: i32,
) {
    let template_reputation = request
        .and_then(|request| {
            data.contracts
                .requests
                .iter()
                .find(|template| template.id == request.template_id)
        })
        .map(|template| template.reputation_reward)
        .unwrap_or_default();
    monster.bond = monster.bond.saturating_add(bond_gain).min(99);
    monster.reputation = monster
        .reputation
        .saturating_add(reputation_gain)
        .saturating_add(template_reputation);
}

pub(crate) fn complete_town_project_if_needed(
    data: &GameData,
    game_state: &mut GameState,
    building_id: &str,
) {
    let Some(building) = data
        .buildings
        .buildings
        .iter()
        .find(|building| building.id == building_id)
    else {
        return;
    };
    if matches!(building.category.as_str(), "project" | "prestige")
        && !game_state.town.completed_project_ids.contains(&building.id)
    {
        game_state
            .town
            .completed_project_ids
            .push(building.id.clone());
    }
}

pub(crate) fn town_project_count(data: &GameData, game_state: &GameState) -> u32 {
    let constructed_projects = game_state
        .town
        .constructed_building_ids
        .iter()
        .filter(|building_id| {
            data.buildings.buildings.iter().any(|building| {
                building.id == **building_id
                    && matches!(building.category.as_str(), "project" | "prestige")
            })
        })
        .count() as u32;
    constructed_projects.max(game_state.town.completed_project_ids.len() as u32)
}

pub(crate) fn tick_town_situations(game_state: &mut GameState) -> Vec<String> {
    let mut expired = Vec::new();
    for situation in &mut game_state.town.active_situations {
        situation.days_remaining = situation.days_remaining.saturating_sub(1);
        if situation.days_remaining == 0 {
            expired.push(format!("{} has passed.", situation.label));
        }
    }
    game_state
        .town
        .active_situations
        .retain(|situation| situation.days_remaining > 0);
    expired
}

pub(crate) fn start_town_situation_from_event(
    game_state: &mut GameState,
    event: &EventData,
) -> Option<String> {
    if event.situation_days == 0 {
        return None;
    }
    let label = event
        .situation_label
        .clone()
        .unwrap_or_else(|| event.text.chars().take(42).collect::<String>());
    if game_state
        .town
        .active_situations
        .iter()
        .any(|situation| situation.event_id == event.id)
    {
        return None;
    }
    game_state.town.active_situations.push(TownSituationState {
        event_id: event.id.clone(),
        label: label.clone(),
        days_remaining: event.situation_days,
        upkeep_pressure_pct: event.situation_upkeep_pressure_pct,
        guest_pressure_bonus: event.situation_guest_bonus,
    });
    Some(format!(
        "{label} will shape the next {} days.",
        event.situation_days
    ))
}

pub(crate) fn upkeep_pressure_pct(game_state: &GameState) -> u32 {
    game_state
        .town
        .active_situations
        .iter()
        .map(|situation| situation.upkeep_pressure_pct)
        .sum()
}

pub(crate) fn active_situation_guest_bonus(game_state: &GameState) -> u32 {
    game_state
        .town
        .active_situations
        .iter()
        .map(|situation| situation.guest_pressure_bonus)
        .sum()
}

fn inferred_room_niche(room: &GuildRoomData) -> &'static str {
    if room.id.contains("stage") {
        "performance"
    } else if room.id.contains("nursery") {
        "hatchery"
    } else if room.id.contains("golemkin") {
        "corruption"
    } else {
        "comfort"
    }
}

fn inferred_mission_role_bonus(mission: &MissionData, party: &[&CompanionState]) -> i32 {
    let role = match mission.reward_focus.as_str() {
        "materials" | "relics" => "delver",
        "residue" => "corruption_adept",
        "eggs" => "hatchery_specialist",
        _ => "versatile",
    };
    party
        .iter()
        .map(|monster| role_affinity(monster, role))
        .sum()
}

fn request_from_template(
    data: &GameData,
    game_state: &GameState,
    template: &ContractData,
    chain_depth: u32,
) -> ContractState {
    let archetype_name = data
        .patron_archetypes
        .archetypes
        .iter()
        .find(|archetype| archetype.id == template.archetype_id)
        .map(|archetype| archetype.name.clone())
        .unwrap_or_else(|| template.archetype_id.clone());
    ContractState {
        request_id: format!(
            "guest_request_{:03}_chain_{chain_depth}",
            game_state.current_day as usize * 10 + game_state.active_contracts.len() + 1
        ),
        template_id: template.id.clone(),
        category: template.category.clone(),
        patron_tier_id: template.patron_tier_id.clone(),
        guest_name: data
            .story_events
            .guest_name_template
            .replace("{archetype}", &archetype_name),
        archetype_id: template.archetype_id.clone(),
        requested_room_id: template.requested_room_id.clone(),
        required_species_ids: template.required_species_ids.clone(),
        minimum_quality_rank: template.minimum_quality_rank,
        required_skill_thresholds: ContractSkillRequirementState {
            scouting: template.required_skill_thresholds.scouting,
            guarding: template.required_skill_thresholds.guarding,
            hospitality: template.required_skill_thresholds.hospitality,
            crafting: template.required_skill_thresholds.crafting,
            charm: template.required_skill_thresholds.charm,
            recovery: template.required_skill_thresholds.recovery,
            bargaining: template.required_skill_thresholds.bargaining,
            navigation: template.required_skill_thresholds.navigation,
            arcana: template.required_skill_thresholds.arcana,
            strength: template.required_skill_thresholds.strength,
        },
        required_work_history_thresholds: ContractHistoryRequirementState {
            scouting_runs: template.required_work_history_thresholds.scouting_runs,
            guard_duties: template.required_work_history_thresholds.guard_duties,
            hospitality_jobs: template.required_work_history_thresholds.hospitality_jobs,
            craft_jobs: template.required_work_history_thresholds.craft_jobs,
            contracts_completed: template
                .required_work_history_thresholds
                .contracts_completed,
            recovery_shifts: template.required_work_history_thresholds.recovery_shifts,
            hatchery_assists: template.required_work_history_thresholds.hatchery_assists,
        },
        reward: ResourcesState {
            gold: template.reward.gold,
            tower_materials: template.reward.tower_materials,
            eggs: template.reward.eggs,
            relics: template.reward.relics,
            arcane_residue: template.reward.arcane_residue,
        },
        penalty_gold: template.penalty_gold,
        deadline_day: game_state.current_day + template.deadline_days,
        preparation_quality_required: template.preparation_quality_required,
        preparation_quality_bonus: template.preparation_quality_bonus,
        status: ContractStatus::Pending,
        assigned_monster_id: None,
        chain_depth,
        partial_progress: 0,
    }
}
