use super::*;
use crate::data::GameData;
use crate::state::{
    ChamberState, GameState, OpeningChapterStep, PlayerTownState, ResourcesState, CompanionWorkHistoryState,
    CompanionSkillState, StoryProgressState,
};

#[test]
fn guest_eligibility_rejects_wrong_species_and_missing_room() {
    let data = test_game_data();
    let game_state = GameState {
        current_day: 4,
        resources: ResourcesState::default(),
        town: PlayerTownState {
            unlocked_room_ids: vec!["vanilla_suite".to_owned()],
            unlocked_species_ids: vec!["slime_girl".to_owned(), "succu_slime".to_owned()],
            ..PlayerTownState::default()
        },
        chamber: ChamberState::default(),
        story_progress: StoryProgressState {
            opening_step: OpeningChapterStep::Complete,
            first_client_completed: true,
            ..StoryProgressState::default()
        },
        ..GameState::default()
    };
    let request = ContractState {
        request_id: "guest_request_001".to_owned(),
        template_id: "succu_salon_booking".to_owned(),
        guest_name: "Veiled Patron".to_owned(),
        archetype_id: "veiled_patron".to_owned(),
        requested_room_id: "public_stage".to_owned(),
        required_species_ids: vec!["succu_slime".to_owned()],
        minimum_quality_rank: 1,
        required_skill_thresholds: ContractSkillRequirementState {
            charm: 2,
            ..ContractSkillRequirementState::default()
        },
        required_work_history_thresholds: ContractHistoryRequirementState::default(),
        reward: ResourcesState::default(),
        penalty_gold: 10,
        deadline_day: 6,
        status: ContractStatus::Pending,
        assigned_monster_id: None,
        chain_depth: 0,
        partial_progress: 0,
    };
    let monster = CompanionState {
        id: "monster_001".to_owned(),
        species_id: "slime_girl".to_owned(),
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
        .any(|reason| reason.contains("Requires Public Stage.")));
    assert!(report
        .failure_reasons
        .iter()
        .any(|reason| reason.contains("Requires Succu-Slime.")));
}

#[test]
fn guest_eligibility_accepts_trained_matching_specialist() {
    let data = test_game_data();
    let game_state = GameState {
        current_day: 4,
        resources: ResourcesState::default(),
        town: PlayerTownState {
            unlocked_room_ids: vec!["nursery_wing".to_owned()],
            unlocked_species_ids: vec!["lamia_binder".to_owned()],
            ..PlayerTownState::default()
        },
        chamber: ChamberState::default(),
        story_progress: StoryProgressState {
            opening_step: OpeningChapterStep::Complete,
            first_client_completed: true,
            ..StoryProgressState::default()
        },
        ..GameState::default()
    };
    let request = ContractState {
        request_id: "guest_request_002".to_owned(),
        template_id: "lamia_binding_rite".to_owned(),
        guest_name: "Veiled Patron".to_owned(),
        archetype_id: "veiled_patron".to_owned(),
        requested_room_id: "nursery_wing".to_owned(),
        required_species_ids: vec!["lamia_binder".to_owned()],
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
        status: ContractStatus::Pending,
        assigned_monster_id: None,
        chain_depth: 0,
        partial_progress: 0,
    };
    let monster = CompanionState {
        id: "monster_001".to_owned(),
        species_id: "lamia_binder".to_owned(),
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

fn test_game_data() -> GameData {
    GameData {
        config: parse_json(include_str!("../../../assets/data/config.json")),
        ui_text: parse_json(include_str!("../../../assets/data/ui_text.json")),
        debt_milestones: parse_json(include_str!("../../../assets/data/debt_milestones.json")),
        patron_archetypes: parse_json(include_str!("../../../assets/data/guest_archetypes.json")),
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
