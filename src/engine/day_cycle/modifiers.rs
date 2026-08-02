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

/// The town's standing contribution to charm lessons, for callers outside this
/// module that cannot cheaply build the whole aggregate.
pub(crate) fn charm_training_bonus(data: &GameData, game_state: &GameState) -> i32 {
    collect_building_modifiers(data, game_state).charm_training_flat
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

/// The clientele a companion of this calibre draws in a given room.
///
/// Parties hire escorts who match what they are attempting. A companion is
/// booked by the best clientele she actually qualifies for — not by the room's
/// grandest patron at a discount. Only when she meets none of the room's
/// clientele does she work understrength, at a reduced fee.
pub(super) fn active_patron_tier_for_room<'a>(
    data: &'a GameData,
    town: &PlayerTownState,
    room: &'a crate::data::GuildRoomData,
    quality_rank: u8,
) -> Result<&'a crate::data::PatronTierData, String> {
    let available = || {
        town.patron_tiers
            .iter()
            .filter(|tier_id| room.patron_tiers.contains(*tier_id))
            .filter_map(|tier_id| {
                data.patron_tiers
                    .patron_tiers
                    .iter()
                    .find(|entry| &entry.id == tier_id)
            })
    };

    let qualified = available()
        .filter(|tier| quality_rank >= tier.minimum_quality_rank)
        .max_by_key(|tier| tier.income_multiplier_pct);

    qualified
        .or_else(|| available().min_by_key(|tier| tier.minimum_quality_rank))
        .ok_or_else(|| format!("Room '{}' has no active patron tier.", room.id))
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
