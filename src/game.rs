//! Top-level game coordinator and explicit phase transitions.

use macroquad::{
    prelude::{
        is_key_down, is_key_pressed, is_mouse_button_pressed, mouse_position, mouse_wheel,
        screen_width, KeyCode, MouseButton,
    },
    window::{request_new_screen_size, set_fullscreen},
};

use crate::data::{load_game_data, GameData};
use crate::engine::{
    advance_opening_step, assign_monster_to_expedition, assign_monster_to_contract,
    assign_monster_to_idle, assign_monster_to_rest, assign_monster_to_room, build_first_room,
    clear_contract_assignment, configure_expedition_plan, convert_egg, create_new_game_state,
    debt_intro_status, hatch_selected_egg, initialize_first_debt, pay_debt_now, purchase_building,
    refresh_contracts, release_monster, replace_monster_with_selected_egg, resolve_day,
    resolve_first_client, validate_game_state_references,
};
use crate::state::{
    load_app_settings, load_compatible_save_data, peek_save_version, save_app_settings,
    save_exists, save_game, settings_exist, AppSettings, GuildHallManagementState,
    HatcheryManagementState, DayResultsState, ExpeditionPlanningState, ExpeditionPriority,
    GamePhase, GameState, ContractDeskState, JournalState, LoadingState, MainMenuState,
    MonsterProfileState, OpeningChapterState, OpeningChapterStep, SaveData, TownManagementState,
    TownOverviewState,
};
use crate::ui::{
    draw_guild_hall_management, draw_hatchery_management, draw_day_results, draw_expedition_planning,
    draw_contract_desk, draw_hatch_reveal, draw_journal, draw_loading_screen, draw_main_menu,
    draw_monster_profile, draw_opening_chapter, draw_settings_modal, draw_town_management,
    draw_town_overview, UiAction,
};

mod actions;
mod input;
mod navigation;
mod settings;

use settings::{apply_display_settings, load_or_default_settings};

fn town_building_group_id(category: &str) -> &str {
    match category {
        "project" | "prestige" => "projects",
        _ => "core",
    }
}

pub struct Game {
    data: Option<GameData>,
    phase: GamePhase,
    game_state: Option<GameState>,
    app_settings: Option<AppSettings>,
    is_settings_open: bool,
    settings_status: Option<String>,
    pending_action: Option<UiAction>,
    last_error: Option<String>,
}

impl Game {
    pub async fn new() -> Self {
        let mut loading_state = LoadingState::new("Bootstrapping data catalogs");
        let data_result = load_game_data().await;

        let (data, app_settings, last_error) = match data_result {
            Ok(loaded_data) => {
                loading_state.mark_ready();
                let settings = load_or_default_settings(&loaded_data);
                apply_display_settings(&settings);
                (Some(loaded_data), Some(settings), None)
            }
            Err(message) => {
                loading_state.set_error(message.clone());
                (None, None, Some(message))
            }
        };

        Self {
            data,
            phase: GamePhase::Loading(loading_state),
            game_state: None,
            app_settings,
            is_settings_open: false,
            settings_status: None,
            pending_action: None,
            last_error,
        }
    }

    pub fn update(&mut self) {
        if self.last_error.is_some() && is_mouse_button_pressed(MouseButton::Left) {
            self.last_error = None;
        }

        self.handle_mouse_wheel();
        self.handle_keyboard_shortcuts();

        if let Some(action) = self.pending_action.take() {
            self.apply_action(action);
        }

        self.update_phase();
    }

