use macroquad::prelude::{screen_height, screen_width};

use crate::data::UiTextData;
use crate::state::MainMenuState;
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_backdrop, BackdropKind};
use crate::ui::chrome::draw_top_utility_bar;
use crate::ui::core::{draw_body_text_in_box, primary_button, secondary_button, utility_button};
use crate::ui::theme;

pub fn draw_main_menu(
    _game_title: &str,
    ui_text: &UiTextData,
    main_menu_state: &MainMenuState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    draw_backdrop(BackdropKind::MainMenu);

    let main_menu_text = &ui_text.main_menu;
    let common_text = &ui_text.common;
    if let Some(action) = draw_top_utility_bar(&common_text.settings_button) {
        return Some(action);
    }

    let button_width = 360.0;
    let button_height = 50.0;
    let button_gap = 12.0;
    let button_x = screen_width() * 0.5 - button_width * 0.5;
    let first_button_y = screen_height() - 182.0;
    let continue_button_y = first_button_y + button_height + button_gap;
    let quit_button_y = continue_button_y + button_height + button_gap;
    if primary_button(
        button_x,
        first_button_y,
        button_width,
        button_height,
        &main_menu_text.new_campaign_button,
    ) {
        return Some(UiAction::StartNewGame);
    }

    if main_menu_state.has_save_file {
        if secondary_button(
            button_x,
            continue_button_y,
            button_width,
            button_height,
            &main_menu_text.continue_campaign_button,
        ) {
            return Some(UiAction::ContinueGame);
        }
        if utility_button(
            button_x,
            quit_button_y,
            button_width,
            button_height,
            &common_text.quit_game_button,
        ) {
            return Some(UiAction::QuitGame);
        }
    } else {
        draw_body_text_in_box(
            &main_menu_text.no_save_message,
            button_x,
            continue_button_y + 12.0,
            button_width,
            24.0,
            16.0,
            theme::TEXT_MUTED,
        );
        if utility_button(
            button_x,
            quit_button_y,
            button_width,
            button_height,
            &common_text.quit_game_button,
        ) {
            return Some(UiAction::QuitGame);
        }
    }

    if let Some(error_message) = last_error {
        draw_body_text_in_box(
            error_message,
            button_x,
            first_button_y - 36.0,
            button_width,
            24.0,
            16.0,
            theme::DANGER,
        );
    }

    None
}
