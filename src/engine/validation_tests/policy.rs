use super::policy_buildings::purchase_priority_buildings;
use super::policy_eggs::hatch_affordable_eggs;
use super::policy_guests::{accepted_guest_booking_count, assign_guest_bookings};
use super::policy_jobs::assign_daily_jobs;
use super::*;

pub(super) fn play_opening_sequence(data: &GameData, game_state: &mut GameState) {
    advance_opening_step(data, game_state).expect("camp step should advance");
    advance_opening_step(data, game_state).expect("discovery step should advance");
    advance_opening_step(data, game_state).expect("incubation step should advance");
    advance_opening_step(data, game_state).expect("hatch step should advance");
    build_first_room(data, game_state).expect("first room should build");
    resolve_first_client(data, game_state).expect("first client should resolve");
    initialize_first_debt(data, game_state).expect("first debt should initialize");
    refresh_contracts(data, game_state).expect("contracts should refresh");
}

pub(super) fn run_daily_policy(data: &GameData, game_state: &mut GameState) -> DailyPolicyMetrics {
    let mut metrics = DailyPolicyMetrics::default();
    metrics.hatches += hatch_affordable_eggs(data, game_state);
    metrics.buildings_purchased += purchase_priority_buildings(data, game_state);
    assign_guest_bookings(data, game_state);
    metrics.guest_bookings_assigned = accepted_guest_booking_count(game_state);
    let (guild_job_workers, expedition_members) = assign_daily_jobs(data, game_state);
    metrics.guild_job_workers_assigned = guild_job_workers;
    metrics.expedition_members_assigned = expedition_members;
    if let Some(expedition) = &game_state.active_expedition {
        metrics.expedition_mission_id = Some(expedition.mission_id.clone());
        metrics.expedition_reward_focus = data
            .missions
            .missions
            .iter()
            .find(|mission| mission.id == expedition.mission_id)
            .map(|mission| mission.reward_focus.clone());
    }
    metrics
}
