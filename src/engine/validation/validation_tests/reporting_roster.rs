//! Roster composition and progression snapshots for campaign reports.

use std::collections::BTreeMap;

use super::*;
use crate::state::GameState;

/// Delegates to `engine::monster_role` rather than classifying again.
///
/// This module used to carry its own copy of that branching — the third in the
/// codebase, after the engine's and the profile screen's. It had already
/// drifted: the engine scores on `effective_stats`, which includes trait
/// `stat_modifiers`, and this copy read the raw `monster.stats` those modifiers
/// adjust. So a companion whose traits pushed charm past power was a performer
/// in play and a delver in every report — the harness measuring role diversity
/// with arithmetic the game does not use.
pub(super) fn species_counts(game_state: &GameState) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for monster in &game_state.monsters {
        *counts.entry(monster.species_id.clone()).or_insert(0) += 1;
    }
    counts
}

pub(super) fn skill_totals(game_state: &GameState) -> BTreeMap<String, u32> {
    crate::engine::SKILL_IDS
        .into_iter()
        .map(|skill_id| {
            let total = game_state
                .monsters
                .iter()
                .map(|monster| crate::engine::companion_skill_value(&monster.skills, skill_id))
                .sum();
            (skill_id.to_owned(), total)
        })
        .collect()
}

pub(super) fn corruption_max(game_state: &GameState) -> u32 {
    game_state
        .monsters
        .iter()
        .map(|monster| monster.corruption)
        .max()
        .unwrap_or(0)
}

pub(super) fn role_diversity(data: &GameData, game_state: &GameState) -> usize {
    let mut roles = game_state
        .monsters
        .iter()
        .map(|monster| crate::engine::monster_role(data, monster))
        .collect::<Vec<_>>();
    roles.sort_unstable();
    roles.dedup();
    roles.len()
}
