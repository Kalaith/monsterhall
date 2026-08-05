//! Debt initialization and milestone helpers.

use crate::data::GameData;
use crate::state::{CampaignFailureState, DebtResolution, DebtState, GameState};

fn escalating_miss_cost(base: u32, miss_number: u32, growth_pct: u32) -> u32 {
    let multiplier_pct = 100u64.saturating_add(
        u64::from(miss_number.saturating_sub(1)).saturating_mul(u64::from(growth_pct)),
    );
    u32::try_from(u64::from(base).saturating_mul(multiplier_pct).div_ceil(100)).unwrap_or(u32::MAX)
}

fn story_text(template: &str, replacements: &[(&str, String)]) -> String {
    let mut output = template.to_owned();
    for (token, value) in replacements {
        output = output.replace(token, value);
    }
    output
}

fn total_scheduled_debt(data: &GameData) -> u32 {
    data.debt_milestones
        .milestones
        .iter()
        .map(|milestone| milestone.amount_due)
        .sum()
}

pub fn initialize_first_debt(data: &GameData, game_state: &mut GameState) -> Result<(), String> {
    if game_state.debt.is_some() {
        return Ok(());
    }

    let first_milestone = data
        .debt_milestones
        .milestones
        .iter()
        .find(|milestone| milestone.id == data.debt_milestones.first_milestone_id)
        .ok_or_else(|| {
            format!(
                "Debt milestone '{}' is missing from debt_milestones.json.",
                data.debt_milestones.first_milestone_id
            )
        })?;

    game_state.debt = Some(DebtState {
        active_milestone_id: first_milestone.id.clone(),
        current_balance_due: first_milestone.amount_due,
        days_until_due: first_milestone.days_allowed,
        missed_payment_count: 0,
        resolved_milestone_ids: Vec::new(),
        last_resolution: None,
        status_message: story_text(
            &data.story_events.debt_status_due_template,
            &[
                ("{name}", first_milestone.name.clone()),
                ("{days}", first_milestone.days_allowed.to_string()),
            ],
        ),
    });
    game_state.story_progress.first_creditor_visit_seen = true;
    game_state.event_log.push(story_text(
        &data.story_events.debt_initialize_log_template,
        &[
            ("{gold}", first_milestone.amount_due.to_string()),
            ("{days}", first_milestone.days_allowed.to_string()),
        ],
    ));

    Ok(())
}

pub fn debt_intro_status(data: &GameData, game_state: &GameState) -> String {
    if let Some(debt) = &game_state.debt {
        story_text(
            &data.story_events.debt_intro_with_debt_template,
            &[
                ("{days}", debt.days_until_due.to_string()),
                ("{gold}", debt.current_balance_due.to_string()),
            ],
        )
    } else {
        data.story_events.debt_intro_without_debt.clone()
    }
}

