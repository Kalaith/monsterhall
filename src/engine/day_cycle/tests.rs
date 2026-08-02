use super::*;
use crate::data::{GuildRoomData, MissionData, TowerFloorData};
use crate::state::{
    CompanionJobState, CompanionSkillState, CompanionState, CompanionWorkHistoryState,
    ContractState, ContractStatus, EggIncubationState, ExpeditionPriority, ExpeditionState,
    GameState, OpeningChapterStep, PlayerTownState, ResourcesState, StoryProgressState,
};

#[test]
fn golemkin_room_and_trait_add_corruption() {
    let room = GuildRoomData {
        id: "packroom_annex".to_owned(),
        name: "golemkin Pit".to_owned(),
        description: String::new(),
        service_summary: "Test service".to_owned(),
        required_building_ids: Vec::new(),
        service_tier: 2,
        base_gold_yield: 1,
        base_residue_yield: 1,
        base_materials_yield: 0,
        reputation_yield: 0,
        stamina_cost: 1,
        patron_tiers: vec!["local_delvers".to_owned()],
        trained_skill_ids: vec!["crafting".to_owned(), "charm".to_owned()],
        work_history_gains: crate::data::CompanionWorkHistoryProgressionData {
            scouting_runs: 0,
            craft_jobs: 1,
            contracts_completed: 0,
            recovery_shifts: 1,
            hatchery_assists: 0,
            ..crate::data::CompanionWorkHistoryProgressionData::default()
        },
        work_history_gain_chance_pct: crate::data::CompanionWorkHistoryProgressionData::default(),
        preferred_trait_ids: Vec::new(),
        preferred_species_ids: Vec::new(),
        strategic_niche: None,
        upgrade_building_ids: Vec::new(),
        fatigue_modifier: 0,
        stress_modifier: 0,
        corruption_pressure: 0,
        guest_appeal: 0,
        job_kind: String::new(),
        preparation_quality_bonus: 0,
        recovery_bonus: 0,
    };
    let monster = test_monster(vec!["corruption_tuned".to_owned()]);

    assert_eq!(guild_job_instability_gain(&room, &monster), 3);
}

#[test]
fn scout_route_rewards_more_corruption() {
    let floor = TowerFloorData {
        id: "floor_4".to_owned(),
        name: "Heart Vault".to_owned(),
        depth: 4,
        description: String::new(),
        difficulty: 10,
        requires_building_ids: Vec::new(),
        requires_surveyed_floor_ids: Vec::new(),
        required_surveys: 1,
        required_roster: Vec::new(),
        mission_ids: vec!["scout_route".to_owned()],
        baseline_rewards: ResourceAmountData::default(),
        egg_species_entries: Vec::new(),
        relic_drop_ids: Vec::new(),
        hazard_tags: Vec::new(),
        egg_grade_bonus: 0,
        corruption_pressure: 0,
    };
    let mission = MissionData {
        id: "scout_route".to_owned(),
        name: "Corruption Dive".to_owned(),
        description: String::new(),
        reward_focus: "residue".to_owned(),
        prep_cost: ResourceAmountData::default(),
        success_bonus_pct: 0,
        materials_multiplier_pct: 100,
        residue_multiplier_pct: 100,
        egg_bonus_flat: 0,
        relic_bonus_flat: 0,
        injury_risk_pct: 0,
        preferred_role: None,
        egg_grade_bonus: 0,
        hazard_risk_modifier_pct: 0,
        survey_value: 1,
    };
    let monster = test_monster(vec!["corruption_tuned".to_owned()]);

    assert_eq!(expedition_corruption_gain(&floor, &mission, &monster), 7);
}

#[test]
fn removing_last_monster_clears_expedition() {
    let mut game_state = GameState {
        current_day: 1,
        resources: ResourcesState {
            gold: 0,
            tower_materials: 0,
            eggs: 0,
            relics: 0,
            arcane_residue: 0,
        },
        town: PlayerTownState {
            constructed_building_ids: Vec::new(),
            unlocked_room_ids: vec!["common_room".to_owned()],
            unlocked_floor_ids: vec!["floor_1".to_owned()],
            unlocked_species_ids: vec!["slime_companion".to_owned()],
            patron_tiers: vec!["local_delvers".to_owned()],
            completed_project_ids: Vec::new(),
            floor_surveys: Vec::new(),
            active_situations: Vec::new(),
            party_size: 3,
            town_job_limit: 2,
        },
        egg_inventory: Vec::new(),
        debt: None,
        active_contracts: Vec::new(),
        monsters: vec![test_monster(Vec::new())],
        active_expedition: Some(ExpeditionState {
            expedition_id: "expedition_001".to_owned(),
            floor_id: "floor_1".to_owned(),
            mission_id: "resource_run".to_owned(),
            priority: ExpeditionPriority::Balanced,
            assigned_monster_ids: vec!["monster_001".to_owned()],
            started_day: 1,
        }),
        resolved_contracts: Vec::new(),
        story_progress: StoryProgressState {
            opening_step: OpeningChapterStep::Complete,
            tower_hole_discovered: true,
            first_egg_created: true,
            first_companion_hatched: true,
            hatched_species_ids: Vec::new(),
            first_room_built: true,
            first_client_completed: true,
            first_creditor_visit_seen: false,
            first_special_guest_seen: false,
        },
        event_log: Vec::new(),
    };

    remove_monster_from_expedition(&mut game_state, "monster_001");

    assert!(game_state.active_expedition.is_none());
}

