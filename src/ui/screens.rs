//! Immediate-mode UI screens. All actions are mouse-clickable.

use crate::data::GameData;
use crate::state::{
    AppSettings, ContractDeskState, ExpeditionPlanningState, GameState, GuildHallManagementState,
    HatchRevealState, HatcheryManagementState, JournalState, MonsterProfileState,
    OpeningChapterState, TownManagementState, TownOverviewState,
};
use crate::ui::actions::UiAction;

mod contract_desk;
mod contract_desk_sections;
mod day_results;
mod expedition_planning;
mod guild_hall_management;
mod guild_hall_management_sections;
mod hatch_reveal;
mod hatchery_management;
mod hatchery_management_sections;
mod journal;
mod loading;
mod main_menu;
mod monster_profile;
mod opening;
mod settings;
mod town_management;
mod town_overview;
mod town_overview_footer;
mod town_overview_sections;

pub use day_results::draw_day_results;
pub use loading::draw_loading_screen;
pub use main_menu::draw_main_menu;

pub fn draw_opening_chapter(
    data: &GameData,
    opening_state: &OpeningChapterState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    opening::draw_opening_chapter(data, opening_state, game_state, last_error)
}

pub fn draw_town_overview(
    data: &GameData,
    town_state: &TownOverviewState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    town_overview::draw_town_overview(data, town_state, game_state, last_error)
}

pub fn draw_monster_profile(
    data: &GameData,
    profile_state: &MonsterProfileState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    monster_profile::draw_monster_profile(data, profile_state, game_state, last_error)
}

pub fn draw_town_management(
    data: &GameData,
    town_state: &TownManagementState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    town_management::draw_town_management(data, town_state, game_state, last_error)
}

pub fn draw_expedition_planning(
    data: &GameData,
    expedition_state: &ExpeditionPlanningState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    expedition_planning::draw_expedition_planning(data, expedition_state, game_state, last_error)
}

pub fn draw_guild_hall_management(
    data: &GameData,
    guild_jobs_state: &GuildHallManagementState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    guild_hall_management::draw_guild_hall_management(
        data,
        guild_jobs_state,
        game_state,
        last_error,
    )
}

pub fn draw_contract_desk(
    data: &GameData,
    guest_state: &ContractDeskState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    contract_desk::draw_contract_desk(data, guest_state, game_state, last_error)
}

pub fn draw_hatchery_management(
    data: &GameData,
    chamber_state: &HatcheryManagementState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    hatchery_management::draw_hatchery_management(data, chamber_state, game_state, last_error)
}

pub fn draw_hatch_reveal(
    data: &GameData,
    hatch_state: &HatchRevealState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    hatch_reveal::draw_hatch_reveal(data, hatch_state, game_state, last_error)
}

pub fn draw_journal(
    data: &GameData,
    journal_state: &JournalState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    journal::draw_journal(data, journal_state, game_state, last_error)
}

pub fn draw_settings_modal(
    data: &GameData,
    app_settings: &AppSettings,
    can_quit: bool,
    status_message: Option<&str>,
    status_is_error: bool,
) -> Option<UiAction> {
    settings::draw_settings_modal(
        data,
        app_settings,
        can_quit,
        status_message,
        status_is_error,
    )
}
