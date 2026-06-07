use super::*;

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
    if settings_exist(settings_identifier_for_data(data)) {
        if let Ok(settings) = load_app_settings(settings_identifier_for_data(data)) {
            return settings;
        }
    }

    let default_resolution = data
        .config
        .display
        .available_resolutions
        .iter()
        .find(|resolution| resolution.id == data.config.display.default_resolution_id)
        .unwrap_or_else(|| &data.config.display.available_resolutions[0]);

    AppSettings {
        fullscreen: data.config.display.start_fullscreen,
        resolution_id: default_resolution.id.clone(),
        resolution_width: default_resolution.width,
        resolution_height: default_resolution.height,
    }
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
