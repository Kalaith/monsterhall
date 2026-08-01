//! JSON-backed data loading for game content catalogs.

#[cfg(not(target_arch = "wasm32"))]
use macroquad_toolkit::data_loader::load_data;
#[cfg(target_arch = "wasm32")]
use serde::de::DeserializeOwned;

use super::types::GameData;

#[cfg(not(target_arch = "wasm32"))]
pub async fn load_game_data() -> Result<GameData, String> {
    let data = GameData {
        config: load_data("config").await?,
        ui_text: load_data("ui_text").await?,
        debt_milestones: load_data("debt_milestones").await?,
        patron_archetypes: load_data("patron_archetypes").await?,
        contracts: load_data("contracts").await?,
        patron_tiers: load_data("patron_tiers").await?,
        missions: load_data("missions").await?,
        mutations: load_data("mutations").await?,
        story_events: load_data("story_events").await?,
        monster_names: load_data("monster_names").await?,
        species: load_data("species").await?,
        buildings: load_data("buildings").await?,
        floors: load_data("floors").await?,
        relics: load_data("relics").await?,
        traits: load_data("traits").await?,
        guild_rooms: load_data("guild_rooms").await?,
        events: load_data("events").await?,
    };

    data.validate()?;
    Ok(data)
}

#[cfg(target_arch = "wasm32")]
pub async fn load_game_data() -> Result<GameData, String> {
    let data = GameData {
        config: parse_embedded("config.json", include_str!("../../assets/data/config.json"))?,
        ui_text: parse_embedded(
            "ui_text.json",
            include_str!("../../assets/data/ui_text.json"),
        )?,
        debt_milestones: parse_embedded(
            "debt_milestones.json",
            include_str!("../../assets/data/debt_milestones.json"),
        )?,
        patron_archetypes: parse_embedded(
            "patron_archetypes.json",
            include_str!("../../assets/data/patron_archetypes.json"),
        )?,
        contracts: parse_embedded(
            "contracts.json",
            include_str!("../../assets/data/contracts.json"),
        )?,
        patron_tiers: parse_embedded(
            "patron_tiers.json",
            include_str!("../../assets/data/patron_tiers.json"),
        )?,
        missions: parse_embedded(
            "missions.json",
            include_str!("../../assets/data/missions.json"),
        )?,
        mutations: parse_embedded(
            "mutations.json",
            include_str!("../../assets/data/mutations.json"),
        )?,
        story_events: parse_embedded(
            "story_events.json",
            include_str!("../../assets/data/story_events.json"),
        )?,
        monster_names: parse_embedded(
            "monster_names.json",
            include_str!("../../assets/data/monster_names.json"),
        )?,
        species: parse_embedded(
            "species.json",
            include_str!("../../assets/data/species.json"),
        )?,
        buildings: parse_embedded(
            "buildings.json",
            include_str!("../../assets/data/buildings.json"),
        )?,
        floors: parse_embedded("floors.json", include_str!("../../assets/data/floors.json"))?,
        relics: parse_embedded("relics.json", include_str!("../../assets/data/relics.json"))?,
        traits: parse_embedded("traits.json", include_str!("../../assets/data/traits.json"))?,
        guild_rooms: parse_embedded(
            "guild_rooms.json",
            include_str!("../../assets/data/guild_rooms.json"),
        )?,
        events: parse_embedded("events.json", include_str!("../../assets/data/events.json"))?,
    };

    data.validate()?;
    Ok(data)
}

#[cfg(target_arch = "wasm32")]
fn parse_embedded<T: DeserializeOwned>(label: &str, json: &str) -> Result<T, String> {
    serde_json::from_str(json).map_err(|error| format!("JSON parse error in {label}: {error}"))
}

/// The shipped catalogs, parsed straight from `assets/data/`. Shared by every
/// test that needs real content rather than a hand-built fixture.
#[cfg(test)]
pub(crate) fn test_game_data() -> GameData {
    GameData {
        config: parse_test_json(include_str!("../../assets/data/config.json")),
        ui_text: parse_test_json(include_str!("../../assets/data/ui_text.json")),
        debt_milestones: parse_test_json(include_str!("../../assets/data/debt_milestones.json")),
        patron_archetypes: parse_test_json(include_str!(
            "../../assets/data/patron_archetypes.json"
        )),
        contracts: parse_test_json(include_str!("../../assets/data/contracts.json")),
        patron_tiers: parse_test_json(include_str!("../../assets/data/patron_tiers.json")),
        missions: parse_test_json(include_str!("../../assets/data/missions.json")),
        mutations: parse_test_json(include_str!("../../assets/data/mutations.json")),
        story_events: parse_test_json(include_str!("../../assets/data/story_events.json")),
        monster_names: parse_test_json(include_str!("../../assets/data/monster_names.json")),
        species: parse_test_json(include_str!("../../assets/data/species.json")),
        buildings: parse_test_json(include_str!("../../assets/data/buildings.json")),
        floors: parse_test_json(include_str!("../../assets/data/floors.json")),
        relics: parse_test_json(include_str!("../../assets/data/relics.json")),
        traits: parse_test_json(include_str!("../../assets/data/traits.json")),
        guild_rooms: parse_test_json(include_str!("../../assets/data/guild_rooms.json")),
        events: parse_test_json(include_str!("../../assets/data/events.json")),
    }
}

#[cfg(test)]
fn parse_test_json<T: serde::de::DeserializeOwned>(json: &str) -> T {
    serde_json::from_str(json).expect("test data should deserialize")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shipped catalogs are only cross-checked at game startup, so a broken
    /// reference between two JSON files would otherwise surface as a launch
    /// failure rather than a red test. Every id content authors write lands here.
    #[test]
    fn shipped_data_catalogs_cross_reference_cleanly() {
        test_game_data()
            .validate()
            .expect("shipped assets/data catalogs should pass load-time validation");
    }
}
