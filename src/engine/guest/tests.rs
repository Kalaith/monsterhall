use super::*;
use crate::data::test_game_data;
use crate::state::{
    CompanionSkillState, CompanionWorkHistoryState, GameState, OpeningChapterStep, PlayerTownState,
    ResourcesState, StoryProgressState,
};

#[test]
fn guest_eligibility_rejects_wrong_species_and_missing_room() {
    let data = test_game_data();
    let game_state = GameState {
        current_day: 4,
        resources: ResourcesState::default(),
        town: PlayerTownState {
            unlocked_room_ids: vec!["common_room".to_owned()],
            unlocked_species_ids: vec!["slime_companion".to_owned(), "residue_slime".to_owned()],
            ..PlayerTownState::default()
        },
        story_progress: StoryProgressState {
            opening_step: OpeningChapterStep::Complete,
            first_client_completed: true,
            ..StoryProgressState::default()
        },
        ..GameState::default()
    };
    let request = ContractState {
        request_id: "contract_001".to_owned(),
        template_id: "residue_consultation".to_owned(),
        category: String::new(),
        patron_tier_id: None,
        guest_name: "Veiled Patron".to_owned(),
        archetype_id: "tower_scholar".to_owned(),
        requested_room_id: "reception_hall".to_owned(),
        required_species_ids: vec!["residue_slime".to_owned()],
        minimum_quality_rank: 1,
        required_skill_thresholds: ContractSkillRequirementState {
            charm: 2,
            ..ContractSkillRequirementState::default()
        },
        required_work_history_thresholds: ContractHistoryRequirementState::default(),
        reward: ResourcesState::default(),
        penalty_gold: 10,
        deadline_day: 6,
        preparation_quality_required: 0,
        preparation_quality_bonus: 0,
        status: ContractStatus::Pending,
        assigned_monster_id: None,
        chain_depth: 0,
    };
    let monster = CompanionState {
        id: "monster_001".to_owned(),
        species_id: "slime_companion".to_owned(),
        name: "Mira".to_owned(),
        skills: CompanionSkillState {
            charm: 2,
            ..CompanionSkillState::default()
        },
        ..CompanionState::default()
    };

    let report = evaluate_contract_eligibility(&data, &game_state, &request, &monster);

    assert!(!report.is_eligible);
    assert!(report
        .failure_reasons
        .iter()
        .any(|reason| reason.contains("Requires Reception Hall.")));
    assert!(report
        .failure_reasons
        .iter()
        .any(|reason| reason.contains("Requires Residue Slime.")));
}

#[test]
fn guest_eligibility_accepts_trained_matching_specialist() {
    let data = test_game_data();
    let game_state = GameState {
        current_day: 4,
        resources: ResourcesState::default(),
        town: PlayerTownState {
            unlocked_room_ids: vec!["nursery_wing".to_owned()],
            unlocked_species_ids: vec!["lamia_routekeeper".to_owned()],
            ..PlayerTownState::default()
        },
        story_progress: StoryProgressState {
            opening_step: OpeningChapterStep::Complete,
            first_client_completed: true,
            ..StoryProgressState::default()
        },
        ..GameState::default()
    };
    let request = ContractState {
        request_id: "contract_002".to_owned(),
        template_id: "lamia_binding_rite".to_owned(),
        category: String::new(),
        patron_tier_id: None,
        guest_name: "Veiled Patron".to_owned(),
        archetype_id: "tower_scholar".to_owned(),
        requested_room_id: "nursery_wing".to_owned(),
        required_species_ids: vec!["lamia_routekeeper".to_owned()],
        minimum_quality_rank: 1,
        required_skill_thresholds: ContractSkillRequirementState {
            scouting: 1,
            hospitality: 2,
            charm: 2,
            ..ContractSkillRequirementState::default()
        },
        required_work_history_thresholds: ContractHistoryRequirementState {
            scouting_runs: 1,
            hospitality_jobs: 2,
            contracts_completed: 1,
            ..ContractHistoryRequirementState::default()
        },
        reward: ResourcesState::default(),
        penalty_gold: 10,
        deadline_day: 6,
        preparation_quality_required: 0,
        preparation_quality_bonus: 0,
        status: ContractStatus::Pending,
        assigned_monster_id: None,
        chain_depth: 0,
    };
    let monster = CompanionState {
        id: "monster_001".to_owned(),
        species_id: "lamia_routekeeper".to_owned(),
        name: "Sesh".to_owned(),
        quality_rank: 2,
        skills: CompanionSkillState {
            scouting: 1,
            hospitality: 2,
            charm: 2,
            ..CompanionSkillState::default()
        },
        work_history: CompanionWorkHistoryState {
            scouting_runs: 1,
            hospitality_jobs: 2,
            contracts_completed: 1,
            ..CompanionWorkHistoryState::default()
        },
        ..CompanionState::default()
    };

    let report = evaluate_contract_eligibility(&data, &game_state, &request, &monster);

    assert!(report.is_eligible);
    assert!(report.failure_reasons.is_empty());
}

