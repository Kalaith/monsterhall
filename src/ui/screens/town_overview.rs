use crate::data::GameData;
use crate::state::{GameState, TownOverviewState};
use crate::ui::actions::UiAction;
use crate::ui::art::draw_town_overview_backdrop;

use super::town_overview_sections::{
    draw_error_panel, draw_footer_actions, draw_header, draw_monster_roster, draw_priority_panel,
    draw_summary_strip, TownOverviewLayout,
};

pub fn draw_town_overview(
    data: &GameData,
    town_state: &TownOverviewState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    draw_town_overview_backdrop();

    let layout = TownOverviewLayout::new(game_state);

    if let Some(action) = draw_header(data, &layout) {
        return Some(action);
    }

    if let Some(action) = draw_priority_panel(data, town_state, game_state, &layout) {
        return Some(action);
    }
    if let Some(action) = draw_summary_strip(data, game_state, &layout) {
        return Some(action);
    }

    if let Some(action) = draw_monster_roster(data, game_state, &layout) {
        return Some(action);
    }

    if let Some(action) = draw_footer_actions(data, game_state, &layout) {
        return Some(action);
    }

    draw_error_panel(&layout, last_error);

    None
}
