use macroquad::prelude::{screen_height, screen_width};
use macroquad_toolkit::ui::full_screen_overlay;

use crate::data::GameData;
use crate::state::AppSettings;
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_backdrop, BackdropKind};
use crate::ui::chrome::{draw_inline_status, draw_modal_panel, draw_tier_panel, PanelTier};
use crate::ui::core::{
    draw_body_text, draw_heading, primary_button, secondary_button, utility_button,
};
use crate::ui::layout;
use crate::ui::theme;

pub fn draw_settings_modal(
    data: &GameData,
    app_settings: &AppSettings,
    can_quit: bool,
    status_message: Option<&str>,
    status_is_error: bool,
) -> Option<UiAction> {
    draw_backdrop(BackdropKind::Settings);
    full_screen_overlay(0.72);
    let common_text = &data.ui_text.common;
    let settings_text = &data.ui_text.settings;

    let panel_width = 560.0;
    let panel_height = if can_quit { 514.0 } else { 474.0 };
    let panel_x = screen_width() * 0.5 - panel_width * 0.5;
    let panel_y = screen_height() * 0.5 - panel_height * 0.5;

    draw_modal_panel(
        panel_x,
        panel_y,
        panel_width,
        panel_height,
        &settings_text.panel_title,
        None,
    );
    draw_body_text(
        &settings_text.display_heading,
        panel_x + layout::PANEL_PADDING,
        panel_y + 98.0,
        24.0,
        theme::TEXT_STRONG,
    );
    draw_body_text(
        "Mode",
        panel_x + layout::PANEL_PADDING,
        panel_y + 128.0,
        14.0,
        theme::TEXT_MUTED,
    );

    draw_tier_panel(
        panel_x + layout::PANEL_PADDING,
        panel_y + 138.0,
        panel_width - layout::PANEL_PADDING * 2.0,
        58.0,
        None,
        PanelTier::Support,
        false,
    );
    let mode_button_y = panel_y + 152.0;
    let mode_button_w = 180.0;
    let mode_button_gap = 14.0;
    let mode_x = panel_x + layout::PANEL_PADDING + 16.0;
    let fullscreen_pressed = if app_settings.fullscreen {
        primary_button(
            mode_x,
            mode_button_y,
            mode_button_w,
            30.0,
            &settings_text.fullscreen_button,
        )
    } else {
        secondary_button(
            mode_x,
            mode_button_y,
            mode_button_w,
            30.0,
            &settings_text.fullscreen_button,
        )
    };
    if fullscreen_pressed {
        return Some(UiAction::ToggleFullscreen(true));
    }
    let windowed_pressed = if app_settings.fullscreen {
        secondary_button(
            mode_x + mode_button_w + mode_button_gap,
            mode_button_y,
            mode_button_w,
            30.0,
            &settings_text.windowed_button,
        )
    } else {
        primary_button(
            mode_x + mode_button_w + mode_button_gap,
            mode_button_y,
            mode_button_w,
            30.0,
            &settings_text.windowed_button,
        )
    };
    if windowed_pressed {
        return Some(UiAction::ToggleFullscreen(false));
    }
    draw_heading(
        &settings_text.resolution_heading,
        panel_x + layout::PANEL_PADDING,
        panel_y + 222.0,
        24.0,
    );
    for (index, resolution) in data.config.display.available_resolutions.iter().enumerate() {
        let tile_x = panel_x + layout::PANEL_PADDING + (index % 2) as f32 * 242.0;
        let tile_y = panel_y + 246.0 + (index / 2) as f32 * 46.0;
        let is_selected = app_settings.resolution_id == resolution.id;
        let button_clicked = if is_selected {
            primary_button(tile_x, tile_y, 218.0, 34.0, &resolution.label)
        } else {
            secondary_button(tile_x, tile_y, 218.0, 34.0, &resolution.label)
        };
        if button_clicked {
            return Some(UiAction::SetResolution(resolution.id.clone()));
        }
    }

    let utility_y = panel_y + panel_height - if can_quit { 80.0 } else { 52.0 };
    if let Some(status_message) = status_message {
        draw_inline_status(
            panel_x + layout::PANEL_PADDING,
            utility_y - 30.0,
            panel_width - layout::PANEL_PADDING * 2.0,
            status_message,
            if status_is_error {
                theme::DANGER
            } else {
                theme::POSITIVE
            },
        );
    }

    if primary_button(
        panel_x + 32.0,
        utility_y,
        156.0,
        layout::UTILITY_BUTTON_H,
        &common_text.save_campaign_button,
    ) {
        return Some(UiAction::SaveGame);
    }

    if primary_button(
        panel_x + panel_width - 150.0,
        utility_y,
        124.0,
        layout::UTILITY_BUTTON_H,
        &common_text.close_button,
    ) {
        return Some(UiAction::CloseSettings);
    }

    if can_quit
        && utility_button(
            panel_x + layout::PANEL_PADDING,
            panel_y + panel_height - 42.0,
            panel_width - layout::PANEL_PADDING * 2.0,
            layout::UTILITY_BUTTON_H,
            &common_text.quit_game_button,
        )
    {
        return Some(UiAction::QuitGame);
    }

    None
}
