//! Validation-policy egg hatching and monster replacement decisions.
use super::policy_growth::{
    can_make_growth_investment, has_unfilled_workforce_demand, GrowthInvestmentKind,
};
use super::*;

pub(super) fn hatch_affordable_eggs(data: &GameData, game_state: &mut GameState) -> usize {
    let mut hatch_count = 0usize;
    loop {
        let mut hatched_any = false;
        let egg_ids = game_state
            .egg_inventory
            .iter()
            .map(|egg| egg.id.clone())
            .collect::<Vec<_>>();

        for egg_id in egg_ids {
            let Some(egg) = game_state
                .egg_inventory
                .iter()
                .find(|entry| entry.id == egg_id)
                .cloned()
            else {
                continue;
            };
            let Some(species_id) =
                cheapest_unlocked_species_option(data, game_state, &egg.possible_species_ids)
                    .map(str::to_owned)
            else {
                continue;
            };
            let Some(species) = data
                .species
                .species
                .iter()
                .find(|entry| entry.id == species_id)
            else {
                continue;
            };

            if game_state.monsters.len() >= day_cycle::effective_population_cap(data, game_state) {
                if let Some((replacement_id, replacement_species_id)) =
                    replacement_plan_for_egg(data, game_state, &egg)
                {
                    let Some(replacement_species) = data
                        .species
                        .species
                        .iter()
                        .find(|entry| entry.id == replacement_species_id)
                    else {
                        continue;
                    };
                    if can_make_growth_investment(
                        game_state,
                        replacement_species.hatching_cost.gold,
                        0,
                        GrowthInvestmentKind::Hatch,
                    ) && replace_monster_with_selected_egg(
                        data,
                        game_state,
                        &egg_id,
                        Some(&replacement_species_id),
                        &replacement_id,
                    )
                    .is_ok()
                    {
                        hatched_any = true;
                        hatch_count += 1;
                    }
                } else if convert_egg(data, game_state, &egg_id, EggConversionKind::Refine)
                    .or_else(|_| convert_egg(data, game_state, &egg_id, EggConversionKind::Sell))
                    .is_ok()
                {
                    hatched_any = true;
                }
                continue;
            }

            if !has_unfilled_workforce_demand(game_state) {
                continue;
            }

            if !hatch_pacing_allows(game_state, hatch_count) {
                continue;
            }

            if !can_make_growth_investment(
                game_state,
                species.hatching_cost.gold,
                1,
                GrowthInvestmentKind::Hatch,
            ) {
                continue;
            }

            if hatch_selected_egg(data, game_state, &egg_id, Some(&species_id)).is_ok() {
                hatched_any = true;
                hatch_count += 1;
            }
        }

        if !hatched_any {
            break;
        }
    }
    hatch_count
}

fn replacement_plan_for_egg(
    data: &GameData,
    game_state: &GameState,
    egg: &crate::state::EggState,
) -> Option<(String, String)> {
    let new_quality = day_cycle::egg_quality_rank(&data.config.day_cycle, egg.grade_score);
    egg.possible_species_ids
        .iter()
        .filter(|species_id| {
            game_state
                .town
                .unlocked_species_ids
                .iter()
                .any(|unlocked_id| unlocked_id == *species_id)
        })
        .filter_map(|species_id| {
            let replacement =
                replacement_candidate_for_species(game_state, species_id, new_quality)?;
            let species = data
                .species
                .species
                .iter()
                .find(|entry| entry.id == *species_id)?;
            let cost_score = species.hatching_cost.gold
                + species.hatching_cost.tower_materials
                + species.hatching_cost.arcane_residue
                + species.hatching_cost.relics * 100;
            Some((
                replacement.id.clone(),
                species_id.clone(),
                species_count(game_state, species_id),
                replacement.quality_rank,
                cost_score,
            ))
        })
        .min_by_key(|(_, _, count, old_quality, cost)| (*count, *old_quality, *cost))
        .map(|(replacement_id, species_id, _, _, _)| (replacement_id, species_id))
}

fn replacement_candidate_for_species<'a>(
    game_state: &'a GameState,
    species_id: &str,
    new_quality: u8,
) -> Option<&'a crate::state::CompanionState> {
    if let Some(upgrade) = game_state
        .monsters
        .iter()
        .filter(|monster| monster.species_id == species_id && monster.quality_rank < new_quality)
        .min_by_key(|monster| (monster.quality_rank, monster_service_score(monster)))
    {
        return Some(upgrade);
    }

    if species_count(game_state, species_id) == 0 {
        return game_state
            .monsters
            .iter()
            .filter(|monster| species_count(game_state, &monster.species_id) > 1)
            .min_by_key(|monster| (monster.quality_rank, monster_service_score(monster)));
    }

    None
}

fn species_count(game_state: &GameState, species_id: &str) -> usize {
    game_state
        .monsters
        .iter()
        .filter(|monster| monster.species_id == species_id)
        .count()
}

pub(super) fn hatch_pacing_allows(game_state: &GameState, hatches_this_policy: usize) -> bool {
    if hatches_this_policy > 0 {
        return false;
    }

    if game_state.monsters.len() < 6 {
        return game_state.current_day.is_multiple_of(3);
    }

    if game_state.monsters.len() >= 12 {
        let late_campaign_cadence = if game_state.current_day >= 240 {
            12
        } else {
            18
        };
        return game_state.current_day.is_multiple_of(late_campaign_cadence);
    }

    let mid_campaign_cadence = if game_state.current_day <= 30 { 12 } else { 6 };
    game_state.current_day.is_multiple_of(mid_campaign_cadence)
}

pub(super) fn cheapest_unlocked_species_option<'a>(
    data: &'a GameData,
    game_state: &GameState,
    species_ids: &'a [String],
) -> Option<&'a str> {
    species_ids
        .iter()
        .filter(|species_id| {
            game_state
                .town
                .unlocked_species_ids
                .iter()
                .any(|unlocked_id| unlocked_id == *species_id)
        })
        .filter_map(|species_id| {
            data.species
                .species
                .iter()
                .find(|entry| entry.id == *species_id)
                .map(|species| {
                    (
                        species_id.as_str(),
                        species.hatching_cost.gold
                            + species.hatching_cost.tower_materials
                            + species.hatching_cost.arcane_residue
                            + species.hatching_cost.relics * 100,
                    )
                })
        })
        .min_by_key(|(_, score)| *score)
        .map(|(species_id, _)| species_id)
}

pub(super) fn monster_service_score(monster: &crate::state::CompanionState) -> u32 {
    monster.skills.scouting
        + monster.skills.guarding
        + monster.skills.hospitality
        + monster.skills.crafting
        + monster.skills.charm
        + monster.work_history.scouting_runs
        + monster.work_history.guard_duties
        + monster.work_history.hospitality_jobs
        + monster.work_history.craft_jobs
        + monster.work_history.contracts_completed
        + monster.stats.charm.max(0) as u32
}
