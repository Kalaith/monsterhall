use macroquad::time::get_time;
use std::collections::HashSet;

use crate::state::{HatchRevealReturn, HatchRevealState};

use super::*;

impl Game {
    pub(super) fn apply_action(&mut self, action: UiAction) {
        match action {
            UiAction::StartNewGame => self.start_new_game(),
            UiAction::ContinueGame => self.continue_game(),
            UiAction::ContinueOpening => self.advance_opening_chapter(),
            UiAction::BuildOpeningRoom => self.build_opening_room(),
            UiAction::ResolveOpeningClient => self.resolve_opening_client(),
            UiAction::QuitGame => self.quit_game(),
            UiAction::SaveGame => self.persist_game_state(),
            UiAction::OpenGuildHallManagement => self.open_guild_hall_management(None),
            UiAction::OpenContractDesk => self.open_contract_desk(None),
            UiAction::OpenTownManagement => self.open_town_management(None),
            UiAction::OpenHatcheryManagement => self.open_hatchery_management(None),
            UiAction::OpenJournal => self.open_journal(None),
            UiAction::OpenExpeditionPlanning => self.open_expedition_planning(None, None, None),
            UiAction::OpenMonsterProfile(monster_id) => self.open_monster_profile(&monster_id),
            UiAction::SelectContractRequest(request_id) => {
                self.open_contract_desk(Some(request_id))
            }
            UiAction::AssignMonsterToGuest(request_id, monster_id) => {
                self.with_game_state(|game_state, data| {
                    assign_monster_to_contract(data, game_state, &request_id, &monster_id)
                });
            }
            UiAction::ClearGuestAssignment(request_id) => {
                self.with_game_state(|game_state, _| {
                    clear_contract_assignment(game_state, &request_id)
                });
            }
            UiAction::SelectChamberEgg(egg_id) => {
                if egg_id.is_empty() {
                    if let GamePhase::HatcheryManagement(state) = &self.phase {
                        self.phase =
                            GamePhase::HatcheryManagement(HatcheryManagementState::with_scroll(
                                None,
                                "Hatchery inventory review",
                                state.inventory_scroll,
                            ));
                        self.last_error = None;
                    }
                } else {
                    self.open_hatchery_management(Some(egg_id));
                }
            }
            UiAction::ReturnToTownOverview => {
                self.phase = GamePhase::TownOverview(TownOverviewState::new("Back in town"));
            }
            UiAction::SelectGuildRoom(room_id) => self.open_guild_hall_management(Some(room_id)),
            UiAction::SelectTownBuilding(building_id) => {
                self.open_town_management(Some(building_id))
            }
            UiAction::SelectTownBuildingGroup(group_id) => {
                self.open_town_management_group(group_id)
            }
            UiAction::SelectExpeditionFloor(floor_id) => {
                self.open_expedition_planning(Some(floor_id), None, None)
            }
            UiAction::SelectExpeditionMission(mission_id) => {
                self.open_expedition_planning(None, Some(mission_id), None)
            }
            UiAction::SetExpeditionPriority(priority) => {
                self.open_expedition_planning(None, None, Some(priority))
            }
            UiAction::OpenSettings => {
                self.is_settings_open = true;
                self.settings_status = None;
            }
            UiAction::CloseSettings => {
                self.is_settings_open = false;
                self.settings_status = None;
            }
            UiAction::ToggleFullscreen(fullscreen) => self.update_fullscreen(fullscreen),
            UiAction::SetResolution(resolution_id) => self.update_resolution(&resolution_id),
            UiAction::AssignMonsterToRoom(monster_id, room_id) => {
                self.with_game_state(|game_state, _| {
                    assign_monster_to_room(game_state, &monster_id, &room_id)
                });
            }
            UiAction::AssignMonsterToExpedition(monster_id, floor_id) => {
                let selection: Result<(String, ExpeditionPriority), String> =
                    if let Some(data) = self.data.as_ref() {
                        match &self.phase {
                            GamePhase::ExpeditionPlanning(expedition_state) => Ok((
                                expedition_state.selected_mission_id.clone(),
                                expedition_state.priority.clone(),
                            )),
                            _ => {
                                let Some(floor) =
                                    data.floors.floors.iter().find(|entry| entry.id == floor_id)
                                else {
                                    return self.last_error =
                                        Some(format!("Unknown floor id '{floor_id}'."));
                                };

                                let Some(mission_id) = floor.mission_ids.first().cloned() else {
                                    return self.last_error =
                                        Some(format!("Floor '{}' has no missions.", floor.name));
                                };

                                Ok((mission_id, ExpeditionPriority::Balanced))
                            }
                        }
                    } else {
                        Err("Game data is not available.".to_owned())
                    };

                let Ok((mission_id, priority)) = selection else {
                    self.last_error = selection.err();
                    return;
                };

                self.with_game_state(|game_state, data| {
                    configure_expedition_plan(game_state, &floor_id, &mission_id, priority);
                    assign_monster_to_expedition(data, game_state, &monster_id, &floor_id)
                });
            }
            UiAction::AssignMonsterToRest(monster_id) => {
                self.with_game_state(|game_state, _| {
                    assign_monster_to_rest(game_state, &monster_id)
                });
            }
            UiAction::AssignMonsterToIdle(monster_id) => {
                self.with_game_state(|game_state, _| {
                    assign_monster_to_idle(game_state, &monster_id)
                });
            }
            UiAction::ReleaseMonster(monster_id) => {
                self.with_game_state(|game_state, _| {
                    release_monster(game_state, &monster_id).map(|_| ())
                });
                if self.last_error.is_none() {
                    self.phase = GamePhase::TownOverview(TownOverviewState::new("Roster updated"));
                }
            }
            UiAction::PurchaseBuilding(building_id) => {
                self.with_game_state(|game_state, data| {
                    purchase_building(data, game_state, &building_id).map(|_| ())
                });
            }
            UiAction::PayDebtNow => {
                self.with_game_state(|game_state, data| pay_debt_now(data, game_state));
                if self.last_error.is_none() {
                    self.apply_phase_status("Debt payment sent");
                }
            }
            UiAction::HatchSelectedEgg(egg_id, species_id_override) => {
                self.hatch_selected_egg_with_reveal(&egg_id, species_id_override.as_deref());
            }
            UiAction::ReplaceMonsterWithEgg(egg_id, species_id_override, monster_id) => {
                self.replace_monster_with_egg_with_reveal(
                    &egg_id,
                    species_id_override.as_deref(),
                    &monster_id,
                );
            }
            UiAction::ConvertEgg(egg_id, conversion) => {
                self.with_game_state(|game_state, data| {
                    convert_egg(data, game_state, &egg_id, conversion).map(|_| ())
                });
            }
            UiAction::ContinueAfterHatch => self.continue_after_hatch(),
            UiAction::ResolveDay => self.resolve_active_day(),
            UiAction::ContinueAfterResults => {
                self.phase =
                    GamePhase::TownOverview(TownOverviewState::new("A new day begins in the keep"));
            }
        }
    }

