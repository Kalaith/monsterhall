use macroquad::time::get_time;
use std::collections::HashSet;

use crate::state::{HatchRevealReturn, HatchRevealState};

use super::*;

impl Game {
    pub(super) fn open_guild_hall_management(&mut self, room_override: Option<String>) {
        let Some(game_state) = self.game_state.as_ref() else {
            self.last_error = Some("No campaign is active.".to_owned());
            return;
        };

        let selected_room_id = room_override
            .or_else(|| match &self.phase {
                GamePhase::GuildHallManagement(state) => Some(state.selected_room_id.clone()),
                _ => None,
            })
            .or_else(|| game_state.town.unlocked_room_ids.first().cloned());

        let Some(selected_room_id) = selected_room_id else {
            self.last_error = Some("No guild rooms are unlocked.".to_owned());
            return;
        };

        self.phase = GamePhase::GuildHallManagement(GuildHallManagementState::new(
            selected_room_id,
            "Room plan ready",
        ));
        self.last_error = None;
    }

    pub(super) fn advance_opening_chapter(&mut self) {
        let Some(current_step) = self
            .game_state
            .as_ref()
            .map(|state| state.story_progress.opening_step)
        else {
            self.last_error = Some("No campaign is active.".to_owned());
            return;
        };

        let hatch_reveal_seed = if current_step == OpeningChapterStep::Hatch {
            self.game_state.as_ref().map(|state| {
                let egg_snapshot = state.egg_inventory.first().cloned();
                let existing_monster_ids = state
                    .monsters
                    .iter()
                    .map(|monster| monster.id.clone())
                    .collect::<HashSet<_>>();
                (egg_snapshot, existing_monster_ids)
            })
        } else {
            None
        };

        self.with_game_state(|game_state, data| advance_opening_step(data, game_state));

        if self.last_error.is_none() {
            let next_step = self
                .game_state
                .as_ref()
                .map(|state| state.story_progress.opening_step)
                .unwrap_or(current_step);
            if let Some((Some(egg_snapshot), existing_monster_ids)) = hatch_reveal_seed {
                let monster_id = self.game_state.as_ref().and_then(|state| {
                    state
                        .monsters
                        .iter()
                        .rev()
                        .find(|monster| !existing_monster_ids.contains(&monster.id))
                        .or_else(|| state.monsters.last())
                        .map(|monster| monster.id.clone())
                });
                if let Some(monster_id) = monster_id {
                    self.phase = GamePhase::HatchReveal(HatchRevealState::new(
                        egg_snapshot,
                        monster_id,
                        get_time(),
                        HatchRevealReturn::OpeningChapter { next_step },
                    ));
                    return;
                }
            }
            self.phase = if next_step == OpeningChapterStep::Complete {
                let status = self
                    .game_state
                    .as_ref()
                    .map(|state| debt_intro_status(self.data.as_ref().expect("game data"), state))
                    .unwrap_or_else(|| {
                        self.data
                            .as_ref()
                            .expect("game data")
                            .story_events
                            .debt_intro_without_debt
                            .clone()
                    });
                GamePhase::TownOverview(TownOverviewState::new(&status))
            } else {
                GamePhase::OpeningChapter(OpeningChapterState::new(next_step))
            };
        }
    }

    pub(super) fn build_opening_room(&mut self) {
        self.with_game_state(|game_state, data| build_first_room(data, game_state));

        if self.last_error.is_none() {
            let step = self
                .game_state
                .as_ref()
                .map(|state| state.story_progress.opening_step)
                .unwrap_or(OpeningChapterStep::BuildRoom);
            self.phase = GamePhase::OpeningChapter(OpeningChapterState::new(step));
        }
    }

    pub(super) fn resolve_opening_client(&mut self) {
        self.with_game_state(|game_state, data| resolve_first_client(data, game_state));
        if self.last_error.is_none() {
            self.with_game_state(|game_state, data| initialize_first_debt(data, game_state));
        }
        if self.last_error.is_none() {
            self.with_game_state(|game_state, data| {
                refresh_contracts(data, game_state).map(|_| ())
            });
        }

        if self.last_error.is_none() {
            let status = self
                .game_state
                .as_ref()
                .map(|state| debt_intro_status(self.data.as_ref().expect("game data"), state))
                .unwrap_or_else(|| {
                    self.data
                        .as_ref()
                        .expect("game data")
                        .story_events
                        .debt_intro_without_debt
                        .clone()
                });
            self.phase = GamePhase::TownOverview(TownOverviewState::new(&status));
        }
    }