pub fn pay_debt_now(data: &GameData, game_state: &mut GameState) -> Result<(), String> {
    if let Some(failure) = &game_state.campaign_failure {
        return Err(failure.reason.clone());
    }
    let Some(mut debt) = game_state.debt.clone() else {
        return Err("There is no active debt to pay.".to_owned());
    };
    let milestone = data
        .debt_milestones
        .milestones
        .iter()
        .find(|milestone| milestone.id == debt.active_milestone_id)
        .cloned()
        .ok_or_else(|| {
            format!(
                "Debt milestone '{}' is missing from debt_milestones.json.",
                debt.active_milestone_id
            )
        })?;

    if game_state.resources.gold < debt.current_balance_due {
        return Err(format!(
            "Need {} gold to pay this debt. Current gold: {}.",
            debt.current_balance_due, game_state.resources.gold
        ));
    }

    game_state.resources.gold -= debt.current_balance_due;
    game_state.resources.tower_materials += milestone.reward.tower_materials;
    game_state.resources.relics += milestone.reward.relics;
    game_state.resources.arcane_residue += milestone.reward.arcane_residue;
    debt.last_resolution = Some(if debt.missed_payment_count > 0 {
        DebtResolution::PaidLate
    } else {
        DebtResolution::PaidOnTime
    });
    debt.resolved_milestone_ids.push(milestone.id.clone());
    game_state.event_log.push(story_text(
        &data.story_events.debt_paid_event_template,
        &[
            ("{name}", milestone.name.clone()),
            ("{gold}", debt.current_balance_due.to_string()),
        ],
    ));

    if let Some(next_milestone_id) = &milestone.next_milestone_id {
        if let Some(next_milestone) = data
            .debt_milestones
            .milestones
            .iter()
            .find(|entry| &entry.id == next_milestone_id)
        {
            debt.active_milestone_id = next_milestone.id.clone();
            debt.current_balance_due = next_milestone.amount_due;
            debt.days_until_due = next_milestone.days_allowed;
            debt.missed_payment_count = 0;
            debt.status_message = story_text(
                &data.story_events.debt_status_due_template,
                &[
                    ("{name}", next_milestone.name.clone()),
                    ("{days}", next_milestone.days_allowed.to_string()),
                ],
            );
            game_state.event_log.push(story_text(
                &data.story_events.debt_next_due_event_template,
                &[
                    ("{name}", next_milestone.name.clone()),
                    ("{days}", next_milestone.days_allowed.to_string()),
                ],
            ));
            game_state.debt = Some(debt);
        } else {
            game_state.debt = None;
        }
    } else {
        game_state
            .event_log
            .push(data.story_events.debt_cleared_event.clone());
        game_state.debt = None;
    }

    Ok(())
}

