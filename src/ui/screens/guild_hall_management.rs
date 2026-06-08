use crate::data::GameData;
use crate::state::{GameState, GuildHallManagementState};
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_backdrop, BackdropKind};

use super::guild_hall_management_sections::{
    draw_footer, draw_header, draw_rooms_panel, draw_selected_room_panel, draw_worker_lists,
    selected_room, GuildHallManagementLayout,
};

pub fn draw_guild_hall_management(
    data: &GameData,
    guild_jobs_state: &GuildHallManagementState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    draw_backdrop(BackdropKind::GuildJobs);

    let layout = GuildHallManagementLayout::new();

    if let Some(action) = draw_header(data, &layout) {
        return Some(action);
    }

    let (available_rooms, selected_room) = selected_room(data, guild_jobs_state, game_state);
    if let Some(action) = draw_rooms_panel(data, &available_rooms, selected_room, &layout) {
        return Some(action);
    }

    draw_selected_room_panel(data, guild_jobs_state, game_state, selected_room, &layout);

    if let Some(action) = draw_worker_lists(data, game_state, selected_room, &layout) {
        return Some(action);
    }
    if let Some(action) = draw_footer(data, &layout, last_error) {
        return Some(action);
    }

    None
}
