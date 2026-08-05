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
            || self.config.day_cycle.contract_income_multiplier_pct == 0
            || self.config.day_cycle.contract_penalty_pct == 0
            || self.config.day_cycle.building_maintenance_cost_divisor == 0
        {
            return Err(
                "config.json day_cycle multipliers and divisors must be greater than zero."
                    .to_owned(),
            );
        }

        self.validate_upkeep_bands()?;
        self.validate_economy_curves()?;
        self.validate_role_thresholds()?;

        if self.config.day_cycle.expedition_egg_reward_threshold <= 0
            || self.config.day_cycle.expedition_relic_reward_threshold <= 0
        {
            return Err(
                "config.json expedition reward thresholds must be greater than zero.".to_owned(),
            );
        }

        let day_cycle = &self.config.day_cycle;
        if day_cycle.expedition_min_success_chance_pct == 0
            || day_cycle.expedition_max_success_chance_pct > 100
            || day_cycle.expedition_min_success_chance_pct
                >= day_cycle.expedition_max_success_chance_pct
            || day_cycle.expedition_failure_salvage_pct >= 100
            || day_cycle.expedition_prep_shortfall_success_penalty == 0
            || day_cycle.recovery_focused_condition_cost_pct == 0
            || day_cycle.recovery_focused_condition_cost_pct >= 100
        {
            return Err(
                "config.json expedition chance, salvage, shortfall, and recovery values must define percentages inside 0..=100."
                    .to_owned(),
            );
        }

        self.validate_quality_rank_ladder()?;

        Ok(())
    }

    fn validate_economy_curves(&self) -> Result<(), String> {
        let event_curve = &self.config.day_cycle.special_event_gold_multipliers_pct;
        if event_curve.len() < self.patron_tiers.patron_tiers.len() || event_curve.contains(&0) {
            return Err(format!(
                "config.json day_cycle.special_event_gold_multipliers_pct must hold one positive entry for each of the {} patron tiers.",
                self.patron_tiers.patron_tiers.len()
            ));
        }
        if event_curve.windows(2).any(|pair| pair[0] > pair[1]) {
            return Err(
                "config.json day_cycle.special_event_gold_multipliers_pct must not decrease."
                    .to_owned(),
            );
        }
        Ok(())
    }

    /// The two axes an upkeep band escalates on, checked against what the game
    /// can actually supply.
    ///
    /// `active_upkeep_band` selects with `count >= threshold` on both axes, so a
    /// threshold of **zero is always satisfied** — it reads like "ignore this
    /// axis" and behaves like "this band is always active", which would pin the
    /// guild to its multipliers from day one. The opposite mistake is just as
    /// quiet: the top band asked for four patron tiers against a catalogue
    /// holding three, so that axis could never fire and the band ran on its
    /// companion count alone with nothing saying so. A band that does not use
    /// the tier axis says `None` now, and both mistakes are rejected here.
    fn validate_upkeep_bands(&self) -> Result<(), String> {
        let patron_tier_count = self.patron_tiers.patron_tiers.len() as u32;
        let max_population_cap = u32::from(self.config.new_game.max_population_cap);

        for (index, band) in self.config.day_cycle.upkeep_bands.iter().enumerate() {
            if band.wage_multiplier_pct == 0
                || band.cleaning_multiplier_pct == 0
                || band.maintenance_multiplier_pct == 0
            {
                return Err(format!(
                    "config.json day_cycle.upkeep_bands[{index}] multipliers must be greater than zero."
                ));
            }

            if band.min_companions == 0 {
                return Err(format!(
                    "config.json day_cycle.upkeep_bands[{index}].min_companions is 0, which is                      always satisfied and would make this band permanently active."
                ));
            }
            if band.min_companions > max_population_cap {
                return Err(format!(
                    "config.json day_cycle.upkeep_bands[{index}].min_companions is {} against a                      population cap of {max_population_cap}, so it can never be reached.",
                    band.min_companions
                ));
            }

            let Some(required_tiers) = band.min_patron_tiers else {
                continue;
            };
            if required_tiers == 0 {
                return Err(format!(
                    "config.json day_cycle.upkeep_bands[{index}].min_patron_tiers is 0, which is                      always satisfied. Omit the field to switch the axis off."
                ));
            }
            if required_tiers > patron_tier_count {
                return Err(format!(
                    "config.json day_cycle.upkeep_bands[{index}].min_patron_tiers is                      {required_tiers} against {patron_tier_count} authored patron tiers, so that                      axis can never fire. Omit the field, or author the tier."
                ));
            }
        }

        Ok(())
    }

    /// The rungs of `monster_role`'s ladder.
    ///
    /// The ladder is ordered, so a rung that catches everybody makes every rung
    /// below it dead. `corruption_adept_minimum` is the dangerous one, because
    /// corruption is only ever added to: a threshold there is a latch, and the
    /// question is only how many days it takes to close. The shipped value was
    /// `10` against rooms that add 1–2 a shift, so it closed inside a fortnight
    /// and `hatchery_specialist`, `performer`, `delver`, `comfort` and
    /// `versatile` were unreachable for the other 350 days of a campaign.
    ///
    /// No magnitude is *safe* — corruption only climbs, so every value is a
    /// latch and the only question is which day it shuts. What can be checked
    /// is whether the meter is pre-empting the mechanism that is supposed to do
    /// this job. Mutation is the game's own statement that corruption has
    /// changed what a companion is, and it carries `corruption_tuned` along
    /// most of its routes. While a companion still has any mutation ahead of
    /// her, the tower has not finished with her, and a raw meter reading must
    /// not overrule that. So a threshold is only accepted above the *last*
    /// mutation, where the meter is the only thing left that can say anything.
    fn validate_role_thresholds(&self) -> Result<(), String> {
        let thresholds = &self.config.day_cycle.role_thresholds;

        if thresholds.hatchery_assist_minimum == 0
            || thresholds.performer_charm_skill_minimum == 0
            || thresholds.performer_charm_margin == 0
            || thresholds.delver_power_margin == 0
            || thresholds.comfort_bond_minimum == 0
        {
            return Err(
                "config.json day_cycle.role_thresholds entries must be greater than zero, or the rung catches every companion and every rung below it is dead."
                    .to_owned(),
            );
        }

        let Some(minimum) = thresholds.corruption_adept_minimum else {
            return Ok(());
        };
        let last_mutation = self
            .mutations
            .mutations
            .iter()
            .map(|mutation| mutation.minimum_corruption)
            .max();
        if let Some(last) = last_mutation {
            if minimum <= last {
                return Err(format!(
                    "config.json day_cycle.role_thresholds.corruption_adept_minimum is {minimum}, at or below the last mutation at {last}. Corruption only ever climbs, so this latches every companion as an adept while the tower still has mutations left to change her by — and every rung below corruption_adept becomes unreachable. Omit the field to leave roles to traits and mutation."
                ));
            }
        }

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
