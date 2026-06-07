use super::*;

pub fn daily_priority_summary(data: &GameData, game_state: &GameState) -> DailyPrioritySummary {
    if game_state.monsters.is_empty() {
        return DailyPrioritySummary {
            title: data.ui_text.town_overview.priority_no_roster_title.clone(),
            detail: data.ui_text.town_overview.priority_no_roster_detail.clone(),
            action_hint: data.ui_text.common.chamber_button.clone(),
            color: theme::INFO,
        };
    }

    if !game_state.egg_inventory.is_empty() {
        return DailyPrioritySummary {
            title: data.ui_text.town_overview.priority_eggs_title.clone(),
            detail: data.ui_text.town_overview.priority_eggs_detail.clone(),
            action_hint: data.ui_text.common.chamber_button.clone(),
            color: theme::WARNING,
        };
    }

    if let Some(debt) = &game_state.debt {
        if debt.days_until_due <= 2 {
            return DailyPrioritySummary {
                title: data.ui_text.town_overview.priority_debt_title.clone(),
                detail: fill_template(
                    &data.ui_text.town_overview.priority_debt_detail_template,
                    &[
                        ("{gold}", debt.current_balance_due.to_string()),
                        ("{days}", debt.days_until_due.to_string()),
                    ],
                ),
                action_hint: data.ui_text.common.guild_jobs_button.clone(),
                color: theme::DANGER,
            };
        }
    }

    if game_state.active_contracts.iter().any(|request| {
        matches!(
            request.status,
            crate::state::ContractStatus::Pending | crate::state::ContractStatus::Accepted
        )
    }) {
        return DailyPrioritySummary {
            title: data.ui_text.town_overview.priority_guests_title.clone(),
            detail: data.ui_text.town_overview.priority_guests_detail.clone(),
            action_hint: data.ui_text.common.guest_desk_button.clone(),
            color: theme::POSITIVE,
        };
    }

    DailyPrioritySummary {
        title: data.ui_text.town_overview.priority_growth_title.clone(),
        detail: data.ui_text.town_overview.priority_growth_detail.clone(),
        action_hint: data.ui_text.common.town_planner_button.clone(),
        color: theme::PRIMARY,
    }
}

pub fn action_from_action_hint(data: &GameData, action_hint: &str) -> UiAction {
    if action_hint == data.ui_text.common.chamber_button {
        UiAction::OpenHatcheryManagement
    } else if action_hint == data.ui_text.common.guest_desk_button {
        UiAction::OpenContractDesk
    } else if action_hint == data.ui_text.common.guild_jobs_button {
        UiAction::OpenGuildHallManagement
    } else if action_hint == data.ui_text.common.journal_button {
        UiAction::OpenJournal
    } else if action_hint == data.ui_text.common.expedition_desk_button {
        UiAction::OpenExpeditionPlanning
    } else {
        UiAction::OpenTownManagement
    }
}

pub fn worker_decision_summary(
    _data: &GameData,
    monster: &crate::state::CompanionState,
    prediction_line: String,
) -> WorkerDecisionSummary {
    let highlight = match monster.current_job {
        CompanionJobState::GuildJob { .. } => theme::POSITIVE,
        CompanionJobState::Resting => theme::WARNING,
        CompanionJobState::OnExpedition { .. } => theme::INFO,
        CompanionJobState::Idle => theme::PRIMARY,
    };
    WorkerDecisionSummary {
        prediction_line,
        highlight,
    }
}

