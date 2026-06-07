//! Read-only UI formatting and lookup helpers for screen renderers.

use crate::data::{GameData, UiTextData};
use crate::engine::evaluate_contract_eligibility;
use crate::state::{GameState, CompanionJobState};
use crate::ui::actions::UiAction;
use crate::ui::theme;
use macroquad::prelude::Color;

#[derive(Debug, Clone)]
pub struct DailyPrioritySummary {
    pub title: String,
    pub detail: String,
    pub action_hint: String,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct WorkerDecisionSummary {
    pub prediction_line: String,
    pub highlight: Color,
}

#[derive(Debug, Clone)]
pub struct BuildingDecisionSummary {
    pub status_label: String,
    pub status_color: Color,
    pub can_afford: bool,
    pub build_count: usize,
    pub effect_lines: Vec<String>,
    pub unlock_labels: Vec<String>,
    pub next_destination: String,
}

#[derive(Debug, Clone)]
pub struct MonsterRoleSummary {
    pub readiness_label: String,
    pub readiness_color: Color,
    pub best_next_use: String,
}

mod decisions;

pub use decisions::{
    action_from_action_hint, building_decision_summary, daily_priority_summary,
    monster_role_summary, worker_decision_summary,
};

pub fn fill_template(template: &str, replacements: &[(&str, String)]) -> String {
    let mut result = template.to_owned();
    for (token, value) in replacements {
        result = result.replace(token, value);
    }
    result
}

pub fn format_resource_cost(
    ui_text: &UiTextData,
    cost: &crate::data::ResourceAmountData,
) -> String {
    let common_text = &ui_text.common;
    let mut parts = Vec::new();

    if cost.gold > 0 {
        parts.push(format!("{} {}", cost.gold, common_text.gold_unit));
    }
    if cost.tower_materials > 0 {
        parts.push(format!(
            "{} {}",
            cost.tower_materials, common_text.materials_unit
        ));
    }
    if cost.eggs > 0 {
        parts.push(format!("{} {}", cost.eggs, common_text.eggs_unit));
    }
    if cost.relics > 0 {
        parts.push(format!("{} {}", cost.relics, common_text.relics_unit));
    }
    if cost.arcane_residue > 0 {
        parts.push(format!("{} {}", cost.arcane_residue, common_text.residue_unit));
    }

    if parts.is_empty() {
        common_text.no_resource_cost_message.clone()
    } else {
        parts.join(" / ")
    }
}

pub fn describe_building_effects(
    ui_text: &UiTextData,
    building: &crate::data::BuildingData,
) -> Vec<String> {
    let common_text = &ui_text.common;
    let mut lines = Vec::new();

    if building.passive_modifiers.guild_income_pct != 0 {
        lines.push(fill_template(
            &common_text.building_guild_income_template,
            &[(
                "{value}",
                building.passive_modifiers.guild_income_pct.to_string(),
            )],
        ));
    }
    if building.passive_modifiers.expedition_success_pct != 0 {
        lines.push(fill_template(
            &common_text.building_expedition_success_template,
            &[(
                "{value}",
                building
                    .passive_modifiers
                    .expedition_success_pct
                    .to_string(),
            )],
        ));
    }
    if building.passive_modifiers.egg_discovery_flat != 0 {
        lines.push(fill_template(
            &common_text.building_egg_discovery_template,
            &[(
                "{value}",
                building.passive_modifiers.egg_discovery_flat.to_string(),
            )],
        ));
    }
    if building.passive_modifiers.injury_recovery_flat != 0 {
        lines.push(fill_template(
            &common_text.building_injury_recovery_template,
            &[(
                "{value}",
                building.passive_modifiers.injury_recovery_flat.to_string(),
            )],
        ));
    }
    if building.passive_modifiers.stress_recovery_flat != 0 {
        lines.push(fill_template(
            &common_text.building_stress_recovery_template,
            &[(
                "{value}",
                building.passive_modifiers.stress_recovery_flat.to_string(),
            )],
        ));
    }
    if building.passive_modifiers.charm_training_flat != 0 {
        lines.push(fill_template(
            &common_text.building_charm_training_template,
            &[(
                "{value}",
                building.passive_modifiers.charm_training_flat.to_string(),
            )],
        ));
    }
    if building.passive_modifiers.population_cap_flat != 0 {
        lines.push(fill_template(
            &common_text.building_population_cap_template,
            &[(
                "{value}",
                building.passive_modifiers.population_cap_flat.to_string(),
            )],
        ));
    }
    if building.passive_modifiers.town_job_limit_flat != 0 {
        lines.push(fill_template(
            &common_text.building_worker_limit_template,
            &[(
                "{value}",
                building
                    .passive_modifiers
                    .town_job_limit_flat
                    .to_string(),
            )],
        ));
    }

    if !building.unlocks.room_ids.is_empty() {
        lines.push(fill_template(
            &common_text.building_unlocks_rooms_template,
            &[("{value}", building.unlocks.room_ids.join(", "))],
        ));
    }
    if !building.unlocks.floor_ids.is_empty() {
        lines.push(fill_template(
            &common_text.building_unlocks_floors_template,
            &[("{value}", building.unlocks.floor_ids.join(", "))],
        ));
    }
    if !building.unlocks.species_ids.is_empty() {
        lines.push(fill_template(
            &common_text.building_unlocks_species_template,
            &[("{value}", building.unlocks.species_ids.join(", "))],
        ));
    }
    if !building.unlocks.patron_tiers.is_empty() {
        lines.push(fill_template(
            &common_text.building_unlocks_clients_template,
            &[("{value}", building.unlocks.patron_tiers.join(", "))],
        ));
    }

    if lines.is_empty() {
        lines.push(common_text.building_no_effect_message.clone());
    }

    lines
}

pub fn assignment_label<'a>(data: &'a GameData, job: &CompanionJobState) -> &'a str {
    let common_text = &data.ui_text.common;
    match job {
        CompanionJobState::Idle => &common_text.assignment_idle_label,
        CompanionJobState::GuildJob { .. } => &common_text.assignment_guild_job_label,
        CompanionJobState::Resting => &common_text.assignment_resting_label,
        CompanionJobState::OnExpedition { .. } => &common_text.assignment_expedition_label,
    }
}

