use super::*;

#[test]
fn invalid_expedition_mission_is_rejected_before_runtime_resolution() {
    let data = test_game_data();
    let mut game_state = crate::engine::create_new_game_state(&data);
    game_state.active_expedition = Some(crate::state::ExpeditionState {
        expedition_id: "expedition_001".to_owned(),
        floor_id: "floor_1_slick_cellars".to_owned(),
        mission_id: "missing_mission".to_owned(),
        priority: crate::state::ExpeditionPriority::Balanced,
        assigned_monster_ids: vec![],
    });

    let error = validate_game_state_references(&data, &game_state)
        .expect_err("invalid mission should be rejected");

    assert!(error.contains("unknown mission 'missing_mission'"));
}

/// The survey chain's unit tests all pass against synthetic floors, so they
/// would keep passing if the hook in `resolve_day` were never reached. This runs
/// the real day cycle and asserts expeditions actually leave survey progress
/// behind — the only thing that makes deeper floors reachable.
#[test]
fn running_expeditions_records_survey_progress_on_the_real_day_cycle() {
    let _rng_guard = simulation_rng_guard();
    seed_simulation(SIMULATION_BASE_SEED ^ 7);
    let data = test_game_data();
    let mut game_state = create_new_game_state(&data);
    play_opening_sequence(&data, &mut game_state);

    let mut expedition_days = 0u32;
    let mut successful_expeditions = 0u32;
    let mut failed_expeditions = 0u32;
    for _ in 0..30 {
        let policy_metrics = run_daily_policy(&data, &mut game_state);
        if policy_metrics.expedition_members_assigned > 0 {
            expedition_days += 1;
        }
        let summary = resolve_day(&data, &mut game_state);
        successful_expeditions += summary.expedition_successes;
        failed_expeditions += summary.expedition_failures;
    }

    assert!(
        expedition_days > 0,
        "the policy should have run at least one expedition in thirty days"
    );
    let surveyed: u32 = game_state
        .town
        .floor_surveys
        .iter()
        .map(|entry| entry.surveys)
        .sum();
    assert!(
        surveyed >= successful_expeditions,
        "each of the {successful_expeditions} successful expeditions should leave at least one survey point, got {surveyed}"
    );
    assert!(
        failed_expeditions > 0 && successful_expeditions + failed_expeditions == expedition_days,
        "the real day cycle should account for every run, got {successful_expeditions} successes and {failed_expeditions} failures across {expedition_days} days"
    );
    for entry in &game_state.town.floor_surveys {
        assert!(
            data.floors
                .floors
                .iter()
                .any(|floor| floor.id == entry.floor_id),
            "survey recorded against unknown floor '{}'",
            entry.floor_id
        );
    }
}