pub fn building_decision_summary(
    data: &GameData,
    game_state: &GameState,
    building: &crate::data::BuildingData,
) -> BuildingDecisionSummary {
    let ui = &data.ui_text.town_management;
    let build_count = game_state
        .town
        .constructed_building_ids
        .iter()
        .filter(|id| *id == &building.id)
        .count();
    let can_afford = game_state.resources.gold >= building.cost.gold
        && game_state.resources.tower_materials >= building.cost.tower_materials
        && game_state.resources.eggs >= building.cost.eggs
        && game_state.resources.relics >= building.cost.relics
        && game_state.resources.arcane_residue >= building.cost.arcane_residue;
    let (status_label, status_color) = if build_count >= usize::from(building.build_limit) {
        (ui.built_out_label.clone(), theme::DANGER)
    } else if can_afford {
        (ui.available_label.clone(), theme::POSITIVE)
    } else {
        (ui.locked_by_cost_label.clone(), theme::WARNING)
    };

    let mut unlock_labels = Vec::new();
    if !building.unlocks.room_ids.is_empty() {
        unlock_labels.push(format!(
            "{}: {}",
            ui.unlocks_rooms_label,
            building
                .unlocks
                .room_ids
                .iter()
                .map(|room_id| room_name_by_id(data, room_id))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !building.unlocks.floor_ids.is_empty() {
        unlock_labels.push(format!(
            "{}: {}",
            ui.unlocks_floors_label,
            building
                .unlocks
                .floor_ids
                .iter()
                .map(|floor_id| floor_name_by_id(data, floor_id))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !building.unlocks.species_ids.is_empty() {
        unlock_labels.push(format!(
            "{}: {}",
            ui.unlocks_species_label,
            building
                .unlocks
                .species_ids
                .iter()
                .map(|species_id| species_name_by_id(data, species_id))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if unlock_labels.is_empty() {
        unlock_labels.push(data.ui_text.common.none_label.clone());
    }

    let next_destination = if !building.unlocks.room_ids.is_empty() {
        data.ui_text.common.guild_jobs_button.clone()
    } else if !building.unlocks.floor_ids.is_empty() {
        data.ui_text.common.expedition_desk_button.clone()
    } else if !building.unlocks.species_ids.is_empty() {
        data.ui_text.common.chamber_button.clone()
    } else {
        data.ui_text.common.return_to_town_button.clone()
    };

    BuildingDecisionSummary {
        status_label,
        status_color,
        can_afford,
        build_count,
        effect_lines: describe_building_effects(&data.ui_text, building),
        unlock_labels,
        next_destination,
    }
}

pub fn monster_role_summary(
    data: &GameData,
    monster: &crate::state::CompanionState,
) -> MonsterRoleSummary {
    let profile = &data.ui_text.monster_profile;
    let role_suffix = format!(" ({})", monster_depth_role_label(monster));
    if monster.injury > 0 || monster.stress >= 3 || monster.fatigue >= 3 {
        return MonsterRoleSummary {
            readiness_label: profile.readiness_hurt_label.clone(),
            readiness_color: theme::WARNING,
            best_next_use: format!("{}{}", profile.best_next_rest_label.as_str(), role_suffix),
        };
    }

    if monster.stats.power >= monster.stats.charm + 2 {
        return MonsterRoleSummary {
            readiness_label: profile.readiness_ready_label.clone(),
            readiness_color: theme::INFO,
            best_next_use: format!(
                "{}{}",
                profile.best_next_expedition_label.as_str(),
                role_suffix
            ),
        };
    }

    if monster.skills.charm >= 2
        || monster.skills.hospitality >= 2
        || monster.skills.guarding >= 2
    {
        return MonsterRoleSummary {
            readiness_label: profile.readiness_ready_label.clone(),
            readiness_color: theme::POSITIVE,
            best_next_use: format!(
                "{}{}",
                profile.best_next_guild_job_label.as_str(),
                role_suffix
            ),
        };
    }

    if matches!(monster.current_job, CompanionJobState::Resting) {
        return MonsterRoleSummary {
            readiness_label: profile.readiness_rest_label.clone(),
            readiness_color: theme::WARNING,
            best_next_use: format!("{}{}", profile.best_next_rest_label.as_str(), role_suffix),
        };
    }

    MonsterRoleSummary {
        readiness_label: profile.readiness_ready_label.clone(),
        readiness_color: theme::PRIMARY,
        best_next_use: format!(
            "{}{}",
            profile.best_next_training_label.as_str(),
            role_suffix
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::GameData;
    use crate::engine::{
        advance_opening_step, build_first_room, create_new_game_state, create_opening_egg,
        initialize_first_debt, refresh_contracts, resolve_first_client,
    };

    #[test]
    fn simulated_opening_flow_points_journal_to_chamber_when_egg_is_waiting() {
        let data = test_game_data();
        let mut game_state = create_new_game_state(&data);

        advance_opening_step(&data, &mut game_state).expect("camp step should advance");
        advance_opening_step(&data, &mut game_state).expect("discovery step should advance");
        advance_opening_step(&data, &mut game_state).expect("incubation step should advance");
        advance_opening_step(&data, &mut game_state).expect("hatch step should advance");
        build_first_room(&data, &mut game_state).expect("first room should build");
        create_opening_egg(&mut game_state, "slime_girl");

        let priority = daily_priority_summary(&data, &game_state);
        let guidance = onboarding_lines(&data, &game_state);

        assert_eq!(
            priority.title,
            data.ui_text.town_overview.priority_eggs_title
        );
        assert!(matches!(
            action_from_action_hint(&data, &priority.action_hint),
            crate::ui::actions::UiAction::OpenHatcheryManagement
        ));
        assert_eq!(
            guidance,
            data.ui_text.town_overview.onboarding_chamber_lines
        );
        assert!(game_state
            .event_log
            .iter()
            .any(|entry| entry.contains("warm opening hidden in the ruined tower wall")));
        assert!(game_state
            .event_log
            .iter()
            .any(|entry| entry.contains("produced a strange egg")));
    }

    #[test]
    fn simulated_opening_completion_records_journal_history_after_first_client() {
        let data = test_game_data();
        let mut game_state = create_new_game_state(&data);

        advance_opening_step(&data, &mut game_state).expect("camp step should advance");
        advance_opening_step(&data, &mut game_state).expect("discovery step should advance");
        advance_opening_step(&data, &mut game_state).expect("incubation step should advance");
        advance_opening_step(&data, &mut game_state).expect("hatch step should advance");
        build_first_room(&data, &mut game_state).expect("first room should build");
        resolve_first_client(&data, &mut game_state).expect("first client should resolve");
        initialize_first_debt(&data, &mut game_state).expect("first debt should initialize");
        refresh_contracts(&data, &mut game_state).expect("contracts should refresh");

        let guidance = onboarding_lines(&data, &game_state);

        assert!(game_state.story_progress.first_client_completed);
        assert!(game_state.debt.is_some());
        assert!(!game_state.active_contracts.is_empty());
        assert!(guidance.iter().any(|line| line.contains("Debt")));
        assert!(game_state
            .event_log
            .iter()
            .any(|entry| entry.contains("proved the tower can pay")));
        assert!(game_state.event_log.len() >= 6);
    }

    fn test_game_data() -> GameData {
        GameData {
            config: parse_json(include_str!("../../../assets/data/config.json")),
            ui_text: parse_json(include_str!("../../../assets/data/ui_text.json")),
            debt_milestones: parse_json(include_str!("../../../assets/data/debt_milestones.json")),
            patron_archetypes: parse_json(include_str!(
                "../../../assets/data/guest_archetypes.json"
            )),
            contracts: parse_json(include_str!("../../../assets/data/guest_requests.json")),
            patron_tiers: parse_json(include_str!("../../../assets/data/client_tiers.json")),
            missions: parse_json(include_str!("../../../assets/data/missions.json")),
            mutations: parse_json(include_str!("../../../assets/data/mutations.json")),
            story_events: parse_json(include_str!("../../../assets/data/story_events.json")),
            monster_names: parse_json(include_str!("../../../assets/data/monster_names.json")),
            species: parse_json(include_str!("../../../assets/data/species.json")),
            buildings: parse_json(include_str!("../../../assets/data/buildings.json")),
            floors: parse_json(include_str!("../../../assets/data/floors.json")),
            traits: parse_json(include_str!("../../../assets/data/traits.json")),
            guild_rooms: parse_json(include_str!("../../../assets/data/guild_rooms.json")),
            events: parse_json(include_str!("../../../assets/data/events.json")),
        }
    }

    fn parse_json<T: serde::de::DeserializeOwned>(json: &str) -> T {
        serde_json::from_str(json).expect("test data should deserialize")
    }
}
