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
    for _ in 0..365 {
        run_daily_policy(&data, &mut game_state);
        resolve_day(&data, &mut game_state);
    }

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