#[test]
fn thirty_day_simulation_keeps_gameplay_state_valid() {
    let _rng_guard = simulation_rng_guard();
    seed_simulation(SIMULATION_BASE_SEED ^ 30);
    let data = test_game_data();
    let mut game_state = create_new_game_state(&data);
    play_opening_sequence(&data, &mut game_state);

    let starting_log_len = game_state.event_log.len();
    let starting_day = game_state.current_day;
    let starting_resources = resources_snapshot(&game_state);
    let mut per_day = Vec::new();
    let mut total_hatches = 0usize;
    let mut total_buildings_purchased = 0usize;
    let mut total_guest_completions = 0usize;
    let mut total_guest_expirations = 0usize;
    let mut total_contracts_generated = 0usize;
    let mut total_contracts_rejected = 0usize;
    let mut total_guild_job_gold = 0u32;
    let mut total_contract_gold = 0u32;
    let mut total_expedition_prep_gold = 0u32;
    let mut total_expedition_prep_materials = 0u32;
    let mut total_expedition_prep_arcane_residue = 0u32;
    let mut total_expedition_prep_shortfall = 0u32;
    let mut total_upkeep_wage_gold = 0u32;
    let mut total_upkeep_cleaning_gold = 0u32;
    let mut total_upkeep_maintenance_gold = 0u32;
    let mut total_upkeep_gold = 0u32;
    let mut total_upkeep_shortfall = 0u32;
    let mut total_special_event_gold_delta = 0i32;
    let mut total_special_event_count = 0u32;
    let mut total_expedition_days = 0u32;
    let mut total_egg_focused_expedition_days = 0u32;
    let mut expedition_days_after_day_90 = 0u32;
    let mut egg_reward_days = 0u32;
    let mut egg_reward_days_after_day_90 = 0u32;
    let mut last_expedition_day = None;
    let mut total_expedition_eggs = 0u32;
    let mut total_expedition_successes = 0u32;
    let mut total_expedition_failures = 0u32;

    for _ in 0..30 {
        let policy_metrics = run_daily_policy(&data, &mut game_state);
        let request_start = request_start_snapshot(&data, &game_state);
        let resolved_day = game_state.current_day;
        let summary = resolve_day(&data, &mut game_state);

        assert_eq!(summary.resolved_day, resolved_day);
        assert_eq!(game_state.current_day, resolved_day + 1);
        assert_eq!(
            game_state.resources.eggs as usize,
            game_state.egg_inventory.len()
        );
        validate_game_state_references(&data, &game_state)
            .expect("simulated day should preserve valid references");
        total_hatches += policy_metrics.hatches;
        total_buildings_purchased += policy_metrics.buildings_purchased;
        let guest_completions = count_guest_completions(&summary);
        let guest_expirations = count_guest_expirations(&summary);
        let guest_pressure = guest_pressure_metrics(&data, &game_state, &request_start, &summary);
        total_guest_completions += guest_completions;
        total_guest_expirations += guest_expirations;
        total_contracts_generated += guest_pressure.generated;
        total_contracts_rejected += guest_pressure.rejected;
        total_guild_job_gold += summary.guild_job_gold;
        total_contract_gold += summary.contract_gold;
        total_expedition_prep_gold += summary.expedition_prep_gold;
        total_expedition_prep_materials += summary.expedition_prep_materials;
        total_expedition_prep_arcane_residue += summary.expedition_prep_arcane_residue;
        total_expedition_prep_shortfall += summary.expedition_prep_shortfall;
        total_upkeep_wage_gold += summary.upkeep_wage_gold;
        total_upkeep_cleaning_gold += summary.upkeep_cleaning_gold;
        total_upkeep_maintenance_gold += summary.upkeep_maintenance_gold;
        total_upkeep_gold += summary.upkeep_gold;
        total_upkeep_shortfall += summary.upkeep_shortfall;
        total_special_event_gold_delta += summary.special_event_gold_delta;
        total_special_event_count += summary.special_event_count;
        if policy_metrics.expedition_members_assigned > 0 {
            total_expedition_days += 1;
            if policy_metrics.expedition_reward_focus.as_deref() == Some("eggs") {
                total_egg_focused_expedition_days += 1;
            }
            last_expedition_day = Some(summary.resolved_day);
            if summary.resolved_day > 90 {
                expedition_days_after_day_90 += 1;
            }
        }
        if summary.expedition_eggs > 0 {
            egg_reward_days += 1;
            if summary.resolved_day > 90 {
                egg_reward_days_after_day_90 += 1;
            }
        }
        total_expedition_eggs += summary.expedition_eggs;
        total_expedition_successes += summary.expedition_successes;
        total_expedition_failures += summary.expedition_failures;
        per_day.push(build_day_report(
            &data,
            &game_state,
            &summary,
            &policy_metrics,
            &request_start,
            guest_completions,
            guest_expirations,
        ));
    }

    assert_eq!(game_state.current_day, 31);
    assert!(game_state.story_progress.first_client_completed);
    assert!(!game_state.monsters.is_empty());
    assert!(game_state.event_log.len() > starting_log_len);
    assert!(
        !game_state.town.constructed_building_ids.is_empty(),
        "thirty-day simulation should show at least one building investment"
    );
    assert!(
        game_state.story_progress.first_creditor_visit_seen,
        "debt flow should remain active in the simulation"
    );
    let report = SimulationReport {
        content_version: data.config.content_version.clone(),
        rng_seed: SIMULATION_BASE_SEED ^ 30,
        simulation_days: 30,
        starting_day,
        ending_day: game_state.current_day,
        opening_event_log_entries: starting_log_len,
        final_event_log_entries: game_state.event_log.len(),
        final_roster_size: game_state.monsters.len(),
        final_buildings: game_state.town.constructed_building_ids.len(),
        final_unlocked_floors: game_state.town.unlocked_floor_ids.len(),
        final_stranded_floor_ids: super::super::super::depth::stranded_floor_ids(
            &data,
            &game_state,
        ),
        final_active_contracts: game_state.active_contracts.len(),
        final_average_bond: average_bond(&game_state),
        final_average_reputation: average_reputation(&game_state),
        final_graded_eggs: graded_egg_count(&game_state),
        final_role_diversity: role_diversity(&data, &game_state),
        final_species_counts: species_counts(&game_state),
        final_corruption_max: corruption_max(&game_state),
        final_town_projects: game_state.town.completed_project_ids.len(),
        sink_absorbed: sink_absorbed(&data, &game_state),
        sink_capacity: sink_capacity(&data),
        total_hatches,
        total_buildings_purchased,
        total_guest_completions,
        total_guest_expirations,
        total_contracts_generated,
        total_contracts_rejected,
        total_guild_job_gold,
        total_contract_gold,
        total_expedition_prep_gold,
        total_expedition_prep_materials,
        total_expedition_prep_arcane_residue,
        total_expedition_prep_shortfall,
        total_upkeep_wage_gold,
        total_upkeep_cleaning_gold,
        total_upkeep_maintenance_gold,
        total_upkeep_gold,
        total_upkeep_shortfall,
        total_special_event_gold_delta,
        total_special_event_count,
        total_expedition_days,
        total_egg_focused_expedition_days,
        expedition_days_after_day_90,
        egg_reward_days,
        egg_reward_days_after_day_90,
        last_expedition_day,
        total_expedition_eggs,
        total_expedition_successes,
        total_expedition_failures,
        final_resources: resources_snapshot(&game_state),
        final_debt: debt_snapshot(&game_state),
        final_upkeep_forecast: upkeep_forecast_snapshot(&data, &game_state),
        surplus_summary: surplus_summary(
            starting_resources,
            &game_state,
            total_upkeep_shortfall,
            total_expedition_prep_shortfall,
        ),
        milestone_snapshots: vec![milestone_snapshot(&data, &game_state, 30)],
        per_day,
    };
    let report_path = write_simulation_report(&report);
    println!("SIMULATION_REPORT={}", report_path.display());
    assert!(
        (5..=8).contains(&game_state.monsters.len()),
        "thirty-day simulation should end with an early stable roster, got {}",
        game_state.monsters.len()
    );
}

