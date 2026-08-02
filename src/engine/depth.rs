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

/// How this companion reads to the tower: the single classifier behind both
/// `role_affinity`'s mission bonus and the label the profile screen shows.
///
/// The UI carried an independent copy of this branching with different label
/// strings. They agreed only by hand — updating one and not the other would have
/// told the player she is a performer while the engine scored her a delver and
/// quietly withheld the mission role bonus.
pub fn monster_role(data: &GameData, monster: &CompanionState) -> &'static str {
    let stats = crate::engine::effective_stats(data, monster);
    if monster.corruption >= 10 || monster.trait_ids.iter().any(|id| id == "corruption_tuned") {
        "corruption_adept"
    } else if monster.work_history.hatchery_assists > 0
        || monster.trait_ids.iter().any(|id| id == "hatchery_attuned")
    {
        "hatchery_specialist"
    } else if monster.skills.charm >= 2 || stats.charm >= stats.power + 2 {
        "performer"
    } else if stats.power >= stats.charm + 2 {
        "delver"
    } else if monster.bond >= 8 || monster.trait_ids.iter().any(|id| id == "calming_presence") {
        "comfort"
    } else {
        "versatile"
    }
}

pub(crate) fn role_affinity(data: &GameData, monster: &CompanionState, role: &str) -> i32 {
    if role.is_empty() {
        return 0;
    }

    let affinity = &data.config.day_cycle.role_affinity;
    let role_of_monster = monster_role(data, monster);
    if role_of_monster == role {
        affinity.matched_bonus
    } else if role_of_monster == "versatile" {
        affinity.versatile_bonus
    } else {
        -off_role_penalty(data, monster)
    }
}

/// What working outside her role costs a companion.
///
/// Off-role used to be a flat zero for everybody, which made a species' tier
/// pure upside: a `gargoyle_stairwarden` was exactly as flexible as a
/// `slime_companion` and strictly stronger, so there was never a reason to keep
/// a low tier once a high one was available. Capability is now paid for with
/// narrowness — the weakest species work anywhere for free, the strongest are
/// specialists who lose ground on unfamiliar work.
///
/// Expressed as a penalty on the strong rather than a bonus to the weak on
/// purpose: crediting low tiers off-role inflated their apparent value and the
/// policy started staffing them onto work they were bad at, which cost the
/// deterministic campaign a building tier and eleven debt payments.
fn off_role_penalty(data: &GameData, monster: &CompanionState) -> i32 {
    let affinity = &data.config.day_cycle.role_affinity;
    let Some(species) = crate::engine::species_of(data, monster) else {
        return 0;
    };
    let stat_total = crate::engine::species_stat_total(species);
    let floor = affinity.flexibility_stat_floor;
    let ceiling = affinity.flexibility_stat_ceiling.max(floor + 1);

    if stat_total <= floor {
        return 0;
    }
    if stat_total >= ceiling {
        return affinity.off_role_penalty_max;
    }
    let above_floor = stat_total - floor;
    let span = ceiling - floor;
    affinity.off_role_penalty_max * above_floor as i32 / span as i32
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

/// Hazard a floor presents to one run, before the party's own strengths.
///
/// Depth and hazard tags push it up; the guild's familiarity with the floor
/// pulls it back down. That second term is what makes a deep tower playable at
/// all — nothing else on the player's side scales with depth, so without it a
/// floor's authored `difficulty` would have to stay nearly flat for twenty-five
/// floors, and depth would stop meaning anything. A floor is brutal on first
/// contact and becomes routine once the guild has walked it enough.
pub(crate) fn floor_hazard_risk(
    data: &GameData,
    game_state: &GameState,
    floor: &TowerFloorData,
    mission: &MissionData,
) -> i32 {
    let day_cycle = &data.config.day_cycle;
    let raw_hazard = floor.depth as i32 * day_cycle.depth_hazard_per_floor
        + floor.hazard_tags.len() as i32 * day_cycle.hazard_tag_risk
        + mission.hazard_risk_modifier_pct;

    (raw_hazard - floor_familiarity_relief(data, game_state, floor)).max(0)
}

/// How much the guild's survey notes take off a floor's hazard, capped so a
/// well-trodden floor never becomes free.
fn floor_familiarity_relief(
    data: &GameData,
    game_state: &GameState,
    floor: &TowerFloorData,
) -> i32 {
    let day_cycle = &data.config.day_cycle;
    let surveys = game_state
        .town
        .floor_surveys
        .iter()
        .find(|entry| entry.floor_id == floor.id)
        .map(|entry| entry.surveys)
        .unwrap_or_default();

    (surveys as i32)
        .saturating_mul(day_cycle.survey_familiarity_relief)
        .clamp(0, day_cycle.max_survey_familiarity_relief.max(0))
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
                .map(|monster| role_affinity(data, monster, role))
                .sum::<i32>()
        })
        .unwrap_or_else(|| inferred_mission_role_bonus(data, mission, party));
    let project_bonus = town_project_count(data, game_state) as i32 * 2;
    let hazard_risk = floor_hazard_risk(data, game_state, floor, mission);
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
        relic_bonus: u32::from(
            mission.reward_focus == "relics" && !floor.relic_drop_ids.is_empty(),
        ),
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
        .map(|role| role_affinity(data, monster, role).max(0) as u32)
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
    // Preparation quality deliberately does *not* gate the partial path. It is
    // what separates full pay from half: a guild that did not staff up still
    // delivers, just not to the standard the booking asked for. Requiring it
    // here as well made the fallback stricter than the main path.
    template.partial_success_score > 0
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
        .live_contracts()
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

