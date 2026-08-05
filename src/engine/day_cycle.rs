//! Stateless day-cycle logic for assignment, construction, and daily resolution.

use std::collections::HashSet;

use super::{
    apply_monster_relationship_gain, companion::effective_stats, complete_town_project_if_needed,
    debt::resolve_debt_cycle, expedition_depth_profile, floor_roster_gate_report,
    guest::resolve_contracts, refresh_contracts, room_depth_profile_for_town,
    start_town_situation_from_event, tick_town_situations, upkeep_pressure_pct,
};
use crate::data::{
    BuildingData, DayCycleConfigData, EggSpeciesEntryData, GameData, ResourceAmountData,
};
use crate::state::{
    CompanionJobState, CompanionSkillState, CompanionState, CompanionWorkHistoryState,
    DayResolutionSummary, EggConversionKind, EggIncubationState, EggState, ExpeditionPriority,
    ExpeditionState, GameState, PlayerTownState,
};

mod actions;
mod condition;
mod eggs;
mod events;
mod expedition_outcomes;
mod helpers;
mod modifiers;
mod previews;
mod progression;
pub(crate) mod random;
mod relics;
mod resolution;
mod surveys;
mod types;
mod upkeep;

#[cfg(test)]
mod tests;

use condition::*;
use eggs::*;
use events::*;
use expedition_outcomes::*;
use helpers::*;
use modifiers::*;
use previews::*;
use progression::*;
use random::gen_range;
use relics::*;
use resolution::*;
use surveys::*;
use upkeep::*;

pub use actions::is_booked_for_contract;
pub use actions::{
    assign_monster_to_expedition, assign_monster_to_idle, assign_monster_to_rest,
    assign_monster_to_room, configure_expedition_plan, convert_egg, hatch_selected_egg,
    hatch_species, missing_building_prerequisite_names, purchase_building, release_monster,
    replace_monster_with_selected_egg,
};
pub(crate) use condition::{companion_effectiveness_pct, scale_by_effectiveness};
pub use eggs::{create_opening_egg, sync_egg_resource_count};
#[cfg(test)]
pub use eggs::{raw_egg_count_for_species, ready_egg_count_for_species};
pub use helpers::{egg_quality_rank, max_quality_rank};
pub(crate) use modifiers::charm_training_bonus;
pub use previews::{
    effective_population_cap, preview_expedition_plan, preview_guild_job, preview_upkeep,
};
pub(crate) use progression::apply_guild_job_progression;
pub use progression::{charm_training_chance_pct, format_skill_name};
pub use resolution::resolve_day;
pub use types::{ExpeditionPlanPreview, GuildJobPreview, UpkeepForecast};
