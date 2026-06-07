use serde::{Deserialize, Serialize};

use super::ui_text::UiTextData;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceAmountData {
    pub gold: u32,
    pub tower_materials: u32,
    pub eggs: u32,
    pub relics: u32,
    pub arcane_residue: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatBlockData {
    pub power: i32,
    pub charm: i32,
    pub endurance: i32,
    pub instinct: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfigData {
    pub primary_mode: String,
    pub keyboard_shortcuts_enabled: bool,
    pub keyboard_shortcuts_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfigData {
    pub native_save_path: String,
    pub web_storage_key: String,
    pub native_settings_path: String,
    pub web_settings_key: String,
    pub autosave_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownUiConfigData {
    pub target_width: u32,
    pub target_height: u32,
    pub town_panels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionOptionData {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfigData {
    pub start_fullscreen: bool,
    pub default_resolution_id: String,
    pub available_resolutions: Vec<ResolutionOptionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarterMonsterData {
    pub species_id: String,
    pub name: String,
    pub extra_traits: Vec<String>,
    pub stat_bonuses: StatBlockData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CompanionSkillProgressionData {
    #[serde(alias = "scouting")]
    pub scouting: u32,
    #[serde(alias = "guarding")]
    pub guarding: u32,
    #[serde(alias = "hospitality")]
    pub hospitality: u32,
    #[serde(alias = "crafting")]
    pub crafting: u32,
    #[serde(alias = "charm")]
    pub charm: u32,
    pub recovery: u32,
    pub bargaining: u32,
    pub navigation: u32,
    pub arcana: u32,
    pub strength: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CompanionWorkHistoryProgressionData {
    #[serde(alias = "kiss_count")]
    pub scouting_runs: u32,
    pub guard_duties: u32,
    pub hospitality_jobs: u32,
    pub craft_jobs: u32,
    #[serde(alias = "contract_count")]
    pub contracts_completed: u32,
    #[serde(alias = "recovery_shift_count")]
    pub recovery_shifts: u32,
    #[serde(alias = "birth_count")]
    pub hatchery_assists: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewGameConfigData {
    pub starting_day: u32,
    pub starting_resources: ResourceAmountData,
    pub starting_building_ids: Vec<String>,
    pub starting_room_ids: Vec<String>,
    pub starting_floor_ids: Vec<String>,
    pub starting_species_ids: Vec<String>,
    pub starter_monsters: Vec<StarterMonsterData>,
    pub party_size: u8,
    #[serde(alias = "guild_job_worker_limit")]
    pub town_job_limit: u8,
    pub population_cap: u16,
    pub max_population_cap: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayCycleConfigData {
    pub guild_job_fatigue: u32,
    pub expedition_fatigue: u32,
    pub guild_job_stress: u32,
    pub expedition_stress: u32,
    pub resting_fatigue_recovery: u32,
    pub resting_stress_recovery: u32,
    pub base_injury_recovery: u32,
    pub base_guild_job_success: i32,
    pub base_expedition_success: i32,
    pub preferred_trait_bonus_pct: i32,
    pub preferred_species_bonus_pct: i32,
    pub worker_charm_gold_multiplier: u32,
    pub worker_instinct_residue_multiplier: u32,
    pub expedition_power_materials_multiplier: u32,
    pub expedition_instinct_residue_multiplier: u32,
    pub expedition_endurance_safety_divisor: u32,
    pub expedition_reward_success_divisor: u32,
    pub girl_food_gold_per_day: u32,
    pub building_maintenance_cost_divisor: u32,
    #[serde(default)]
    pub upkeep_bands: Vec<UpkeepBandData>,
    pub expedition_egg_reward_threshold: i32,
    pub expedition_relic_reward_threshold: i32,
    pub expedition_injury_threshold: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpkeepBandData {
    pub min_girls: u32,
    #[serde(alias = "min_client_tiers")]
    pub min_patron_tiers: u32,
    pub food_multiplier_pct: u32,
    pub cleaning_multiplier_pct: u32,
    pub maintenance_multiplier_pct: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfigData {
    pub title: String,
    pub content_version: String,
    pub save_version: u32,
    pub input: InputConfigData,
    pub persistence: PersistenceConfigData,
    pub display: DisplayConfigData,
    pub ui: TownUiConfigData,
    pub new_game: NewGameConfigData,
    pub day_cycle: DayCycleConfigData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesCatalogData {
    pub version: String,
    pub species: Vec<SpeciesData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterNameCatalogData {
    pub version: String,
    pub name_pools: Vec<MonsterNamePoolData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterNamePoolData {
    pub id: String,
    pub species_ids: Vec<String>,
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub portrait_key: String,
    pub base_stats: StatBlockData,
    pub starting_traits: Vec<String>,
    pub preferred_room_ids: Vec<String>,
    pub hatching_cost: ResourceAmountData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingCatalogData {
    pub version: String,
    pub buildings: Vec<BuildingData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatronTierCatalogData {
    pub version: String,
    #[serde(alias = "client_tiers")]
    pub patron_tiers: Vec<PatronTierData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatronTierData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub income_multiplier_pct: u32,
    pub residue_multiplier_pct: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingUnlocksData {
    pub room_ids: Vec<String>,
    pub floor_ids: Vec<String>,
    pub species_ids: Vec<String>,
    #[serde(alias = "client_tiers")]
    pub patron_tiers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuildingModifierData {
    pub guild_income_pct: i32,
    pub expedition_success_pct: i32,
    #[serde(default)]
    pub egg_discovery_flat: i32,
    pub injury_recovery_flat: i32,
    pub stress_recovery_flat: i32,
    pub charm_training_flat: i32,
    pub population_cap_flat: i32,
    pub town_job_limit_flat: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingData {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub build_limit: u8,
    pub cost: ResourceAmountData,
    pub unlocks: BuildingUnlocksData,
    pub passive_modifiers: BuildingModifierData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerFloorCatalogData {
    pub version: String,
    pub floors: Vec<TowerFloorData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionCatalogData {
    pub version: String,
    pub missions: Vec<MissionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub reward_focus: String,
    #[serde(default)]
    pub prep_cost: ResourceAmountData,
    pub success_bonus_pct: i32,
    pub materials_multiplier_pct: u32,
    pub residue_multiplier_pct: u32,
    pub egg_bonus_flat: u32,
    pub relic_bonus_flat: u32,
    pub injury_risk_pct: i32,
    #[serde(default)]
    pub preferred_role: Option<String>,
    #[serde(default)]
    pub egg_grade_bonus: u32,
    #[serde(default)]
    pub hazard_risk_modifier_pct: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationCatalogData {
    pub version: String,
    pub mutations: Vec<MutationData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationData {
    pub id: String,
    pub source_species_id: String,
    pub target_species_id: String,
    pub minimum_corruption: u32,
    pub required_trait_ids: Vec<String>,
    pub granted_trait_ids: Vec<String>,
    pub event_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryEventCatalogData {
    pub version: String,
    pub opening_steps: Vec<OpeningStoryStepData>,
    pub camp_discovery_log: String,
    pub discovery_incubation_log: String,
    pub hatch_loyalty_log: String,
    pub build_first_room_not_enough_resources_error: String,
    pub build_first_room_completion_log: String,
    pub first_room_cost: ResourceAmountData,
    pub first_client_reward: ResourceAmountData,
    pub first_client_skill_gains: CompanionSkillProgressionData,
    #[serde(alias = "first_client_history_gains")]
    pub first_client_work_history_gains: CompanionWorkHistoryProgressionData,
    pub first_client_missing_monster_error: String,
    pub first_client_completion_log_template: String,
    pub hatch_missing_monster_error: String,
    pub first_hatched_monster_name: String,
    pub first_hatch_log: String,
    pub debt_status_due_template: String,
    pub debt_initialize_log_template: String,
    pub debt_intro_with_debt_template: String,
    pub debt_intro_without_debt: String,
    pub debt_paid_update_template: String,
    pub debt_paid_event_template: String,
    pub debt_next_due_update_template: String,
    pub debt_next_due_event_template: String,
    pub debt_cleared_event: String,
    pub debt_missed_status_template: String,
    pub debt_missed_update_template: String,
    pub debt_missed_event_template: String,
    pub debt_missed_stress_template: String,
    pub guest_name_template: String,
    pub guest_missing_assigned_girl_event_template: String,
    pub guest_failed_event_template: String,
    pub guest_satisfied_event_template: String,
    pub guest_completed_update_template: String,
    pub guest_completed_roster_template: String,
    pub guest_expired_update_template: String,
    pub guest_expired_event_template: String,
    pub guest_requires_template: String,
    pub guest_already_on_expedition_reason: String,
    pub guest_requirement_detail_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtMilestoneCatalogData {
    pub version: String,
    pub first_milestone_id: String,
    pub milestones: Vec<DebtMilestoneData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtMilestoneData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub amount_due: u32,
    pub days_allowed: u32,
    pub reward: ResourceAmountData,
    pub failure_penalty_gold: u32,
    pub failure_stress_flat: u32,
    pub next_milestone_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatronArchetypeCatalogData {
    pub version: String,
    pub archetypes: Vec<PatronArchetypeData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatronArchetypeData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub spawn_weight: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractCatalogData {
    pub version: String,
    pub requests: Vec<ContractData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractData {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub patron_tier_id: Option<String>,
    pub archetype_id: String,
    pub requested_room_id: String,
    pub required_species_ids: Vec<String>,
    #[serde(default = "default_quality_rank")]
    pub minimum_quality_rank: u8,
    pub required_skill_thresholds: CompanionSkillProgressionData,
    #[serde(alias = "required_history_thresholds")]
    pub required_work_history_thresholds: CompanionWorkHistoryProgressionData,
    pub reward: ResourceAmountData,
    pub penalty_gold: u32,
    pub deadline_days: u32,
    #[serde(default)]
    pub preparation_quality_required: u32,
    #[serde(default)]
    pub preparation_quality_bonus: u32,
    pub is_special: bool,
    #[serde(default)]
    pub preferred_trait_ids: Vec<String>,
    #[serde(default)]
    pub preferred_role: Option<String>,
    #[serde(default)]
    pub follow_up_request_id: Option<String>,
    #[serde(default)]
    pub partial_success_score: u32,
    #[serde(default)]
    pub reputation_reward: i32,
}

fn default_quality_rank() -> u8 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpeningStoryStepData {
    pub id: String,
    pub title: String,
    pub body_lines: Vec<String>,
    pub primary_action_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerFloorData {
    pub id: String,
    pub name: String,
    pub depth: u32,
    pub description: String,
    pub difficulty: u32,
    pub requires_building_ids: Vec<String>,
    #[serde(default)]
    pub required_roster: Vec<FloorRosterRequirementData>,
    pub mission_ids: Vec<String>,
    pub baseline_rewards: ResourceAmountData,
    pub egg_species_entries: Vec<EggSpeciesEntryData>,
    pub relic_drop_ids: Vec<String>,
    #[serde(default)]
    pub hazard_tags: Vec<String>,
    #[serde(default)]
    pub egg_grade_bonus: u32,
    #[serde(default)]
    pub corruption_pressure: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FloorRosterRequirementData {
    pub species_id: String,
    #[serde(default = "default_quality_rank")]
    pub minimum_quality_rank: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EggSpeciesEntryData {
    pub species_id: String,
    pub weight: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitCatalogData {
    pub version: String,
    pub traits: Vec<TraitData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitData {
    pub id: String,
    pub name: String,
    pub category: String,
    pub icon_key: String,
    pub description: String,
    pub stat_modifiers: StatBlockData,
    pub guild_income_pct: i32,
    pub expedition_success_pct: i32,
    pub injury_risk_pct: i32,
    pub stress_change_flat: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildRoomCatalogData {
    pub version: String,
    pub rooms: Vec<GuildRoomData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildRoomData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub service_summary: String,
    pub required_building_ids: Vec<String>,
    pub service_tier: u8,
    pub base_gold_yield: u32,
    pub base_residue_yield: u32,
    #[serde(default)]
    pub base_materials_yield: u32,
    #[serde(default)]
    pub reputation_yield: i32,
    pub stamina_cost: u32,
    #[serde(alias = "client_tiers")]
    pub patron_tiers: Vec<String>,
    pub trained_skill_ids: Vec<String>,
    #[serde(alias = "history_gains")]
    pub work_history_gains: CompanionWorkHistoryProgressionData,
    pub preferred_trait_ids: Vec<String>,
    pub preferred_species_ids: Vec<String>,
    #[serde(default)]
    pub strategic_niche: Option<String>,
    #[serde(default)]
    pub upgrade_building_ids: Vec<String>,
    #[serde(default)]
    pub fatigue_modifier: i32,
    #[serde(default)]
    pub stress_modifier: i32,
    #[serde(default)]
    pub corruption_pressure: u32,
    #[serde(default)]
    pub guest_appeal: u32,
    #[serde(default)]
    pub job_kind: String,
    #[serde(default)]
    pub preparation_quality_bonus: u32,
    #[serde(default)]
    pub recovery_bonus: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCatalogData {
    pub version: String,
    pub events: Vec<EventData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    pub id: String,
    pub category: String,
    pub phase: String,
    pub text: String,
    pub required_trait_ids: Vec<String>,
    pub required_species_ids: Vec<String>,
    #[serde(default)]
    pub required_building_ids: Vec<String>,
    #[serde(default)]
    pub event_tags: Vec<String>,
    pub weight: Option<u32>,
    pub trigger_chance_pct: Option<i32>,
    pub min_day: Option<u32>,
    pub reward: Option<ResourceAmountData>,
    pub cost: Option<ResourceAmountData>,
    #[serde(default)]
    pub situation_days: u32,
    #[serde(default)]
    pub situation_label: Option<String>,
    #[serde(default)]
    pub situation_upkeep_pressure_pct: u32,
    #[serde(default)]
    pub situation_guest_bonus: u32,
}

#[derive(Debug, Clone)]
pub struct GameData {
    pub config: GameConfigData,
    pub ui_text: UiTextData,
    pub debt_milestones: DebtMilestoneCatalogData,
    pub patron_archetypes: PatronArchetypeCatalogData,
    pub contracts: ContractCatalogData,
    pub patron_tiers: PatronTierCatalogData,
    pub missions: MissionCatalogData,
    pub mutations: MutationCatalogData,
    pub story_events: StoryEventCatalogData,
    pub monster_names: MonsterNameCatalogData,
    pub species: SpeciesCatalogData,
    pub buildings: BuildingCatalogData,
    pub floors: TowerFloorCatalogData,
    pub traits: TraitCatalogData,
    pub guild_rooms: GuildRoomCatalogData,
    pub events: EventCatalogData,
}