pub fn floor_name_by_id(data: &GameData, floor_id: &str) -> String {
    data.floors
        .floors
        .iter()
        .find(|floor| floor.id == floor_id)
        .map(|floor| floor.name.clone())
        .unwrap_or_else(|| floor_id.to_owned())
}

pub fn room_name_by_id(data: &GameData, room_id: &str) -> String {
    data.guild_rooms
        .rooms
        .iter()
        .find(|room| room.id == room_id)
        .map(|room| room.name.clone())
        .unwrap_or_else(|| room_id.to_owned())
}

pub fn monster_name_by_id(game_state: &GameState, monster_id: &str) -> String {
    game_state
        .monsters
        .iter()
        .find(|monster| monster.id == monster_id)
        .map(|monster| monster.name.clone())
        .unwrap_or_else(|| monster_id.to_owned())
}

pub fn format_resources_state(data: &GameData, resources: &crate::state::ResourcesState) -> String {
    let resource_amount = crate::data::ResourceAmountData {
        gold: resources.gold,
        tower_materials: resources.tower_materials,
        eggs: resources.eggs,
        relics: resources.relics,
        arcane_residue: resources.arcane_residue,
    };
    format_resource_cost(&data.ui_text, &resource_amount)
}

pub fn guest_status_label<'a>(
    data: &'a GameData,
    status: &crate::state::ContractStatus,
) -> &'a str {
    let common_text = &data.ui_text.common;
    match status {
        crate::state::ContractStatus::Pending => &common_text.guest_status_pending_label,
        crate::state::ContractStatus::Accepted => &common_text.guest_status_accepted_label,
        crate::state::ContractStatus::Completed => &common_text.guest_status_completed_label,
        crate::state::ContractStatus::Failed => &common_text.guest_status_failed_label,
        crate::state::ContractStatus::Declined => &common_text.guest_status_declined_label,
    }
}