    fn hatch_selected_egg_with_reveal(&mut self, egg_id: &str, species_id_override: Option<&str>) {
        let inventory_scroll = match &self.phase {
            GamePhase::HatcheryManagement(state) => state.inventory_scroll,
            _ => 0,
        };
        let Some(data) = self.data.as_ref() else {
            self.last_error = Some("Game data is not available.".to_owned());
            return;
        };
        let Some(game_state) = self.game_state.as_mut() else {
            self.last_error = Some("No campaign is active.".to_owned());
            return;
        };

        let Some(egg_snapshot) = game_state
            .egg_inventory
            .iter()
            .find(|egg| egg.id == egg_id)
            .cloned()
        else {
            self.last_error = Some(format!("Unknown egg id '{egg_id}'."));
            return;
        };
        let existing_monster_ids = game_state
            .monsters
            .iter()
            .map(|monster| monster.id.clone())
            .collect::<HashSet<_>>();

        match hatch_selected_egg(data, game_state, egg_id, species_id_override) {
            Ok(_) => {
                let monster_id = game_state
                    .monsters
                    .iter()
                    .rev()
                    .find(|monster| !existing_monster_ids.contains(&monster.id))
                    .or_else(|| game_state.monsters.last())
                    .map(|monster| monster.id.clone());

                let Some(monster_id) = monster_id else {
                    self.last_error = Some(
                        "Hatch completed, but the new companion could not be identified."
                            .to_owned(),
                    );
                    return;
                };

                self.phase = GamePhase::HatchReveal(HatchRevealState::new(
                    egg_snapshot,
                    monster_id,
                    get_time(),
                    HatchRevealReturn::HatcheryManagement { inventory_scroll },
                ));
                self.last_error = None;
                self.autosave_game_state();
            }
            Err(message) => {
                self.last_error = Some(message);
            }
        }
    }

