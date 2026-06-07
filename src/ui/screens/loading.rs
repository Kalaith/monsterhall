use crate::state::LoadingState;
use crate::ui::actions::UiAction;
use crate::ui::chrome::{draw_inline_status, draw_tier_panel, PanelTier};
use crate::ui::core::{draw_body_text_in_box, draw_heading_in_box};
use crate::ui::theme;

pub fn draw_loading_screen(
    loading_state: &LoadingState,
    panel_title: &str,
    game_title: &str,
    loading_message: &str,
) -> Option<UiAction> {
    let panel_x = macroquad::prelude::screen_width() * 0.5 - 220.0;
    let panel_y = macroquad::prelude::screen_height() * 0.5 - 90.0;
    let panel_width = 440.0;
    let panel_height = 180.0;

    draw_tier_panel(
        panel_x,
        panel_y,
        panel_width,
        panel_height,
        Some(panel_title),
        PanelTier::Primary,
        true,
    );
    draw_heading_in_box(
        game_title,
        panel_x + 20.0,
        panel_y + 34.0,
        panel_width - 40.0,
        36.0,
        32.0,
    );
    draw_body_text_in_box(
        &loading_state.status_message,
        panel_x + 20.0,
        panel_y + 82.0,
        panel_width - 40.0,
        34.0,
        24.0,
        theme::TEXT_BODY,
    );

    if let Some(error_message) = &loading_state.error_message {
        draw_inline_status(
            panel_x + 20.0,
            panel_y + 126.0,
            panel_width - 40.0,
            error_message,
            theme::DANGER,
        );
    } else {
        draw_body_text_in_box(
            loading_message,
            panel_x + 20.0,
            panel_y + 122.0,
            panel_width - 40.0,
            42.0,
            20.0,
            theme::TEXT_MUTED,
        );
    }

    None
}