    pub(super) fn open_town_management(&mut self, building_override: Option<String>) {
        let Some(data) = self.data.as_ref() else {
            self.last_error = Some("Game data is not available.".to_owned());
            return;
        };

        let selected_building_id = building_override
            .or_else(|| match &self.phase {
                GamePhase::TownManagement(state) => Some(state.selected_building_id.clone()),
                _ => None,
            })
            .or_else(|| {
                data.buildings
                    .buildings
                    .first()
                    .map(|building| building.id.clone())
            });

        let Some(selected_building_id) = selected_building_id else {
            self.last_error = Some("No buildings are defined.".to_owned());
            return;
        };

        let selected_group_id = match &self.phase {
            GamePhase::TownManagement(state) => state.selected_group_id.clone(),
            _ => town_building_group_id(
                data.buildings
                    .buildings
                    .iter()
                    .find(|building| building.id == selected_building_id)
                    .map(|building| building.category.as_str())
                    .unwrap_or(""),
            )
            .to_owned(),
        };

        self.phase = GamePhase::TownManagement(TownManagementState::with_group(
            selected_building_id,
            selected_group_id,
            "Town growth review",
        ));
        self.last_error = None;
    }

    pub(super) fn open_town_management_group(&mut self, group_id: String) {
        let Some(data) = self.data.as_ref() else {
            self.last_error = Some("Game data is not available.".to_owned());
            return;
        };

        let selected_building_id = data
            .buildings
            .buildings
            .iter()
            .find(|building| town_building_group_id(&building.category) == group_id)
            .or_else(|| data.buildings.buildings.first())
            .map(|building| building.id.clone());

        let Some(selected_building_id) = selected_building_id else {
            self.last_error = Some("No buildings are defined.".to_owned());
            return;
        };

        self.phase = GamePhase::TownManagement(TownManagementState::with_group(
            selected_building_id,
            group_id,
            "Town growth review",
        ));
        self.last_error = None;
    }

    pub(super) fn open_contract_desk(&mut self, request_override: Option<String>) {
        let Some(game_state) = self.game_state.as_ref() else {
            self.last_error = Some("No campaign is active.".to_owned());
            return;
        };

        let selected_request_id = request_override
            .or_else(|| match &self.phase {
                GamePhase::ContractDesk(state) => state.selected_request_id.clone(),
                _ => None,
            })
            .or_else(|| {
                game_state
                    .active_contracts
                    .first()
                    .map(|request| request.request_id.clone())
            });

        self.phase = GamePhase::ContractDesk(ContractDeskState::new(
            selected_request_id,
            "Contract desk ready",
        ));
        self.last_error = None;
    }

    pub(super) fn open_expedition_planning(
        &mut self,
        floor_override: Option<String>,
        mission_override: Option<String>,
        priority_override: Option<ExpeditionPriority>,
    ) {
        let Some(data) = self.data.as_ref() else {
            self.last_error = Some("Game data is not available.".to_owned());
            return;
        };
        let Some(game_state) = self.game_state.as_ref() else {
            self.last_error = Some("No campaign is active.".to_owned());
            return;
        };

        let selected_floor_id = floor_override
            .or_else(|| match &self.phase {
                GamePhase::ExpeditionPlanning(state) => Some(state.selected_floor_id.clone()),
                _ => None,
            })
            .or_else(|| game_state.town.unlocked_floor_ids.first().cloned());

        let Some(selected_floor_id) = selected_floor_id else {
            self.last_error = Some("No expedition floors are unlocked.".to_owned());
            return;
        };

        let Some(selected_floor) = data
            .floors
            .floors
            .iter()
            .find(|floor| floor.id == selected_floor_id)
        else {
            self.last_error = Some(format!("Unknown floor id '{}'.", selected_floor_id));
            return;
        };

        let selected_mission_id = mission_override
            .or_else(|| match &self.phase {
                GamePhase::ExpeditionPlanning(state) => Some(state.selected_mission_id.clone()),
                _ => None,
            })
            .filter(|mission_id| selected_floor.mission_ids.contains(mission_id))
            .or_else(|| selected_floor.mission_ids.first().cloned());

        let Some(selected_mission_id) = selected_mission_id else {
            self.last_error = Some(format!("Floor '{}' has no missions.", selected_floor.name));
            return;
        };

        let priority = priority_override
            .or_else(|| match &self.phase {
                GamePhase::ExpeditionPlanning(state) => Some(state.priority.clone()),
                _ => None,
            })
            .unwrap_or(ExpeditionPriority::Balanced);

        self.phase = GamePhase::ExpeditionPlanning(ExpeditionPlanningState::new(
            selected_floor_id,
            selected_mission_id,
            priority,
            "Plan expedition",
        ));
        self.last_error = None;
    }

