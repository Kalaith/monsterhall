//! Top-level game coordinator and explicit phase transitions.

use macroquad::{
    prelude::{
        is_key_down, is_key_pressed, is_mouse_button_pressed, mouse_position, mouse_wheel, KeyCode,
        MouseButton,
    },
    window::{request_new_screen_size, set_fullscreen},
};

use crate::data::{load_game_data, GameData};
use crate::engine::{
    advance_opening_step, assign_monster_to_contract, assign_monster_to_expedition,
    assign_monster_to_idle, assign_monster_to_rest, assign_monster_to_room, build_first_room,
    clear_contract_assignment, configure_expedition_plan, convert_egg, create_new_game_state,
    debt_intro_status, decline_contract, hatch_selected_egg, initialize_first_debt, pay_debt_now,
    purchase_building, reconcile_game_state_after_load, refresh_contracts, release_monster,
    replace_monster_with_selected_egg, resolve_day, resolve_first_client,
    validate_game_state_references,
};
use crate::state::{
    load_app_settings, load_compatible_save_data, peek_save_version, save_app_settings,
    save_exists, save_game, settings_exist, AppSettings, ContractDeskState, DayResultsState,
    ExpeditionPlanningState, ExpeditionPriority, GamePhase, GameState, GuildHallManagementState,
    HatcheryManagementState, JournalState, LoadingState, MainMenuState, MonsterProfileState,
    OpeningChapterState, OpeningChapterStep, SaveData, TownManagementState, TownOverviewState,
};
use crate::ui::{
    draw_contract_desk, draw_day_results, draw_expedition_planning, draw_guild_hall_management,
    draw_hatch_reveal, draw_hatchery_management, draw_journal, draw_loading_screen, draw_main_menu,
    draw_monster_profile, draw_opening_chapter, draw_settings_modal, draw_town_management,
    draw_town_overview, UiAction,
};

mod actions;
mod capture;
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
    /// Wall-clock seconds, sampled once per frame in `update`.
    ///
    /// `apply_action` used to call `get_time()` inline to stamp a hatch reveal,
    /// which panics without a macroquad window — so the whole opening chapter
    /// was undrivable from a test, and it is the sequence every new player hits
    /// first. Sampling it at the frame boundary makes the action layer a pure
    /// function of state, and the reveal's own animation still reads real time
    /// when it draws.
    now_seconds: f64,
}

