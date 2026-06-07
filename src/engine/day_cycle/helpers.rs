use super::*;

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

pub(super) fn egg_quality_rank(grade_score: u32) -> u8 {
    match grade_score {
        0..=2 => 1,
        3..=4 => 2,
        _ => 3,
    }
}

pub(super) fn quality_income_multiplier_pct(quality_rank: u8) -> u32 {
    match quality_rank.clamp(1, 3) {
        1 => 100,
        2 => 200,
        _ => 450,
    }
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
