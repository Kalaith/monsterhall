use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};

use macroquad::rand::srand;
use serde::Serialize;

use super::*;
use crate::engine::day_cycle;
use crate::engine::{
    advance_opening_step, assign_monster_to_expedition, assign_monster_to_contract,
    assign_monster_to_rest, assign_monster_to_room, build_first_room, configure_expedition_plan,
    convert_egg, create_new_game_state, create_opening_egg, hatch_selected_egg,
    initialize_first_debt, preview_expedition_plan, purchase_building, refresh_contracts,
    replace_monster_with_selected_egg, resolve_day, resolve_first_client,
};
use crate::state::{EggConversionKind, ExpeditionPriority, ContractStatus};

const SIMULATION_BASE_SEED: u64 = 0x5EED_1EAD_CAFE_BABE;
const LONG_CAMPAIGN_SEEDS: [(u32, u64); 3] = [
    (90, SIMULATION_BASE_SEED ^ 90),
    (180, SIMULATION_BASE_SEED ^ 180),
    (365, SIMULATION_BASE_SEED ^ 365),
];
const MULTI_SAMPLE_365_SEEDS: [u64; 10] = [
    0x3650_0001,
    0x3650_0002,
    0x3650_0003,
    0x3650_0004,
    0x3650_0005,
    0x3650_0006,
    0x3650_0007,
    0x3650_0008,
    0x3650_0009,
    0x3650_000A,
];

fn simulation_rng_guard() -> MutexGuard<'static, ()> {
    static SIMULATION_RNG_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    SIMULATION_RNG_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

mod fixtures;
mod policy;
mod reporting;
mod scenarios;

use fixtures::*;
use policy::*;
use reporting::*;