impl Game {
    pub async fn new() -> Self {
        let mut loading_state = LoadingState::new("Bootstrapping data catalogs");
        let data_result = load_game_data().await;

        let (data, app_settings, last_error) = match data_result {
            Ok(loaded_data) => {
                loading_state.mark_ready();
                let settings = load_or_default_settings(&loaded_data);
                // Saved display settings would override the fixed capture
                // window (fullscreen/resolution), breaking deterministic
                // screenshots — keep window_conf's size while capturing.
                if !macroquad_toolkit::capture::capture_requested("MONSTERHALL") {
                    apply_display_settings(&settings);
                }
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
            now_seconds: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.now_seconds = macroquad::time::get_time();
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

                draw_hatchery_management(
                    data,
                    chamber_state,
                    game_state,
                    self.last_error.as_deref(),
                )
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

                draw_guild_hall_management(
                    data,
                    guild_jobs_state,
                    game_state,
                    self.last_error.as_deref(),
                )
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
                    self.game_state.is_some()
                        && !matches!(self.phase, GamePhase::MainMenu(_) | GamePhase::Loading(_)),
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

#[cfg(test)]
mod tests {
    use super::*;

    /// A `Game` wired to test data with autosave off, so driving it touches no
    /// files. Everything else is the real thing: the same `apply_action`
    /// dispatch and the same phase machine a player goes through.
    fn headless_game() -> Game {
        let mut data = crate::data::test_game_data();
        data.config.persistence.autosave_enabled = false;
        Game {
            data: Some(data),
            phase: GamePhase::MainMenu(MainMenuState::new(false)),
            game_state: None,
            app_settings: None,
            is_settings_open: false,
            settings_status: None,
            pending_action: None,
            last_error: None,
            now_seconds: 0.0,
        }
    }

    /// Plays the scripted opening through the same actions the opening screen
    /// sends, and leaves the campaign where a player would be on day one.
    ///
    /// This became drivable only once `apply_action` stopped calling
    /// `get_time()` inline: that panics without a macroquad window, so the
    /// opening — the sequence every new player hits first — could not be
    /// exercised through the action layer at all.
    fn game_through_the_opening() -> Game {
        let mut game = headless_game();
        game.game_state = Some(crate::engine::create_new_game_state(
            game.data.as_ref().expect("test data should load"),
        ));
        game.phase = GamePhase::OpeningChapter(OpeningChapterState::new(OpeningChapterStep::Camp));

        for _ in 0..24 {
            match &game.phase {
                GamePhase::OpeningChapter(state) => match state.step {
                    OpeningChapterStep::BuildRoom => game.apply_action(UiAction::BuildOpeningRoom),
                    OpeningChapterStep::FirstClient => {
                        game.apply_action(UiAction::ResolveOpeningClient)
                    }
                    OpeningChapterStep::Complete => break,
                    _ => game.apply_action(UiAction::ContinueOpening),
                },
                GamePhase::HatchReveal(_) => game.apply_action(UiAction::ContinueAfterHatch),
                _ => break,
            }
            assert!(
                game.last_error.is_none(),
                "the opening should never refuse a step a player can only take in order: {:?}",
                game.last_error
            );
        }
        game
    }

    /// The opening is a linear phase with no way to earn, so a step the player
    /// cannot afford is a permanent soft-lock on every new campaign. Driving it
    /// through the real actions checks the dispatch and the phase transitions
    /// too, not just the engine arithmetic two journal tests already cover.
    #[test]
    fn the_opening_plays_out_through_the_actions_a_player_sends() {
        let game = game_through_the_opening();
        let state = game
            .game_state
            .as_ref()
            .expect("the opening should leave a campaign");

        assert_eq!(
            state.story_progress.opening_step,
            OpeningChapterStep::Complete,
            "the opening did not finish"
        );
        assert!(state.story_progress.first_companion_hatched);
        assert!(state.story_progress.first_room_built);
        assert!(state.story_progress.first_client_completed);
        assert_eq!(state.monsters.len(), 1, "the guild should have its founder");
        assert!(
            state
                .town
                .unlocked_room_ids
                .iter()
                .any(|id| id == "common_room"),
            "the first room should be open for business"
        );
    }

    /// Starts a campaign at the point the scripted opening ends.
    fn game_past_the_opening() -> Game {
        let mut game = game_through_the_opening();
        let data = game.data.as_ref().expect("test data should load");
        let mut game_state = game.game_state.take().expect("opening leaves a campaign");
        crate::engine::initialize_first_debt(data, &mut game_state)
            .expect("first debt should initialize");
        game.game_state = Some(game_state);
        game.phase = GamePhase::TownOverview(TownOverviewState::new("ready"));
        game
    }

    /// Plays sixty days through the actions a player actually sends.
    ///
    /// Every other test in this repo calls engine functions directly, and so
    /// does the balance harness — nothing exercised `apply_action`, the phase
    /// machine, or the transitions between them. That is the same blind spot
    /// that hid the save-path bugs: a surface the simulation never runs. This
    /// staffs the hall, books the desk and sends a party down each day before
    /// ending it, so the assignment rules and their refusals are on the path
    /// too.
    #[test]
    fn a_campaign_plays_through_the_action_layer_without_getting_stuck() {
        let mut game = game_past_the_opening();
        let mut days_played = 0;

        for _ in 0..200 {
            match &game.phase {
                GamePhase::DayResults(_) => game.apply_action(UiAction::ContinueAfterResults),
                _ => {
                    let (monster_ids, offered, floor_id) = {
                        let state = game.game_state.as_ref().expect("campaign should be active");
                        (
                            state
                                .monsters
                                .iter()
                                .map(|m| m.id.clone())
                                .collect::<Vec<_>>(),
                            state
                                .active_contracts
                                .iter()
                                .filter(|contract| {
                                    matches!(contract.status, crate::state::ContractStatus::Pending)
                                })
                                .map(|contract| contract.request_id.clone())
                                .collect::<Vec<_>>(),
                            state.town.unlocked_floor_ids.first().cloned(),
                        )
                    };

                    // Spread the roster across all three kinds of work. Some of
                    // these are legitimately refused — a booked companion cannot
                    // take a room shift — so the errors are cleared rather than
                    // asserted on; what matters is that nothing wedges.
                    for (index, monster_id) in monster_ids.iter().enumerate() {
                        match index % 3 {
                            0 => game.apply_action(UiAction::AssignMonsterToRoom(
                                monster_id.clone(),
                                "common_room".to_owned(),
                            )),
                            1 => {
                                if let Some(request_id) = offered.first() {
                                    game.apply_action(UiAction::AssignMonsterToGuest(
                                        request_id.clone(),
                                        monster_id.clone(),
                                    ));
                                }
                            }
                            _ => {
                                if let Some(floor_id) = &floor_id {
                                    game.apply_action(UiAction::AssignMonsterToExpedition(
                                        monster_id.clone(),
                                        floor_id.clone(),
                                    ));
                                }
                            }
                        }
                        game.last_error = None;
                    }

                    let before = game.game_state.as_ref().map(|state| state.current_day);
                    game.apply_action(UiAction::ResolveDay);
                    let after = game.game_state.as_ref().map(|state| state.current_day);
                    assert_ne!(
                        before, after,
                        "ending the day did not advance it: {:?}",
                        game.last_error
                    );
                    days_played += 1;
                    if days_played == 60 {
                        break;
                    }
                }
            }
        }

        assert_eq!(days_played, 60, "the campaign stalled before sixty days");
        let data = game.data.as_ref().expect("test data");
        let state = game
            .game_state
            .as_ref()
            .expect("campaign should still exist");
        crate::engine::validate_game_state_references(data, state)
            .expect("sixty days of play should leave a valid campaign");
    }
}