fn inferred_mission_role_bonus(
    data: &GameData,
    mission: &MissionData,
    party: &[&CompanionState],
) -> i32 {
    let role = match mission.reward_focus.as_str() {
        "materials" | "relics" => "delver",
        "residue" => "corruption_adept",
        "eggs" => "hatchery_specialist",
        _ => "versatile",
    };
    party
        .iter()
        .map(|monster| role_affinity(data, monster, role))
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
            "contract_{:03}_chain_{chain_depth}",
            game_state.current_day as usize * 10 + game_state.live_contract_count() + 1
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
    }
}

/// Floors that can never open again, as distinct from floors nobody has walked
/// to yet.
///
/// The survey chain is serial, so one unrunnable link strands every floor
/// beneath it — and nothing else in a simulation report notices, because a floor
/// nobody enters changes no number. The recorded case was a mutation consuming
/// the guild's last Minotaur Porter, which closed `broodpens` and eleven floors
/// under it while every other assertion stayed green.
///
/// A floor is stranded when a `required_roster` species is neither on the roster
/// nor obtainable: no unlocked floor drops its egg and no egg in inventory can
/// become one. Being merely un-surveyed is progress, not a dead end, and must
/// not be reported here — that distinction is the whole point.
#[cfg(test)]
pub(crate) fn stranded_floor_ids(data: &GameData, game_state: &GameState) -> Vec<String> {
    let is_unlocked = |floor_id: &str| {
        game_state
            .town
            .unlocked_floor_ids
            .iter()
            .any(|id| id == floor_id)
    };

    let species_obtainable = |species_id: &str, minimum_quality_rank: u8| {
        let minimum = minimum_quality_rank.max(1);
        if game_state
            .monsters
            .iter()
            .any(|monster| monster.species_id == species_id && monster.quality_rank >= minimum)
        {
            return true;
        }
        if game_state
            .egg_inventory
            .iter()
            .any(|egg| egg.possible_species_ids.iter().any(|id| id == species_id))
        {
            return true;
        }
        // Any unlocked floor that drops this egg means the guild can go and
        // farm one. Rank is left out on purpose: grade is a roll, so a floor
        // that can produce the species can eventually produce a good one.
        data.floors.floors.iter().any(|floor| {
            is_unlocked(&floor.id)
                && floor
                    .egg_species_entries
                    .iter()
                    .any(|entry| entry.species_id == species_id)
        })
    };

    data.floors
        .floors
        .iter()
        .filter(|floor| !is_unlocked(&floor.id))
        .filter(|floor| {
            floor.required_roster.iter().any(|requirement| {
                !species_obtainable(&requirement.species_id, requirement.minimum_quality_rank)
            })
        })
        .map(|floor| floor.id.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::test_game_data;
    use crate::state::FloorSurveyState;

    fn floor_named<'a>(data: &'a GameData, id: &str) -> &'a TowerFloorData {
        data.floors
            .floors
            .iter()
            .find(|floor| floor.id == id)
            .expect("test floor should exist")
    }

    fn mission_named<'a>(data: &'a GameData, id: &str) -> &'a MissionData {
        data.missions
            .missions
            .iter()
            .find(|mission| mission.id == id)
            .expect("test mission should exist")
    }

    #[test]
    fn familiarity_relief_lowers_hazard_and_then_stops() {
        let data = test_game_data();
        let mut game_state = crate::engine::create_new_game_state(&data);
        let floor = floor_named(&data, "floor_1_slick_cellars").clone();
        let mission = mission_named(&data, "resource_run").clone();

        let unknown = floor_hazard_risk(&data, &game_state, &floor, &mission);

        game_state.town.floor_surveys.push(FloorSurveyState {
            floor_id: floor.id.clone(),
            surveys: 2,
        });
        let walked_twice = floor_hazard_risk(&data, &game_state, &floor, &mission);
        assert!(
            walked_twice < unknown,
            "a surveyed floor should be less hazardous: {walked_twice} vs {unknown}"
        );

        game_state.town.floor_surveys[0].surveys = 500;
        let walked_forever = floor_hazard_risk(&data, &game_state, &floor, &mission);
        let cap = data.config.day_cycle.max_survey_familiarity_relief;
        assert_eq!(
            walked_forever,
            unknown - cap,
            "relief must stop at the configured cap, never make a floor free"
        );
    }

    #[test]
    fn hazard_never_falls_below_zero() {
        let data = test_game_data();
        let mut game_state = crate::engine::create_new_game_state(&data);
        let mut floor = floor_named(&data, "floor_1_slick_cellars").clone();
        floor.hazard_tags.clear();
        floor.depth = 1;
        let mission = mission_named(&data, "resource_run").clone();
        game_state.town.floor_surveys.push(FloorSurveyState {
            floor_id: floor.id.clone(),
            surveys: 9_999,
        });

        assert!(floor_hazard_risk(&data, &game_state, &floor, &mission) >= 0);
    }

    /// The relic gate used to read `floor.depth >= 3`, which silently ignored
    /// the relics a floor actually declares. A shallow floor holding a named
    /// relic should yield it; a deep floor holding none should not.
    #[test]
    fn relic_yield_follows_declared_drops_not_depth() {
        let data = test_game_data();
        let game_state = crate::engine::create_new_game_state(&data);
        let relic_raid = mission_named(&data, "relic_raid").clone();
        let base = floor_named(&data, "floor_2_molten_baths").clone();

        let mut shallow_with_relic = base.clone();
        shallow_with_relic.depth = 1;
        shallow_with_relic.relic_drop_ids = vec!["molten_collar".to_owned()];
        let profile = expedition_depth_profile(
            &data,
            &game_state,
            &shallow_with_relic,
            &relic_raid,
            &ExpeditionPriority::Balanced,
            &[],
        );
        assert_eq!(profile.relic_bonus, 1);

        let mut deep_without_relic = base.clone();
        deep_without_relic.depth = 20;
        deep_without_relic.relic_drop_ids.clear();
        let profile = expedition_depth_profile(
            &data,
            &game_state,
            &deep_without_relic,
            &relic_raid,
            &ExpeditionPriority::Balanced,
            &[],
        );
        assert_eq!(
            profile.relic_bonus, 0,
            "depth alone must not conjure a relic the floor does not hold"
        );
    }

    /// A relic-bearing floor still only pays out on a relic-focused mission.
    #[test]
    fn a_salvage_run_does_not_yield_relics() {
        let data = test_game_data();
        let game_state = crate::engine::create_new_game_state(&data);
        let floor = floor_named(&data, "floor_3_gilded_kennels").clone();
        assert!(!floor.relic_drop_ids.is_empty());

        let profile = expedition_depth_profile(
            &data,
            &game_state,
            &floor,
            mission_named(&data, "resource_run"),
            &ExpeditionPriority::Balanced,
            &[],
        );

        assert_eq!(profile.relic_bonus, 0);
    }
}
