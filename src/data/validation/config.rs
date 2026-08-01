//! Config, display, and new-game validation.
use super::validate_reference_list;
use super::IdIndex;
use crate::data::types::*;
use std::collections::HashSet;

impl GameData {
    pub(super) fn validate_config(&self) -> Result<(), String> {
        if self.config.input.primary_mode != "mouse" {
            return Err("config.json must set input.primary_mode to \"mouse\".".to_owned());
        }

        if self.config.input.keyboard_shortcuts_visible {
            return Err(
                "config.json must keep keyboard shortcuts hidden from on-screen UI.".to_owned(),
            );
        }

        if self.config.persistence.native_save_path.trim().is_empty() {
            return Err("config.json must define a native save path.".to_owned());
        }

        if self.config.persistence.web_storage_key.trim().is_empty() {
            return Err("config.json must define a web storage key.".to_owned());
        }

        if self
            .config
            .persistence
            .native_settings_path
            .trim()
            .is_empty()
        {
            return Err("config.json must define a native settings path.".to_owned());
        }

        if self.config.persistence.web_settings_key.trim().is_empty() {
            return Err("config.json must define a web settings key.".to_owned());
        }

        if self.config.save_version == 0 {
            return Err("config.json save_version must be greater than zero.".to_owned());
        }

        if self.config.display.available_resolutions.is_empty() {
            return Err("config.json must define at least one display resolution.".to_owned());
        }

        let mut resolution_ids = HashSet::new();
        let mut found_default_resolution = false;
        for resolution in &self.config.display.available_resolutions {
            if resolution.width == 0 || resolution.height == 0 {
                return Err(format!(
                    "display resolution '{}' must have positive width and height.",
                    resolution.id
                ));
            }

            if !resolution_ids.insert(resolution.id.as_str()) {
                return Err(format!(
                    "config.json contains duplicate display resolution id '{}'.",
                    resolution.id
                ));
            }

            if resolution.id == self.config.display.default_resolution_id {
                found_default_resolution = true;
            }
        }

        if !found_default_resolution {
            return Err(format!(
                "config.json default_resolution_id '{}' was not found in available_resolutions.",
                self.config.display.default_resolution_id
            ));
        }
        Ok(())
    }

    pub(super) fn validate_new_game(&self, ids: &IdIndex<'_>) -> Result<(), String> {
        let IdIndex {
            trait_ids,
            species_ids,
            ..
        } = ids;
        for monster in &self.config.new_game.starter_monsters {
            if monster.name.trim().is_empty() {
                return Err("config.new_game.starter_monsters contains a blank name.".to_owned());
            }
            if !species_ids.contains(monster.species_id.as_str()) {
                return Err(format!(
                    "config.new_game.starter_monsters references unknown species '{}'.",
                    monster.species_id
                ));
            }
            validate_reference_list(
                &monster.extra_traits,
                trait_ids,
                &format!("starter monster '{}'.extra_traits", monster.name),
            )?;
        }

        if self.config.new_game.party_size == 0 {
            return Err("config.new_game.party_size must be greater than zero.".to_owned());
        }

        if self.config.new_game.town_job_limit == 0 {
            return Err("config.new_game.town_job_limit must be greater than zero.".to_owned());
        }

        if self.config.new_game.population_cap == 0 {
            return Err("config.new_game.population_cap must be greater than zero.".to_owned());
        }

        if self.config.new_game.max_population_cap < self.config.new_game.population_cap {
            return Err(
                "config.new_game.max_population_cap must be at least population_cap.".to_owned(),
            );
        }

        if self.config.new_game.starter_monsters.len()
            > usize::from(self.config.new_game.population_cap)
        {
            return Err(
                "config.new_game.starter_monsters exceeds config.new_game.population_cap."
                    .to_owned(),
            );
        }

        if self.config.day_cycle.worker_charm_gold_multiplier == 0
            || self.config.day_cycle.worker_instinct_residue_multiplier == 0
            || self.config.day_cycle.expedition_power_materials_multiplier == 0
            || self.config.day_cycle.expedition_instinct_residue_multiplier == 0
            || self.config.day_cycle.expedition_endurance_safety_divisor == 0
            || self.config.day_cycle.expedition_reward_success_divisor == 0
            || self.config.day_cycle.companion_food_gold_per_day == 0
            || self.config.day_cycle.building_maintenance_cost_divisor == 0
        {
            return Err(
                "config.json day_cycle multipliers and divisors must be greater than zero."
                    .to_owned(),
            );
        }

        for (index, band) in self.config.day_cycle.upkeep_bands.iter().enumerate() {
            if band.food_multiplier_pct == 0
                || band.cleaning_multiplier_pct == 0
                || band.maintenance_multiplier_pct == 0
            {
                return Err(format!(
                    "config.json day_cycle.upkeep_bands[{index}] multipliers must be greater than zero."
                ));
            }
        }

        if self.config.day_cycle.expedition_egg_reward_threshold <= 0
            || self.config.day_cycle.expedition_relic_reward_threshold <= 0
        {
            return Err(
                "config.json expedition reward thresholds must be greater than zero.".to_owned(),
            );
        }

        Ok(())
    }
}