pub fn resolve_debt_cycle(
    data: &GameData,
    game_state: &mut GameState,
    debt_updates: &mut Vec<String>,
    event_lines: &mut Vec<String>,
    roster_updates: &mut Vec<String>,
) {
    let Some(mut debt) = game_state.debt.clone() else {
        return;
    };

    if debt.days_until_due > 0 {
        debt.days_until_due -= 1;
    }

    let Some(milestone) = data
        .debt_milestones
        .milestones
        .iter()
        .find(|milestone| milestone.id == debt.active_milestone_id)
    else {
        game_state.debt = Some(debt);
        return;
    };

    let is_final_milestone = milestone.next_milestone_id.is_none();
    let early_settlement_reserve = if is_final_milestone {
        total_scheduled_debt(data) / 10
    } else {
        0
    };
    let can_settle_early = is_final_milestone
        && game_state.resources.gold
            >= debt
                .current_balance_due
                .saturating_add(early_settlement_reserve);
    let should_collect_payment = game_state.resources.gold >= debt.current_balance_due
        && (debt.days_until_due <= 1 || debt.missed_payment_count > 0 || can_settle_early);

    if should_collect_payment {
        game_state.resources.gold -= debt.current_balance_due;
        game_state.resources.tower_materials += milestone.reward.tower_materials;
        game_state.resources.relics += milestone.reward.relics;
        game_state.resources.arcane_residue += milestone.reward.arcane_residue;
        debt.last_resolution = Some(if debt.missed_payment_count > 0 {
            DebtResolution::PaidLate
        } else {
            DebtResolution::PaidOnTime
        });
        debt.resolved_milestone_ids.push(milestone.id.clone());
        debt_updates.push(story_text(
            &data.story_events.debt_paid_update_template,
            &[
                ("{name}", milestone.name.clone()),
                ("{gold}", debt.current_balance_due.to_string()),
            ],
        ));
        event_lines.push(story_text(
            &data.story_events.debt_paid_event_template,
            &[
                ("{name}", milestone.name.clone()),
                ("{gold}", debt.current_balance_due.to_string()),
            ],
        ));

        if let Some(next_milestone_id) = &milestone.next_milestone_id {
            if let Some(next_milestone) = data
                .debt_milestones
                .milestones
                .iter()
                .find(|entry| &entry.id == next_milestone_id)
            {
                debt.active_milestone_id = next_milestone.id.clone();
                debt.current_balance_due = next_milestone.amount_due;
                debt.days_until_due = next_milestone.days_allowed;
                debt.missed_payment_count = 0;
                debt.status_message = story_text(
                    &data.story_events.debt_status_due_template,
                    &[
                        ("{name}", next_milestone.name.clone()),
                        ("{days}", next_milestone.days_allowed.to_string()),
                    ],
                );
                debt_updates.push(story_text(
                    &data.story_events.debt_next_due_update_template,
                    &[
                        ("{name}", next_milestone.name.clone()),
                        ("{days}", next_milestone.days_allowed.to_string()),
                    ],
                ));
                event_lines.push(story_text(
                    &data.story_events.debt_next_due_event_template,
                    &[
                        ("{name}", next_milestone.name.clone()),
                        ("{days}", next_milestone.days_allowed.to_string()),
                    ],
                ));
                game_state.debt = Some(debt);
            } else {
                game_state.debt = None;
            }
        } else {
            event_lines.push(data.story_events.debt_cleared_event.clone());
            game_state.debt = None;
        }
    } else if debt.days_until_due == 0 {
        debt.last_resolution = Some(DebtResolution::Missed);
        debt.missed_payment_count += 1;
        let penalty_gold = escalating_miss_cost(
            milestone.failure_penalty_gold,
            debt.missed_payment_count,
            data.debt_milestones.miss_penalty_growth_pct,
        );
        let stress = escalating_miss_cost(
            milestone.failure_stress_flat,
            debt.missed_payment_count,
            data.debt_milestones.miss_penalty_growth_pct,
        );
        debt.current_balance_due = debt.current_balance_due.saturating_add(penalty_gold);
        debt.days_until_due = data.debt_milestones.miss_grace_days;
        let missed_update = story_text(
            &data.story_events.debt_missed_update_template,
            &[
                ("{name}", milestone.name.clone()),
                ("{gold}", debt.current_balance_due.to_string()),
                ("{days}", debt.days_until_due.to_string()),
            ],
        );
        debt_updates.push(missed_update);
        game_state.resources.gold = game_state.resources.gold.saturating_sub(penalty_gold);
        for monster in &mut game_state.monsters {
            monster.stress = monster.stress.saturating_add(stress);
        }
        event_lines.push(story_text(
            &data.story_events.debt_missed_event_template,
            &[
                ("{name}", milestone.name.clone()),
                ("{gold}", penalty_gold.to_string()),
            ],
        ));
        roster_updates.push(story_text(
            &data.story_events.debt_missed_stress_template,
            &[("{stress}", stress.to_string())],
        ));

        if debt.missed_payment_count >= data.debt_milestones.maximum_consecutive_misses {
            debt.days_until_due = 0;
            debt.status_message = story_text(
                &data.story_events.debt_foreclosed_status_template,
                &[
                    ("{day}", game_state.current_day.to_string()),
                    ("{misses}", debt.missed_payment_count.to_string()),
                ],
            );
            let reason = story_text(
                &data.story_events.debt_foreclosed_event_template,
                &[
                    ("{name}", milestone.name.clone()),
                    ("{misses}", debt.missed_payment_count.to_string()),
                ],
            );
            debt_updates.push(debt.status_message.clone());
            event_lines.push(reason.clone());
            game_state.campaign_failure = Some(CampaignFailureState {
                day: game_state.current_day,
                reason,
            });
        } else {
            debt.status_message = story_text(
                &data.story_events.debt_missed_status_template,
                &[
                    ("{gold}", debt.current_balance_due.to_string()),
                    ("{days}", debt.days_until_due.to_string()),
                ],
            );
        }
        game_state.debt = Some(debt);
    } else {
        debt.status_message = story_text(
            &data.story_events.debt_status_due_template,
            &[
                ("{name}", milestone.name.clone()),
                ("{days}", debt.days_until_due.to_string()),
            ],
        );
        debt_updates.push(story_text(
            &data.story_events.debt_initialize_log_template,
            &[
                ("{gold}", debt.current_balance_due.to_string()),
                ("{days}", debt.days_until_due.to_string()),
            ],
        ));
        game_state.debt = Some(debt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::test_game_data;
    use crate::state::{
        CompanionState, GameState, OpeningChapterStep, PlayerTownState, ResourcesState,
        StoryProgressState,
    };

    #[test]
    fn missed_debt_payment_adds_penalty_and_stress() {
        let data = test_game_data();
        let mut game_state = GameState {
            current_day: 4,
            resources: ResourcesState {
                gold: 0,
                ..ResourcesState::default()
            },
            town: PlayerTownState::default(),
            monsters: vec![CompanionState::default()],
            story_progress: StoryProgressState {
                opening_step: OpeningChapterStep::Complete,
                first_client_completed: true,
                ..StoryProgressState::default()
            },
            ..GameState::default()
        };
        initialize_first_debt(&data, &mut game_state).expect("first debt should initialize");
        if let Some(debt) = &mut game_state.debt {
            debt.days_until_due = 1;
        }

        let mut debt_updates = Vec::new();
        let mut event_lines = Vec::new();
        let mut roster_updates = Vec::new();

        resolve_debt_cycle(
            &data,
            &mut game_state,
            &mut debt_updates,
            &mut event_lines,
            &mut roster_updates,
        );

        let debt = game_state.debt.expect("missed debt should remain active");
        assert!(debt.current_balance_due > 120);
        assert_eq!(debt.days_until_due, data.debt_milestones.miss_grace_days);
        assert!(game_state.monsters[0].stress > 0);
        assert!(debt_updates.iter().any(|line| line.contains("Missed")));
    }

    #[test]
    fn repeated_misses_escalate_until_the_creditors_foreclose() {
        let data = test_game_data();
        let mut game_state = GameState {
            current_day: 42,
            town: PlayerTownState::default(),
            monsters: vec![CompanionState::default()],
            story_progress: StoryProgressState {
                opening_step: OpeningChapterStep::Complete,
                first_client_completed: true,
                ..StoryProgressState::default()
            },
            ..GameState::default()
        };
        initialize_first_debt(&data, &mut game_state).expect("first debt should initialize");

        let mut balance = game_state
            .debt
            .as_ref()
            .expect("debt should be active")
            .current_balance_due;
        let mut penalties = Vec::new();
        for miss in 1..=data.debt_milestones.maximum_consecutive_misses {
            game_state
                .debt
                .as_mut()
                .expect("debt should remain visible")
                .days_until_due = 1;
            resolve_debt_cycle(
                &data,
                &mut game_state,
                &mut Vec::new(),
                &mut Vec::new(),
                &mut Vec::new(),
            );
            let debt = game_state
                .debt
                .as_ref()
                .expect("debt should remain visible");
            penalties.push(debt.current_balance_due - balance);
            balance = debt.current_balance_due;
            assert_eq!(debt.missed_payment_count, miss);
        }

        assert_eq!(penalties, vec![25, 50, 75, 100, 125]);
        assert_eq!(game_state.monsters[0].stress, 60);
        let failure = game_state
            .campaign_failure
            .as_ref()
            .expect("the authored maximum consecutive miss should foreclose");
        assert_eq!(failure.day, 42);
        assert!(failure.reason.contains("campaign can no longer advance"));

        let frozen_day = game_state.current_day;
        let summary = crate::engine::resolve_day(&data, &mut game_state);
        assert_eq!(summary.resolved_day, frozen_day);
        assert_eq!(game_state.current_day, frozen_day);
        assert!(pay_debt_now(&data, &mut game_state).is_err());
    }

    #[test]
    fn late_debt_collects_once_player_can_cover_balance() {
        let data = test_game_data();
        let mut game_state = GameState {
            current_day: 6,
            resources: ResourcesState {
                gold: 130,
                ..ResourcesState::default()
            },
            town: PlayerTownState::default(),
            monsters: vec![CompanionState::default()],
            story_progress: StoryProgressState {
                opening_step: OpeningChapterStep::Complete,
                first_client_completed: true,
                ..StoryProgressState::default()
            },
            ..GameState::default()
        };
        initialize_first_debt(&data, &mut game_state).expect("first debt should initialize");
        if let Some(debt) = &mut game_state.debt {
            debt.current_balance_due = 130;
            debt.days_until_due = data.debt_milestones.miss_grace_days;
            debt.missed_payment_count = 1;
        }

        let mut debt_updates = Vec::new();
        let mut event_lines = Vec::new();
        let mut roster_updates = Vec::new();

        resolve_debt_cycle(
            &data,
            &mut game_state,
            &mut debt_updates,
            &mut event_lines,
            &mut roster_updates,
        );

        let debt = game_state
            .debt
            .expect("next debt milestone should become active after late payment");
        assert_eq!(game_state.resources.gold, 0);
        assert_eq!(debt.missed_payment_count, 0);
        assert!(matches!(
            debt.last_resolution,
            Some(DebtResolution::PaidLate)
        ));
        assert_ne!(
            debt.active_milestone_id,
            data.debt_milestones.first_milestone_id
        );
        assert!(debt_updates.iter().any(|line| line.contains("Paid")));
    }

    #[test]
    fn final_debt_settles_early_only_after_reserve_target() {
        let data = test_game_data();
        let final_milestone = data
            .debt_milestones
            .milestones
            .iter()
            .find(|milestone| milestone.next_milestone_id.is_none())
            .expect("test data should have final debt milestone");
        let reserve_target = total_scheduled_debt(&data) / 10;
        let mut game_state = GameState {
            current_day: 300,
            resources: ResourcesState {
                gold: final_milestone.amount_due + reserve_target,
                ..ResourcesState::default()
            },
            town: PlayerTownState::default(),
            monsters: vec![CompanionState::default()],
            story_progress: StoryProgressState {
                opening_step: OpeningChapterStep::Complete,
                first_client_completed: true,
                ..StoryProgressState::default()
            },
            ..GameState::default()
        };
        game_state.debt = Some(DebtState {
            active_milestone_id: final_milestone.id.clone(),
            current_balance_due: final_milestone.amount_due,
            days_until_due: 30,
            missed_payment_count: 0,
            resolved_milestone_ids: Vec::new(),
            last_resolution: None,
            status_message: String::new(),
        });

        let mut debt_updates = Vec::new();
        let mut event_lines = Vec::new();
        let mut roster_updates = Vec::new();

        resolve_debt_cycle(
            &data,
            &mut game_state,
            &mut debt_updates,
            &mut event_lines,
            &mut roster_updates,
        );

        assert!(game_state.debt.is_none());
        assert_eq!(game_state.resources.gold, reserve_target);
    }

    #[test]
    fn manual_debt_payment_advances_active_milestone() {
        let data = test_game_data();
        let mut game_state = GameState {
            current_day: 12,
            resources: ResourcesState {
                gold: 250,
                ..ResourcesState::default()
            },
            town: PlayerTownState::default(),
            monsters: vec![CompanionState::default()],
            story_progress: StoryProgressState {
                opening_step: OpeningChapterStep::Complete,
                first_client_completed: true,
                ..StoryProgressState::default()
            },
            ..GameState::default()
        };
        initialize_first_debt(&data, &mut game_state).expect("first debt should initialize");

        pay_debt_now(&data, &mut game_state).expect("manual payment should succeed");

        let debt = game_state
            .debt
            .expect("manual payment should advance to next milestone");
        assert_eq!(game_state.resources.gold, 0);
        assert_ne!(
            debt.active_milestone_id,
            data.debt_milestones.first_milestone_id
        );
        assert!(game_state
            .event_log
            .iter()
            .any(|entry| entry.contains("paid")));
    }
}
