use super::*;

pub(super) fn select_event_text(
    data: &GameData,
    category: &str,
    phase: &str,
    monster: &CompanionState,
) -> Option<String> {
    let matches = data
        .events
        .events
        .iter()
        .filter(|event| {
            event.category == category
                && event.phase == phase
                && event
                    .required_trait_ids
                    .iter()
                    .all(|trait_id| monster.trait_ids.contains(trait_id))
                && (event.required_species_ids.is_empty()
                    || event.required_species_ids.contains(&monster.species_id))
        })
        .collect::<Vec<_>>();

    if matches.is_empty() {
        None
    } else {
        let index = gen_range(0, matches.len() as i32) as usize;
        Some(matches[index].text.clone())
    }
}

pub(super) fn select_town_event_text(
    data: &GameData,
    monsters: &[CompanionState],
) -> Option<String> {
    let matches = data
        .events
        .events
        .iter()
        .filter(|event| event.category == "town" && event.phase == "day_end")
        .filter(|event| {
            if event.required_trait_ids.is_empty() && event.required_species_ids.is_empty() {
                return true;
            }

            monsters.iter().any(|monster| {
                event
                    .required_trait_ids
                    .iter()
                    .all(|trait_id| monster.trait_ids.contains(trait_id))
                    && (event.required_species_ids.is_empty()
                        || event.required_species_ids.contains(&monster.species_id))
            })
        })
        .collect::<Vec<_>>();

    if matches.is_empty() {
        None
    } else {
        let index = gen_range(0, matches.len() as i32) as usize;
        Some(matches[index].text.clone())
    }
}

pub(super) fn select_special_event<'a>(
    data: &'a GameData,
    game_state: &GameState,
) -> Option<&'a crate::data::EventData> {
    let matches = data
        .events
        .events
        .iter()
        .filter(|event| event.category == "special" && event.phase == "day_end")
        .filter(|event| {
            event
                .min_day
                .is_none_or(|min_day| game_state.current_day >= min_day)
        })
        .filter(|event| {
            event.required_trait_ids.is_empty()
                || game_state.monsters.iter().any(|monster| {
                    event
                        .required_trait_ids
                        .iter()
                        .all(|trait_id| monster.trait_ids.contains(trait_id))
                })
        })
        .filter(|event| {
            event.required_species_ids.is_empty()
                || game_state
                    .monsters
                    .iter()
                    .any(|monster| event.required_species_ids.contains(&monster.species_id))
        })
        .filter(|event| {
            event.required_building_ids.iter().all(|building_id| {
                game_state
                    .town
                    .constructed_building_ids
                    .contains(building_id)
            })
        })
        .filter(|event| {
            let trigger_chance_pct = event.trigger_chance_pct.unwrap_or(100);
            gen_range(0, 100) < trigger_chance_pct
        })
        .collect::<Vec<_>>();

    if matches.is_empty() {
        None
    } else {
        let total_weight = matches
            .iter()
            .map(|event| event.weight.unwrap_or(1))
            .sum::<u32>();
        let mut roll = gen_range(0, total_weight as i32) as u32;
        matches.into_iter().find(|event| {
            let weight = event.weight.unwrap_or(1);
            if roll < weight {
                true
            } else {
                roll -= weight;
                false
            }
        })
    }
}
