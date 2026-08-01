//! Naming what a party actually brings up.
//!
//! `relic_drop_ids` decides whether a floor can pay a relic, and the resource
//! ledger counts how many. Neither says *what* was found, so every relic in a
//! twenty-five floor tower read the same way: the number went up. A floor's
//! declared relics are real objects with names and histories; this reports the
//! one that came home so it lands in the day's log and the journal.

use super::*;

/// Picks one of the floor's declared relics and returns its discovery note.
///
/// Rotates on the day rather than rolling, deliberately. This line is flavour,
/// and drawing from the seeded RNG for it would shift every downstream roll in
/// the day cycle — the first version did exactly that and moved a 365-day
/// campaign from seventeen buildings to eight. Cosmetic output must not perturb
/// the simulation.
///
/// Returns `None` when the floor declares none, or the id has no catalogue entry
/// — the latter cannot happen in shipped data, which load-time validation
/// enforces in both directions, but a save or a mod could still ask.
pub(super) fn relic_discovery_line(
    data: &GameData,
    floor: &crate::data::TowerFloorData,
    resolved_day: u32,
) -> Option<String> {
    if floor.relic_drop_ids.is_empty() {
        return None;
    }
    let index = resolved_day as usize % floor.relic_drop_ids.len();
    let relic_id = floor.relic_drop_ids.get(index)?;
    let relic = data
        .relics
        .relics
        .iter()
        .find(|entry| &entry.id == relic_id)?;
    Some(relic.discovery_note.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::test_game_data;

    fn floor<'a>(data: &'a GameData, id: &str) -> &'a crate::data::TowerFloorData {
        data.floors
            .floors
            .iter()
            .find(|floor| floor.id == id)
            .expect("test floor should exist")
    }

    #[test]
    fn a_floor_that_holds_a_relic_names_what_came_up() {
        let data = test_game_data();

        let line = relic_discovery_line(&data, floor(&data, "floor_2_molten_baths"), 1)
            .expect("a relic-bearing floor should name its find");

        assert!(!line.trim().is_empty());
    }

    #[test]
    fn a_floor_with_no_relics_names_nothing() {
        let data = test_game_data();
        let bare = floor(&data, "floor_1_slick_cellars");
        assert!(bare.relic_drop_ids.is_empty());

        assert!(relic_discovery_line(&data, bare, 1).is_none());
    }

    /// Rotation must cover every relic a floor declares, or the second relic on
    /// a two-relic floor is prose nobody ever sees.
    #[test]
    fn rotation_reaches_every_relic_a_floor_declares() {
        let data = test_game_data();
        let two_relic = floor(&data, "handlers_vault");
        assert_eq!(two_relic.relic_drop_ids.len(), 2);

        let seen: std::collections::HashSet<_> = (0..8)
            .filter_map(|day| relic_discovery_line(&data, two_relic, day))
            .collect();

        assert_eq!(
            seen.len(),
            2,
            "both relics should appear across successive days"
        );
    }

    /// Every relic a floor can drop has to be reportable, or the player collects
    /// a number instead of an object.
    #[test]
    fn every_declared_relic_has_a_discovery_note() {
        let data = test_game_data();

        for floor in &data.floors.floors {
            for relic_id in &floor.relic_drop_ids {
                let relic = data
                    .relics
                    .relics
                    .iter()
                    .find(|entry| &entry.id == relic_id)
                    .unwrap_or_else(|| panic!("floor '{}' drops unknown relic", floor.id));
                assert!(
                    !relic.discovery_note.trim().is_empty(),
                    "relic '{relic_id}' has no discovery note"
                );
            }
        }
    }
}
