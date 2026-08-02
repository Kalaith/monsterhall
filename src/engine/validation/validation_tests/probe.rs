use super::*;

/// Reports which floors a 365-day campaign actually reaches, unlocks, and runs.
///
/// The simulation reports record missions but not floors, which hides the
/// failure mode that matters most for a deep tower: a floor that is unlocked and
/// then never chosen. A survey chain is serial, so one unattractive link stalls
/// every floor below it — and every balance assertion stays green throughout,
/// because a floor nobody visits changes nothing.
///
/// Ignored by default because it prints rather than asserts. Run it whenever a
/// band is authored or a floor's rewards move:
///
/// ```text
/// cargo test probe_floor_usage -- --ignored --nocapture
/// ```
#[test]
#[ignore]
fn probe_floor_usage() {
    let _rng_guard = simulation_rng_guard();
    srand(SIMULATION_BASE_SEED ^ 365);
    let data = test_game_data();
    let mut game_state = create_new_game_state(&data);
    play_opening_sequence(&data, &mut game_state);
    // The day a species unlocks is the whole story for a deep one: unlocked on
    // day 324 of 365 is not content, however good the entry reads.
    let mut unlock_day: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    for day in 0..365u32 {
        run_daily_policy(&data, &mut game_state);
        resolve_day(&data, &mut game_state);
        for species_id in &game_state.town.unlocked_species_ids {
            unlock_day.entry(species_id.clone()).or_insert(day);
        }
    }

    println!(
        "ECONOMY gold={} companions={} cap={} buildings={} eggs={}",
        game_state.resources.gold,
        game_state.monsters.len(),
        crate::engine::effective_population_cap(&data, &game_state),
        game_state.town.constructed_building_ids.len(),
        game_state.egg_inventory.len(),
    );

    // Score every unlocked floor for the strongest idle companion, to see what
    // the planner actually values rather than guessing from outcomes.
    if let Some(monster) = game_state.monsters.first().cloned() {
        let mut sim = game_state.clone();
        for m in &mut sim.monsters {
            m.current_job = crate::state::CompanionJobState::Idle;
        }
        for floor in &data.floors.floors {
            if !sim.town.unlocked_floor_ids.contains(&floor.id) {
                continue;
            }
            for mission_id in &floor.mission_ids {
                crate::engine::configure_expedition_plan(
                    &mut sim,
                    &floor.id,
                    mission_id,
                    ExpeditionPriority::Balanced,
                );
                if crate::engine::assign_monster_to_expedition(
                    &data,
                    &mut sim,
                    &monster.id,
                    &floor.id,
                )
                .is_err()
                {
                    println!("SCORE {:<22} {:<14} ASSIGN-FAILED", floor.id, mission_id);
                    continue;
                }
                match crate::engine::preview_expedition_plan(
                    &data,
                    &sim,
                    &floor.id,
                    mission_id,
                    &ExpeditionPriority::Balanced,
                ) {
                    Ok(p) => println!(
                        "SCORE {:<22} {:<14} eggs={} relics={} mat={} injury={} success={}",
                        floor.id,
                        mission_id,
                        p.projected_eggs,
                        p.projected_relics,
                        p.projected_materials,
                        p.injury_risk_score,
                        p.success_score
                    ),
                    Err(e) => println!("SCORE {:<22} {:<14} PREVIEW-ERR {e}", floor.id, mission_id),
                }
            }
        }
    }

    let mut ranks = [0usize; 6];
    for monster in &game_state.monsters {
        ranks[usize::from(monster.quality_rank).min(5)] += 1;
    }
    println!("RANKS (1..5): {:?}", &ranks[1..]);

    // A species the campaign never hatches is invisible content, and the
    // standard reports count companions rather than what they are.
    for species in &data.species.species {
        let in_roster = game_state
            .monsters
            .iter()
            .filter(|monster| monster.species_id == species.id)
            .count();
        match unlock_day.get(&species.id) {
            Some(day) => println!(
                "SPECIES {:<24} unlocked_day={day:<4} in_roster={in_roster}",
                species.id
            ),
            None => println!("SPECIES {:<24} never unlocked", species.id),
        }
    }
    // Corruption only climbs, so mutation thresholds have to be read against
    // what a campaign actually reaches rather than guessed. The first tree was
    // authored at 8/16/18 against a roster that ends between 45 and 144.
    let mut corruption = game_state
        .monsters
        .iter()
        .map(|monster| (monster.corruption, monster.species_id.clone()))
        .collect::<Vec<_>>();
    corruption.sort();
    println!("CORRUPTION {corruption:?}");

    println!(
        "BUILDINGS {:?}",
        game_state.town.constructed_building_ids.clone()
    );

    for floor in &data.floors.floors {
        let surveys = game_state
            .town
            .floor_surveys
            .iter()
            .find(|entry| entry.floor_id == floor.id)
            .map(|entry| entry.surveys)
            .unwrap_or_default();
        let unlocked = game_state.town.unlocked_floor_ids.contains(&floor.id);
        println!(
            "FLOOR depth {:>2} {:<24} unlocked={:<5} surveys={}",
            floor.depth, floor.id, unlocked, surveys
        );
    }
}
