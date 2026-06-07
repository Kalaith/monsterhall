use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiTextData {
    pub version: String,
    pub common: CommonUiText,
    pub loading: LoadingUiText,
    pub main_menu: MainMenuUiText,
    pub opening: OpeningUiText,
    pub town_overview: TownOverviewUiText,
    pub monster_profile: MonsterProfileUiText,
    pub town_management: TownManagementUiText,
    pub guild_hall_management: GuildHallManagementUiText,
    pub contract_desk: ContractDeskUiText,
    pub hatchery_management: HatcheryManagementUiText,
    pub journal: JournalUiText,
    pub expedition_planning: ExpeditionPlanningUiText,
    pub day_results: DayResultsUiText,
    pub settings: SettingsUiText,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonUiText {
    pub settings_button: String,
    pub return_to_town_button: String,
    pub main_menu_button: String,
    pub end_day_button: String,
    pub save_campaign_button: String,
    pub expedition_desk_button: String,
    pub guild_jobs_button: String,
    pub guest_desk_button: String,
    pub chamber_button: String,
    pub journal_button: String,
    pub town_planner_button: String,
    pub quit_game_button: String,
    pub close_button: String,
    pub assign_button: String,
    pub rest_button: String,
    pub idle_button: String,
    pub selected_suffix: String,
    pub none_label: String,
    pub unknown_label: String,
    pub error_panel_title: String,
    pub gold_unit: String,
    pub materials_unit: String,
    pub eggs_unit: String,
    pub relics_unit: String,
    pub residue_unit: String,
    pub no_resource_cost_message: String,
    pub assignment_idle_label: String,
    pub assignment_guild_job_label: String,
    pub assignment_resting_label: String,
    pub assignment_expedition_label: String,
    pub guest_status_pending_label: String,
    pub guest_status_accepted_label: String,
    pub guest_status_completed_label: String,
    pub guest_status_failed_label: String,
    pub guest_status_declined_label: String,
    pub species_any_label: String,
    pub skill_label_scouting: String,
    pub skill_label_guarding: String,
    pub skill_label_hospitality: String,
    pub skill_label_crafting: String,
    pub skill_label_charm: String,
    pub work_history_label_scouting: String,
    pub work_history_label_guarding: String,
    pub work_history_label_hospitality: String,
    pub work_history_label_crafting: String,
    pub work_history_label_contracts: String,
    pub work_history_label_recovery: String,
    pub work_history_label_hatchery: String,
    pub egg_grade_origin_label: String,
    pub egg_grade_common_label: String,
    pub egg_grade_unusual_label: String,
    pub egg_grade_rare_label: String,
    pub egg_grade_deepborn_label: String,
    pub egg_locked_outcome_message: String,
    pub egg_possible_species_template: String,
    pub egg_prepared_for_template: String,
    pub skill_summary_template: String,
    pub work_history_summary_template: String,
    pub building_guild_income_template: String,
    pub building_expedition_success_template: String,
    pub building_egg_discovery_template: String,
    pub building_injury_recovery_template: String,
    pub building_stress_recovery_template: String,
    pub building_charm_training_template: String,
    pub building_population_cap_template: String,
    pub building_worker_limit_template: String,
    pub building_unlocks_rooms_template: String,
    pub building_unlocks_floors_template: String,
    pub building_unlocks_species_template: String,
    pub building_unlocks_clients_template: String,
    pub building_no_effect_message: String,
    pub corruption_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadingUiText {
    pub panel_title: String,
    pub game_title: String,
    pub loading_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainMenuUiText {
    pub panel_title: String,
    pub description_lines: Vec<String>,
    pub new_campaign_button: String,
    pub continue_campaign_button: String,
    pub no_save_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpeningUiText {
    pub panel_title: String,
    pub status_day_label: String,
    pub status_gold_label: String,
    pub status_materials_label: String,
    pub status_eggs_label: String,
    pub status_roster_label: String,
    pub first_client_reward_template: String,
    pub skill_gains_label: String,
    pub work_history_gains_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownOverviewUiText {
    pub subtitle: String,
    pub priority_panel_title: String,
    pub priority_no_roster_title: String,
    pub priority_no_roster_detail: String,
    pub priority_eggs_title: String,
    pub priority_eggs_detail: String,
    pub priority_debt_title: String,
    pub priority_debt_detail_template: String,
    pub priority_guests_title: String,
    pub priority_guests_detail: String,
    pub priority_growth_title: String,
    pub priority_growth_detail: String,
    pub snapshot_panel_title: String,
    pub roster_panel_title: String,
    pub campaign_panel_title: String,
    pub resources_panel_title: String,
    pub debt_panel_title: String,
    pub guest_pressure_panel_title: String,
    pub onboarding_panel_title: String,
    pub town_actions_panel_title: String,
    pub no_active_debt_message: String,
    pub no_deadline_message: String,
    pub no_egg_sources_message: String,
    pub onboarding_empty_roster_lines: Vec<String>,
    pub onboarding_chamber_lines: Vec<String>,
    pub onboarding_room_setup_lines: Vec<String>,
    pub onboarding_debt_lines: Vec<String>,
    pub onboarding_active_roster_lines: Vec<String>,
    pub campaign_day_label: String,
    pub campaign_workers_label: String,
    pub campaign_status_label: String,
    pub campaign_queued_expedition_template: String,
    pub resources_gold_label: String,
    pub resources_materials_label: String,
    pub resources_eggs_label: String,
    pub resources_relics_label: String,
    pub resources_arcane_residue_label: String,
    pub debt_milestone_label: String,
    pub debt_due_gold_label: String,
    pub debt_days_remaining_label: String,
    pub debt_missed_payments_label: String,
    pub guest_active_requests_label: String,
    pub guest_accepted_label: String,
    pub guest_next_deadline_label: String,
    pub guest_egg_stock_label: String,
    pub guest_egg_sources_label: String,
    pub monster_species_label: String,
    pub monster_portrait_label: String,
    pub monster_skills_label: String,
    pub monster_history_label: String,
    pub monster_tower_stats_template: String,
    pub monster_traits_label: String,
    pub monster_condition_template: String,
    pub monster_assignment_label: String,
    pub monster_profile_button: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterProfileUiText {
    pub title_template: String,
    pub subtitle: String,
    pub profile_summary_panel_title: String,
    pub readiness_panel_title: String,
    pub best_next_use_panel_title: String,
    pub portrait_panel_title: String,
    pub core_stats_panel_title: String,
    pub skills_panel_title: String,
    pub work_history_panel_title: String,
    pub traits_panel_title: String,
    pub profile_status_label: String,
    pub species_label: String,
    pub portrait_key_label: String,
    pub assignment_label: String,
    pub condition_panel_title: String,
    pub readiness_ready_label: String,
    pub readiness_rest_label: String,
    pub readiness_hurt_label: String,
    pub best_next_guild_job_label: String,
    pub best_next_rest_label: String,
    pub best_next_expedition_label: String,
    pub best_next_training_label: String,
    pub release_button: String,
    pub fatigue_label: String,
    pub stress_label: String,
    pub injury_label: String,
    pub power_label: String,
    pub charm_label: String,
    pub endurance_label: String,
    pub instinct_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownManagementUiText {
    pub title: String,
    pub subtitle: String,
    pub buildings_panel_title: String,
    pub selected_building_panel_title: String,
    pub progression_panel_title: String,
    pub build_selected_button: String,
    pub cost_panel_title: String,
    pub effects_panel_title: String,
    pub unlocks_panel_title: String,
    pub milestone_panel_title: String,
    pub cost_template: String,
    pub category_label: String,
    pub passive_label: String,
    pub unlocks_rooms_label: String,
    pub unlocks_floors_label: String,
    pub unlocks_species_label: String,
    pub built_count_label: String,
    pub unlocked_rooms_label: String,
    pub unlocked_floors_label: String,
    pub unlocked_species_label: String,
    pub status_label: String,
    pub available_label: String,
    pub built_out_label: String,
    pub locked_by_cost_label: String,
    pub built_label: String,
    pub rooms_label: String,
    pub floors_label: String,
    pub species_label: String,
    pub next_destination_label: String,
    pub build_ready_message: String,
    pub built_out_message: String,
    pub blocked_by_cost_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildHallManagementUiText {
    pub title: String,
    pub subtitle: String,
    pub no_rooms_message: String,
    pub rooms_panel_title: String,
    pub room_details_panel_title: String,
    pub worker_preview_panel_title: String,
    pub available_workers_panel_title: String,
    pub assign_button: String,
    pub rest_button: String,
    pub idle_button: String,
    pub service_label: String,
    pub service_tier_label: String,
    pub trains_label: String,
    pub adds_history_label: String,
    pub active_client_tier_label: String,
    pub supports_label: String,
    pub room_job_kind_label: String,
    pub preparation_quality_label: String,
    pub materials_label: String,
    pub reputation_label: String,
    pub status_label: String,
    pub assigned_preview_gold_label: String,
    pub assigned_preview_residue_label: String,
    pub room_work_label: String,
    pub room_focus_bonus_template: String,
    pub no_preview_message: String,
    pub worker_summary_template: String,
    pub worker_card_template: String,
    pub selected_room_panel_title: String,
    pub assigned_here_panel_title: String,
    pub available_panel_title: String,
    pub no_room_selected_title: String,
    pub empty_bucket_title: String,
    pub empty_bucket_detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDeskUiText {
    pub title: String,
    pub subtitle: String,
    pub active_requests_panel_title: String,
    pub selected_request_panel_title: String,
    pub campaign_context_panel_title: String,
    pub eligible_girls_panel_title: String,
    pub no_active_requests_message: String,
    pub no_selected_request_message: String,
    pub no_requests_title: String,
    pub no_selected_request_title: String,
    pub clear_assignment_button: String,
    pub assign_to_request_button: String,
    pub assigned_button: String,
    pub status_label: String,
    pub deadline_day_template: String,
    pub room_label: String,
    pub reward_label: String,
    pub guest_label: String,
    pub category_label: String,
    pub patron_tier_label: String,
    pub preparation_quality_label: String,
    pub species_label: String,
    pub skill_thresholds_label: String,
    pub history_thresholds_label: String,
    pub penalty_gold_template: String,
    pub assigned_label: String,
    pub debt_label: String,
    pub debt_due_template: String,
    pub debt_not_active_message: String,
    pub context_day_label: String,
    pub context_gold_label: String,
    pub roster_label: String,
    pub context_roster_template: String,
    pub context_eggs_ready_label: String,
    pub context_accepted_requests_label: String,
    pub eligible_summary_template: String,
    pub eligible_label: String,
    pub blocked_label: String,
    pub no_roster_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HatcheryManagementUiText {
    pub title: String,
    pub subtitle: String,
    pub status_panel_title: String,
    pub inventory_panel_title: String,
    pub selected_egg_panel_title: String,
    pub possible_outcomes_heading: String,
    pub hatch_button: String,
    pub no_eggs_message: String,
    pub select_button: String,
    pub no_selected_egg_title: String,
    pub no_selected_egg_message: String,
    pub inventory_empty_title: String,
    pub status_label: String,
    pub inventory_count_template: String,
    pub sources_label: String,
    pub hatchable_now_label: String,
    pub egg_id_label: String,
    pub grade_label: String,
    pub source_floor_label: String,
    pub potential_outcomes_label: String,
    pub outcome_preview_label: String,
    pub locked_outcome_label: String,
    pub prepared_outcome_label: String,
    pub finish_hatch_cost_label: String,
    pub prepared_from_label: String,
    pub bound_message: String,
    pub review_required_message: String,
    pub scroll_up_message: String,
    pub scroll_down_message: String,
    pub unknown_outcome_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalUiText {
    pub title: String,
    pub subtitle: String,
    pub current_priority_panel_title: String,
    pub guidance_panel_title: String,
    pub event_log_panel_title: String,
    pub priority_label: String,
    pub recent_events_empty_message: String,
    pub scroll_up_message: String,
    pub scroll_down_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpeditionPlanningUiText {
    pub title: String,
    pub subtitle: String,
    pub floors_panel_title: String,
    pub floor_details_panel_title: String,
    pub mission_panel_title: String,
    pub priority_panel_title: String,
    pub team_panel_title: String,
    pub preview_panel_title: String,
    pub balanced_button: String,
    pub aggressive_button: String,
    pub safe_button: String,
    pub recovery_button: String,
    pub curiosity_button: String,
    pub assign_button: String,
    pub rest_button: String,
    pub idle_button: String,
    pub floor_depth_template: String,
    pub difficulty_label: String,
    pub available_missions_label: String,
    pub preview_floor_label: String,
    pub preview_mission_label: String,
    pub success_score_label: String,
    pub projected_materials_label: String,
    pub projected_arcane_residue_label: String,
    pub projected_eggs_label: String,
    pub projected_relics_label: String,
    pub injury_risk_score_label: String,
    pub status_label: String,
    pub no_preview_message: String,
    pub worker_assignment_template: String,
    pub no_floor_title: String,
    pub no_floor_message: String,
    pub team_empty_title: String,
    pub team_empty_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayResultsUiText {
    pub title_template: String,
    pub subtitle_template: String,
    pub guild_jobs_panel_title: String,
    pub expedition_panel_title: String,
    pub debt_panel_title: String,
    pub guests_panel_title: String,
    pub roster_updates_panel_title: String,
    pub event_log_panel_title: String,
    pub continue_button: String,
    pub no_debt_change_message: String,
    pub no_guest_contract_message: String,
    pub gold_earned_label: String,
    pub upkeep_paid_label: String,
    pub upkeep_shortfall_label: String,
    pub arcane_residue_earned_label: String,
    pub materials_label: String,
    pub arcane_residue_label: String,
    pub eggs_label: String,
    pub relics_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsUiText {
    pub panel_title: String,
    pub display_heading: String,
    pub display_subtitle: String,
    pub fullscreen_on_button: String,
    pub fullscreen_off_button: String,
    pub fullscreen_button: String,
    pub windowed_button: String,
    pub resolution_heading: String,
}

impl UiTextData {
    pub fn validate(&self) -> Result<(), String> {
        let value = serde_json::to_value(self)
            .map_err(|error| format!("ui_text could not be validated: {error}"))?;
        validate_ui_text_value(&value, "ui_text")
    }
}

fn validate_ui_text_value(value: &serde_json::Value, label: &str) -> Result<(), String> {
    match value {
        serde_json::Value::String(text) => validate_non_empty(text, label),
        serde_json::Value::Array(values) => validate_ui_text_array(values, label),
        serde_json::Value::Object(fields) => {
            for (field, nested) in fields {
                validate_ui_text_value(nested, &format!("{label}.{field}"))?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn validate_ui_text_array(values: &[serde_json::Value], label: &str) -> Result<(), String> {
    if values.is_empty() {
        return Err(format!("{label} must contain at least one line."));
    }

    for (index, value) in values.iter().enumerate() {
        match value {
            serde_json::Value::String(text) => {
                validate_non_empty(text, &format!("{label}[{index}]"))?
            }
            _ => validate_ui_text_value(value, &format!("{label}[{index}]"))?,
        }
    }

    Ok(())
}

fn validate_non_empty(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{label} must not be empty."))
    } else {
        Ok(())
    }
}
