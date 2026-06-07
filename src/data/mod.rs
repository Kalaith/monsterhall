//! Static game content definitions and JSON loading.

mod depth_validation;
mod loader;
mod types;
mod ui_text;
mod validation;
mod validation_helpers;

pub use loader::load_game_data;
pub use types::{
    GuildRoomData, BuildingData, PatronTierData, EggSpeciesEntryData, EventData, GameData,
    ContractData, MissionData, ResourceAmountData, CompanionWorkHistoryProgressionData,
    CompanionSkillProgressionData, SpeciesData, StatBlockData, TowerFloorData, UpkeepBandData,
};
pub use ui_text::UiTextData;
