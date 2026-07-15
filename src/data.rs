//! Static game content definitions and JSON loading.

mod depth_validation;
mod loader;
mod types;
mod ui_text;
mod validation;
mod validation_helpers;

pub use loader::load_game_data;
pub use types::{
    BuildingData, CompanionSkillProgressionData, CompanionWorkHistoryProgressionData, ContractData,
    EggSpeciesEntryData, EventData, GameData, GuildRoomData, MissionData, PatronTierData,
    ResourceAmountData, SpeciesData, StatBlockData, TowerFloorData, UpkeepBandData,
};
pub use ui_text::UiTextData;