#[test]
fn debt_chain_stays_active_beyond_three_months() {
    let _rng_guard = simulation_rng_guard();
    seed_simulation(SIMULATION_BASE_SEED ^ 91);
    let data = test_game_data();
    let mut game_state = create_new_game_state(&data);
    play_opening_sequence(&data, &mut game_state);

    for _ in 0..90 {
        run_daily_policy(&data, &mut game_state);
        resolve_day(&data, &mut game_state);

        validate_game_state_references(&data, &game_state)
            .expect("simulated day should preserve valid references");
    }

    assert!(
        game_state.debt.is_some(),
        "debt chain should remain active beyond three months for the longer campaign arc"
    );
    assert!(
        game_state.current_day >= 91,
        "simulation should advance past the first three months"
    );
}

#[test]
fn thirty_day_policy_averages_about_six_companions() {
    let _rng_guard = simulation_rng_guard();
    let data = test_game_data();
    let simulation_count = 25usize;
    let mut final_roster_total = 0usize;
    let mut final_roster_min = usize::MAX;
    let mut final_roster_max = 0usize;
    let mut sample_outcomes = Vec::new();

    for sample_index in 0..simulation_count {
        seed_simulation(SIMULATION_BASE_SEED ^ 0x30 ^ sample_index as u64);
        let mut game_state = create_new_game_state(&data);
        play_opening_sequence(&data, &mut game_state);

        let mut successes = 0u32;
        let mut failures = 0u32;
        for _ in 0..30 {
            run_daily_policy(&data, &mut game_state);
            let summary = resolve_day(&data, &mut game_state);
            successes += summary.expedition_successes;
            failures += summary.expedition_failures;

            assert_eq!(
                game_state.resources.eggs as usize,
                game_state.egg_inventory.len()
            );
            validate_game_state_references(&data, &game_state)
                .expect("simulated day should preserve valid references");
        }

        let final_roster = game_state.monsters.len();
        final_roster_total += final_roster;
        final_roster_min = final_roster_min.min(final_roster);
        final_roster_max = final_roster_max.max(final_roster);
        sample_outcomes.push((sample_index, final_roster, successes, failures));
    }

    let average_final_roster = final_roster_total as f32 / simulation_count as f32;
    assert!(
        (6.0..=8.0).contains(&average_final_roster),
        "thirty-day policy should average an early stable roster, got {average_final_roster:.2}"
    );
    assert!(
        final_roster_min >= 5,
        "thirty-day policy should not collapse below 5 companions, got min {final_roster_min}: {sample_outcomes:?}"
    );
    assert!(
        final_roster_max <= 9,
        "thirty-day policy should not overshoot beyond 9 companions, got max {final_roster_max}"
    );
}

