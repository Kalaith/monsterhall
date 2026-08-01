//! Floor surveys: how depth is earned.
//!
//! Buildings open a *band* of the tower. Running the floors of that band is what
//! opens the floors themselves — a twenty-five floor tower cannot mean
//! twenty-five buildings. A floor accumulates survey points every time an
//! expedition completes on it, and a deeper floor lists the floors it wants
//! surveyed plus how deeply, in `floors.json`.

use super::*;
use crate::state::FloorSurveyState;

/// Records one completed expedition against `floor_id`, worth the mission's
/// `survey_value`. Runs regardless of haul: the party walked the route, so the
/// guild knows the route a little better.
pub(super) fn record_floor_survey(town: &mut PlayerTownState, floor_id: &str, survey_value: u32) {
    if survey_value == 0 {
        return;
    }
    if let Some(entry) = town
        .floor_surveys
        .iter_mut()
        .find(|entry| entry.floor_id == floor_id)
    {
        entry.surveys = entry.surveys.saturating_add(survey_value);
        return;
    }
    town.floor_surveys.push(FloorSurveyState {
        floor_id: floor_id.to_owned(),
        surveys: survey_value,
    });
}

/// Records the survey a finished expedition earned, worth whatever its mission
/// is worth. An unknown mission still counts for the default single point — the
/// party walked the floor either way.
pub(super) fn record_expedition_survey(
    data: &GameData,
    town: &mut PlayerTownState,
    expedition: &ExpeditionState,
    floor_id: &str,
) {
    let survey_value = data
        .missions
        .missions
        .iter()
        .find(|mission| mission.id == expedition.mission_id)
        .map(|mission| mission.survey_value)
        .unwrap_or(1);
    record_floor_survey(town, floor_id, survey_value);
}

pub(super) fn survey_count(town: &PlayerTownState, floor_id: &str) -> u32 {
    town.floor_surveys
        .iter()
        .find(|entry| entry.floor_id == floor_id)
        .map(|entry| entry.surveys)
        .unwrap_or_default()
}

/// True when every prerequisite floor is surveyed deeply enough and every
/// required building stands. Floors with no survey prerequisites are never
/// chain-unlocked by this path — a building's `unlocks.floor_ids` opens those.
fn chain_unlock_is_satisfied(town: &PlayerTownState, floor: &crate::data::TowerFloorData) -> bool {
    if floor.requires_surveyed_floor_ids.is_empty() {
        return false;
    }
    floor
        .requires_surveyed_floor_ids
        .iter()
        .all(|required_id| survey_count(town, required_id) >= floor.required_surveys)
        && floor
            .requires_building_ids
            .iter()
            .all(|building_id| town.constructed_building_ids.contains(building_id))
}

