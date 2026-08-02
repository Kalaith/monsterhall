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
            || self.config.day_cycle.companion_base_wage_gold == 0
            || self.config.day_cycle.building_maintenance_cost_divisor == 0
        {
            return Err(
                "config.json day_cycle multipliers and divisors must be greater than zero."
                    .to_owned(),
            );
        }

        for (index, band) in self.config.day_cycle.upkeep_bands.iter().enumerate() {
            if band.wage_multiplier_pct == 0
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

        self.validate_quality_rank_ladder()?;

        Ok(())
    }

    /// The star ladder and everything indexed by it.
    ///
    /// `egg_quality_rank` reads a rank off `egg_quality_rank_thresholds` by
    /// counting how many it clears, so an unsorted list silently mis-ranks eggs.
    /// And every curve indexed by rank holds at its last entry when it runs
    /// short, so a curve authored for the old three-rank ladder pays ranks 4 and
    /// 5 whatever rank 3 got, with nothing to say so. Both are the shape that
    /// left the refinery capped at three stars for a whole escort economy.
    fn validate_quality_rank_ladder(&self) -> Result<(), String> {
        let day_cycle = &self.config.day_cycle;
        let thresholds = &day_cycle.egg_quality_rank_thresholds;
        if thresholds.is_empty() {
            return Err(
                "config.json day_cycle.egg_quality_rank_thresholds must not be empty.".to_owned(),
            );
        }
        if thresholds.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(
                "config.json day_cycle.egg_quality_rank_thresholds must ascend strictly."
                    .to_owned(),
            );
        }

        let max_rank = thresholds.len() + 1;
        for (field, curve) in [
            (
                "quality_income_multipliers_pct",
                &day_cycle.quality_income_multipliers_pct,
            ),
            (
                "quality_wage_multipliers_pct",
                &day_cycle.quality_wage_multipliers_pct,
            ),
            ("egg_sale_gold_by_rank", &day_cycle.egg_sale_gold_by_rank),
            (
                "egg_dissolve_residue_by_rank",
                &day_cycle.egg_dissolve_residue_by_rank,
            ),
        ] {
            if curve.len() < max_rank {
                return Err(format!(
                    "config.json day_cycle.{field} has {} entries against a {max_rank}-rank ladder.",
                    curve.len()
                ));
            }
        }

        if day_cycle.egg_dissolve_relic_minimum_rank == 0
            || usize::from(day_cycle.egg_dissolve_relic_minimum_rank) > max_rank
        {
            return Err(format!(
                "config.json day_cycle.egg_dissolve_relic_minimum_rank must fall inside 1..={max_rank}."
            ));
        }

        Ok(())
    }
}
