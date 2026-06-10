//! Cross-platform persistence with `save.json` on native and localStorage on web.

use crate::state::{AppSettings, SaveData};
use serde::{de::DeserializeOwned, Serialize};

#[cfg(target_arch = "wasm32")]
const TOOLKIT_GAME_NAME: &str = "monsterhall";

pub fn save_game(save_key: &str, save_data: &SaveData) -> Result<(), String> {
    save_json(save_key, save_data)
}

pub fn load_save_data(save_key: &str) -> Result<SaveData, String> {
    load_json(save_key)
}

pub fn load_compatible_save_data(
    save_key: &str,
    current_save_version: u32,
) -> Result<(SaveData, bool), String> {
    let mut save_data = load_save_data(save_key)?;
    if save_data.version > current_save_version {
        return Err(format!(
            "Save version {} is newer than supported version {}.",
            save_data.version, current_save_version
        ));
    }

    let was_migrated = save_data.version < current_save_version;
    if was_migrated {
        save_data.version = current_save_version;
    }

    Ok((save_data, was_migrated))
}

pub fn peek_save_version(save_key: &str) -> Result<Option<u32>, String> {
    let serialized = load_raw_json(save_key)?;
    macroquad_toolkit::persistence::peek_version_from_str(&serialized)?
        .map(|version| {
            version
                .parse::<u32>()
                .map_err(|error| format!("Invalid save version in {save_key}: {error}"))
        })
        .transpose()
}

pub fn save_exists(save_key: &str) -> bool {
    exists(save_key)
}

pub fn save_app_settings(settings_key: &str, settings: &AppSettings) -> Result<(), String> {
    save_json(settings_key, settings)
}

pub fn load_app_settings(settings_key: &str) -> Result<AppSettings, String> {
    load_json(settings_key)
}

pub fn settings_exist(settings_key: &str) -> bool {
    exists(settings_key)
}

#[cfg(target_arch = "wasm32")]
fn save_json<T: Serialize>(save_key: &str, value: &T) -> Result<(), String> {
    macroquad_toolkit::persistence::save_json_key(TOOLKIT_GAME_NAME, save_key, value)
        .map_err(|error| format!("Failed to write browser save key {save_key}: {error}"))
}

#[cfg(not(target_arch = "wasm32"))]
fn save_json<T: Serialize>(save_key: &str, value: &T) -> Result<(), String> {
    macroquad_toolkit::persistence::save_json_atomic(save_key, value)
        .map_err(|error| format!("Failed to write {save_key}: {error}"))
}

#[cfg(target_arch = "wasm32")]
fn load_json<T: DeserializeOwned>(save_key: &str) -> Result<T, String> {
    let serialized = load_raw_json(save_key)?;
    serde_json::from_str(&serialized)
        .map_err(|error| format!("Failed to parse browser save key {save_key}: {error}"))
}

#[cfg(not(target_arch = "wasm32"))]
fn load_json<T: DeserializeOwned>(save_key: &str) -> Result<T, String> {
    macroquad_toolkit::persistence::load_json(save_key)
        .map_err(|error| format!("Failed to load {save_key}: {error}"))
}

#[cfg(target_arch = "wasm32")]
fn load_raw_json(save_key: &str) -> Result<String, String> {
    let serialized = macroquad_toolkit::persistence::load_string_key(TOOLKIT_GAME_NAME, save_key)
        .or_else(|_| load_legacy_raw_browser_save(save_key))
        .map_err(|error| format!("Failed to load browser save key {save_key}: {error}"))?;
    if serialized.trim().is_empty() {
        return Err(format!("No browser save found for key '{save_key}'."));
    }
    Ok(serialized)
}

#[cfg(target_arch = "wasm32")]
fn load_legacy_raw_browser_save(save_key: &str) -> Result<String, String> {
    let serialized = macroquad_toolkit::wasm_storage::storage_get(save_key)
        .ok_or_else(|| format!("No legacy browser save found for key '{save_key}'."))?;
    if serialized.trim().is_empty() {
        return Err(format!(
            "No legacy browser save found for key '{save_key}'."
        ));
    }

    let _ =
        macroquad_toolkit::persistence::save_string_key(TOOLKIT_GAME_NAME, save_key, &serialized);
    Ok(serialized)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_raw_json(save_key: &str) -> Result<String, String> {
    std::fs::read_to_string(save_key).map_err(|error| format!("Failed to read {save_key}: {error}"))
}

#[cfg(target_arch = "wasm32")]
fn exists(save_key: &str) -> bool {
    load_raw_json(save_key).is_ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn exists(save_key: &str) -> bool {
    std::path::Path::new(save_key).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn peek_save_version_reads_version_without_full_deserialize() {
        let path = std::env::temp_dir().join(format!(
            "monsterhall_save_version_{}.json",
            std::process::id()
        ));
        std::fs::write(
            &path,
            r#"{"version":2,"game_state":{"unexpected":"older schema"}}"#,
        )
        .expect("temp save should be writable");

        let version = peek_save_version(path.to_str().expect("temp path should be utf-8"))
            .expect("peek should parse");

        assert_eq!(version, Some(2));

        let _ = std::fs::remove_file(path);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn load_compatible_save_data_migrates_older_partial_save() {
        let path = std::env::temp_dir().join(format!(
            "monsterhall_save_migration_{}.json",
            std::process::id()
        ));
        std::fs::write(
            &path,
            r#"{
  "version": 7,
  "game_state": {
    "current_day": 4,
    "resources": { "gold": 55, "tower_materials": 12, "eggs": 1, "relics": 0, "arcane_residue": 5 },
    "town": { "unlocked_floor_ids": ["floor_1_slick_cellars"], "unlocked_species_ids": ["slime_girl"] },
    "story_progress": { "opening_step": "Complete", "first_client_completed": true }
  }
}"#,
        )
        .expect("temp save should be writable");

        let (save_data, was_migrated) =
            load_compatible_save_data(path.to_str().expect("temp path should be utf-8"), 9)
                .expect("older save should migrate");

        assert!(was_migrated);
        assert_eq!(save_data.version, 9);
        assert_eq!(save_data.game_state.current_day, 4);
        assert!(save_data
            .game_state
            .town
            .constructed_building_ids
            .is_empty());
        assert_eq!(save_data.game_state.chamber.exposure_risk, 0);
        assert!(!save_data.game_state.story_progress.first_special_guest_seen);

        let _ = std::fs::remove_file(path);
    }
}