    fn replace_monster_with_egg_with_reveal(
        &mut self,
        egg_id: &str,
        species_id_override: Option<&str>,
        replacement_monster_id: &str,
    ) {
        let inventory_scroll = match &self.phase {
            GamePhase::HatcheryManagement(state) => state.inventory_scroll,
            _ => 0,
        };
        let Some(data) = self.data.as_ref() else {
            self.last_error = Some("Game data is not available.".to_owned());
            return;
        };
        let Some(game_state) = self.game_state.as_mut() else {
            self.last_error = Some("No campaign is active.".to_owned());
            return;
        };

        let Some(egg_snapshot) = game_state
            .egg_inventory
            .iter()
            .find(|egg| egg.id == egg_id)
            .cloned()
        else {
            self.last_error = Some(format!("Unknown egg id '{egg_id}'."));
            return;
        };
        let existing_monster_ids = game_state
            .monsters
            .iter()
            .map(|monster| monster.id.clone())
            .collect::<HashSet<_>>();

        match replace_monster_with_selected_egg(
            data,
            game_state,
            egg_id,
            species_id_override,
            replacement_monster_id,
        ) {
            Ok(_) => {
                let monster_id = game_state
                    .monsters
                    .iter()
                    .rev()
                    .find(|monster| !existing_monster_ids.contains(&monster.id))
                    .or_else(|| game_state.monsters.last())
                    .map(|monster| monster.id.clone());

                let Some(monster_id) = monster_id else {
                    self.last_error = Some(
                        "Replacement completed, but the new companion could not be identified."
                            .to_owned(),
                    );
                    return;
                };

                self.phase = GamePhase::HatchReveal(HatchRevealState::new(
                    egg_snapshot,
                    monster_id,
                    get_time(),
                    HatchRevealReturn::HatcheryManagement { inventory_scroll },
                ));
                self.last_error = None;
                self.autosave_game_state();
            }
            Err(message) => {
                self.last_error = Some(message);
            }
        }
    }

    fn continue_after_hatch(&mut self) {
        let return_to = match &self.phase {
            GamePhase::HatchReveal(hatch_state) => hatch_state.return_to.clone(),
            _ => return,
        };

        match return_to {
            HatchRevealReturn::HatcheryManagement { inventory_scroll } => {
                let selected_egg_id = self
                    .game_state
                    .as_ref()
                    .and_then(|state| state.egg_inventory.first().map(|egg| egg.id.clone()));
                self.phase = GamePhase::HatcheryManagement(HatcheryManagementState::with_scroll(
                    selected_egg_id,
                    "Hatch complete",
                    inventory_scroll,
                ));
            }
            HatchRevealReturn::OpeningChapter { next_step } => {
                self.phase = GamePhase::OpeningChapter(OpeningChapterState::new(next_step));
            }
        }
        self.last_error = None;
    }