pub fn guest_species_requirement_label(
    data: &GameData,
    request: &crate::state::ContractState,
) -> String {
    if request.required_species_ids.is_empty() {
        data.ui_text.common.species_any_label.clone()
    } else {
        request
            .required_species_ids
            .iter()
            .map(|species_id| species_name_by_id(data, species_id))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

pub fn guest_skill_requirement_label(
    data: &GameData,
    skills: &crate::state::ContractSkillRequirementState,
) -> String {
    let common_text = &data.ui_text.common;
    let mut parts = Vec::new();
    if skills.scouting > 0 {
        parts.push(format!(
            "{}{}",
            common_text.skill_label_scouting.chars().next().unwrap_or('K'),
            skills.scouting
        ));
    }
    if skills.guarding > 0 {
        parts.push(format!(
            "{}{}",
            common_text.skill_label_guarding.chars().next().unwrap_or('O'),
            skills.guarding
        ));
    }
    if skills.hospitality > 0 {
        parts.push(format!(
            "{}{}",
            common_text
                .skill_label_hospitality
                .chars()
                .next()
                .unwrap_or('V'),
            skills.hospitality
        ));
    }
    if skills.crafting > 0 {
        parts.push(format!(
            "{}{}",
            common_text.skill_label_crafting.chars().next().unwrap_or('A'),
            skills.crafting
        ));
    }
    if skills.charm > 0 {
        parts.push(format!(
            "{}{}",
            common_text
                .skill_label_charm
                .chars()
                .next()
                .unwrap_or('S'),
            skills.charm
        ));
    }
    if parts.is_empty() {
        common_text.none_label.clone()
    } else {
        parts.join(" / ")
    }
}

pub fn guest_history_requirement_label(
    data: &GameData,
    history: &crate::state::ContractHistoryRequirementState,
) -> String {
    let common_text = &data.ui_text.common;
    let mut parts = Vec::new();
    if history.scouting_runs > 0 {
        parts.push(format!(
            "{} {}",
            common_text.work_history_label_scouting, history.scouting_runs
        ));
    }
    if history.guard_duties > 0 {
        parts.push(format!(
            "{} {}",
            common_text.work_history_label_guarding, history.guard_duties
        ));
    }
    if history.hospitality_jobs > 0 {
        parts.push(format!(
            "{} {}",
            common_text.work_history_label_hospitality, history.hospitality_jobs
        ));
    }
    if history.craft_jobs > 0 {
        parts.push(format!(
            "{} {}",
            common_text.work_history_label_crafting, history.craft_jobs
        ));
    }
    if history.contracts_completed > 0 {
        parts.push(format!(
            "{} {}",
            common_text.work_history_label_contracts, history.contracts_completed
        ));
    }
    if history.recovery_shifts > 0 {
        parts.push(format!(
            "{} {}",
            common_text.work_history_label_recovery, history.recovery_shifts
        ));
    }
    if history.hatchery_assists > 0 {
        parts.push(format!(
            "{} {}",
            common_text.work_history_label_hatchery, history.hatchery_assists
        ));
    }
    if parts.is_empty() {
        common_text.none_label.clone()
    } else {
        parts.join(" / ")
    }
}

pub fn evaluate_guest_candidate(
    data: &GameData,
    game_state: &GameState,
    request: &crate::state::ContractState,
    monster: &crate::state::CompanionState,
) -> crate::engine::ContractEligibilityReport {
    evaluate_contract_eligibility(data, game_state, request, monster)
}

pub fn quality_label(quality_rank: u8) -> String {
    format!("{} star", quality_rank.clamp(1, 3))
}

pub fn monster_quality_label(monster: &crate::state::CompanionState) -> String {
    quality_label(monster.quality_rank)
}

pub fn egg_quality_rank(egg: &crate::state::EggState) -> u8 {
    match egg.grade_score {
        0..=2 => 1,
        3..=4 => 2,
        _ => 3,
    }
}

pub fn egg_quality_label(egg: &crate::state::EggState) -> String {
    quality_label(egg_quality_rank(egg))
}

pub fn egg_grade_label<'a>(egg: &crate::state::EggState, data: &'a GameData) -> &'a str {
    let common_text = &data.ui_text.common;
    if egg.source_floor_id == "tower_core" {
        return &common_text.egg_grade_origin_label;
    }

    let grade_score = if egg.grade_score > 0 {
        egg.grade_score
    } else {
        data.floors
            .floors
            .iter()
            .find(|floor| floor.id == egg.source_floor_id)
            .map(|floor| floor.depth)
            .unwrap_or(1)
    };

    match grade_score {
        0 | 1 => &common_text.egg_grade_common_label,
        2 => &common_text.egg_grade_unusual_label,
        3 => &common_text.egg_grade_rare_label,
        _ => &common_text.egg_grade_deepborn_label,
    }
}

pub fn monster_depth_role_label(monster: &crate::state::CompanionState) -> &'static str {
    if monster.corruption >= 10 || monster.trait_ids.iter().any(|id| id == "corruption_tuned") {
        "instability adept"
    } else if monster.work_history.hatchery_assists > 0
        || monster.trait_ids.iter().any(|id| id == "hatchery_attuned")
    {
        "hatchery specialist"
    } else if monster.skills.charm >= 2 || monster.stats.charm >= monster.stats.power + 2 {
        "performer"
    } else if monster.stats.power >= monster.stats.charm + 2 {
        "delver"
    } else if monster.bond >= 8 || monster.trait_ids.iter().any(|id| id == "submissive") {
        "comfort specialist"
    } else {
        "versatile"
    }
}

