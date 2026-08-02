//! Static game content definitions and JSON loading.

mod config_types;
mod depth_validation;
mod loader;
mod types;
mod ui_text;
mod validation;
mod validation_helpers;

pub use config_types::{
    CompanionSkillProgressionData, CompanionWorkHistoryProgressionData, DayCycleConfigData,
    UpkeepBandData,
};
pub use loader::load_game_data;
#[cfg(test)]
pub(crate) use loader::test_game_data;
pub use types::{
    BuildingData, ContractData, EggSpeciesEntryData, EventData, GameData, GuildRoomData,
    MissionData, PatronTierData, ResourceAmountData, SpeciesData, StatBlockData, TowerFloorData,
};
pub use ui_text::UiTextData;