#[test]
fn hatching_respects_population_cap() {
    let data = test_game_data();
    let mut game_state = create_new_game_state(&data);
    play_opening_sequence(&data, &mut game_state);

    while game_state.monsters.len() < day_cycle::effective_population_cap(&data, &game_state) {
        let mut monster = game_state.monsters[0].clone();
        let next_number = game_state.monsters.len() + 1;
        monster.id = format!("monster_{next_number:03}");
        monster.name = format!("Cap Test {next_number}");
        game_state.monsters.push(monster);
    }

    game_state.resources.gold = 10_000;
    game_state.resources.arcane_residue = 10_000;
    create_opening_egg(&mut game_state, "slime_companion");

    let error = hatch_selected_egg(&data, &mut game_state, "egg_001", Some("slime_companion"))
        .expect_err("hatching at the population cap should fail");

    assert!(error.contains("population cap"));
    assert_eq!(
        game_state.monsters.len(),
        day_cycle::effective_population_cap(&data, &game_state)
    );
    assert_eq!(
        game_state.resources.eggs as usize,
        game_state.egg_inventory.len()
    );
}

#[test]
fn long_campaign_simulation_reports_stay_valid() {
    let _rng_guard = simulation_rng_guard();
    let data = test_game_data();
    for (simulation_days, rng_seed) in LONG_CAMPAIGN_SEEDS {
        let report = run_simulation_report_with_seed(&data, simulation_days, rng_seed);
        let report_path = write_named_simulation_report(
            &report,
            &format!("{simulation_days}_day_simulation_report.json"),
        );
        println!(
            "SIMULATION_REPORT_{}={}",
            simulation_days,
            report_path.display()
        );
        assert_eq!(report.simulation_days, simulation_days);
        assert!(
            report.final_roster_size <= usize::from(data.config.new_game.max_population_cap),
            "{}-day simulation exceeded population cap: {}",
            simulation_days,
            report.final_roster_size
        );
        match simulation_days {
            90 => assert!(
                (7..=16).contains(&report.final_roster_size),
                "90-day simulation should still show steady growth, got {} companions",
                report.final_roster_size
            ),
            180 => {
                assert!(
                    report.final_roster_size >= 10,
                    "180-day simulation should normally exceed 9 companions, got {}",
                    report.final_roster_size
                );
                assert!(
                    max_active_contracts(&report) >= 5,
                    "180-day simulation should show scaled guest pressure, max active {}",
                    max_active_contracts(&report)
                );
            }
            365 => {
                let room_gold = report
                    .total_guild_job_gold
                    .saturating_sub(report.total_contract_gold);
                assert!(
                    report.total_contract_gold > 0,
                    "the contract desk should contribute campaign income"
                );
                assert!(
                    u64::from(room_gold) * 100
                        <= u64::from(report.total_guild_job_gold) * 85,
                    "one repeatable source should not own the economy: room work supplied {room_gold} of {} earned gold",
                    report.total_guild_job_gold
                );
                // Deliberately relaxed from `== population_cap` when wages
                // landed. Filling the last slots used to be free and therefore
                // inevitable: a companion cost a flat 4 gold a day whatever she
                // was worth. Wages scale with rank and skill, so the final hires
                // are now a decision taken under debt pressure rather than a
                // formality. What still must hold is that the guild grows to
                // near its ceiling; a roster that stalls well short means the
                // wage load has broken growth.
                let population_cap = usize::from(data.config.new_game.max_population_cap);
                assert!(
                    report.final_roster_size + 2 >= population_cap,
                    "365-day simulation should grow to within two of the population cap of {population_cap}, got {} companions",
                    report.final_roster_size
                );
                assert!(
                    report.final_roster_size <= population_cap,
                    "365-day simulation exceeded the population cap of {population_cap}, got {} companions",
                    report.final_roster_size
                );
                assert!(
                    report.final_buildings >= 9,
                    "365-day simulation should unlock the current three-floor support chain, got {} buildings",
                    report.final_buildings
                );
                // The survey chain is serial, so anything that makes one link
                // unrunnable strands every floor beneath it — and nothing else
                // in this report notices, because a floor nobody enters changes
                // no number here. A mutation that consumed the last Minotaur
                // Porter closed `broodpens` and with it eleven floors, with
                // every other assertion in this test still green.
                //
                // This used to be asserted as `final_unlocked_floors == 25`,
                // which conflates a floor that *cannot* open with one the guild
                // simply never walked to. Route choice is not a defect, and
                // pinning it made the test fire on four separate correct
                // gameplay changes while catching nothing. `stranded_floor_ids`
                // measures the thing the comment above actually describes, and
                // does it regardless of where the simulated guild chose to go.
                assert!(
                    report.final_stranded_floor_ids.is_empty(),
                    "365-day simulation stranded floors behind gates that can never be met: {:?}",
                    report.final_stranded_floor_ids
                );
                assert!(
                    report.total_expedition_successes > report.total_expedition_failures
                        && report.total_expedition_failures > 0,
                    "tower resolution should produce both consequences and more successes than failures, got {} successes and {} failures",
                    report.total_expedition_successes,
                    report.total_expedition_failures
                );
                let final_debt = report.final_debt.as_ref().expect(
                    "365-day simulation should leave Founder's Due active for future floors",
                );
                assert!(
                    final_debt.status_message.contains("Founder's Due"),
                    "365-day simulation should only have final debt remaining, got '{}'",
                    final_debt.status_message
                );
                assert!(
                    report.final_resources.gold < final_debt.current_balance_due,
                    "three-floor economy should not fully cover Founder's Due, got {} gold against {} due",
                    report.final_resources.gold,
                    final_debt.current_balance_due
                );
                assert!(
                    final_debt.missed_payment_count <= 5,
                    "365-day simulation should not spiral into repeated debt misses, got {}",
                    final_debt.missed_payment_count
                );
            }
            _ => {}
        }
    }
}