#[test]
fn release_monster_clears_assignments_without_emptying_roster() {
    let mut retained_monster = test_monster(Vec::new());
    retained_monster.id = "monster_001".to_owned();
    retained_monster.name = "Mira".to_owned();
    let mut released_monster = test_monster(Vec::new());
    released_monster.id = "monster_002".to_owned();
    released_monster.name = "Liora".to_owned();
    released_monster.current_job = CompanionJobState::OnExpedition {
        expedition_id: "expedition_001".to_owned(),
    };
    let mut game_state = GameState {
        current_day: 1,
        resources: ResourcesState::default(),
        town: PlayerTownState {
            constructed_building_ids: Vec::new(),
            unlocked_room_ids: vec!["common_room".to_owned()],
            unlocked_floor_ids: vec!["floor_1".to_owned()],
            unlocked_species_ids: vec!["slime_companion".to_owned()],
            patron_tiers: vec!["local_delvers".to_owned()],
            completed_project_ids: Vec::new(),
            floor_surveys: Vec::new(),
            active_situations: Vec::new(),
            party_size: 3,
            town_job_limit: 2,
        },
        egg_inventory: Vec::new(),
        debt: None,
        active_contracts: vec![ContractState {
            request_id: "contract_001".to_owned(),
            status: ContractStatus::Accepted,
            assigned_monster_id: Some("monster_002".to_owned()),
            ..ContractState::default()
        }],
        monsters: vec![retained_monster, released_monster],
        active_expedition: Some(ExpeditionState {
            expedition_id: "expedition_001".to_owned(),
            floor_id: "floor_1".to_owned(),
            mission_id: "resource_run".to_owned(),
            priority: ExpeditionPriority::Balanced,
            assigned_monster_ids: vec!["monster_002".to_owned()],
            started_day: 1,
        }),
        resolved_contracts: Vec::new(),
        story_progress: StoryProgressState::default(),
        event_log: Vec::new(),
    };

    let message = release_monster(&mut game_state, "monster_002").expect("release should work");

    assert_eq!(message, "Liora left Monsterhall.");
    assert_eq!(game_state.monsters.len(), 1);
    assert!(game_state.active_expedition.is_none());
    assert!(matches!(
        game_state.active_contracts[0].status,
        ContractStatus::Pending
    ));
    assert!(game_state.active_contracts[0].assigned_monster_id.is_none());
    assert!(release_monster(&mut game_state, "monster_001").is_err());
}

#[test]
fn trained_room_skills_add_guild_job_bonus() {
    let room = GuildRoomData {
        id: "common_room".to_owned(),
        name: "Vanilla Suite".to_owned(),
        description: String::new(),
        service_summary: "Soft service".to_owned(),
        required_building_ids: Vec::new(),
        service_tier: 1,
        base_gold_yield: 30,
        base_residue_yield: 6,
        base_materials_yield: 0,
        reputation_yield: 0,
        stamina_cost: 10,
        patron_tiers: vec!["local_delvers".to_owned()],
        trained_skill_ids: vec![
            "scouting".to_owned(),
            "hospitality".to_owned(),
            "charm".to_owned(),
        ],
        work_history_gains: crate::data::CompanionWorkHistoryProgressionData {
            scouting_runs: 1,
            hospitality_jobs: 1,
            contracts_completed: 1,
            recovery_shifts: 0,
            hatchery_assists: 0,
            ..crate::data::CompanionWorkHistoryProgressionData::default()
        },
        work_history_gain_chance_pct: crate::data::CompanionWorkHistoryProgressionData::default(),
        preferred_trait_ids: Vec::new(),
        preferred_species_ids: vec!["slime_companion".to_owned()],
        strategic_niche: None,
        upgrade_building_ids: Vec::new(),
        fatigue_modifier: 0,
        stress_modifier: 0,
        corruption_pressure: 0,
        guest_appeal: 0,
        job_kind: String::new(),
        preparation_quality_bonus: 0,
        recovery_bonus: 0,
    };

    let novice = test_monster(Vec::new());
    let mut trained = test_monster(Vec::new());
    trained.skills.scouting = 3;
    trained.skills.hospitality = 4;
    trained.skills.charm = 2;

    assert_eq!(guild_job_skill_bonus(&novice, &room), 0);
    assert_eq!(guild_job_skill_bonus(&trained, &room), 12);
}