    pub(super) fn open_hatchery_management(&mut self, egg_override: Option<String>) {
        let Some(game_state) = self.game_state.as_ref() else {
            self.last_error = Some("No campaign is active.".to_owned());
            return;
        };

        let inventory_scroll = match &self.phase {
            GamePhase::HatcheryManagement(state) => state.inventory_scroll,
            _ => 0,
        };
        let selected_egg_id = egg_override
            .or_else(|| match &self.phase {
                GamePhase::HatcheryManagement(state) => state.selected_egg_id.clone(),
                _ => None,
            })
            .filter(|egg_id| game_state.egg_inventory.iter().any(|egg| egg.id == *egg_id))
            .or_else(|| game_state.egg_inventory.first().map(|egg| egg.id.clone()));

        self.phase = GamePhase::HatcheryManagement(HatcheryManagementState::with_scroll(
            selected_egg_id,
            "Hatchery inventory review",
            inventory_scroll,
        ));
        self.last_error = None;
    }

    pub(super) fn open_journal(&mut self, scroll_override: Option<usize>) {
        let Some(game_state) = self.game_state.as_ref() else {
            self.last_error = Some("No campaign is active.".to_owned());
            return;
        };

        let max_scroll = game_state.event_log.len().saturating_sub(12);
        let event_log_scroll = scroll_override
            .or_else(|| match &self.phase {
                GamePhase::Journal(state) => Some(state.event_log_scroll),
                _ => None,
            })
            .unwrap_or(0)
            .min(max_scroll);

        self.phase = GamePhase::Journal(JournalState::new(event_log_scroll));
        self.last_error = None;
    }

    pub(super) fn open_monster_profile(&mut self, monster_id: &str) {
        let Some(game_state) = self.game_state.as_ref() else {
            self.last_error = Some("No campaign is active.".to_owned());
            return;
        };

        let Some(monster) = game_state
            .monsters
            .iter()
            .find(|monster| monster.id == monster_id)
        else {
            self.last_error = Some(format!("Unknown monster id '{monster_id}'."));
            return;
        };

        self.phase = GamePhase::MonsterProfile(MonsterProfileState::new(
            monster.id.clone(),
            &format!("Inspecting {}", monster.name),
        ));
        self.last_error = None;
    }

    pub(super) fn apply_phase_status(&mut self, status_message: &str) {
        self.phase = match &self.phase {
            GamePhase::Loading(state) => GamePhase::Loading(state.clone()),
            GamePhase::MainMenu(state) => GamePhase::MainMenu(state.clone()),
            GamePhase::OpeningChapter(state) => GamePhase::OpeningChapter(state.clone()),
            GamePhase::TownOverview(_) => {
                GamePhase::TownOverview(TownOverviewState::new(status_message))
            }
            GamePhase::MonsterProfile(state) => GamePhase::MonsterProfile(
                MonsterProfileState::new(state.selected_monster_id.clone(), status_message),
            ),
            GamePhase::TownManagement(state) => {
                GamePhase::TownManagement(TownManagementState::with_group(
                    state.selected_building_id.clone(),
                    state.selected_group_id.clone(),
                    status_message,
                ))
            }
            GamePhase::ContractDesk(state) => GamePhase::ContractDesk(ContractDeskState::new(
                state.selected_request_id.clone(),
                status_message,
            )),
            GamePhase::HatcheryManagement(state) => {
                GamePhase::HatcheryManagement(HatcheryManagementState::with_scroll(
                    state.selected_egg_id.clone(),
                    status_message,
                    state.inventory_scroll,
                ))
            }
            GamePhase::Journal(state) => GamePhase::Journal(state.clone()),
            GamePhase::GuildHallManagement(state) => GamePhase::GuildHallManagement(
                GuildHallManagementState::new(state.selected_room_id.clone(), status_message),
            ),
            GamePhase::ExpeditionPlanning(state) => {
                GamePhase::ExpeditionPlanning(ExpeditionPlanningState::new(
                    state.selected_floor_id.clone(),
                    state.selected_mission_id.clone(),
                    state.priority.clone(),
                    status_message,
                ))
            }
            GamePhase::HatchReveal(state) => GamePhase::HatchReveal(state.clone()),
            GamePhase::DayResults(state) => GamePhase::DayResults(state.clone()),
        };
    }
}
