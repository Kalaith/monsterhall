use super::*;

pub(super) fn test_game_data() -> GameData {
    GameData {
        config: parse_json(include_str!("../../../assets/data/config.json")),
        ui_text: parse_json(include_str!("../../../assets/data/ui_text.json")),
        debt_milestones: parse_json(include_str!("../../../assets/data/debt_milestones.json")),
        patron_archetypes: parse_json(include_str!("../../../assets/data/guest_archetypes.json")),
        contracts: parse_json(include_str!("../../../assets/data/guest_requests.json")),
        patron_tiers: parse_json(include_str!("../../../assets/data/client_tiers.json")),
        missions: parse_json(include_str!("../../../assets/data/missions.json")),
        mutations: parse_json(include_str!("../../../assets/data/mutations.json")),
        story_events: parse_json(include_str!("../../../assets/data/story_events.json")),
        monster_names: parse_json(include_str!("../../../assets/data/monster_names.json")),
        species: parse_json(include_str!("../../../assets/data/species.json")),
        buildings: parse_json(include_str!("../../../assets/data/buildings.json")),
        floors: parse_json(include_str!("../../../assets/data/floors.json")),
        traits: parse_json(include_str!("../../../assets/data/traits.json")),
        guild_rooms: parse_json(include_str!("../../../assets/data/guild_rooms.json")),
        events: parse_json(include_str!("../../../assets/data/events.json")),
    }
}

fn parse_json<T: serde::de::DeserializeOwned>(json: &str) -> T {
    serde_json::from_str(json).expect("test data should deserialize")
}
