use super::*;
use crate::data::SpeciesData;

pub(super) fn find_monster_mut<'a>(
    game_state: &'a mut GameState,
    monster_id: &str,
) -> Result<&'a mut CompanionState, String> {
    game_state
        .monsters
        .iter_mut()
        .find(|monster| monster.id == monster_id)
        .ok_or_else(|| format!("Unknown monster id '{monster_id}'."))
}

pub(super) fn can_afford(
    resources: &crate::state::ResourcesState,
    cost: &ResourceAmountData,
) -> bool {
    resources.gold >= cost.gold
        && resources.tower_materials >= cost.tower_materials
        && resources.eggs >= cost.eggs
        && resources.relics >= cost.relics
        && resources.arcane_residue >= cost.arcane_residue
}

pub(super) fn spend_resources(
    resources: &mut crate::state::ResourcesState,
    cost: &ResourceAmountData,
) {
    resources.gold -= cost.gold;
    resources.tower_materials -= cost.tower_materials;
    resources.eggs -= cost.eggs;
    resources.relics -= cost.relics;
    resources.arcane_residue -= cost.arcane_residue;
}

/// Rank a hatched companion lands at, from the grade score of their egg.
///
/// The ladder is data so the tower can keep producing better companions all the
/// way down. It used to top out at rank 3, which a depth-3 floor already
/// reached — every floor below that added nothing to the roster.
/// Star rating an egg of this grade will hatch into.
///
/// Config-driven, and the only correct answer. The hatchery screen carried its
/// own hardcoded copy that capped at three stars, so a grade-17 egg — which
/// hatches a rank-5 companion earning ten times a rank-1 — was displayed as a
/// three, and the at-cap replacement suggestion was computed against the wrong
/// number too.
pub fn egg_quality_rank(day_cycle: &DayCycleConfigData, grade_score: u32) -> u8 {
    let rank = day_cycle
        .egg_quality_rank_thresholds
        .iter()
        .filter(|threshold| grade_score >= **threshold)
        .count()
        + 1;
    rank.min(max_quality_rank(day_cycle) as usize) as u8
}

pub(super) fn max_quality_rank(day_cycle: &DayCycleConfigData) -> u8 {
    (day_cycle.egg_quality_rank_thresholds.len() + 1).max(1) as u8
}

/// What an adventuring party pays for an escort of this calibre.
pub(super) fn quality_income_multiplier_pct(
    day_cycle: &DayCycleConfigData,
    quality_rank: u8,
) -> u32 {
    rank_multiplier_pct(&day_cycle.quality_income_multipliers_pct, quality_rank)
}

/// What the guild pays that escort. Trails the income curve on purpose: a
/// stronger roster is still worth having, just not for free.
pub(super) fn quality_wage_multiplier_pct(day_cycle: &DayCycleConfigData, quality_rank: u8) -> u32 {
    rank_multiplier_pct(&day_cycle.quality_wage_multipliers_pct, quality_rank)
}

/// What a rank-indexed payout curve pays this rank, holding at the last entry
/// rather than falling to zero if the ladder ever outgrows the curve.
pub(super) fn rank_payout(curve: &[u32], quality_rank: u8) -> u32 {
    let index = usize::from(quality_rank.max(1)) - 1;
    curve
        .get(index)
        .or_else(|| curve.last())
        .copied()
        .unwrap_or(0)
}

fn rank_multiplier_pct(curve: &[u32], quality_rank: u8) -> u32 {
    let index = usize::from(quality_rank.max(1)) - 1;
    curve
        .get(index)
        .or_else(|| curve.last())
        .copied()
        .unwrap_or(100)
}

/// What a companion costs to keep, from what she is rather than only what she
/// hatched as.
///
/// `species` is the species she is *now*. Mutation rewrites that mid-campaign
/// and every step up the tree raises her base stats, so without this term a
/// `gargoyle_stairwarden` at 10/4/10/6 cost exactly what a `slime_companion` at
/// 3/2/5/2 cost — mutating up was free power. That is why the simulated guild
/// funnelled almost its whole roster into one late species and never wanted a
/// low tier again: nothing on the ledger said the strong ones were expensive.
pub(super) fn companion_daily_wage(
    day_cycle: &DayCycleConfigData,
    species: Option<&SpeciesData>,
    monster: &CompanionState,
) -> u32 {
    let rank_wage = day_cycle
        .companion_base_wage_gold
        .saturating_mul(quality_wage_multiplier_pct(day_cycle, monster.quality_rank))
        .div_ceil(100);
    // Every trained skill, not the five the wage formula was written against.
    // Wages are the guild's answer to a roster that earns more as it gets
    // stronger, and recovery and bargaining now feed `guild_job_skill_bonus`
    // exactly like the original five — leaving them out made training them free.
    let skill_total = monster.skills.scouting
        + monster.skills.guarding
        + monster.skills.hospitality
        + monster.skills.crafting
        + monster.skills.charm
        + monster.skills.recovery
        + monster.skills.bargaining
        + monster.skills.navigation
        + monster.skills.arcana
        + monster.skills.strength;
    // Base stats are already the game's measure of how capable a species is, so
    // the tier term reads them rather than adding a second authored ranking that
    // could drift from them.
    let species_wage = species.map_or(0, |species| {
        crate::engine::species_stat_total(species) / day_cycle.species_stat_wage_divisor.max(1)
    });
    rank_wage
        .saturating_add(skill_total / day_cycle.skill_wage_divisor.max(1))
        .saturating_add(species_wage)
}

pub(super) fn next_monster_id(game_state: &GameState) -> String {
    let next_number = game_state
        .monsters
        .iter()
        .filter_map(|monster| monster.id.strip_prefix("monster_"))
        .filter_map(|suffix| suffix.parse::<u32>().ok())
        .max()
        .unwrap_or(0)
        .saturating_add(1);
    format!("monster_{next_number:03}")
}

pub(super) fn next_egg_id(game_state: &GameState) -> String {
    let next_number = game_state
        .egg_inventory
        .iter()
        .filter_map(|egg| egg.id.strip_prefix("egg_"))
        .filter_map(|suffix| suffix.parse::<u32>().ok())
        .max()
        .unwrap_or(0)
        .saturating_add(1);
    format!("egg_{next_number:03}")
}

pub(super) fn unlock_building_content(
    town: &mut crate::state::PlayerTownState,
    building: &BuildingData,
) {
    add_missing_ids(&mut town.unlocked_room_ids, &building.unlocks.room_ids);
    add_missing_ids(&mut town.unlocked_floor_ids, &building.unlocks.floor_ids);
    add_missing_ids(
        &mut town.unlocked_species_ids,
        &building.unlocks.species_ids,
    );
    add_missing_ids(&mut town.patron_tiers, &building.unlocks.patron_tiers);
    if building.passive_modifiers.town_job_limit_flat > 0 {
        town.town_job_limit = town
            .town_job_limit
            .saturating_add(building.passive_modifiers.town_job_limit_flat as u8);
    }
}

pub(super) fn add_missing_ids(target: &mut Vec<String>, source: &[String]) {
    for value in source {
        if !target.contains(value) {
            target.push(value.clone());
        }
    }
}