#[test]
fn multi_seed_365_simulation_summary_reports_variance() {
    let _rng_guard = simulation_rng_guard();
    let data = test_game_data();
    let samples = MULTI_SAMPLE_365_SEEDS
        .iter()
        .enumerate()
        .map(|(index, seed)| {
            let report = run_simulation_report_with_seed(&data, 365, *seed);
            let final_debt = report.final_debt.as_ref();
            MultiSeedSimulationSample {
                sample: index + 1,
                rng_seed: *seed,
                companions: report.final_roster_size,
                buildings: report.final_buildings,
                gold: report.final_resources.gold,
                debt_gap: report.surplus_summary.debt_gold_gap,
                relics: report.final_resources.relics,
                arcane_residue: report.final_resources.arcane_residue,
                missed_payments: final_debt
                    .map(|debt| debt.missed_payment_count)
                    .unwrap_or(0),
                debt_milestone_id: final_debt.map(|debt| debt.active_milestone_id.clone()),
                debt_status: final_debt
                    .map(|debt| debt.status_message.clone())
                    .unwrap_or_else(|| "Debt cleared".to_owned()),
                max_active_requests: max_active_contracts(&report),
                expirations: report.total_guest_expirations,
            }
        })
        .collect::<Vec<_>>();
    let summary = MultiSeedSimulationSummary {
        simulation_days: 365,
        companions: summarize_usize(samples.iter().map(|sample| sample.companions)),
        buildings: summarize_usize(samples.iter().map(|sample| sample.buildings)),
        gold: summarize_u32(samples.iter().map(|sample| sample.gold)),
        debt_gap: summarize_i64(samples.iter().map(|sample| sample.debt_gap)),
        relics: summarize_u32(samples.iter().map(|sample| sample.relics)),
        arcane_residue: summarize_u32(samples.iter().map(|sample| sample.arcane_residue)),
        missed_payments: summarize_u32(samples.iter().map(|sample| sample.missed_payments)),
        max_active_requests: summarize_usize(
            samples.iter().map(|sample| sample.max_active_requests),
        ),
        expirations: summarize_usize(samples.iter().map(|sample| sample.expirations)),
        samples,
    };
    let report_path =
        write_named_simulation_report(&summary, "365_day_multi_seed_simulation_summary.json");
    println!("SIMULATION_MULTI_SEED_365={}", report_path.display());
    assert_eq!(summary.samples.len(), MULTI_SAMPLE_365_SEEDS.len());
    assert!(
        summary
            .samples
            .iter()
            .all(|sample| sample.companions <= usize::from(data.config.new_game.max_population_cap)),
        "multi-seed samples should stay within population cap"
    );
    // Also relaxed with the wage economy. Across seeds the roster now lands in
    // a band rather than pinning to the ceiling, which is the variance wages are
    // supposed to introduce. Two things still have to be true: some campaigns do
    // fill the hall, and none of them collapse.
    let population_cap = usize::from(data.config.new_game.max_population_cap);
    assert!(
        summary
            .samples
            .iter()
            .any(|sample| sample.companions == population_cap),
        "at least one seed should still fill the hall by day 365"
    );
    assert!(
        summary.companions.average >= (population_cap as f64) * 0.75,
        "multi-seed roster averaged {:.1}, well short of the {population_cap} cap - the wage load has broken growth",
        summary.companions.average
    );
    // Deliberately changed with the wage economy, and traded for a stricter
    // invariant. Every seed used to arrive at the founder's due by day 365
    // because gold was close to free; wages introduce real pacing variance, so
    // some campaigns are still working through the broker's compact. What must
    // hold now is that nobody is *failing*: not one seed may miss a payment,
    // and most must still reach the final window.
    assert!(
        summary.missed_payments.max == 0,
        "no seed should miss a debt payment, worst case missed {}",
        summary.missed_payments.max
    );
    let reached_final_window = summary
        .samples
        .iter()
        .filter(|sample| sample.debt_milestone_id.as_deref() == Some("founders_due_7"))
        .count();
    let cleared = summary
        .samples
        .iter()
        .filter(|sample| sample.debt_milestone_id.is_none())
        .count();
    assert!(
        reached_final_window + cleared > summary.samples.len() / 2,
        "most seeds should reach or clear the final debt window by day 365, got {reached_final_window} at it and {cleared} past it",
        );
    // Rewritten for an eleven-floor tower. This used to read "current
    // three-floor samples should not fully clear Founder's Due", from a point
    // where the campaign simply could not be finished. With bands one and two
    // authored and the escort economy paying for depth, some campaigns now win
    // — and that is the intended shape. What must stay true is that winning is
    // not a formality: it has to remain out of reach for a fair share of runs.
    assert!(
        cleared > 0,
        "an eleven-floor tower should let some campaigns clear Founder's Due"
    );
    assert!(
        cleared < summary.samples.len(),
        "clearing Founder's Due should not be guaranteed, but all {} seeds managed it",
        summary.samples.len()
    );
    assert!(
        summary.debt_gap.average < -500_000.0,
        "average day-365 debt gap should leave room for future floors, got {:.1}",
        summary.debt_gap.average
    );
    assert!(
        summary
            .samples
            .iter()
            .all(|sample| sample.missed_payments <= 5),
        "multi-seed samples should not spiral into repeated debt misses"
    );
}
pub(super) fn seed_simulation(seed: u64) {
    srand(seed);
}