#[test]
fn incubating_and_hatching_use_egg_inventory() {
    let mut game_state = GameState {
        current_day: 1,
        resources: ResourcesState {
            gold: 0,
            tower_materials: 0,
            eggs: 0,
            relics: 0,
            arcane_residue: 0,
        },
        town: PlayerTownState {
            constructed_building_ids: Vec::new(),
            unlocked_room_ids: Vec::new(),
            unlocked_floor_ids: vec!["floor_1".to_owned()],
            unlocked_species_ids: vec!["slime_companion".to_owned()],
            patron_tiers: vec!["local_delvers".to_owned()],
            completed_project_ids: Vec::new(),
            floor_surveys: Vec::new(),
            active_situations: Vec::new(),
            party_size: 3,
            town_job_limit: 2,
        },
        egg_inventory: Vec::new(),
        debt: None,
        active_contracts: Vec::new(),
        monsters: Vec::new(),
        active_expedition: None,
        resolved_contracts: Vec::new(),
        story_progress: StoryProgressState {
            opening_step: OpeningChapterStep::Complete,
            tower_hole_discovered: true,
            first_egg_created: true,
            first_companion_hatched: false,
            hatched_species_ids: Vec::new(),
            first_room_built: false,
            first_client_completed: false,
            first_creditor_visit_seen: false,
            first_special_guest_seen: false,
        },
        event_log: Vec::new(),
    };

    create_opening_egg(&mut game_state, "slime_companion");
    assert_eq!(game_state.resources.eggs, 1);
    assert_eq!(raw_egg_count_for_species(&game_state, "slime_companion"), 1);

    let mut egg = game_state.egg_inventory[0].clone();
    egg.selected_species_id = Some("slime_companion".to_owned());
    egg.incubation_state = EggIncubationState::ReadyToHatch;
    egg.loyalty_imprinted = true;
    game_state.egg_inventory[0] = egg;

    assert_eq!(
        ready_egg_count_for_species(&game_state, "slime_companion"),
        1
    );
}

#[test]
fn a_rested_companion_works_at_full_rate() {
    let data = crate::data::test_game_data();
    let monster = test_monster(Vec::new());

    assert_eq!(
        companion_effectiveness_pct(&data.config.day_cycle, &monster),
        100
    );
}

#[test]
fn condition_allowances_absorb_a_single_shift() {
    let data = crate::data::test_game_data();
    let effects = &data.config.day_cycle.condition_effects;
    let mut monster = test_monster(Vec::new());
    monster.fatigue = effects.fatigue_allowance;
    monster.stress = effects.stress_allowance;

    assert_eq!(
        companion_effectiveness_pct(&data.config.day_cycle, &monster),
        100,
        "one honest day's work must not dent the payout"
    );
}

#[test]
fn sustained_condition_damage_costs_output() {
    let data = crate::data::test_game_data();
    let mut monster = test_monster(Vec::new());
    monster.fatigue = data.config.day_cycle.condition_effects.fatigue_allowance + 50;

    let worn = companion_effectiveness_pct(&data.config.day_cycle, &monster);
    assert!(worn < 100, "fatigue past the allowance must cost output");

    monster.injury = 6;
    assert!(
        companion_effectiveness_pct(&data.config.day_cycle, &monster) < worn,
        "injury must stack on top of fatigue"
    );
}

#[test]
fn effectiveness_never_falls_through_its_floor() {
    let data = crate::data::test_game_data();
    let mut monster = test_monster(Vec::new());
    monster.fatigue = 5_000;
    monster.stress = 5_000;
    monster.injury = 5_000;

    assert_eq!(
        companion_effectiveness_pct(&data.config.day_cycle, &monster),
        data.config
            .day_cycle
            .condition_effects
            .min_effectiveness_pct,
        "even a wreck still shows up for the shift"
    );
}

