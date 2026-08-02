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
        name: "Packroom Annex".to_owned(),
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
        charm_training_chance_pct: 0,
        charm_training_booking_chance_pct: 0,
        shift_instability_gain: 2,
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

    // The room's own exposure plus what the companion's own tuning attracts.
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
        }),
        resolved_contracts: Vec::new(),
        story_progress: StoryProgressState {
            opening_step: OpeningChapterStep::Complete,
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
        charm_training_chance_pct: 0,
        charm_training_booking_chance_pct: 0,
        shift_instability_gain: 0,
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

/// The guild-hall card quotes what a companion's shift adds to the hall's
/// readiness; the contract desk scores bookings against the town's total. Those
/// were two copies of one formula and only the preview scaled by condition, so
/// resting someone before a demanding booking visibly changed the quoted number
/// and changed nothing that was scored.
#[test]
fn the_prep_quality_quoted_is_the_prep_quality_scored() {
    let data = crate::data::test_game_data();
    let room = data
        .guild_rooms
        .rooms
        .iter()
        .find(|room| room.preparation_quality_bonus > 0)
        .expect("some room must contribute preparation quality");

    let mut worker = test_monster(Vec::new());
    worker.skills.scouting = 6;
    worker.skills.guarding = 4;
    worker.current_job = CompanionJobState::GuildJob {
        room_id: room.id.clone(),
    };

    let mut game_state = crate::engine::create_new_game_state(&data);
    game_state.monsters = vec![worker];
    game_state.active_contracts.clear();
    for tier_id in &room.patron_tiers {
        if !game_state.town.patron_tiers.contains(tier_id) {
            game_state.town.patron_tiers.push(tier_id.clone());
        }
    }

    let building_bonus = collect_building_modifiers(&data, &game_state);
    let quoted = |game_state: &GameState| {
        preview_guild_job_for_town(
            &data,
            &game_state.town,
            &building_bonus,
            &game_state.monsters[0],
            &room.id,
        )
        .expect("preview")
        .preparation_quality
    };
    // The town total carries project and contract bonuses the per-companion
    // figure does not, so the two are compared by how they *move*, not by value.
    let rested_quoted = quoted(&game_state);
    let rested_scored = crate::engine::depth::town_preparation_quality(&data, &game_state);
    assert!(rested_quoted > 0, "test needs a shift worth quoting");

    game_state.monsters[0].fatigue = 220;
    game_state.monsters[0].stress = 140;
    let worn_quoted = quoted(&game_state);
    let worn_scored = crate::engine::depth::town_preparation_quality(&data, &game_state);

    assert!(
        worn_quoted < rested_quoted,
        "wearing a companion down must lower what the hall card quotes"
    );
    assert_eq!(
        rested_quoted - worn_quoted,
        rested_scored - worn_scored,
        "the desk must lose exactly what the card said it would"
    );
}

/// A companion booked onto a contract cannot also be rostered elsewhere.
///
/// `resolve_contracts` runs first and skips everyone it serviced in the job
/// loop, so a double-booked companion did the contract and her other assignment
/// was silently thrown away — while the Guild Hall kept quoting her projected
/// gold and the Expedition Desk kept counting her stats into the party preview.
/// With only two guild-job slots, burning one on work the day cycle discards is
/// half the hall's income for that day.
#[test]
fn a_booked_companion_cannot_be_rostered_for_other_work() {
    let data = crate::data::test_game_data();
    let mut game_state = crate::engine::create_new_game_state(&data);
    let mut worker = test_monster(Vec::new());
    worker.id = "monster_001".to_owned();
    game_state.monsters = vec![worker];
    let room_id = data.guild_rooms.rooms[0].id.clone();
    let floor_id = game_state.town.unlocked_floor_ids[0].clone();

    // Free: both assignments are available.
    assert!(assign_monster_to_room(&mut game_state, "monster_001", &room_id).is_ok());
    assign_monster_to_idle(&mut game_state, "monster_001").expect("idle should clear the room");

    game_state.active_contracts = vec![ContractState {
        request_id: "contract_001".to_owned(),
        status: ContractStatus::Accepted,
        assigned_monster_id: Some("monster_001".to_owned()),
        ..ContractState::default()
    }];

    let room_error = assign_monster_to_room(&mut game_state, "monster_001", &room_id)
        .expect_err("a booked companion must not take a room shift");
    assert!(
        room_error.contains("booked"),
        "the refusal should say why: {room_error}"
    );
    assert!(
        assign_monster_to_expedition(&data, &mut game_state, "monster_001", &floor_id).is_err(),
        "a booked companion must not join an expedition either"
    );

    // Clearing the booking releases her again.
    crate::engine::clear_contract_assignment(&mut game_state, "contract_001")
        .expect("clearing should work");
    assert!(assign_monster_to_room(&mut game_state, "monster_001", &room_id).is_ok());
}

/// Only an *accepted* booking reserves her. A pending offer she has merely been
/// pencilled against must not lock her out of the hall.
#[test]
fn a_pending_offer_does_not_reserve_a_companion() {
    let data = crate::data::test_game_data();
    let mut game_state = crate::engine::create_new_game_state(&data);
    let mut worker = test_monster(Vec::new());
    worker.id = "monster_001".to_owned();
    game_state.monsters = vec![worker];
    game_state.active_contracts = vec![ContractState {
        request_id: "contract_001".to_owned(),
        status: ContractStatus::Pending,
        assigned_monster_id: Some("monster_001".to_owned()),
        ..ContractState::default()
    }];

    let room_id = data.guild_rooms.rooms[0].id.clone();
    assert!(assign_monster_to_room(&mut game_state, "monster_001", &room_id).is_ok());
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

#[test]
fn every_skill_the_data_names_can_actually_be_trained_and_scored() {
    let data = crate::data::test_game_data();
    let mut skills = CompanionSkillState::default();

    // A skill id that reaches `increment_skill` and is not recognised silently
    // trains nothing and renders as "Unknown". Anything a room claims to teach
    // has to survive the whole round trip.
    for room in &data.guild_rooms.rooms {
        for skill_id in &room.trained_skill_ids {
            assert!(
                increment_skill(&mut skills, skill_id, 1),
                "room '{}' trains '{skill_id}', which increment_skill does not recognise",
                room.id
            );
            assert_ne!(
                format_skill_name(skill_id),
                "Unknown",
                "room '{}' trains '{skill_id}', which has no display name",
                room.id
            );
            assert!(
                companion_skill_value(&skills, skill_id) > 0,
                "room '{}' trains '{skill_id}', which companion_skill_value reads as zero",
                room.id
            );
        }
    }
}

#[test]
fn traits_change_what_a_companion_is_worth() {
    let data = crate::data::test_game_data();
    let plain = test_monster(Vec::new());
    let stonebound = test_monster(vec!["stonebound".to_owned()]);

    let plain_stats = crate::engine::effective_stats(&data, &plain);
    let bonus_stats = crate::engine::effective_stats(&data, &stonebound);

    assert_eq!(plain_stats.endurance, plain.stats.endurance);
    assert!(
        bonus_stats.endurance > plain_stats.endurance,
        "stonebound authors an endurance bonus; effective_stats must apply it"
    );
    // Summed on demand rather than baked in at hatch, because mutations grant
    // traits after a companion exists.
    assert_eq!(
        stonebound.stats.endurance, plain.stats.endurance,
        "the bonus must not have been written into the base stats"
    );
}

#[test]
fn a_room_can_only_teach_a_skill_its_own_work_feeds() {
    let data = crate::data::test_game_data();

    // `skill_ids_from_work_history_gains` maps banked work to skills, and
    // `apply_guild_job_progression` filters that against the room's
    // `trained_skill_ids`. A room listing a skill no shift there can bank
    // advertises a lesson it never gives — the intersection is empty and the
    // entry does nothing but inflate `guild_job_skill_bonus`.
    for room in &data.guild_rooms.rooms {
        let bankable =
            skill_ids_from_work_history_gains(&crate::data::CompanionWorkHistoryProgressionData {
                scouting_runs: room.work_history_gains.scouting_runs,
                guard_duties: room.work_history_gains.guard_duties,
                hospitality_jobs: room.work_history_gains.hospitality_jobs,
                craft_jobs: room.work_history_gains.craft_jobs,
                contracts_completed: room.work_history_gains.contracts_completed,
                recovery_shifts: room.work_history_gains.recovery_shifts,
                hatchery_assists: room.work_history_gains.hatchery_assists,
            });

        for skill_id in &room.trained_skill_ids {
            // Charm has its own path through `should_gain_charm` and is not fed
            // by a work-history category.
            if skill_id == "charm" {
                continue;
            }
            assert!(
                bankable.iter().any(|id| id == skill_id),
                "room '{}' trains '{skill_id}' but banks no work that teaches it; it can bank {bankable:?}",
                room.id
            );
        }
    }
}

/// The same promise, checked on charm's own path.
///
/// Charm odds were a `match` on room id in Rust until this pass, so nothing
/// could line them up against `trained_skill_ids` — a room could advertise charm
/// and never teach it, or teach it without saying so, and the only way to find
/// out was to read the match arm.
#[test]
fn charm_odds_and_the_advertised_lesson_agree() {
    let data = crate::data::test_game_data();

    for room in &data.guild_rooms.rooms {
        let advertised = room.trained_skill_ids.iter().any(|id| id == "charm");
        let teaches =
            charm_training_chance_pct(room, false) > 0 || charm_training_chance_pct(room, true) > 0;

        assert_eq!(
            advertised, teaches,
            "room '{}' advertises charm: {advertised}, actually teaches it: {teaches}",
            room.id
        );

        for is_booking in [false, true] {
            assert!(
                charm_training_chance_pct(room, is_booking) <= 100,
                "room '{}' authors charm odds above certainty",
                room.id
            );
        }

        // Closing a booking is where charm is really learned, so no room may
        // teach less of it on a contract than on a quiet shift.
        assert!(
            charm_training_chance_pct(room, true) >= charm_training_chance_pct(room, false),
            "room '{}' teaches less charm on a booking than on an ordinary shift",
            room.id
        );
    }
}

/// Every skill has to count towards what a companion is worth keeping.
///
/// This sum was written out longhand in three places against the five skills
/// that existed at the time. The wage was fixed once; the hatchery's release
/// recommendation and the validation policy's service score were still counting
/// five, and both of those choose **who gets released** — so training recovery
/// or bargaining made a companion cheaper to throw away and more expensive to
/// keep at the same time.
#[test]
fn every_skill_counts_towards_what_a_companion_is_worth() {
    use crate::engine::companion_skill_total;

    let base = CompanionSkillState::default();
    assert_eq!(companion_skill_total(&base), 0);

    for skill_id in [
        "scouting",
        "guarding",
        "hospitality",
        "crafting",
        "charm",
        "recovery",
        "bargaining",
        "navigation",
        "arcana",
        "strength",
    ] {
        let mut skills = CompanionSkillState::default();
        assert!(
            increment_skill(&mut skills, skill_id, 5),
            "'{skill_id}' should be a real skill"
        );
        assert_eq!(
            companion_skill_total(&skills),
            5,
            "'{skill_id}' is trainable but does not count towards a companion's worth"
        );
    }
}

#[test]
fn every_trained_skill_raises_a_companions_wage() {
    let data = crate::data::test_game_data();
    let day_cycle = &data.config.day_cycle;
    let base = companion_daily_wage(day_cycle, None, &test_monster(Vec::new()));

    // Wages are the guild's answer to a roster that earns more as it gets
    // stronger. The formula was written against five skills and never revisited
    // when the other five became trainable, which made training them free.
    let divisor = day_cycle.skill_wage_divisor.max(1);
    for skill_id in [
        "scouting",
        "guarding",
        "hospitality",
        "crafting",
        "charm",
        "recovery",
        "bargaining",
        "navigation",
        "arcana",
        "strength",
    ] {
        let mut monster = test_monster(Vec::new());
        assert!(increment_skill(&mut monster.skills, skill_id, divisor));
        assert!(
            companion_daily_wage(day_cycle, None, &monster) > base,
            "'{skill_id}' can be trained but does not cost the guild anything"
        );
    }
}

#[test]
fn egg_star_ratings_span_the_whole_rank_ladder() {
    let data = crate::data::test_game_data();
    let day_cycle = &data.config.day_cycle;
    let top_rank = max_quality_rank(day_cycle);

    // The hatchery screen carried a hardcoded copy of this that capped at three
    // against a five-rank ladder, so every egg at grade 10 or better was shown
    // worse than it was — and rank drives `quality_income_multipliers_pct`, where
    // rank 5 earns ten times rank 1. Anything that recomputes a rating instead of
    // asking the config will drift the same way.
    let highest_threshold = day_cycle
        .egg_quality_rank_thresholds
        .iter()
        .copied()
        .max()
        .expect("the rank ladder must have thresholds");
    assert_eq!(
        egg_quality_rank(day_cycle, highest_threshold),
        top_rank,
        "an egg at the top authored threshold must hatch the top rank"
    );
    assert_eq!(egg_quality_rank(day_cycle, 0), 1);

    // Every rank on the ladder has to be reachable by some grade, or the
    // thresholds describe ranks the game can never hand out.
    let reachable = (0..=highest_threshold + 1)
        .map(|grade| egg_quality_rank(day_cycle, grade))
        .collect::<std::collections::HashSet<_>>();
    for rank in 1..=top_rank {
        assert!(
            reachable.contains(&rank),
            "rank {rank} is authored but no grade score produces it"
        );
    }
}

/// Refining is the only way to *make* a better egg rather than find one, and it
/// used to stop at rank 3 — a literal left behind when the ladder grew to five.
/// Ranks 4 and 5 earn 7x and 10x, so the two ranks that matter most were the two
/// the refinery refused to reach.
#[test]
fn refining_climbs_every_rung_of_the_star_ladder() {
    let data = crate::data::test_game_data();
    let day_cycle = &data.config.day_cycle;
    let top_rank = max_quality_rank(day_cycle);

    for rank in 1..top_rank {
        let mut game_state = crate::engine::create_new_game_state(&data);
        // Rank 1 is everything below the first threshold; every rank above it
        // starts exactly at the threshold before it.
        let grade = if rank == 1 {
            0
        } else {
            day_cycle.egg_quality_rank_thresholds[usize::from(rank) - 2]
        };
        assert_eq!(
            egg_quality_rank(day_cycle, grade),
            rank,
            "test setup: grade {grade} should be rank {rank}"
        );

        game_state.egg_inventory.clear();
        for index in 0..2u32 {
            game_state.egg_inventory.push(EggState {
                id: format!("egg_{index:03}"),
                source_floor_id: "tower_core".to_owned(),
                possible_species_ids: vec!["slime_companion".to_owned()],
                selected_species_id: None,
                incubation_state: EggIncubationState::Raw,
                grade_score: grade,
                preparation_focus: None,
            });
        }
        sync_egg_resource_count(&mut game_state);

        convert_egg(&data, &mut game_state, "egg_000", EggConversionKind::Refine)
            .unwrap_or_else(|error| panic!("refining two rank-{rank} eggs should work: {error}"));

        let refined = game_state
            .egg_inventory
            .first()
            .expect("refining leaves one egg behind");
        assert_eq!(
            egg_quality_rank(day_cycle, refined.grade_score),
            rank + 1,
            "two rank-{rank} eggs should refine into exactly one rank-{}",
            rank + 1
        );
    }
}

/// And it must still refuse at the top, or refining becomes an infinite ladder.
#[test]
fn refining_stops_at_the_top_of_the_ladder() {
    let data = crate::data::test_game_data();
    let day_cycle = &data.config.day_cycle;
    let top_grade = day_cycle
        .egg_quality_rank_thresholds
        .iter()
        .copied()
        .max()
        .expect("the rank ladder must have thresholds");
    let mut game_state = crate::engine::create_new_game_state(&data);

    game_state.egg_inventory.clear();
    for index in 0..2u32 {
        game_state.egg_inventory.push(EggState {
            id: format!("egg_{index:03}"),
            source_floor_id: "tower_core".to_owned(),
            possible_species_ids: vec!["slime_companion".to_owned()],
            selected_species_id: None,
            incubation_state: EggIncubationState::Raw,
            grade_score: top_grade,
            preparation_focus: None,
        });
    }
    sync_egg_resource_count(&mut game_state);

    assert!(
        convert_egg(&data, &mut game_state, "egg_000", EggConversionKind::Refine).is_err(),
        "two top-rank eggs must not refine into a rank the ladder does not have"
    );
}

#[test]
fn an_unassigned_expedition_reports_no_injury_risk_rather_than_a_number() {
    let data = crate::data::test_game_data();
    let mut game_state = crate::engine::create_new_game_state(&data);
    game_state.monsters = vec![test_monster(Vec::new())];
    let floor = data
        .floors
        .floors
        .iter()
        .find(|floor| game_state.town.unlocked_floor_ids.contains(&floor.id))
        .expect("a starting floor is unlocked");
    let mission_id = floor
        .mission_ids
        .first()
        .expect("the starting floor has a mission");

    let preview = preview_expedition_plan(
        &data,
        &game_state,
        &floor.id,
        mission_id,
        &ExpeditionPriority::Balanced,
    )
    .expect("preview");

    // Nobody assigned means nobody to hurt. This used to fall back to
    // `i32::MIN / 2`, which the planning screen printed as
    // "Injury Risk -1073741824" every time it was opened before a party existed.
    assert!(
        preview.injury_risk_score.is_none(),
        "an empty party must not produce an injury number, got {:?}",
        preview.injury_risk_score
    );
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

/// A companion who mutates into a stronger species has to become more expensive
/// to keep, or climbing the mutation tree is free power.
///
/// Wages were `quality_rank` plus skills — both properties of the egg she
/// hatched from — so a `gargoyle_stairwarden` at 10/4/10/6 cost exactly what a
/// `slime_companion` at 3/2/5/2 cost. The simulated guild funnelled 18 of 20
/// companions into one late species and never wanted a low tier again; nothing
/// on the ledger said the strong ones were expensive.
#[test]
fn a_stronger_species_costs_more_to_keep() {
    let data = crate::data::test_game_data();
    let day_cycle = &data.config.day_cycle;
    let monster = test_monster(Vec::new());

    let species_by_id = |id: &str| {
        data.species
            .species
            .iter()
            .find(|species| species.id == id)
            .expect("species should exist")
    };
    let slime = species_by_id("slime_companion");
    let gargoyle = species_by_id("gargoyle_stairwarden");

    let slime_wage = companion_daily_wage(day_cycle, Some(slime), &monster);
    let gargoyle_wage = companion_daily_wage(day_cycle, Some(gargoyle), &monster);

    assert!(
        gargoyle_wage > slime_wage,
        "the same companion should cost more as a gargoyle ({gargoyle_wage}) than as a slime ({slime_wage})"
    );

    // Every species with better total stats than another must cost at least as
    // much, so a future species cannot be authored as strictly free power.
    let stat_total = |species: &crate::data::SpeciesData| {
        species.base_stats.power
            + species.base_stats.charm
            + species.base_stats.endurance
            + species.base_stats.instinct
    };
    for stronger in &data.species.species {
        for weaker in &data.species.species {
            if stat_total(stronger) <= stat_total(weaker) {
                continue;
            }
            assert!(
                companion_daily_wage(day_cycle, Some(stronger), &monster)
                    >= companion_daily_wage(day_cycle, Some(weaker), &monster),
                "'{}' outclasses '{}' but does not cost more to keep",
                stronger.id,
                weaker.id
            );
        }
    }
}

/// Capability has to be paid for with narrowness, or a high tier is strictly
/// better than a low one and there is never a reason to keep the low one.
///
/// A matching role pays the same to everybody; what differs is what working
/// *outside* that role costs. The weakest species pay nothing, the strongest pay
/// the full penalty.
#[test]
fn a_stronger_species_is_less_flexible_outside_its_role() {
    let data = crate::data::test_game_data();

    // Power well above charm makes both `delver`, so tier is the only thing
    // separating them. A `versatile` companion is deliberately exempt — being
    // flexible is what that role *is* — so this must not test one.
    let with_species = |species_id: &str| {
        let mut monster = test_monster(Vec::new());
        monster.species_id = species_id.to_owned();
        monster.stats.power = 9;
        monster.stats.charm = 2;
        monster
    };
    let slime = with_species("slime_companion");
    let gargoyle = with_species("gargoyle_stairwarden");
    let role = crate::engine::monster_role(&data, &slime);
    assert_eq!(
        role,
        crate::engine::monster_role(&data, &gargoyle),
        "this test only isolates tier if both companions hold the same role"
    );
    assert_ne!(role, "versatile", "versatile is exempt from the penalty");

    // On her own role, tier costs nothing.
    assert_eq!(
        crate::engine::depth::role_affinity(&data, &slime, role),
        crate::engine::depth::role_affinity(&data, &gargoyle, role),
        "a matching role should pay the same whatever the species"
    );

    // Off it, the stronger species gives up more.
    let off_role = "comfort";
    let slime_off = crate::engine::depth::role_affinity(&data, &slime, off_role);
    let gargoyle_off = crate::engine::depth::role_affinity(&data, &gargoyle, off_role);
    assert!(
        gargoyle_off < slime_off,
        "a gargoyle ({gargoyle_off}) should lose more off-role than a slime ({slime_off})"
    );
}
