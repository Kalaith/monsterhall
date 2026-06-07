//! Runtime state, explicit phases, and persistence.

mod game_state;
mod persistence;

pub use game_state::{
    AppSettings, GuildHallManagementState, HatcheryManagementState, ChamberState,
    DayResolutionSummary, DayResultsState, DebtResolution, DebtState, EggConversionKind,
    EggIncubationState, EggState, ExpeditionPlanningState, ExpeditionPriority, ExpeditionState,
    GamePhase, GameState, ContractHistoryRequirementState, ContractDeskState, ContractState,
    ContractStatus, ContractSkillRequirementState, HatchRevealReturn, HatchRevealState,
    JournalState, LoadingState, MainMenuState, CompanionState, CompanionJobState,
    MonsterProfileState, OpeningChapterState, OpeningChapterStep, PlayerTownState, ResourcesState,
    SaveData, CompanionWorkHistoryState, CompanionSkillState, StoryProgressState, TownManagementState,
    TownOverviewState, TownSituationState,
};
pub use persistence::{
    load_app_settings, load_compatible_save_data, peek_save_version, save_app_settings,
    save_exists, save_game, settings_exist,
};
