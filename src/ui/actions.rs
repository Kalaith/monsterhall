//! UI intent definitions returned by screen renderers.

use crate::state::{EggConversionKind, ExpeditionPriority};

#[derive(Debug, Clone)]
pub enum UiAction {
    StartNewGame,
    ContinueGame,
    ContinueOpening,
    BuildOpeningRoom,
    ResolveOpeningClient,
    QuitGame,
    /// Leave a running campaign for the main menu. Saves on the way out.
    ReturnToMainMenu,
    SaveGame,
    OpenGuildHallManagement,
    OpenContractDesk,
    OpenTownManagement,
    OpenHatcheryManagement,
    OpenJournal,
    OpenExpeditionPlanning,
    OpenMonsterProfile(String),
    SelectContractRequest(String),
    AssignMonsterToGuest(String, String),
    ClearGuestAssignment(String),
    SelectChamberEgg(String),
    ReturnToTownOverview,
    SelectGuildRoom(String),
    SelectTownBuilding(String),
    SelectTownBuildingGroup(String),
    SelectExpeditionFloor(String),
    SelectExpeditionMission(String),
    SetExpeditionPriority(ExpeditionPriority),
    /// Page the roster card grid on whichever screen is showing one. Both the
    /// Expedition Desk and the Contract Desk carry their own page, so the active
    /// phase decides which one this moves.
    ShowRosterPage(usize),
    OpenSettings,
    CloseSettings,
    ToggleFullscreen(bool),
    SetResolution(String),
    AssignMonsterToRoom(String, String),
    AssignMonsterToExpedition(String, String),
    AssignMonsterToRest(String),
    AssignMonsterToIdle(String),
    ReleaseMonster(String),
    PurchaseBuilding(String),
    PayDebtNow,
    HatchSelectedEgg(String, Option<String>),
    ReplaceMonsterWithEgg(String, Option<String>, String),
    ConvertEgg(String, EggConversionKind),
    ContinueAfterHatch,
    ResolveDay,
    ContinueAfterResults,
}
