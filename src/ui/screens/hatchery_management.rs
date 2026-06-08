use crate::data::GameData;
use crate::state::{GameState, HatcheryManagementState};
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_backdrop, BackdropKind};

use super::hatchery_management_sections::{
    draw_footer, draw_header, draw_inventory_panel, draw_selected_egg_panel, draw_status_panel,
    HatcheryManagementLayout,
};

pub fn draw_hatchery_management(
    data: &GameData,
    chamber_state: &HatcheryManagementState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    draw_backdrop(BackdropKind::Chamber);

    let layout = HatcheryManagementLayout::new();

    if let Some(action) = draw_header(data) {
        return Some(action);
    }

    draw_status_panel(data, chamber_state, game_state, &layout);

    if let Some(action) = draw_inventory_panel(data, chamber_state, game_state, &layout) {
        return Some(action);
    }
    if let Some(action) = draw_selected_egg_panel(
        data,
        game_state,
        super::hatchery_management_sections::selected_egg(chamber_state, game_state),
        last_error,
        &layout,
    ) {
        return Some(action);
    }
    if let Some(action) = draw_footer(data, &layout) {
        return Some(action);
    }

    None
}