/// Opens every floor whose survey chain is now satisfied. Swept once per day so
/// it catches both the run that finished the survey and the building raised
/// afterwards. Returns one log line per floor opened.
pub(super) fn unlock_surveyed_floors(data: &GameData, town: &mut PlayerTownState) -> Vec<String> {
    let mut opened = Vec::new();
    for floor in &data.floors.floors {
        if town.unlocked_floor_ids.contains(&floor.id) {
            continue;
        }
        if !chain_unlock_is_satisfied(town, floor) {
            continue;
        }
        town.unlocked_floor_ids.push(floor.id.clone());
        opened.push(format!(
            "Survey notes from the floors above chart a way down to {}.",
            floor.name
        ));
    }
    opened
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floor(id: &str, requires: &[&str], required_surveys: u32) -> crate::data::TowerFloorData {
        crate::data::TowerFloorData {
            id: id.to_owned(),
            name: id.to_owned(),
            depth: 1,
            description: String::new(),
            difficulty: 10,
            requires_building_ids: Vec::new(),
            requires_surveyed_floor_ids: requires.iter().map(|id| (*id).to_owned()).collect(),
            required_surveys,
            required_roster: Vec::new(),
            mission_ids: vec!["resource_run".to_owned()],
            baseline_rewards: Default::default(),
            egg_species_entries: Vec::new(),
            relic_drop_ids: Vec::new(),
            hazard_tags: Vec::new(),
            egg_grade_bonus: 0,
            corruption_pressure: 0,
        }
    }

    fn town_with(unlocked: &[&str]) -> PlayerTownState {
        PlayerTownState {
            unlocked_floor_ids: unlocked.iter().map(|id| (*id).to_owned()).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn surveys_accumulate_per_floor_and_by_mission_worth() {
        let mut town = PlayerTownState::default();

        record_floor_survey(&mut town, "floor_a", 1);
        record_floor_survey(&mut town, "floor_a", 2);
        record_floor_survey(&mut town, "floor_b", 1);

        assert_eq!(survey_count(&town, "floor_a"), 3);
        assert_eq!(survey_count(&town, "floor_b"), 1);
        assert_eq!(survey_count(&town, "floor_never_run"), 0);
    }

    #[test]
    fn a_worthless_survey_records_nothing() {
        let mut town = PlayerTownState::default();

        record_floor_survey(&mut town, "floor_a", 0);

        assert!(town.floor_surveys.is_empty());
    }

    #[test]
    fn a_floor_opens_only_once_its_chain_is_surveyed_deeply_enough() {
        let mut town = town_with(&["floor_a"]);
        let floors = [floor("floor_b", &["floor_a"], 2)];

        record_floor_survey(&mut town, "floor_a", 1);
        assert!(!chain_unlock_is_satisfied(&town, &floors[0]));

        record_floor_survey(&mut town, "floor_a", 1);
        assert!(chain_unlock_is_satisfied(&town, &floors[0]));
    }

    #[test]
    fn every_prerequisite_floor_must_be_surveyed_not_just_one() {
        let mut town = town_with(&["floor_a", "floor_b"]);
        let deep = floor("floor_c", &["floor_a", "floor_b"], 1);

        record_floor_survey(&mut town, "floor_a", 5);
        assert!(!chain_unlock_is_satisfied(&town, &deep));

        record_floor_survey(&mut town, "floor_b", 1);
        assert!(chain_unlock_is_satisfied(&town, &deep));
    }

    /// The band gate: surveys alone do not open a floor whose building is unbuilt.
    #[test]
    fn a_required_building_still_gates_a_surveyed_floor() {
        let mut town = town_with(&["floor_a"]);
        let mut deep = floor("floor_b", &["floor_a"], 1);
        deep.requires_building_ids = vec!["route_map_chamber".to_owned()];
        record_floor_survey(&mut town, "floor_a", 3);

        assert!(!chain_unlock_is_satisfied(&town, &deep));

        town.constructed_building_ids
            .push("route_map_chamber".to_owned());
        assert!(chain_unlock_is_satisfied(&town, &deep));
    }

    /// Floors with no survey prerequisites keep their old behaviour exactly:
    /// a building's `unlocks.floor_ids` is the only thing that opens them.
    #[test]
    fn a_floor_without_a_survey_chain_is_never_opened_by_this_path() {
        let mut town = town_with(&[]);
        let building_gated = floor("floor_b", &[], 1);
        record_floor_survey(&mut town, "floor_a", 99);

        assert!(!chain_unlock_is_satisfied(&town, &building_gated));
    }

    #[test]
    fn an_already_unlocked_floor_is_not_reopened() {
        let data_floors = vec![floor("floor_b", &["floor_a"], 1)];
        let mut town = town_with(&["floor_a", "floor_b"]);
        record_floor_survey(&mut town, "floor_a", 4);

        let opened = unlock_floors_for(&data_floors, &mut town);

        assert!(opened.is_empty());
        assert_eq!(town.unlocked_floor_ids.len(), 2);
    }

    /// One deep run can open a cascade when several floors share a prerequisite.
    #[test]
    fn one_sweep_opens_every_floor_the_chain_now_reaches() {
        let data_floors = vec![
            floor("floor_b", &["floor_a"], 2),
            floor("floor_c", &["floor_a"], 2),
            floor("floor_d", &["floor_a"], 9),
        ];
        let mut town = town_with(&["floor_a"]);
        record_floor_survey(&mut town, "floor_a", 2);

        let opened = unlock_floors_for(&data_floors, &mut town);

        assert_eq!(opened.len(), 2);
        assert!(town.unlocked_floor_ids.contains(&"floor_b".to_owned()));
        assert!(town.unlocked_floor_ids.contains(&"floor_c".to_owned()));
        assert!(
            !town.unlocked_floor_ids.contains(&"floor_d".to_owned()),
            "floor_d wants a deeper survey than has been earned"
        );
    }

    /// Mirrors `unlock_surveyed_floors` without needing a whole `GameData`.
    fn unlock_floors_for(
        floors: &[crate::data::TowerFloorData],
        town: &mut PlayerTownState,
    ) -> Vec<String> {
        let mut opened = Vec::new();
        for floor in floors {
            if town.unlocked_floor_ids.contains(&floor.id) {
                continue;
            }
            if !chain_unlock_is_satisfied(town, floor) {
                continue;
            }
            town.unlocked_floor_ids.push(floor.id.clone());
            opened.push(floor.name.clone());
        }
        opened
    }
}
