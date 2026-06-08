//! Runtime state, explicit phases, and persistence.

mod game_state;
mod persistence;

pub use game_state::{
    AppSettings, ChamberState, CompanionJobState, CompanionSkillState, CompanionState,
    CompanionWorkHistoryState, ContractDeskState, ContractHistoryRequirementState,
    ContractSkillRequirementState, ContractState, ContractStatus, DayResolutionSummary,
    DayResultsState, DebtResolution, DebtState, EggConversionKind, EggIncubationState, EggState,
    ExpeditionPlanningState, ExpeditionPriority, ExpeditionState, GamePhase, GameState,
    GuildHallManagementState, HatchRevealReturn, HatchRevealState, HatcheryManagementState,
    JournalState, LoadingState, MainMenuState, MonsterProfileState, OpeningChapterState,
    OpeningChapterStep, PlayerTownState, ResourcesState, SaveData, StoryProgressState,
    TownManagementState, TownOverviewState, TownSituationState,
};
pub use persistence::{
    load_app_settings, load_compatible_save_data, peek_save_version, save_app_settings,
    save_exists, save_game, settings_exist,
};