    pub(super) fn start_new_game(&mut self) {
        let Some(data) = self.data.as_ref() else {
            self.last_error = Some("Game data is not available.".to_owned());
            return;
        };

        const REPLACE_SAVE_CONFIRMATION: &str =
            "A saved campaign exists. Click New Campaign again to replace it, or Continue Campaign to load it.";
        if save_exists(self.save_identifier())
            && self.last_error.as_deref() != Some(REPLACE_SAVE_CONFIRMATION)
        {
            self.phase = GamePhase::MainMenu(MainMenuState::new(true));
            self.last_error = Some(REPLACE_SAVE_CONFIRMATION.to_owned());
            return;
        }

        let game_state = create_new_game_state(data);
        let save_data = SaveData::new(data.config.save_version, game_state.clone());

        if let Err(message) = save_game(self.save_identifier(), &save_data) {
            self.last_error = Some(message);
            return;
        }

        self.game_state = Some(game_state);
        self.phase = GamePhase::OpeningChapter(OpeningChapterState::new(OpeningChapterStep::Camp));
        self.last_error = None;
    }

    pub(super) fn continue_game(&mut self) {
        let Some(data) = self.data.as_ref() else {
            self.last_error = Some("Game data is not available.".to_owned());
            return;
        };

        match load_compatible_save_data(self.save_identifier(), data.config.save_version) {
            Ok((save_data, was_migrated)) => {
                let opening_step = save_data.game_state.story_progress.opening_step;
                let first_client_completed =
                    save_data.game_state.story_progress.first_client_completed;
                let loaded_game_state = save_data.game_state;
                if let Err(message) = validate_game_state_references(data, &loaded_game_state) {
                    self.last_error = Some(format!("Save data is no longer compatible: {message}"));
                    return;
                }
                self.game_state = Some(loaded_game_state.clone());
                self.phase = if first_client_completed {
                    GamePhase::TownOverview(TownOverviewState::new("Campaign loaded from save"))
                } else {
                    GamePhase::OpeningChapter(OpeningChapterState::new(opening_step))
                };
                self.last_error = if was_migrated {
                    let migrated_save = SaveData::new(data.config.save_version, loaded_game_state);
                    match save_game(self.save_identifier(), &migrated_save) {
                        Ok(()) => Some(format!(
                            "Older save migrated to version {} and loaded.",
                            data.config.save_version
                        )),
                        Err(message) => Some(format!(
                            "Save loaded, but failed to persist migrated version: {}",
                            message
                        )),
                    }
                } else {
                    None
                };
            }
            Err(message) => {
                let version_message = peek_save_version(self.save_identifier()).ok().flatten().map(
                    |saved_version| {
                        format!(
                            "Save version {} is newer than supported save version {}. Start a new campaign for this build.",
                            saved_version, data.config.save_version
                        )
                    },
                );
                self.last_error = Some(version_message.unwrap_or(message));
            }
        }
    }

    pub(super) fn persist_game_state(&mut self) {
        let Some(data) = self.data.as_ref() else {
            self.last_error = Some("Game data is not available.".to_owned());
            return;
        };

        let Some(game_state) = self.game_state.clone() else {
            self.last_error = Some("No campaign is active.".to_owned());
            return;
        };

        let save_data = SaveData::new(data.config.save_version, game_state);
        match save_game(self.save_identifier(), &save_data) {
            Ok(()) => {
                self.last_error = None;
                self.settings_status = Some("Campaign saved.".to_owned());
                self.apply_phase_status("Campaign saved");
            }
            Err(message) => {
                self.settings_status = Some("Save failed.".to_owned());
                self.last_error = Some(message);
            }
        }
    }

    pub(super) fn autosave_game_state(&mut self) {
        let Some(data) = self.data.as_ref() else {
            return;
        };
        if !data.config.persistence.autosave_enabled {
            return;
        }

        let Some(game_state) = self.game_state.clone() else {
            return;
        };

        let save_data = SaveData::new(data.config.save_version, game_state);
        if let Err(message) = save_game(self.save_identifier(), &save_data) {
            self.last_error = Some(message);
        }
    }

    pub(super) fn resolve_active_day(&mut self) {
        let Some(data) = self.data.as_ref() else {
            self.last_error = Some("Game data is not available.".to_owned());
            return;
        };

        let Some(game_state) = &mut self.game_state else {
            self.last_error = Some("No campaign is active.".to_owned());
            return;
        };

        if game_state.monsters.is_empty() {
            self.last_error = Some(
                "Hatch a monster companion before ending the day. A campaign cannot progress with an empty roster."
                    .to_owned(),
            );
            return;
        }

        let summary = resolve_day(data, game_state);
        self.phase = GamePhase::DayResults(DayResultsState::new(summary));
        self.last_error = None;
        self.autosave_game_state();
    }

