//! JSON-backed data loading for game content catalogs.

use macroquad_toolkit::data_loader::load_data;

use super::types::GameData;

pub async fn load_game_data() -> Result<GameData, String> {
    let data = GameData {
        config: load_data("config").await?,
        ui_text: load_data("ui_text").await?,
        debt_milestones: load_data("debt_milestones").await?,
        patron_archetypes: load_data("guest_archetypes").await?,
        contracts: load_data("guest_requests").await?,
        patron_tiers: load_data("client_tiers").await?,
        missions: load_data("missions").await?,
        mutations: load_data("mutations").await?,
        story_events: load_data("story_events").await?,
        monster_names: load_data("monster_names").await?,
        species: load_data("species").await?,
        buildings: load_data("buildings").await?,
        floors: load_data("floors").await?,
        traits: load_data("traits").await?,
        guild_rooms: load_data("guild_rooms").await?,
        events: load_data("events").await?,
    };

    data.validate()?;
    Ok(data)
}
