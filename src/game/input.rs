use macroquad::time::get_time;

use super::*;

impl Game {
    pub(super) fn handle_keyboard_shortcuts(&mut self) {
        let Some(data) = self.data.as_ref() else {
            return;
        };

        if is_key_pressed(KeyCode::Escape) {
            self.is_settings_open = !self.is_settings_open;
            self.settings_status = None;
            return;
        }

        if !data.config.input.keyboard_shortcuts_enabled || self.is_settings_open {
            return;
        }

        let shortcut_action = match &self.phase {
            GamePhase::Loading(_) => None,
            GamePhase::MainMenu(menu_state) => {
                if is_key_pressed(KeyCode::N) || is_key_pressed(KeyCode::Enter) {
                    Some(UiAction::StartNewGame)
                } else if menu_state.has_save_file
                    && (is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::Space))
                {
                    Some(UiAction::ContinueGame)
                } else {
                    None
                }
            }
            GamePhase::OpeningChapter(_) => {
                if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
                    Some(UiAction::SaveGame)
                } else {
                    None
                }
            }
            GamePhase::TownOverview(_) => {
                if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
                    Some(UiAction::SaveGame)
                } else if is_key_pressed(KeyCode::Enter) {
                    Some(UiAction::ResolveDay)
                } else {
                    None
                }
            }
            GamePhase::MonsterProfile(_) => {
                if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
                    Some(UiAction::SaveGame)
                } else {
                    None
                }
            }
            GamePhase::TownManagement(_) => {
                if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
                    Some(UiAction::SaveGame)
                } else {
                    None
                }
            }
            GamePhase::ContractDesk(_) => {
                if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
                    Some(UiAction::SaveGame)
                } else {
                    None
                }
            }
            GamePhase::HatcheryManagement(_) => {
                if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
                    Some(UiAction::SaveGame)
                } else {
                    None
                }
            }
            GamePhase::Journal(_) => {
                if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
                    Some(UiAction::SaveGame)
                } else {
                    None
                }
            }
            GamePhase::GuildHallManagement(_) => {
                if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
                    Some(UiAction::SaveGame)
                } else if is_key_pressed(KeyCode::Enter) {
                    Some(UiAction::ResolveDay)
                } else {
                    None
                }
            }
            GamePhase::ExpeditionPlanning(_) => {
                if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
                    Some(UiAction::SaveGame)
                } else if is_key_pressed(KeyCode::Enter) {
                    Some(UiAction::ResolveDay)
                } else {
                    None
                }
            }
            GamePhase::HatchReveal(hatch_state) => {
                if hatch_state.is_complete(get_time())
                    && (is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space))
                {
                    Some(UiAction::ContinueAfterHatch)
                } else {
                    None
                }
            }
            GamePhase::DayResults(_) => {
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    Some(UiAction::ContinueAfterResults)
                } else {
                    None
                }
            }
        };

        if shortcut_action.is_some() {
            self.pending_action = shortcut_action;
        }
    }

    pub(super) fn update_phase(&mut self) {
        if let GamePhase::Loading(loading_state) = &self.phase {
            if loading_state.is_ready && self.data.is_some() {
                let has_save_file = save_exists(self.save_identifier());
                self.phase = GamePhase::MainMenu(MainMenuState::new(has_save_file));
            }
        }
    }

    pub(super) fn handle_mouse_wheel(&mut self) {
        let (_, wheel_y) = mouse_wheel();
        if wheel_y == 0.0 {
            return;
        }

        let Some(game_state) = self.game_state.as_ref() else {
            return;
        };

        match &mut self.phase {
            GamePhase::HatcheryManagement(state) => {
                let left_margin = 24.0;
                let content_width = screen_width() - left_margin * 2.0;
                let inventory_x = left_margin;
                let inventory_width = content_width;
                let inventory_y = 24.0 + 206.0;
                let inventory_height = 436.0;
                let (mouse_x, mouse_y) = mouse_position();

                if mouse_x < inventory_x
                    || mouse_x > inventory_x + inventory_width
                    || mouse_y < inventory_y
                    || mouse_y > inventory_y + inventory_height
                {
                    return;
                }

                let visible_rows = 4usize;
                let max_scroll = game_state.egg_inventory.len().saturating_sub(visible_rows);
                if max_scroll == 0 {
                    state.inventory_scroll = 0;
                    return;
                }

                if wheel_y < 0.0 {
                    state.inventory_scroll = (state.inventory_scroll + 1).min(max_scroll);
                } else {
                    state.inventory_scroll = state.inventory_scroll.saturating_sub(1);
                }
            }
            GamePhase::Journal(state) => {
                let left_margin = 24.0;
                let content_width = screen_width() - left_margin * 2.0;
                let log_x = left_margin;
                let log_y = 320.0;
                let log_height = 720.0;
                let (mouse_x, mouse_y) = mouse_position();

                if mouse_x < log_x
                    || mouse_x > log_x + content_width
                    || mouse_y < log_y
                    || mouse_y > log_y + log_height
                {
                    return;
                }

                let visible_rows = 12usize;
                let max_scroll = game_state.event_log.len().saturating_sub(visible_rows);
                if max_scroll == 0 {
                    state.event_log_scroll = 0;
                    return;
                }

                if wheel_y < 0.0 {
                    state.event_log_scroll = (state.event_log_scroll + 1).min(max_scroll);
                } else {
                    state.event_log_scroll = state.event_log_scroll.saturating_sub(1);
                }
            }
            _ => {}
        }
    }
}