pub fn egg_outcome_count_label(data: &GameData, egg: &crate::state::EggState) -> String {
    if egg.selected_species_id.is_some() {
        data.ui_text.common.egg_locked_outcome_message.clone()
    } else {
        fill_template(
            &data.ui_text.common.egg_possible_species_template,
            &[("{count}", egg.possible_species_ids.len().to_string())],
        )
    }
}

pub fn has_hatched_species(game_state: &GameState, species_id: &str) -> bool {
    game_state
        .story_progress
        .hatched_species_ids
        .iter()
        .any(|id| id == species_id)
        || game_state
            .monsters
            .iter()
            .any(|monster| monster.species_id == species_id)
}

pub fn known_species_name_by_id(
    data: &GameData,
    game_state: &GameState,
    species_id: &str,
) -> String {
    if has_hatched_species(game_state, species_id) {
        species_name_by_id(data, species_id)
    } else {
        data.ui_text.common.unknown_label.clone()
    }
}

pub fn egg_outcome_preview_label(
    egg: &crate::state::EggState,
    game_state: &GameState,
    data: &GameData,
) -> String {
    if let Some(species_id) = &egg.selected_species_id {
        return fill_template(
            &data.ui_text.common.egg_prepared_for_template,
            &[(
                "{species}",
                known_species_name_by_id(data, game_state, species_id),
            )],
        );
    }

    egg.possible_species_ids
        .iter()
        .take(3)
        .map(|species_id| known_species_name_by_id(data, game_state, species_id))
        .collect::<Vec<_>>()
        .join(" / ")
}

pub fn egg_origin_summary(game_state: &GameState, data: &GameData) -> String {
    let mut origins = game_state
        .egg_inventory
        .iter()
        .map(|egg| floor_name_by_id(data, &egg.source_floor_id))
        .collect::<Vec<_>>();
    origins.sort();
    origins.dedup();

    if origins.is_empty() {
        data.ui_text.common.none_label.clone()
    } else {
        origins.join(", ")
    }
}

pub fn species_name_by_id(data: &GameData, species_id: &str) -> String {
    data.species
        .species
        .iter()
        .find(|species| species.id == species_id)
        .map(|species| species.name.clone())
        .unwrap_or_else(|| species_id.to_owned())
}

pub fn species_portrait_key_by_id(data: &GameData, species_id: &str) -> String {
    data.species
        .species
        .iter()
        .find(|species| species.id == species_id)
        .map(|species| species.portrait_key.clone())
        .unwrap_or_else(|| "portrait_missing".to_owned())
}

pub fn trait_names_for_monster(
    data: &GameData,
    monster: &crate::state::CompanionState,
) -> String {
    let names = monster
        .trait_ids
        .iter()
        .filter_map(|trait_id| {
            data.traits
                .traits
                .iter()
                .find(|trait_data| trait_data.id == *trait_id)
                .map(|trait_data| format!("{} ({})", trait_data.name, trait_data.icon_key))
        })
        .collect::<Vec<_>>();

    if names.is_empty() {
        data.ui_text.common.none_label.clone()
    } else {
        names.join(", ")
    }
}

pub fn onboarding_lines(data: &GameData, game_state: &GameState) -> Vec<String> {
    if game_state.monsters.is_empty() {
        return data
            .ui_text
            .town_overview
            .onboarding_empty_roster_lines
            .clone();
    }

    if !game_state.story_progress.first_room_built || game_state.town.unlocked_room_ids.is_empty() {
        return data
            .ui_text
            .town_overview
            .onboarding_room_setup_lines
            .clone();
    }

    if !game_state.egg_inventory.is_empty() {
        return data.ui_text.town_overview.onboarding_chamber_lines.clone();
    }

    if game_state.debt.is_some() {
        return data.ui_text.town_overview.onboarding_debt_lines.clone();
    }

    data.ui_text
        .town_overview
        .onboarding_active_roster_lines
        .clone()
}

pub fn trained_skills_label(data: &GameData, skill_ids: &[String]) -> String {
    let common_text = &data.ui_text.common;
    let labels = skill_ids
        .iter()
        .map(|skill_id| match skill_id.as_str() {
            "scouting" => common_text.skill_label_scouting.as_str(),
            "guarding" => common_text.skill_label_guarding.as_str(),
            "hospitality" => common_text.skill_label_hospitality.as_str(),
            "crafting" => common_text.skill_label_crafting.as_str(),
            "charm" => common_text.skill_label_charm.as_str(),
            _ => common_text.unknown_label.as_str(),
        })
        .collect::<Vec<_>>();

    if labels.is_empty() {
        common_text.none_label.clone()
    } else {
        labels.join(", ")
    }
}

