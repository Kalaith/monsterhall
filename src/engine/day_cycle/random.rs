//! Game randomness with a thread-owned stream available to deterministic tests.
//!
//! Macroquad's ordinary generator is process-global. That is appropriate for
//! play, but a seeded campaign test can run beside another random test and have
//! its stream advanced between any two draws. The override uses Macroquad's own
//! generator type and algorithm, so a seed produces the same sequence as
//! `macroquad::rand::srand` without sharing it with another test thread.

use std::cell::RefCell;

use macroquad::rand::{RandGenerator, RandomRange};

thread_local! {
    static THREAD_RNG: RefCell<Option<RandGenerator>> = const { RefCell::new(None) };
}

pub(super) fn gen_range<T>(low: T, high: T) -> T
where
    T: RandomRange,
{
    THREAD_RNG.with(|slot| {
        if let Some(generator) = slot.borrow().as_ref() {
            generator.gen_range(low, high)
        } else {
            macroquad_toolkit::rng::gen_range(low, high)
        }
    })
}

#[cfg(test)]
pub(crate) fn seed_simulation_rng(seed: u64) {
    let generator = RandGenerator::new();
    generator.srand(seed);
    THREAD_RNG.with(|slot| {
        slot.replace(Some(generator));
    });
}

#[cfg(test)]
pub(crate) fn clear_simulation_rng() {
    THREAD_RNG.with(|slot| {
        slot.replace(None);
    });
}
