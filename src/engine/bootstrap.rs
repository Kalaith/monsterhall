//! Campaign bootstrap logic for a new save.

use crate::data::{GameData, ResourceAmountData, StatBlockData};
use crate::state::{
    CompanionJobState, CompanionSkillState, CompanionState, CompanionWorkHistoryState, GameState,
    OpeningChapterStep, PlayerTownState, ResourcesState, StoryProgressState,
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
        patron_tiers: vec!["local_delvers".to_owned()],
        completed_project_ids: Vec::new(),
        floor_surveys: Vec::new(),
        active_situations: Vec::new(),
        party_size: new_game.party_size,
        town_job_limit: new_game.town_job_limit,
    };

    GameState {
        current_day: new_game.starting_day,
        resources: resources_from_data(&new_game.starting_resources),
        town,
        egg_inventory: Vec::new(),
        debt: None,
        active_contracts: Vec::new(),
        monsters,
        active_expedition: None,
        resolved_contracts: Vec::new(),
        story_progress: StoryProgressState {
            opening_step: OpeningChapterStep::Camp,
            tower_hole_discovered: false,
            first_egg_created: false,
            first_companion_hatched: false,
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
    use crate::data::test_game_data;

    #[test]
    fn new_game_bootstrap_preserves_configured_starting_rooms() {
        let mut data = test_game_data();
        data.config.new_game.starting_room_ids =
            vec!["common_room".to_owned(), "packroom_annex".to_owned()];

        let game_state = create_new_game_state(&data);

        assert_eq!(
            game_state.town.unlocked_room_ids,
            vec!["common_room".to_owned(), "packroom_annex".to_owned()]
        );
    }
}
