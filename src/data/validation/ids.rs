//! Unique-id domain checks and the collected id index.
use super::validate_unique_ids;
use crate::data::types::*;

impl GameData {
    pub(super) fn validate_unique_id_domains(&self) -> Result<(), String> {
        validate_unique_ids(
            self.species.species.iter().map(|entry| entry.id.as_str()),
            "species",
        )?;
        validate_unique_ids(
            self.buildings
                .buildings
                .iter()
                .map(|entry| entry.id.as_str()),
            "buildings",
        )?;
        validate_unique_ids(
            self.debt_milestones
                .milestones
                .iter()
                .map(|entry| entry.id.as_str()),
            "debt milestones",
        )?;
        validate_unique_ids(
            self.patron_archetypes
                .archetypes
                .iter()
                .map(|entry| entry.id.as_str()),
            "guest archetypes",
        )?;
        validate_unique_ids(
            self.contracts
                .requests
                .iter()
                .map(|entry| entry.id.as_str()),
            "contracts",
        )?;
        validate_unique_ids(
            self.patron_tiers
                .patron_tiers
                .iter()
                .map(|entry| entry.id.as_str()),
            "patron tiers",
        )?;
        validate_unique_ids(
            self.floors.floors.iter().map(|entry| entry.id.as_str()),
            "floors",
        )?;
        validate_unique_ids(
            self.missions.missions.iter().map(|entry| entry.id.as_str()),
            "missions",
        )?;
        validate_unique_ids(
            self.mutations
                .mutations
                .iter()
                .map(|entry| entry.id.as_str()),
            "mutations",
        )?;
        validate_unique_ids(
            self.monster_names
                .name_pools
                .iter()
                .map(|entry| entry.id.as_str()),
            "monster names",
        )?;
        validate_unique_ids(
            self.traits.traits.iter().map(|entry| entry.id.as_str()),
            "traits",
        )?;
        validate_unique_ids(
            self.guild_rooms.rooms.iter().map(|entry| entry.id.as_str()),
            "guild rooms",
        )?;
        validate_unique_ids(
            self.events.events.iter().map(|entry| entry.id.as_str()),
            "events",
        )?;
        validate_unique_ids(
            self.story_events
                .opening_steps
                .iter()
                .map(|entry| entry.id.as_str()),
            "story opening steps",
        )?;
        Ok(())
    }
}
