//! Campaign bootstrap logic for a new save.

use crate::data::{GameData, ResourceAmountData, StatBlockData};
use crate::state::{
    ChamberState, GameState, CompanionState, CompanionJobState, OpeningChapterStep,
    PlayerTownState, ResourcesState, CompanionWorkHistoryState, CompanionSkillState, StoryProgressState,
};

pub fn create_new_game_state(data: &GameData) -> GameState {
    let new_game = &data.config.new_game;

    let monsters = new_game
        .starter_monsters
        .iter()
        .enumerate()
        .map(|(index, starter)| {
            let species = data
                .species
                .species
                .iter()
                .find(|species| species.id == starter.species_id)
                .expect("validated species references must exist");

            let combined_stats = add_stats(&species.base_stats, &starter.stat_bonuses);
            let mut trait_ids = species.starting_traits.clone();
            for trait_id in &starter.extra_traits {
                if !trait_ids.contains(trait_id) {
                    trait_ids.push(trait_id.clone());
                }
            }

            CompanionState {
                id: format!("monster_{:03}", index + 1),
                species_id: starter.species_id.clone(),
                name: starter.name.clone(),
                quality_rank: 1,
                stats: combined_stats,
                trait_ids,
                current_job: CompanionJobState::Idle,
                skills: CompanionSkillState::default(),
                work_history: CompanionWorkHistoryState::default(),
                fatigue: 0,
                stress: 0,
                injury: 0,
                corruption: 0,
                bond: 1,
                reputation: 0,
            }
        })
        .collect();

    let town = PlayerTownState {
        constructed_building_ids: new_game.starting_building_ids.clone(),
        unlocked_room_ids: new_game.starting_room_ids.clone(),
        unlocked_floor_ids: new_game.starting_floor_ids.clone(),
        unlocked_species_ids: collect_unlocked_species_ids(data),
        patron_tiers: vec!["local_adventurers".to_owned()],
        completed_project_ids: Vec::new(),
        active_situations: Vec::new(),
        party_size: new_game.party_size,
        town_job_limit: new_game.town_job_limit,
    };

    GameState {
        current_day: new_game.starting_day,
        resources: resources_from_data(&new_game.starting_resources),
        town,
        egg_inventory: Vec::new(),
        chamber: ChamberState {
            exposure_risk: 0,
            is_secret_intact: true,
        },
        debt: None,
        active_contracts: Vec::new(),
        monsters,
        active_expedition: None,
        story_progress: StoryProgressState {
            opening_step: OpeningChapterStep::Camp,
            tower_hole_discovered: false,
            first_egg_created: false,
            first_slimegirl_hatched: false,
            hatched_species_ids: Vec::new(),
            first_room_built: false,
            first_client_completed: false,
            first_creditor_visit_seen: false,
            first_special_guest_seen: false,
        },
        event_log: vec!["The ruined keep stirs back to life above the tower.".to_owned()],
    }
}

fn add_stats(base: &StatBlockData, bonus: &StatBlockData) -> StatBlockData {
    StatBlockData {
        power: base.power + bonus.power,
        charm: base.charm + bonus.charm,
        endurance: base.endurance + bonus.endurance,
        instinct: base.instinct + bonus.instinct,
    }
}

fn resources_from_data(resources: &ResourceAmountData) -> ResourcesState {
    ResourcesState {
        gold: resources.gold,
        tower_materials: resources.tower_materials,
        eggs: resources.eggs,
        relics: resources.relics,
        arcane_residue: resources.arcane_residue,
    }
}

fn collect_unlocked_species_ids(data: &GameData) -> Vec<String> {
    let mut unlocked_species_ids = data.config.new_game.starting_species_ids.clone();

    for starter in &data.config.new_game.starter_monsters {
        if !unlocked_species_ids.contains(&starter.species_id) {
            unlocked_species_ids.push(starter.species_id.clone());
        }
    }

    for building_id in &data.config.new_game.starting_building_ids {
        let building = data
            .buildings
            .buildings
            .iter()
            .find(|building| building.id == *building_id)
            .expect("validated building references must exist");

        for species_id in &building.unlocks.species_ids {
            if !unlocked_species_ids.contains(species_id) {
                unlocked_species_ids.push(species_id.clone());
            }
        }
    }

    unlocked_species_ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_bootstrap_preserves_configured_starting_rooms() {
        let mut data = test_game_data();
        data.config.new_game.starting_room_ids =
            vec!["vanilla_suite".to_owned(), "packroom_annex".to_owned()];

        let game_state = create_new_game_state(&data);

        assert_eq!(
            game_state.town.unlocked_room_ids,
            vec!["vanilla_suite".to_owned(), "packroom_annex".to_owned()]
        );
    }

    fn test_game_data() -> GameData {
        GameData {
            config: parse_json(include_str!("../../assets/data/config.json")),
            ui_text: parse_json(include_str!("../../assets/data/ui_text.json")),
            debt_milestones: parse_json(include_str!("../../assets/data/debt_milestones.json")),
            patron_archetypes: parse_json(include_str!("../../assets/data/guest_archetypes.json")),
            contracts: parse_json(include_str!("../../assets/data/guest_requests.json")),
            patron_tiers: parse_json(include_str!("../../assets/data/client_tiers.json")),
            missions: parse_json(include_str!("../../assets/data/missions.json")),
            mutations: parse_json(include_str!("../../assets/data/mutations.json")),
            story_events: parse_json(include_str!("../../assets/data/story_events.json")),
            monster_names: parse_json(include_str!("../../assets/data/monster_names.json")),
            species: parse_json(include_str!("../../assets/data/species.json")),
            buildings: parse_json(include_str!("../../assets/data/buildings.json")),
            floors: parse_json(include_str!("../../assets/data/floors.json")),
            traits: parse_json(include_str!("../../assets/data/traits.json")),
            guild_rooms: parse_json(include_str!("../../assets/data/guild_rooms.json")),
            events: parse_json(include_str!("../../assets/data/events.json")),
        }
    }

    fn parse_json<T: serde::de::DeserializeOwned>(json: &str) -> T {
        serde_json::from_str(json).expect("test data should deserialize")
    }
}
