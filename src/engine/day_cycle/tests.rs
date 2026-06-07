use super::*;
use crate::data::{GuildRoomData, MissionData, TowerFloorData};
use crate::state::{
    ChamberState, EggIncubationState, ExpeditionPriority, ExpeditionState, GameState,
    ContractState, ContractStatus, CompanionState, CompanionJobState, OpeningChapterStep,
    PlayerTownState, ResourcesState, CompanionWorkHistoryState, CompanionSkillState, StoryProgressState,
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
        stamina_cost: 1,
        patron_tiers: vec!["local_adventurers".to_owned()],
        trained_skill_ids: vec!["crafting".to_owned(), "charm".to_owned()],
        work_history_gains: crate::data::CompanionWorkHistoryProgressionData {
            scouting_runs: 0,
            craft_jobs: 1,
            contracts_completed: 0,
            recovery_shifts: 1,
            hatchery_assists: 0,
            ..crate::data::CompanionWorkHistoryProgressionData::default()
        },
        preferred_trait_ids: Vec::new(),
        preferred_species_ids: Vec::new(),
        strategic_niche: None,
        upgrade_building_ids: Vec::new(),
        fatigue_modifier: 0,
        stress_modifier: 0,
        corruption_pressure: 0,
        guest_appeal: 0,
    };
    let monster = test_monster(vec!["corruption_tuned".to_owned()]);

    assert_eq!(guild_job_instability_gain(&room, &monster), 3);
}

#[test]
fn corruption_dive_rewards_more_corruption() {
    let floor = TowerFloorData {
        id: "floor_4".to_owned(),
        name: "Heart Vault".to_owned(),
        depth: 4,
        description: String::new(),
        difficulty: 10,
        requires_building_ids: Vec::new(),
        required_roster: Vec::new(),
        mission_ids: vec!["corruption_dive".to_owned()],
        baseline_rewards: ResourceAmountData::default(),
        egg_species_entries: Vec::new(),
        relic_drop_ids: Vec::new(),
        hazard_tags: Vec::new(),
        egg_grade_bonus: 0,
        corruption_pressure: 0,
    };
    let mission = MissionData {
        id: "corruption_dive".to_owned(),
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
            unlocked_room_ids: vec!["vanilla_suite".to_owned()],
            unlocked_floor_ids: vec!["floor_1".to_owned()],
            unlocked_species_ids: vec!["slime_girl".to_owned()],
            patron_tiers: vec!["local_adventurers".to_owned()],
            completed_project_ids: Vec::new(),
            active_situations: Vec::new(),
            party_size: 3,
            town_job_limit: 2,
        },
        egg_inventory: Vec::new(),
        chamber: ChamberState {
            exposure_risk: 0,
            is_secret_intact: true,
        },
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
        story_progress: StoryProgressState {
            opening_step: OpeningChapterStep::Complete,
            tower_hole_discovered: true,
            first_egg_created: true,
            first_slimegirl_hatched: true,
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
            unlocked_room_ids: vec!["vanilla_suite".to_owned()],
            unlocked_floor_ids: vec!["floor_1".to_owned()],
            unlocked_species_ids: vec!["slime_girl".to_owned()],
            patron_tiers: vec!["local_adventurers".to_owned()],
            completed_project_ids: Vec::new(),
            active_situations: Vec::new(),
            party_size: 3,
            town_job_limit: 2,
        },
        egg_inventory: Vec::new(),
        chamber: ChamberState::default(),
        debt: None,
        active_contracts: vec![ContractState {
            request_id: "guest_request_001".to_owned(),
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
        story_progress: StoryProgressState::default(),
        event_log: Vec::new(),
    };

    let message = release_monster(&mut game_state, "monster_002").expect("release should work");

    assert_eq!(message, "Liora left the keep.");
    assert_eq!(game_state.monsters.len(), 1);
    assert!(game_state.active_expedition.is_none());
    assert!(matches!(
        game_state.active_contracts[0].status,
        ContractStatus::Pending
    ));
    assert!(game_state.active_contracts[0]
        .assigned_monster_id
        .is_none());
    assert!(release_monster(&mut game_state, "monster_001").is_err());
}

#[test]
fn trained_room_skills_add_guild_job_bonus() {
    let room = GuildRoomData {
        id: "vanilla_suite".to_owned(),
        name: "Vanilla Suite".to_owned(),
        description: String::new(),
        service_summary: "Soft service".to_owned(),
        required_building_ids: Vec::new(),
        service_tier: 1,
        base_gold_yield: 30,
        base_residue_yield: 6,
        stamina_cost: 10,
        patron_tiers: vec!["local_adventurers".to_owned()],
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
        preferred_trait_ids: Vec::new(),
        preferred_species_ids: vec!["slime_girl".to_owned()],
        strategic_niche: None,
        upgrade_building_ids: Vec::new(),
        fatigue_modifier: 0,
        stress_modifier: 0,
        corruption_pressure: 0,
        guest_appeal: 0,
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
            unlocked_species_ids: vec!["slime_girl".to_owned()],
            patron_tiers: vec!["local_adventurers".to_owned()],
            completed_project_ids: Vec::new(),
            active_situations: Vec::new(),
            party_size: 3,
            town_job_limit: 2,
        },
        egg_inventory: Vec::new(),
        chamber: ChamberState {
            exposure_risk: 0,
            is_secret_intact: true,
        },
        debt: None,
        active_contracts: Vec::new(),
        monsters: Vec::new(),
        active_expedition: None,
        story_progress: StoryProgressState {
            opening_step: OpeningChapterStep::Complete,
            tower_hole_discovered: true,
            first_egg_created: true,
            first_slimegirl_hatched: false,
            hatched_species_ids: Vec::new(),
            first_room_built: false,
            first_client_completed: false,
            first_creditor_visit_seen: false,
            first_special_guest_seen: false,
        },
        event_log: Vec::new(),
    };

    create_opening_egg(&mut game_state, "slime_girl");
    assert_eq!(game_state.resources.eggs, 1);
    assert_eq!(raw_egg_count_for_species(&game_state, "slime_girl"), 1);

    let mut egg = game_state.egg_inventory[0].clone();
    egg.selected_species_id = Some("slime_girl".to_owned());
    egg.incubation_state = EggIncubationState::ReadyToHatch;
    egg.loyalty_imprinted = true;
    game_state.egg_inventory[0] = egg;

    assert_eq!(ready_egg_count_for_species(&game_state, "slime_girl"), 1);
}

fn test_monster(trait_ids: Vec<String>) -> CompanionState {
    CompanionState {
        id: "monster_001".to_owned(),
        species_id: "slime_girl".to_owned(),
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
