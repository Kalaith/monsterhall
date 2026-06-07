//! Runtime state types used by the game loop.

use serde::{Deserialize, Serialize};

use crate::data::StatBlockData;

#[derive(Debug, Clone)]
pub enum GamePhase {
    Loading(LoadingState),
    MainMenu(MainMenuState),
    OpeningChapter(OpeningChapterState),
    TownOverview(TownOverviewState),
    MonsterProfile(MonsterProfileState),
    TownManagement(TownManagementState),
    ContractDesk(ContractDeskState),
    HatcheryManagement(HatcheryManagementState),
    Journal(JournalState),
    GuildHallManagement(GuildHallManagementState),
    ExpeditionPlanning(ExpeditionPlanningState),
    HatchReveal(HatchRevealState),
    DayResults(DayResultsState),
}

#[derive(Debug, Clone)]
pub struct LoadingState {
    pub status_message: String,
    pub is_ready: bool,
    pub error_message: Option<String>,
}

impl LoadingState {
    pub fn new(status_message: &str) -> Self {
        Self {
            status_message: status_message.to_owned(),
            is_ready: false,
            error_message: None,
        }
    }

    pub fn mark_ready(&mut self) {
        self.is_ready = true;
    }

    pub fn set_error(&mut self, error_message: String) {
        self.error_message = Some(error_message);
    }
}

#[derive(Debug, Clone)]
pub struct MainMenuState {
    pub has_save_file: bool,
}

impl MainMenuState {
    pub fn new(has_save_file: bool) -> Self {
        Self { has_save_file }
    }
}

#[derive(Debug, Clone)]
pub struct OpeningChapterState {
    pub step: OpeningChapterStep,
}