#[test]
fn a_worn_down_worker_earns_the_guild_less() {
    let data = crate::data::test_game_data();
    let room = data
        .guild_rooms
        .rooms
        .first()
        .expect("at least one guild room is authored");
    let town = PlayerTownState {
        constructed_building_ids: Vec::new(),
        unlocked_room_ids: vec![room.id.clone()],
        unlocked_floor_ids: Vec::new(),
        unlocked_species_ids: vec!["slime_companion".to_owned()],
        patron_tiers: room.patron_tiers.clone(),
        completed_project_ids: Vec::new(),
        floor_surveys: Vec::new(),
        active_situations: Vec::new(),
        party_size: 3,
        town_job_limit: 2,
    };
    let building_bonus = BuildingAggregate::default();

    let fresh = test_monster(Vec::new());
    let mut worn = test_monster(Vec::new());
    worn.fatigue = 200;
    worn.stress = 120;
    worn.injury = 30;

    let fresh_preview = preview_guild_job_for_town(&data, &town, &building_bonus, &fresh, &room.id)
        .expect("preview");
    let worn_preview = preview_guild_job_for_town(&data, &town, &building_bonus, &worn, &room.id)
        .expect("preview");

    assert_eq!(fresh_preview.effectiveness_pct, 100);
    assert!(worn_preview.effectiveness_pct < 100);
    assert!(
        worn_preview.projected_gold < fresh_preview.projected_gold,
        "a burned-out escort must not earn what a rested one does"
    );
    assert!(worn_preview.preparation_quality <= fresh_preview.preparation_quality);
}

#[test]
fn a_room_only_rolls_the_work_it_can_actually_bank() {
    let data = crate::data::test_game_data();

    for room in &data.guild_rooms.rooms {
        let max = &room.work_history_gains;
        let chance = &room.work_history_gain_chance_pct;
        for (label, max_gain, chance_pct) in [
            ("scouting_runs", max.scouting_runs, chance.scouting_runs),
            ("guard_duties", max.guard_duties, chance.guard_duties),
            (
                "hospitality_jobs",
                max.hospitality_jobs,
                chance.hospitality_jobs,
            ),
            ("craft_jobs", max.craft_jobs, chance.craft_jobs),
            (
                "contracts_completed",
                max.contracts_completed,
                chance.contracts_completed,
            ),
            (
                "recovery_shifts",
                max.recovery_shifts,
                chance.recovery_shifts,
            ),
            (
                "hatchery_assists",
                max.hatchery_assists,
                chance.hatchery_assists,
            ),
        ] {
            assert!(
                chance_pct <= 100,
                "{} authors {label} at {chance_pct}%, which is not a probability",
                room.id
            );
            // A category with odds but no ceiling is a content mistake that reads
            // as working: the preview would quote odds for work the room can
            // never bank.
            if chance_pct > 0 {
                assert!(
                    max_gain > 0,
                    "{} gives {label} a {chance_pct}% chance but banks none of it",
                    room.id
                );
            }
        }
    }
}

#[test]
fn a_resolved_contract_never_re_enters_the_workload() {
    let data = crate::data::test_game_data();
    let mut game_state = crate::engine::create_new_game_state(&data);
    let mut monster = test_monster(Vec::new());
    monster.id = "monster_001".to_owned();
    let mut keeper = test_monster(Vec::new());
    keeper.id = "monster_002".to_owned();
    game_state.monsters = vec![monster, keeper];
    game_state.resolved_contracts = vec![ContractState {
        request_id: "contract_done".to_owned(),
        status: ContractStatus::Completed,
        assigned_monster_id: Some("monster_001".to_owned()),
        ..ContractState::default()
    }];

    // The whole reason resolved contracts live in their own list: the offer
    // limit, the request-id sequence and the booking policy all count
    // `active_contracts`, and a finished booking sitting in it moves every one
    // of them.
    assert!(
        game_state
            .active_contracts
            .iter()
            .all(|request| request.status.is_live()),
        "active_contracts must only ever hold live work"
    );
    assert_eq!(
        game_state.live_contract_count(),
        game_state.active_contracts.len()
    );

    // And releasing the companion afterwards must not put it back on the desk.
    release_monster(&mut game_state, "monster_001").expect("release should work");
    assert!(matches!(
        game_state.resolved_contracts[0].status,
        ContractStatus::Completed
    ));
}

fn test_monster(trait_ids: Vec<String>) -> CompanionState {
    CompanionState {
        id: "monster_001".to_owned(),
        species_id: "slime_companion".to_owned(),
        name: "Mira".to_owned(),
        quality_rank: 1,
        stats: crate::data::StatBlockData {
            power: 3,
            charm: 4,
            endurance: 5,
            instinct: 4,
        },
        trait_ids,
        current_job: CompanionJobState::Idle,
        skills: CompanionSkillState::default(),
        work_history: CompanionWorkHistoryState::default(),
        fatigue: 0,
        stress: 0,
        injury: 0,
        corruption: 0,
        bond: 0,
        reputation: 0,
    }
}