    pub fn draw(&mut self) {
        let base_action = match &self.phase {
            GamePhase::Loading(loading_state) => draw_loading_screen(
                loading_state,
                "Boot",
                "Monsterhall",
                "Loading game rules, content catalogs, and save metadata.",
            ),
            GamePhase::MainMenu(main_menu_state) => {
                let Some(data) = self.data.as_ref() else {
                    return;
                };

                draw_main_menu(
                    data.config.title.as_str(),
                    &data.ui_text,
                    main_menu_state,
                    self.last_error.as_deref(),
                )
            }
            GamePhase::OpeningChapter(opening_state) => {
                let Some(data) = self.data.as_ref() else {
                    return;
                };
                let Some(game_state) = self.game_state.as_ref() else {
                    return;
                };

                draw_opening_chapter(data, opening_state, game_state, self.last_error.as_deref())
            }
            GamePhase::TownOverview(town_state) => {
                let Some(data) = self.data.as_ref() else {
                    return;
                };
                let Some(game_state) = self.game_state.as_ref() else {
                    return;
                };

                draw_town_overview(data, town_state, game_state, self.last_error.as_deref())
            }
            GamePhase::MonsterProfile(profile_state) => {
                let Some(data) = self.data.as_ref() else {
                    return;
                };
                let Some(game_state) = self.game_state.as_ref() else {
                    return;
                };

                draw_monster_profile(data, profile_state, game_state, self.last_error.as_deref())
            }
            GamePhase::TownManagement(town_state) => {
                let Some(data) = self.data.as_ref() else {
                    return;
                };
                let Some(game_state) = self.game_state.as_ref() else {
                    return;
                };

                draw_town_management(data, town_state, game_state, self.last_error.as_deref())
            }
            GamePhase::ContractDesk(guest_state) => {
                let Some(data) = self.data.as_ref() else {
                    return;
                };
                let Some(game_state) = self.game_state.as_ref() else {
                    return;
                };

                draw_contract_desk(data, guest_state, game_state, self.last_error.as_deref())
            }
            GamePhase::HatcheryManagement(chamber_state) => {
                let Some(data) = self.data.as_ref() else {
                    return;
                };
                let Some(game_state) = self.game_state.as_ref() else {
                    return;
                };

                draw_hatchery_management(data, chamber_state, game_state, self.last_error.as_deref())
            }
            GamePhase::Journal(journal_state) => {
                let Some(data) = self.data.as_ref() else {
                    return;
                };
                let Some(game_state) = self.game_state.as_ref() else {
                    return;
                };

                draw_journal(data, journal_state, game_state, self.last_error.as_deref())
            }
            GamePhase::GuildHallManagement(guild_jobs_state) => {
                let Some(data) = self.data.as_ref() else {
                    return;
                };
                let Some(game_state) = self.game_state.as_ref() else {
                    return;
                };

                draw_guild_hall_management(data, guild_jobs_state, game_state, self.last_error.as_deref())
            }
            GamePhase::ExpeditionPlanning(expedition_state) => {
                let Some(data) = self.data.as_ref() else {
                    return;
                };
                let Some(game_state) = self.game_state.as_ref() else {
                    return;
                };

                draw_expedition_planning(
                    data,
                    expedition_state,
                    game_state,
                    self.last_error.as_deref(),
                )
            }
            GamePhase::HatchReveal(hatch_state) => {
                let Some(data) = self.data.as_ref() else {
                    return;
                };
                let Some(game_state) = self.game_state.as_ref() else {
                    return;
                };

                draw_hatch_reveal(data, hatch_state, game_state, self.last_error.as_deref())
            }
            GamePhase::DayResults(results_state) => {
                let Some(data) = self.data.as_ref() else {
                    return;
                };

                draw_day_results(data, results_state, self.last_error.as_deref())
            }
        };

        let overlay_action = if self.is_settings_open {
            if let (Some(data), Some(app_settings)) =
                (self.data.as_ref(), self.app_settings.as_ref())
            {
                let status_message = self
                    .last_error
                    .as_deref()
                    .or(self.settings_status.as_deref());
                draw_settings_modal(
                    data,
                    app_settings,
                    cfg!(not(target_arch = "wasm32")),
                    status_message,
                    self.last_error.is_some(),
                )
            } else {
                None
            }
        } else {
            None
        };

        self.pending_action = overlay_action.or(base_action);
    }
}
