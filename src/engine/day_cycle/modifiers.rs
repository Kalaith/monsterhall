use super::*;

pub(super) fn collect_building_modifiers(
    data: &GameData,
    game_state: &GameState,
) -> BuildingAggregate {
    let mut combined = BuildingAggregate::default();

    for building_id in &game_state.town.constructed_building_ids {
        if let Some(building) = data
            .buildings
            .buildings
            .iter()
            .find(|entry| entry.id == *building_id)
        {
            combined.guild_income_pct += building.passive_modifiers.guild_income_pct;
            combined.expedition_success_pct += building.passive_modifiers.expedition_success_pct;
            combined.egg_discovery_flat += building.passive_modifiers.egg_discovery_flat;
            combined.injury_recovery_flat += building.passive_modifiers.injury_recovery_flat;
            combined.stress_recovery_flat += building.passive_modifiers.stress_recovery_flat;
            combined.charm_training_flat += building.passive_modifiers.charm_training_flat;
            combined.population_cap_flat += building.passive_modifiers.population_cap_flat;
        }
    }

    combined
}

pub(super) fn collect_trait_modifiers(data: &GameData, monster: &CompanionState) -> TraitAggregate {
    let mut aggregate = TraitAggregate::default();

    for trait_id in &monster.trait_ids {
        if let Some(trait_data) = data
            .traits
            .traits
            .iter()
            .find(|entry| entry.id == *trait_id)
        {
            aggregate.guild_income_pct += trait_data.guild_income_pct;
            aggregate.expedition_success_pct += trait_data.expedition_success_pct;
            aggregate.injury_risk_pct += trait_data.injury_risk_pct;
            aggregate.stress_change_flat += trait_data.stress_change_flat;
        }
    }

    aggregate
}

pub(super) fn active_client_tier_for_room<'a>(
    data: &'a GameData,
    town: &PlayerTownState,
    room: &'a crate::data::GuildRoomData,
) -> Result<&'a crate::data::PatronTierData, String> {
    let active_tier_id = town
        .patron_tiers
        .iter()
        .filter(|tier_id| room.patron_tiers.contains(*tier_id))
        .filter_map(|tier_id| {
            data.patron_tiers
                .patron_tiers
                .iter()
                .position(|entry| entry.id == *tier_id)
                .map(|index| (index, tier_id))
        })
        .max_by_key(|(index, _)| *index)
        .map(|(_, tier_id)| tier_id.clone())
        .ok_or_else(|| format!("Room '{}' has no active patron tier.", room.id))?;

    data.patron_tiers
        .patron_tiers
        .iter()
        .find(|entry| entry.id == active_tier_id)
        .ok_or_else(|| format!("Unknown patron tier '{}'.", active_tier_id))
}

#[derive(Default)]
pub(super) struct TraitAggregate {
    pub(super) guild_income_pct: i32,
    pub(super) expedition_success_pct: i32,
    pub(super) injury_risk_pct: i32,
    pub(super) stress_change_flat: i32,
}

#[derive(Default)]
pub(super) struct BuildingAggregate {
    pub(super) guild_income_pct: i32,
    pub(super) expedition_success_pct: i32,
    pub(super) egg_discovery_flat: i32,
    pub(super) injury_recovery_flat: i32,
    pub(super) stress_recovery_flat: i32,
    pub(super) charm_training_flat: i32,
    pub(super) population_cap_flat: i32,
}