impl OpeningChapterState {
    pub fn new(step: OpeningChapterStep) -> Self {
        Self { step }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum OpeningChapterStep {
    #[default]
    Camp,
    Discovery,
    Incubation,
    Hatch,
    BuildRoom,
    FirstClient,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StoryProgressState {
    pub opening_step: OpeningChapterStep,
    pub tower_hole_discovered: bool,
    pub first_egg_created: bool,
    pub first_slimegirl_hatched: bool,
    pub hatched_species_ids: Vec<String>,
    pub first_room_built: bool,
    pub first_client_completed: bool,
    pub first_creditor_visit_seen: bool,
    pub first_special_guest_seen: bool,
}

#[derive(Debug, Clone)]
pub struct TownOverviewState {
    pub status_message: String,
}

impl TownOverviewState {
    pub fn new(status_message: &str) -> Self {
        Self {
            status_message: status_message.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonsterProfileState {
    pub selected_monster_id: String,
    pub status_message: String,
}

impl MonsterProfileState {
    pub fn new(selected_monster_id: String, status_message: &str) -> Self {
        Self {
            selected_monster_id,
            status_message: status_message.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HatcheryManagementState {
    pub selected_egg_id: Option<String>,
    pub status_message: String,
    pub inventory_scroll: usize,
}

impl HatcheryManagementState {
    pub fn with_scroll(
        selected_egg_id: Option<String>,
        status_message: &str,
        inventory_scroll: usize,
    ) -> Self {
        Self {
            selected_egg_id,
            status_message: status_message.to_owned(),
            inventory_scroll,
        }
    }
}

#[derive(Debug, Clone)]
pub enum HatchRevealReturn {
    HatcheryManagement { inventory_scroll: usize },
    OpeningChapter { next_step: OpeningChapterStep },
}

#[derive(Debug, Clone)]
pub struct HatchRevealState {
    pub egg: EggState,
    pub monster_id: String,
    pub started_at_seconds: f64,
    pub return_to: HatchRevealReturn,
}

impl HatchRevealState {
    pub fn new(
        egg: EggState,
        monster_id: String,
        started_at_seconds: f64,
        return_to: HatchRevealReturn,
    ) -> Self {
        Self {
            egg,
            monster_id,
            started_at_seconds,
            return_to,
        }
    }

    pub fn elapsed_seconds(&self, now_seconds: f64) -> f32 {
        (now_seconds - self.started_at_seconds).max(0.0) as f32
    }

    pub fn is_complete(&self, now_seconds: f64) -> bool {
        self.elapsed_seconds(now_seconds) >= 2.2
    }
}

#[derive(Debug, Clone)]
pub struct JournalState {
    pub event_log_scroll: usize,
}

impl JournalState {
    pub fn new(event_log_scroll: usize) -> Self {
        Self { event_log_scroll }
    }
}

#[derive(Debug, Clone)]
pub struct GuildHallManagementState {
    pub selected_room_id: String,
    pub status_message: String,
}

impl GuildHallManagementState {
    pub fn new(selected_room_id: String, status_message: &str) -> Self {
        Self {
            selected_room_id,
            status_message: status_message.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TownManagementState {
    pub selected_building_id: String,
    pub selected_group_id: String,
    pub status_message: String,
}

impl TownManagementState {
    pub fn with_group(
        selected_building_id: String,
        selected_group_id: String,
        status_message: &str,
    ) -> Self {
        Self {
            selected_building_id,
            selected_group_id,
            status_message: status_message.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContractDeskState {
    pub selected_request_id: Option<String>,
    pub status_message: String,
}

impl ContractDeskState {
    pub fn new(selected_request_id: Option<String>, status_message: &str) -> Self {
        Self {
            selected_request_id,
            status_message: status_message.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExpeditionPlanningState {
    pub selected_floor_id: String,
    pub selected_mission_id: String,
    pub priority: ExpeditionPriority,
    pub status_message: String,
}

impl ExpeditionPlanningState {
    pub fn new(
        selected_floor_id: String,
        selected_mission_id: String,
        priority: ExpeditionPriority,
        status_message: &str,
    ) -> Self {
        Self {
            selected_floor_id,
            selected_mission_id,
            priority,
            status_message: status_message.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DayResultsState {
    pub summary: DayResolutionSummary,
}

impl DayResultsState {
    pub fn new(summary: DayResolutionSummary) -> Self {
        Self { summary }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AppSettings {
    pub fullscreen: bool,
    pub resolution_id: String,
    pub resolution_width: u32,
    pub resolution_height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ResourcesState {
    pub gold: u32,
    pub tower_materials: u32,
    pub eggs: u32,
    pub relics: u32,
    pub arcane_residue: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PlayerTownState {
    pub constructed_building_ids: Vec<String>,
    pub unlocked_room_ids: Vec<String>,
    pub unlocked_floor_ids: Vec<String>,
    pub unlocked_species_ids: Vec<String>,
    #[serde(alias = "client_tiers")]
    pub patron_tiers: Vec<String>,
    pub completed_project_ids: Vec<String>,
    pub active_situations: Vec<TownSituationState>,
    pub party_size: u8,
    #[serde(alias = "guild_job_worker_limit")]
    pub town_job_limit: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TownSituationState {
    pub event_id: String,
    pub label: String,
    pub days_remaining: u32,
    pub upkeep_pressure_pct: u32,
    pub guest_pressure_bonus: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EggIncubationState {
    #[default]
    Raw,
    ReadyToHatch,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EggConversionKind {
    Sell,
    Dissolve,
    Refine,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EggState {
    pub id: String,
    pub source_floor_id: String,
    pub possible_species_ids: Vec<String>,
    pub selected_species_id: Option<String>,
    pub incubation_state: EggIncubationState,
    pub grade_score: u32,
    pub preparation_focus: Option<String>,
    pub loyalty_imprinted: bool,
    pub secrecy_locked: bool,
}

fn default_quality_rank() -> u8 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChamberState {
    pub exposure_risk: u32,
    pub is_secret_intact: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum DebtResolution {
    PaidOnTime,
    PaidLate,
    #[default]
    Missed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DebtState {
    pub active_milestone_id: String,
    pub current_balance_due: u32,
    pub days_until_due: u32,
    pub missed_payment_count: u32,
    pub resolved_milestone_ids: Vec<String>,
    pub last_resolution: Option<DebtResolution>,
    pub status_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ContractSkillRequirementState {
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
pub struct ContractHistoryRequirementState {
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ContractStatus {
    #[default]
    Pending,
    Accepted,
    Completed,
    Failed,
    Declined,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ContractState {
    pub request_id: String,
    pub template_id: String,
    pub guest_name: String,
    pub category: String,
    pub patron_tier_id: Option<String>,
    pub archetype_id: String,
    pub requested_room_id: String,
    pub required_species_ids: Vec<String>,
    #[serde(default = "default_quality_rank")]
    pub minimum_quality_rank: u8,
    pub required_skill_thresholds: ContractSkillRequirementState,
    pub required_work_history_thresholds: ContractHistoryRequirementState,
    pub reward: ResourcesState,
    pub penalty_gold: u32,
    pub deadline_day: u32,
    pub preparation_quality_required: u32,
    pub preparation_quality_bonus: u32,
    pub status: ContractStatus,
    pub assigned_monster_id: Option<String>,
    pub chain_depth: u32,
    pub partial_progress: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum CompanionJobState {
    #[default]
    Idle,
    #[serde(alias = "GuildJob")]
    GuildJob {
        room_id: String,
    },
    Resting,
    OnExpedition {
        expedition_id: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CompanionSkillState {
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
pub struct CompanionWorkHistoryState {
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CompanionState {
    pub id: String,
    pub species_id: String,
    pub name: String,
    #[serde(default = "default_quality_rank")]
    pub quality_rank: u8,
    pub stats: StatBlockData,
    pub trait_ids: Vec<String>,
    pub current_job: CompanionJobState,
    #[serde(alias = "companion_skills")]
    pub skills: CompanionSkillState,
    #[serde(alias = "work_history")]
    pub work_history: CompanionWorkHistoryState,
    pub fatigue: u32,
    pub stress: u32,
    pub injury: u32,
    pub corruption: u32,
    pub bond: u32,
    pub reputation: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ExpeditionPriority {
    #[default]
    Balanced,
    Aggressive,
    Safe,
    RecoveryFocused,
    #[serde(alias = "Curiosity")]
    Curiosity,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ExpeditionState {
    pub expedition_id: String,
    pub floor_id: String,
    pub mission_id: String,
    pub priority: ExpeditionPriority,
    pub assigned_monster_ids: Vec<String>,
    pub started_day: u32,
}

#[derive(Debug, Clone)]
pub struct DayResolutionSummary {
    pub resolved_day: u32,
    pub guild_job_gold: u32,
    pub guild_job_arcane_residue: u32,
    pub expedition_prep_gold: u32,
    pub expedition_prep_materials: u32,
    pub expedition_prep_arcane_residue: u32,
    pub expedition_prep_shortfall: u32,
    pub expedition_materials: u32,
    pub expedition_arcane_residue: u32,
    pub expedition_eggs: u32,
    pub expedition_relics: u32,
    pub upkeep_food_gold: u32,
    pub upkeep_cleaning_gold: u32,
    pub upkeep_maintenance_gold: u32,
    pub upkeep_gold: u32,
    pub upkeep_shortfall: u32,
    pub special_event_gold_delta: i32,
    pub special_event_count: u32,
    pub contracts_generated: usize,
    pub contracts_rejected: usize,
    pub special_event_lines: Vec<String>,
    pub debt_updates: Vec<String>,
    pub contract_updates: Vec<String>,
    pub event_lines: Vec<String>,
    pub roster_updates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GameState {
    pub current_day: u32,
    pub resources: ResourcesState,
    pub town: PlayerTownState,
    pub egg_inventory: Vec<EggState>,
    pub chamber: ChamberState,
    pub debt: Option<DebtState>,
    #[serde(alias = "active_guest_requests")]
    pub active_contracts: Vec<ContractState>,
    pub monsters: Vec<CompanionState>,
    pub active_expedition: Option<ExpeditionState>,
    pub story_progress: StoryProgressState,
    pub event_log: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SaveData {
    pub version: u32,
    pub game_state: GameState,
}

impl SaveData {
    pub fn new(version: u32, game_state: GameState) -> Self {
        Self {
            version,
            game_state,
        }
    }
}
