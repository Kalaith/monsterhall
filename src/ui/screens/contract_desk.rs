use crate::data::GameData;
use crate::state::{ContractDeskState, GameState};
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_backdrop, BackdropKind};

use super::contract_desk_sections::{
    draw_eligible_panel, draw_footer_actions, draw_header, draw_no_requests_state,
    draw_requests_panel, draw_selected_request_panel, selected_request, ContractDeskLayout,
};

pub fn draw_contract_desk(
    data: &GameData,
    guest_state: &ContractDeskState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    draw_backdrop(BackdropKind::GuestDesk);

    let layout = ContractDeskLayout::new();

    if let Some(action) = draw_header(data) {
        return Some(action);
    }

    let (requests, selected_request) = selected_request(guest_state, game_state);

    if requests.is_empty() {
        draw_no_requests_state(data, game_state, &layout);
        if let Some(action) = draw_footer_actions(data, &layout) {
            return Some(action);
        }
        return None;
    }

    if let Some(action) = draw_requests_panel(data, guest_state, &requests, &layout) {
        return Some(action);
    }
    if let Some(action) = draw_selected_request_panel(
        data,
        game_state,
        &requests,
        selected_request,
        last_error,
        &layout,
    ) {
        return Some(action);
    }

    if let Some(action) = draw_eligible_panel(data, game_state, selected_request, &layout) {
        return Some(action);
    }
    if let Some(action) = draw_footer_actions(data, &layout) {
        return Some(action);
    }

    None
}
