use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiTextData {
    pub version: String,
    pub common: CommonUiText,
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
    pub none_label: String,
    pub unknown_label: String,
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
    /// Shown instead of a job state when a companion is already committed to a
    /// contract that resolves today, so the card explains why she cannot be
    /// rostered rather than offering a button that errors.
    pub assignment_booked_label: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainMenuUiText {
    pub panel_title: String,
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
    pub debt_panel_title: String,
    pub onboarding_empty_roster_lines: Vec<String>,
    pub onboarding_chamber_lines: Vec<String>,
    pub onboarding_room_setup_lines: Vec<String>,
    pub onboarding_debt_lines: Vec<String>,
    pub onboarding_active_roster_lines: Vec<String>,
    pub resources_eggs_label: String,
    pub resources_relics_label: String,
    pub resources_arcane_residue_label: String,
    pub monster_profile_button: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterProfileUiText {
    pub title_template: String,
    pub subtitle: String,
    pub profile_summary_panel_title: String,
    pub best_next_use_panel_title: String,
    pub portrait_panel_title: String,
    pub core_stats_panel_title: String,
    pub traits_panel_title: String,
    pub species_label: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownManagementUiText {
    /// The repeatable-project sink, which is otherwise invisible: a build limit
    /// of forty and no unlocks reads as pointless unless the screen says what
    /// the thing is for.
    pub projects_status_template: String,
    pub projects_none_message: String,
    pub title: String,
    pub subtitle: String,
    pub buildings_panel_title: String,
    pub selected_building_panel_title: String,
    pub progression_panel_title: String,
    pub build_selected_button: String,
    pub cost_panel_title: String,
    pub effects_panel_title: String,
    pub category_label: String,
    pub unlocks_rooms_label: String,
    pub unlocks_floors_label: String,
    pub unlocks_species_label: String,
    pub built_count_label: String,
    pub status_label: String,
    pub available_label: String,
    pub built_out_label: String,
    pub locked_by_cost_label: String,
    pub built_label: String,
    pub rooms_label: String,
    pub floors_label: String,
    pub species_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildHallManagementUiText {
    pub title: String,
    pub subtitle: String,
    pub no_rooms_message: String,
    pub rooms_panel_title: String,
    pub assign_button: String,
    pub rest_button: String,
    pub idle_button: String,
    pub room_job_kind_label: String,
    pub preparation_quality_label: String,
    pub materials_label: String,
    pub status_label: String,
    pub no_preview_message: String,
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
    pub eligible_companions_panel_title: String,
    pub no_active_requests_message: String,
    pub no_selected_request_message: String,
    pub no_requests_title: String,
    pub no_selected_request_title: String,
    pub clear_assignment_button: String,
    pub assigned_button: String,
    pub status_label: String,
    pub deadline_day_template: String,
    pub category_label: String,
    pub patron_tier_label: String,
    pub preparation_quality_label: String,
    pub species_label: String,
    pub penalty_gold_template: String,
    pub assigned_label: String,
    pub context_gold_label: String,
    pub roster_label: String,
    pub context_accepted_requests_label: String,
    pub eligible_summary_template: String,
    pub eligible_label: String,
    pub blocked_label: String,
    pub no_roster_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HatcheryManagementUiText {
    /// Where an egg came from. A refined egg was indistinguishable from a wild
    /// find in the inventory — same card, same everything but a higher grade,
    /// with nothing saying two eggs had been spent to make it.
    pub egg_origin_wild_label: String,
    pub egg_origin_refined_label: String,
    pub egg_origin_prepared_label: String,
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
    pub sources_label: String,
    pub locked_outcome_label: String,
    pub prepared_outcome_label: String,
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
    pub priority_panel_title: String,
    pub team_panel_title: String,
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
    pub status_label: String,
    pub no_preview_message: String,
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
    /// Where the day's operating costs actually went. The summary has carried
    /// the wage/cleaning/maintenance split all along and only ever showed the
    /// total, which is the one number a player cannot act on.
    pub upkeep_breakdown_template: String,
    pub expedition_prep_template: String,
    pub expedition_prep_shortfall_label: String,
    pub special_events_template: String,
    pub contract_offers_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsUiText {
    pub panel_title: String,
    pub display_heading: String,
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
