use super::*;
use crate::data::ResolutionOptionData;

impl Game {
    pub(super) fn update_fullscreen(&mut self, fullscreen: bool) {
        let Some(settings) = &mut self.app_settings else {
            return;
        };

        settings.fullscreen = fullscreen;
        apply_display_settings(settings);
        self.persist_settings();
        self.last_error = None;
    }

    pub(super) fn update_resolution(&mut self, resolution_id: &str) {
        let Some(data) = self.data.as_ref() else {
            self.last_error = Some("Game data is not available.".to_owned());
            return;
        };
        let Some(settings) = &mut self.app_settings else {
            return;
        };

        let Some(resolution) = data
            .config
            .display
            .available_resolutions
            .iter()
            .find(|entry| entry.id == resolution_id)
        else {
            self.last_error = Some(format!("Unknown resolution '{resolution_id}'."));
            return;
        };

        settings.resolution_id = resolution.id.clone();
        settings.resolution_width = resolution.width;
        settings.resolution_height = resolution.height;
        apply_display_settings(settings);
        self.persist_settings();
        self.last_error = None;
    }

    pub(super) fn persist_settings(&mut self) {
        let Some(settings) = self.app_settings.as_ref() else {
            return;
        };

        if let Err(message) = save_app_settings(self.settings_identifier(), settings) {
            self.last_error = Some(message);
        }
    }

    pub(super) fn save_identifier(&self) -> &str {
        self.data
            .as_ref()
            .map(|data| {
                #[cfg(target_arch = "wasm32")]
                {
                    data.config.persistence.web_storage_key.as_str()
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    data.config.persistence.native_save_path.as_str()
                }
            })
            .unwrap_or({
                #[cfg(target_arch = "wasm32")]
                {
                    "monsterhall.save"
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    "save.json"
                }
            })
    }

    pub(super) fn settings_identifier(&self) -> &str {
        self.data
            .as_ref()
            .map(|data| {
                #[cfg(target_arch = "wasm32")]
                {
                    data.config.persistence.web_settings_key.as_str()
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    data.config.persistence.native_settings_path.as_str()
                }
            })
            .unwrap_or({
                #[cfg(target_arch = "wasm32")]
                {
                    "monsterhall.settings"
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    "settings.json"
                }
            })
    }
}

pub(super) fn load_or_default_settings(data: &GameData) -> AppSettings {
    let mut settings = None;
    if settings_exist(settings_identifier_for_data(data)) {
        settings = load_app_settings(settings_identifier_for_data(data)).ok();
    }

    let default_resolution = data
        .config
        .display
        .available_resolutions
        .iter()
        .find(|resolution| resolution.id == data.config.display.default_resolution_id)
        .unwrap_or_else(|| &data.config.display.available_resolutions[0]);

    let Some(settings) = settings else {
        return AppSettings {
            fullscreen: data.config.display.start_fullscreen,
            resolution_id: default_resolution.id.clone(),
            resolution_width: default_resolution.width,
            resolution_height: default_resolution.height,
        };
    };
    reconcile_resolution_against(
        settings,
        &data.config.display.available_resolutions,
        default_resolution,
    )
}

/// A saved file is not automatically a usable one. `AppSettings` is
/// `#[serde(default)]`, so a truncated or older save deserializes with the size
/// fields at zero, and `apply_display_settings` would hand `0.0, 0.0` to
/// `request_new_screen_size`. A resolution dropped from `available_resolutions`
/// in a later patch is subtler: the window still opens, but the settings screen
/// highlights nothing, so the player cannot tell which mode is live.
///
/// Both fall back to the configured default rather than being trusted.
fn reconcile_resolution_against(
    mut settings: AppSettings,
    available: &[ResolutionOptionData],
    default_resolution: &ResolutionOptionData,
) -> AppSettings {
    let saved = available
        .iter()
        .find(|resolution| resolution.id == settings.resolution_id);

    match saved {
        // Trust the list's dimensions over the file's: they are the same value,
        // and only one of the two is guaranteed to still be current.
        Some(resolution) => {
            settings.resolution_width = resolution.width;
            settings.resolution_height = resolution.height;
        }
        None => {
            settings.resolution_id = default_resolution.id.clone();
            settings.resolution_width = default_resolution.width;
            settings.resolution_height = default_resolution.height;
        }
    }
    settings
}

pub(super) fn apply_display_settings(settings: &AppSettings) {
    set_fullscreen(settings.fullscreen);
    request_new_screen_size(
        settings.resolution_width as f32,
        settings.resolution_height as f32,
    );
}

pub(super) fn settings_identifier_for_data(data: &GameData) -> &str {
    #[cfg(target_arch = "wasm32")]
    {
        data.config.persistence.web_settings_key.as_str()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        data.config.persistence.native_settings_path.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options() -> Vec<ResolutionOptionData> {
        vec![
            ResolutionOptionData {
                id: "1280x720".to_owned(),
                width: 1280,
                height: 720,
                label: "1280 x 720".to_owned(),
            },
            ResolutionOptionData {
                id: "1920x1080".to_owned(),
                width: 1920,
                height: 1080,
                label: "1920 x 1080".to_owned(),
            },
        ]
    }

    /// The failure this guards: a settings file written before a field existed
    /// deserializes to zero through `#[serde(default)]`, and the zero reaches
    /// `request_new_screen_size` unchallenged.
    #[test]
    fn a_saved_size_of_zero_is_replaced_by_the_lists_own_dimensions() {
        let options = options();
        let saved = AppSettings {
            fullscreen: false,
            resolution_id: "1280x720".to_owned(),
            resolution_width: 0,
            resolution_height: 0,
        };

        let reconciled = reconcile_resolution_against(saved, &options, &options[1]);

        assert_eq!(reconciled.resolution_id, "1280x720");
        assert_eq!(
            (reconciled.resolution_width, reconciled.resolution_height),
            (1280, 720)
        );
    }

    /// A resolution dropped from `available_resolutions` leaves the settings
    /// screen with nothing highlighted, so the player cannot see what is live.
    #[test]
    fn a_resolution_no_longer_offered_falls_back_to_the_configured_default() {
        let options = options();
        let saved = AppSettings {
            fullscreen: true,
            resolution_id: "800x600".to_owned(),
            resolution_width: 800,
            resolution_height: 600,
        };

        let reconciled = reconcile_resolution_against(saved, &options, &options[1]);

        assert_eq!(reconciled.resolution_id, "1920x1080");
        assert_eq!(
            (reconciled.resolution_width, reconciled.resolution_height),
            (1920, 1080)
        );
        assert!(
            reconciled.fullscreen,
            "unrelated preferences should survive"
        );
    }
}