pub fn primary_skill_label<'a>(data: &'a GameData, skill_ids: &[String]) -> &'a str {
    let common_text = &data.ui_text.common;
    skill_ids
        .first()
        .map(|skill_id| match skill_id.as_str() {
            "scouting" => common_text.skill_label_scouting.as_str(),
            "guarding" => common_text.skill_label_guarding.as_str(),
            "hospitality" => common_text.skill_label_hospitality.as_str(),
            "crafting" => common_text.skill_label_crafting.as_str(),
            "charm" => common_text.skill_label_charm.as_str(),
            _ => common_text.unknown_label.as_str(),
        })
        .unwrap_or(&common_text.unknown_label)
}

pub fn companion_skill_summary(data: &GameData, monster: &crate::state::CompanionState) -> String {
    let skill_summary = fill_template(
        &data.ui_text.common.skill_summary_template,
        &[
            ("{scouting}", monster.skills.scouting.to_string()),
            ("{guarding}", monster.skills.guarding.to_string()),
            ("{hospitality}", monster.skills.hospitality.to_string()),
            ("{crafting}", monster.skills.crafting.to_string()),
            ("{charm}", monster.skills.charm.to_string()),
        ],
    );
    format!(
        "{} | Bond {} / Rep {}",
        skill_summary, monster.bond, monster.reputation
    )
}

pub fn work_history_summary(data: &GameData, monster: &crate::state::CompanionState) -> String {
    fill_template(
        &data.ui_text.common.work_history_summary_template,
        &[
            ("{scouting runs}", monster.work_history.scouting_runs.to_string()),
            ("{guarding}", monster.work_history.guard_duties.to_string()),
            ("{hospitality}", monster.work_history.hospitality_jobs.to_string()),
            ("{crafting}", monster.work_history.craft_jobs.to_string()),
            (
                "{completed contracts}",
                monster.work_history.contracts_completed.to_string(),
            ),
        ],
    )
}

pub fn history_gain_label(data: &GameData, history: &crate::state::CompanionWorkHistoryState) -> String {
    let mut parts = Vec::new();

    if history.scouting_runs > 0 {
        parts.push(format!("K+{}", history.scouting_runs));
    }
    if history.guard_duties > 0 {
        parts.push(format!("O+{}", history.guard_duties));
    }
    if history.hospitality_jobs > 0 {
        parts.push(format!("V+{}", history.hospitality_jobs));
    }
    if history.craft_jobs > 0 {
        parts.push(format!("A+{}", history.craft_jobs));
    }
    if history.contracts_completed > 0 {
        parts.push(format!("C+{}", history.contracts_completed));
    }
    if history.recovery_shifts > 0 {
        parts.push(format!("M+{}", history.recovery_shifts));
    }
    if history.hatchery_assists > 0 {
        parts.push(format!("B+{}", history.hatchery_assists));
    }

    if parts.is_empty() {
        data.ui_text.common.none_label.clone()
    } else {
        parts.join(" ")
    }
}

pub fn history_gain_label_from_progress(
    data: &GameData,
    history: &crate::data::CompanionWorkHistoryProgressionData,
) -> String {
    let state_like = crate::state::CompanionWorkHistoryState {
        scouting_runs: history.scouting_runs,
        guard_duties: history.guard_duties,
        hospitality_jobs: history.hospitality_jobs,
        craft_jobs: history.craft_jobs,
        contracts_completed: history.contracts_completed,
        recovery_shifts: history.recovery_shifts,
        hatchery_assists: history.hatchery_assists,
    };

    history_gain_label(data, &state_like)
}

pub fn opening_skill_gain_label(
    data: &GameData,
    skills: &crate::data::CompanionSkillProgressionData,
) -> String {
    let mut parts = Vec::new();

    if skills.scouting > 0 {
        parts.push(format!("K+{}", skills.scouting));
    }
    if skills.guarding > 0 {
        parts.push(format!("O+{}", skills.guarding));
    }
    if skills.hospitality > 0 {
        parts.push(format!("V+{}", skills.hospitality));
    }
    if skills.crafting > 0 {
        parts.push(format!("A+{}", skills.crafting));
    }
    if skills.charm > 0 {
        parts.push(format!("S+{}", skills.charm));
    }

    if parts.is_empty() {
        data.ui_text.common.none_label.clone()
    } else {
        parts.join(" ")
    }
}
