//! Cross-platform persistence with `save.json` on native and localStorage on web.

use crate::state::{AppSettings, SaveData};
use serde::{de::DeserializeOwned, Serialize};

#[cfg(target_arch = "wasm32")]
const TOOLKIT_GAME_NAME: &str = "monsterhall";

pub fn save_game(save_key: &str, save_data: &SaveData) -> Result<(), String> {
    save_json(save_key, save_data)
}

/// Save version that first stored the renamed content ids. Saves written before
/// it still name species, rooms, buildings, traits, patron tiers and missions by
/// their pre-rename ids, which no longer resolve against the data catalogs.
const RENAMED_CONTENT_ID_SAVE_VERSION: u32 = 10;

/// Old id -> new id, applied to saves older than [`RENAMED_CONTENT_ID_SAVE_VERSION`].
/// Ids are matched as whole quoted JSON tokens, so companion names and prose are
/// never touched.
const RENAMED_CONTENT_IDS: &[(&str, &str)] = &[
    // species
    ("slime_girl", "slime_companion"),
    ("succu_slime", "residue_slime"),
    ("goblin_maid", "imp_runner"),
    ("harpy_hostess", "harpy_lookout"),
    ("lamia_binder", "lamia_routekeeper"),
    ("hellhound_bouncer", "minotaur_porter"),
    ("moth_priestess", "moth_archivist"),
    // guild rooms
    ("vanilla_suite", "common_room"),
    ("public_stage", "reception_hall"),
    // buildings
    ("warm_love_nest", "nursery_habitat"),
    ("silk_rope_forge", "forge_corner"),
    ("monster_kink_archive", "species_archive"),
    ("aftercare_lounge", "recovery_lounge"),
    ("luxury_room_renovation", "guild_room_renovation"),
    ("healing_hot_springs", "recovery_baths"),
    // traits
    ("submissive", "calming_presence"),
    ("dominant", "commanding"),
    ("greedy", "opportunistic"),
    ("feral_instinct", "sharp_instinct"),
    // patron tiers
    ("local_adventurers", "local_delvers"),
    ("wealthy_nobles", "guild_sponsors"),
    ("monster_diplomats", "frontier_factions"),
    // missions
    ("corruption_dive", "scout_route"),
    // renamed story flag
    ("first_slimegirl_hatched", "first_companion_hatched"),
];

pub fn load_compatible_save_data(
    save_key: &str,
    current_save_version: u32,
) -> Result<(SaveData, bool), String> {
    let serialized = load_raw_json(save_key)?;
    let on_disk_version = macroquad_toolkit::persistence::peek_version_from_str(&serialized)?
        .and_then(|version| version.parse::<u32>().ok())
        .unwrap_or_default();

    if on_disk_version > current_save_version {
        return Err(format!(
            "Save version {on_disk_version} is newer than supported version {current_save_version}.",
        ));
    }

    let serialized = if on_disk_version < RENAMED_CONTENT_ID_SAVE_VERSION {
        migrate_renamed_content_ids(&serialized)
    } else {
        serialized
    };

    let mut save_data: SaveData = serde_json::from_str(&serialized)
        .map_err(|error| format!("Failed to parse {save_key}: {error}"))?;

    let was_migrated = save_data.version < current_save_version;
    if was_migrated {
        save_data.version = current_save_version;
    }

    Ok((save_data, was_migrated))
}

fn migrate_renamed_content_ids(serialized: &str) -> String {
    let mut migrated = serialized.to_owned();
    for (old_id, new_id) in RENAMED_CONTENT_IDS {
        migrated = migrated.replace(&format!("\"{old_id}\""), &format!("\"{new_id}\""));
    }
    migrated
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
    use crate::state::{CampaignFailureState, GameState};

    #[test]
    fn foreclosure_survives_a_save_round_trip() {
        let save = SaveData::new(
            11,
            GameState {
                current_day: 42,
                campaign_failure: Some(CampaignFailureState {
                    day: 42,
                    reason: "Monsterhall was foreclosed.".to_owned(),
                }),
                ..GameState::default()
            },
        );

        let encoded = serde_json::to_string(&save).expect("save should serialize");
        let restored: SaveData = serde_json::from_str(&encoded).expect("save should deserialize");

        assert_eq!(
            restored.game_state.campaign_failure,
            save.game_state.campaign_failure
        );
    }

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
            load_compatible_save_data(path.to_str().expect("temp path should be utf-8"), 10)
                .expect("older save should migrate");

        assert!(was_migrated);
        assert_eq!(save_data.version, 10);
        assert_eq!(save_data.game_state.current_day, 4);
        assert!(save_data
            .game_state
            .town
            .constructed_building_ids
            .is_empty());
        assert!(!save_data.game_state.story_progress.first_special_guest_seen);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn load_compatible_save_data_renames_pre_rename_content_ids() {
        let path = std::env::temp_dir().join(format!(
            "monsterhall_save_id_rename_{}.json",
            std::process::id()
        ));
        std::fs::write(
            &path,
            r#"{
  "version": 9,
  "game_state": {
    "current_day": 12,
    "resources": { "gold": 90, "tower_materials": 30, "eggs": 0, "relics": 0, "arcane_residue": 8 },
    "town": {
      "constructed_building_ids": ["aftercare_lounge"],
      "unlocked_room_ids": ["vanilla_suite", "public_stage"],
      "unlocked_floor_ids": ["floor_1_slick_cellars"],
      "unlocked_species_ids": ["slime_girl", "succu_slime"],
      "patron_tiers": ["wealthy_nobles"]
    },
    "monsters": [
      {
        "id": "monster_1",
        "name": "Dominant Kora",
        "species_id": "hellhound_bouncer",
        "trait_ids": ["dominant", "submissive"]
      }
    ]
  }
}"#,
        )
        .expect("temp save should be writable");

        let (save_data, was_migrated) =
            load_compatible_save_data(path.to_str().expect("temp path should be utf-8"), 10)
                .expect("pre-rename save should migrate");

        assert!(was_migrated);
        let town = &save_data.game_state.town;
        assert_eq!(town.constructed_building_ids, vec!["recovery_lounge"]);
        assert_eq!(
            town.unlocked_room_ids,
            vec!["common_room", "reception_hall"]
        );
        assert_eq!(
            town.unlocked_species_ids,
            vec!["slime_companion", "residue_slime"]
        );
        assert_eq!(town.patron_tiers, vec!["guild_sponsors"]);

        let monster = &save_data.game_state.monsters[0];
        assert_eq!(monster.species_id, "minotaur_porter");
        assert_eq!(monster.trait_ids, vec!["commanding", "calming_presence"]);
        // Prose and companion names are left alone: only whole quoted ids are rewritten.
        assert_eq!(monster.name, "Dominant Kora");

        let _ = std::fs::remove_file(path);
    }
}
