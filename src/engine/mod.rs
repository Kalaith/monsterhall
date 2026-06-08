//! Stateless business logic services.

mod bootstrap;
mod day_cycle;
mod debt;
mod depth;
mod guest;
mod opening;
mod validation;

pub use bootstrap::create_new_game_state;
pub use day_cycle::{
    assign_monster_to_expedition, assign_monster_to_idle, assign_monster_to_rest,
    assign_monster_to_room, configure_expedition_plan, convert_egg, create_opening_egg,
    effective_population_cap, hatch_selected_egg, hatch_species, preview_expedition_plan,
    preview_guild_job, preview_upkeep, purchase_building, release_monster,
    replace_monster_with_selected_egg, resolve_day,
};
pub use debt::{debt_intro_status, initialize_first_debt, pay_debt_now};
pub(crate) use depth::{
    active_situation_guest_bonus, apply_monster_relationship_gain, complete_town_project_if_needed,
    contract_depth_score, contract_follow_up_request, contract_partial_success,
    expedition_depth_profile, floor_roster_gate_report, room_depth_profile_for_town,
    start_town_situation_from_event, tick_town_situations, upkeep_pressure_pct,
};
pub use guest::{
    assign_monster_to_contract, clear_contract_assignment, evaluate_contract_eligibility,
    refresh_contracts, ContractEligibilityReport,
};
pub use opening::{advance_opening_step, build_first_room, resolve_first_client};
pub use validation::validate_game_state_references;