    pub(super) fn quit_game(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::process::exit(0);
        }

        #[cfg(target_arch = "wasm32")]
        {
            self.is_settings_open = false;
            self.last_error = Some(
                "Quit Game is not available in browser builds. Close the tab instead.".to_owned(),
            );
        }
    }

    pub(super) fn with_game_state<F>(&mut self, apply: F)
    where
        F: FnOnce(&mut GameState, &GameData) -> Result<(), String>,
    {
        let Some(data) = self.data.as_ref() else {
            self.last_error = Some("Game data is not available.".to_owned());
            return;
        };
        let Some(game_state) = &mut self.game_state else {
            self.last_error = Some("No campaign is active.".to_owned());
            return;
        };

        match apply(game_state, data) {
            Ok(()) => {
                self.last_error = None;
                match &self.phase {
                    GamePhase::TownOverview(_) => {
                        self.phase =
                            GamePhase::TownOverview(TownOverviewState::new("Assignments updated"));
                    }
                    GamePhase::TownManagement(town_state) => {
                        self.phase = GamePhase::TownManagement(TownManagementState::with_group(
                            town_state.selected_building_id.clone(),
                            town_state.selected_group_id.clone(),
                            "Town plan updated",
                        ));
                    }
                    GamePhase::ContractDesk(guest_state) => {
                        let selected_request_id = guest_state
                            .selected_request_id
                            .clone()
                            .filter(|request_id| {
                                self.game_state.as_ref().is_some_and(|state| {
                                    state
                                        .active_contracts
                                        .iter()
                                        .any(|request| request.request_id == *request_id)
                                })
                            })
                            .or_else(|| {
                                self.game_state.as_ref().and_then(|state| {
                                    state
                                        .active_contracts
                                        .first()
                                        .map(|request| request.request_id.clone())
                                })
                            });
                        self.phase = GamePhase::ContractDesk(ContractDeskState::new(
                            selected_request_id,
                            "Contract assignments updated",
                        ));
                    }
                    GamePhase::HatcheryManagement(_) => {
                        let (selected_egg_id, inventory_scroll) = match &self.phase {
                            GamePhase::HatcheryManagement(state) => {
                                (state.selected_egg_id.clone(), state.inventory_scroll)
                            }
                            _ => (None, 0),
                        };
                        let selected_egg_id = selected_egg_id
                            .filter(|egg_id| {
                                self.game_state.as_ref().is_some_and(|state| {
                                    state.egg_inventory.iter().any(|egg| egg.id == *egg_id)
                                })
                            })
                            .or_else(|| {
                                self.game_state.as_ref().and_then(|state| {
                                    state.egg_inventory.first().map(|egg| egg.id.clone())
                                })
                            });
                        self.phase =
                            GamePhase::HatcheryManagement(HatcheryManagementState::with_scroll(
                                selected_egg_id,
                                "Hatchery workflow updated",
                                inventory_scroll,
                            ));
                    }
                    GamePhase::Journal(state) => {
                        self.phase = GamePhase::Journal(JournalState::new(state.event_log_scroll));
                    }
                    GamePhase::GuildHallManagement(guild_jobs_state) => {
                        self.phase = GamePhase::GuildHallManagement(GuildHallManagementState::new(
                            guild_jobs_state.selected_room_id.clone(),
                            "Guild assignments updated",
                        ));
                    }
                    GamePhase::ExpeditionPlanning(expedition_state) => {
                        self.phase = GamePhase::ExpeditionPlanning(ExpeditionPlanningState::new(
                            expedition_state.selected_floor_id.clone(),
                            expedition_state.selected_mission_id.clone(),
                            expedition_state.priority.clone(),
                            "Expedition team updated",
                        ));
                    }
                    _ => {}
                }
                self.autosave_game_state();
            }
            Err(message) => {
                self.last_error = Some(message);
            }
        }
    }
}
