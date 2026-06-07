//! Stateless day-cycle logic for assignment, construction, and daily resolution.

use std::collections::HashSet;

use macroquad::rand::gen_range;

use super::{
    apply_monster_relationship_gain, complete_town_project_if_needed, debt::resolve_debt_cycle,
    expedition_depth_profile, floor_roster_gate_report, guest::resolve_contracts,
    refresh_contracts, room_depth_profile_for_town, start_town_situation_from_event,
    tick_town_situations, upkeep_pressure_pct,
};
use crate::data::{BuildingData, EggSpeciesEntryData, GameData, ResourceAmountData};
use crate::state::{
    DayResolutionSummary, EggConversionKind, EggIncubationState, EggState, ExpeditionPriority,
    ExpeditionState, GameState, CompanionState, CompanionJobState, PlayerTownState,
    CompanionWorkHistoryState, CompanionSkillState,
};

mod actions;
mod eggs;
mod events;
mod helpers;
mod modifiers;
mod previews;
mod progression;
mod resolution;
mod types;

#[cfg(test)]
mod tests;

use eggs::*;
use events::*;
use helpers::*;
use modifiers::*;
use previews::*;
use progression::*;
use resolution::*;

pub use actions::{
    assign_monster_to_expedition, assign_monster_to_idle, assign_monster_to_rest,
    assign_monster_to_room, configure_expedition_plan, convert_egg, hatch_selected_egg,
    hatch_species, purchase_building, release_monster, replace_monster_with_selected_egg,
};
pub use eggs::{create_opening_egg, sync_egg_resource_count};
#[cfg(test)]
pub use eggs::{raw_egg_count_for_species, ready_egg_count_for_species};
pub use previews::{
    effective_population_cap, preview_guild_job, preview_expedition_plan, preview_upkeep,
};
pub(crate) use progression::apply_guild_job_progression;
pub use resolution::resolve_day;
pub use types::{GuildJobPreview, ExpeditionPlanPreview, UpkeepForecast};