/// Every work-history category the contract desk can refuse a booking over must
/// name itself in the guild's current vocabulary.
///
/// Two of these labels were still "Kiss Count" and "Birth Count" from the
/// premise this game was reskinned from — the only place the retired wording
/// survived, and it was on the player's screen. They are string arguments rather
/// than content ids, so the rename pass and every id validation walked straight
/// past them.
#[test]
fn no_work_history_label_carries_the_retired_vocabulary() {
    use super::eligibility::WORK_HISTORY_LABELS;

    let retired = ["kiss", "birth", "girl", "client", "guest request"];
    for (category, label) in WORK_HISTORY_LABELS {
        let lowered = label.to_lowercase();
        for word in retired {
            assert!(
                !lowered.contains(word),
                "'{category}' is shown to the player as '{label}', which is retired vocabulary"
            );
        }
        assert!(
            !label.is_empty(),
            "'{category}' has no label to show the player"
        );
    }
}

/// And each category must be named after the work it actually counts, so a
/// refusal reason cannot point at the wrong meter.
#[test]
fn a_refused_booking_names_the_work_it_is_short_of() {
    let data = crate::data::test_game_data();
    let game_state = crate::engine::create_new_game_state(&data);
    let request = ContractState {
        required_work_history_thresholds: ContractHistoryRequirementState {
            hatchery_assists: 3,
            ..ContractHistoryRequirementState::default()
        },
        ..ContractState::default()
    };
    let monster = CompanionState {
        quality_rank: 1,
        ..CompanionState::default()
    };

    let report = evaluate_contract_eligibility(&data, &game_state, &request, &monster);

    assert!(
        report
            .failure_reasons
            .iter()
            .any(|reason| reason.contains("Hatchery Assists")),
        "a booking short of hatchery work should say so: {:?}",
        report.failure_reasons
    );
}

/// `preparation_quality_required` is authored on 13 of 16 contracts and printed
/// on the desk, and for the game's whole life nothing read it. It cannot gate
/// the booking — a guild books before it staffs, so the figure is zero at that
/// moment — so it settles at resolution: the hall that did not staff up still
/// delivers, for half.
#[test]
fn a_hall_that_did_not_staff_up_delivers_the_booking_for_half() {
    let data = test_game_data();
    let room = data
        .guild_rooms
        .rooms
        .iter()
        .max_by_key(|room| room.preparation_quality_bonus)
        .expect("the catalogue has rooms");
    let required = room.preparation_quality_bonus;
    assert!(
        required > 0,
        "this test needs a room that contributes preparation quality"
    );

    let resolve = |staffed: bool| {
        let mut game_state = GameState {
            current_day: 4,
            town: PlayerTownState {
                unlocked_room_ids: vec![room.id.clone()],
                unlocked_species_ids: vec!["slime_companion".to_owned()],
                ..PlayerTownState::default()
            },
            story_progress: StoryProgressState {
                opening_step: OpeningChapterStep::Complete,
                first_client_completed: true,
                ..StoryProgressState::default()
            },
            ..GameState::default()
        };
        game_state.monsters = vec![
            CompanionState {
                id: "monster_001".to_owned(),
                species_id: "slime_companion".to_owned(),
                name: "Mira".to_owned(),
                quality_rank: 1,
                ..CompanionState::default()
            },
            CompanionState {
                id: "monster_002".to_owned(),
                species_id: "slime_companion".to_owned(),
                name: "Tess".to_owned(),
                quality_rank: 1,
                current_job: if staffed {
                    CompanionJobState::GuildJob {
                        room_id: room.id.clone(),
                    }
                } else {
                    CompanionJobState::Idle
                },
                ..CompanionState::default()
            },
        ];
        game_state.active_contracts = vec![ContractState {
            request_id: "contract_001".to_owned(),
            template_id: "starter_slime_bedding".to_owned(),
            guest_name: "Veiled Patron".to_owned(),
            archetype_id: "tower_scholar".to_owned(),
            requested_room_id: room.id.clone(),
            minimum_quality_rank: 1,
            reward: ResourcesState {
                gold: 100,
                ..ResourcesState::default()
            },
            deadline_day: 6,
            preparation_quality_required: required,
            status: ContractStatus::Accepted,
            assigned_monster_id: Some("monster_001".to_owned()),
            ..ContractState::default()
        }];

        let mut gold = 0;
        let mut residue = 0;
        let (mut updates, mut events, mut roster) = (Vec::new(), Vec::new(), Vec::new());
        resolve_contracts(
            &data,
            &mut game_state,
            &mut gold,
            &mut residue,
            &mut updates,
            &mut events,
            &mut roster,
        );
        (gold, updates)
    };

    let (prepared_gold, prepared_updates) = resolve(true);
    let (short_gold, short_updates) = resolve(false);

    assert!(prepared_gold > 0, "a staffed hall should be paid in full");
    assert_eq!(
        short_gold,
        prepared_gold / 2,
        "an under-prepared hall should be paid half"
    );
    // And it must say so, because a booking that quietly pays half reads as a
    // reward that was authored lower.
    assert!(
        short_updates
            .iter()
            .any(|line| line.contains("under-prepared")),
        "the day's contract updates should name the shortfall: {short_updates:?}"
    );
    assert!(
        !prepared_updates
            .iter()
            .any(|line| line.contains("under-prepared")),
        "a prepared hall should not be told it fell short: {prepared_updates:?}"
    );
}
