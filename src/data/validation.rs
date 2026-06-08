use std::collections::HashSet;

use super::depth_validation::validate_depth_systems;
use super::types::*;
use super::validation_helpers::validate_story_event_text_fields;

impl GameData {
    pub(super) fn validate(&self) -> Result<(), String> {
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

        validate_unique_ids(
            self.species.species.iter().map(|entry| entry.id.as_str()),
            "species",
        )?;
        validate_unique_ids(
            self.buildings
                .buildings
                .iter()
                .map(|entry| entry.id.as_str()),
            "buildings",
        )?;
        validate_unique_ids(
            self.debt_milestones
                .milestones
                .iter()
                .map(|entry| entry.id.as_str()),
            "debt milestones",
        )?;
        validate_unique_ids(
            self.patron_archetypes
                .archetypes
                .iter()
                .map(|entry| entry.id.as_str()),
            "guest archetypes",
        )?;
        validate_unique_ids(
            self.contracts
                .requests
                .iter()
                .map(|entry| entry.id.as_str()),
            "contracts",
        )?;
        validate_unique_ids(
            self.patron_tiers
                .patron_tiers
                .iter()
                .map(|entry| entry.id.as_str()),
            "patron tiers",
        )?;
        validate_unique_ids(
            self.floors.floors.iter().map(|entry| entry.id.as_str()),
            "floors",
        )?;
        validate_unique_ids(
            self.missions.missions.iter().map(|entry| entry.id.as_str()),
            "missions",
        )?;
        validate_unique_ids(
            self.mutations
                .mutations
                .iter()
                .map(|entry| entry.id.as_str()),
            "mutations",
        )?;
        validate_unique_ids(
            self.monster_names
                .name_pools
                .iter()
                .map(|entry| entry.id.as_str()),
            "monster names",
        )?;
        validate_unique_ids(
            self.traits.traits.iter().map(|entry| entry.id.as_str()),
            "traits",
        )?;
        validate_unique_ids(
            self.guild_rooms.rooms.iter().map(|entry| entry.id.as_str()),
            "guild rooms",
        )?;
        validate_unique_ids(
            self.events.events.iter().map(|entry| entry.id.as_str()),
            "events",
        )?;
        validate_unique_ids(
            self.story_events
                .opening_steps
                .iter()
                .map(|entry| entry.id.as_str()),
            "story opening steps",
        )?;

        let trait_ids = collect_ids(self.traits.traits.iter().map(|entry| entry.id.as_str()));
        let debt_milestone_ids = collect_ids(
            self.debt_milestones
                .milestones
                .iter()
                .map(|entry| entry.id.as_str()),
        );
        let guest_archetype_ids = collect_ids(
            self.patron_archetypes
                .archetypes
                .iter()
                .map(|entry| entry.id.as_str()),
        );
        let client_tier_ids = collect_ids(
            self.patron_tiers
                .patron_tiers
                .iter()
                .map(|entry| entry.id.as_str()),
        );
        let room_ids = collect_ids(self.guild_rooms.rooms.iter().map(|entry| entry.id.as_str()));
        let building_ids = collect_ids(
            self.buildings
                .buildings
                .iter()
                .map(|entry| entry.id.as_str()),
        );
        let floor_ids = collect_ids(self.floors.floors.iter().map(|entry| entry.id.as_str()));
        let species_ids = collect_ids(self.species.species.iter().map(|entry| entry.id.as_str()));
        let mission_ids = collect_ids(self.missions.missions.iter().map(|entry| entry.id.as_str()));
        let opening_step_ids = collect_ids(
            self.story_events
                .opening_steps
                .iter()
                .map(|entry| entry.id.as_str()),
        );

        if self.debt_milestones.milestones.is_empty() {
            return Err("debt_milestones.json must define at least one milestone.".to_owned());
        }
        if self.patron_archetypes.archetypes.is_empty() {
            return Err("patron archetype catalog must define at least one archetype.".to_owned());
        }
        if self.contracts.requests.is_empty() {
            return Err("contract catalog must define at least one request.".to_owned());
        }

        if !debt_milestone_ids.contains(self.debt_milestones.first_milestone_id.as_str()) {
            return Err(format!(
                "debt_milestones.json first_milestone_id '{}' was not found in milestones.",
                self.debt_milestones.first_milestone_id
            ));
        }

        for required_step_id in [
            "camp",
            "discovery",
            "incubation",
            "hatch",
            "build_room",
            "first_client",
        ] {
            if !opening_step_ids.contains(required_step_id) {
                return Err(format!(
                    "story_events.json must define opening step '{}'.",
                    required_step_id
                ));
            }
        }

        for opening_step in &self.story_events.opening_steps {
            if opening_step.title.trim().is_empty() {
                return Err(format!(
                    "opening step '{}' must contain a title.",
                    opening_step.id
                ));
            }
            if opening_step.body_lines.is_empty() {
                return Err(format!(
                    "opening step '{}' must contain at least one body line.",
                    opening_step.id
                ));
            }
            if opening_step.primary_action_label.trim().is_empty() {
                return Err(format!(
                    "opening step '{}' must contain a primary action label.",
                    opening_step.id
                ));
            }
        }

        for milestone in &self.debt_milestones.milestones {
            if milestone.name.trim().is_empty() {
                return Err(format!(
                    "debt milestone '{}' must contain a name.",
                    milestone.id
                ));
            }
            if milestone.description.trim().is_empty() {
                return Err(format!(
                    "debt milestone '{}' must contain a description.",
                    milestone.id
                ));
            }
            if milestone.amount_due == 0 {
                return Err(format!(
                    "debt milestone '{}' must define a positive amount_due.",
                    milestone.id
                ));
            }
            if milestone.days_allowed == 0 {
                return Err(format!(
                    "debt milestone '{}' must define a positive days_allowed.",
                    milestone.id
                ));
            }
            if let Some(next_milestone_id) = &milestone.next_milestone_id {
                if next_milestone_id == &milestone.id {
                    return Err(format!(
                        "debt milestone '{}' cannot point to itself as next_milestone_id.",
                        milestone.id
                    ));
                }
                if !debt_milestone_ids.contains(next_milestone_id.as_str()) {
                    return Err(format!(
                        "debt milestone '{}' references unknown next_milestone_id '{}'.",
                        milestone.id, next_milestone_id
                    ));
                }
            }
        }

        for archetype in &self.patron_archetypes.archetypes {
            if archetype.name.trim().is_empty() {
                return Err(format!(
                    "guest archetype '{}' must contain a name.",
                    archetype.id
                ));
            }
            if archetype.description.trim().is_empty() {
                return Err(format!(
                    "guest archetype '{}' must contain a description.",
                    archetype.id
                ));
            }
            if archetype.spawn_weight == 0 {
                return Err(format!(
                    "guest archetype '{}' must define a positive spawn_weight.",
                    archetype.id
                ));
            }
        }

        for request in &self.contracts.requests {
            if request.name.trim().is_empty() {
                return Err(format!("contract '{}' must contain a name.", request.id));
            }
            if request.description.trim().is_empty() {
                return Err(format!(
                    "contract '{}' must contain a description.",
                    request.id
                ));
            }
            if !guest_archetype_ids.contains(request.archetype_id.as_str()) {
                return Err(format!(
                    "contract '{}' references unknown archetype '{}'.",
                    request.id, request.archetype_id
                ));
            }
            if !room_ids.contains(request.requested_room_id.as_str()) {
                return Err(format!(
                    "contract '{}' references unknown room '{}'.",
                    request.id, request.requested_room_id
                ));
            }
            validate_reference_list(
                &request.required_species_ids,
                &species_ids,
                &format!("contract '{}'.required_species_ids", request.id),
            )?;
            if !(1..=3).contains(&request.minimum_quality_rank) {
                return Err(format!(
                    "contract '{}'.minimum_quality_rank must be between 1 and 3.",
                    request.id
                ));
            }
            if request.deadline_days == 0 {
                return Err(format!(
                    "contract '{}' must define a positive deadline_days.",
                    request.id
                ));
            }
            if companion_skill_progression_is_empty(&request.required_skill_thresholds)
                && work_history_progression_is_empty(&request.required_work_history_thresholds)
                && request.required_species_ids.is_empty()
                && request.minimum_quality_rank <= 1
            {
                return Err(format!(
                    "contract '{}' must define at least one real requirement.",
                    request.id
                ));
            }
        }

        if self.story_events.first_client_skill_gains.hospitality == 0
            && self.story_events.first_client_skill_gains.scouting == 0
            && self.story_events.first_client_skill_gains.charm == 0
        {
            return Err(
                "story_events.json must define meaningful first_client_skill_gains.".to_owned(),
            );
        }

        if self
            .story_events
            .first_client_work_history_gains
            .hospitality_jobs
            == 0
        {
            return Err(
                "story_events.json must define first_client_work_history_gains.hospitality_jobs."
                    .to_owned(),
            );
        }
        validate_story_event_text_fields(&self.story_events)?;

        for name_pool in &self.monster_names.name_pools {
            validate_reference_list(
                &name_pool.species_ids,
                &species_ids,
                &format!("monster name pool '{}'.species_ids", name_pool.id),
            )?;
            if name_pool.names.is_empty() {
                return Err(format!(
                    "monster name pool '{}' must contain at least one name.",
                    name_pool.id
                ));
            }
            for name in &name_pool.names {
                if name.trim().is_empty() {
                    return Err(format!(
                        "monster name pool '{}' contains a blank name.",
                        name_pool.id
                    ));
                }
            }
        }

        for species in &self.species.species {
            validate_non_negative_stats(&species.base_stats, &format!("species '{}'", species.id))?;
            validate_reference_list(
                &species.starting_traits,
                &trait_ids,
                &format!("species '{}'.starting_traits", species.id),
            )?;
            validate_reference_list(
                &species.preferred_room_ids,
                &room_ids,
                &format!("species '{}'.preferred_room_ids", species.id),
            )?;
            if !self
                .monster_names
                .name_pools
                .iter()
                .any(|pool| pool.species_ids.iter().any(|id| id == &species.id))
            {
                return Err(format!(
                    "species '{}' must have at least one monster name pool.",
                    species.id
                ));
            }
        }

        for mutation in &self.mutations.mutations {
            if !species_ids.contains(mutation.source_species_id.as_str()) {
                return Err(format!(
                    "mutation '{}' references unknown source species '{}'.",
                    mutation.id, mutation.source_species_id
                ));
            }
            if !species_ids.contains(mutation.target_species_id.as_str()) {
                return Err(format!(
                    "mutation '{}' references unknown target species '{}'.",
                    mutation.id, mutation.target_species_id
                ));
            }
            validate_reference_list(
                &mutation.required_trait_ids,
                &trait_ids,
                &format!("mutation '{}'.required_trait_ids", mutation.id),
            )?;
            validate_reference_list(
                &mutation.granted_trait_ids,
                &trait_ids,
                &format!("mutation '{}'.granted_trait_ids", mutation.id),
            )?;
            if mutation.event_text.trim().is_empty() {
                return Err(format!(
                    "mutation '{}' must contain event_text.",
                    mutation.id
                ));
            }
        }

        for room in &self.guild_rooms.rooms {
            if room.service_summary.trim().is_empty() {
                return Err(format!(
                    "guild room '{}' must define a service_summary.",
                    room.id
                ));
            }
            validate_reference_list(
                &room.required_building_ids,
                &building_ids,
                &format!("guild room '{}'.required_building_ids", room.id),
            )?;
            validate_reference_list(
                &room.preferred_trait_ids,
                &trait_ids,
                &format!("guild room '{}'.preferred_trait_ids", room.id),
            )?;
            validate_reference_list(
                &room.preferred_species_ids,
                &species_ids,
                &format!("guild room '{}'.preferred_species_ids", room.id),
            )?;
            validate_reference_list(
                &room.patron_tiers,
                &client_tier_ids,
                &format!("guild room '{}'.patron_tiers", room.id),
            )?;
            if room.trained_skill_ids.is_empty() {
                return Err(format!(
                    "guild room '{}' must define at least one trained_skill_id.",
                    room.id
                ));
            }
            for skill_id in &room.trained_skill_ids {
                if !is_valid_companion_skill_id(skill_id) {
                    return Err(format!(
                        "guild room '{}' references unknown trained skill '{}'.",
                        room.id, skill_id
                    ));
                }
            }
            if room.base_gold_yield == 0 {
                return Err(format!("guild room '{}' must generate gold.", room.id));
            }
            if room.work_history_gains.scouting_runs == 0
                && room.work_history_gains.guard_duties == 0
                && room.work_history_gains.hospitality_jobs == 0
                && room.work_history_gains.craft_jobs == 0
                && room.work_history_gains.contracts_completed == 0
                && room.work_history_gains.recovery_shifts == 0
                && room.work_history_gains.hatchery_assists == 0
            {
                return Err(format!(
                    "guild room '{}' must define at least one history gain.",
                    room.id
                ));
            }
        }

        for building in &self.buildings.buildings {
            validate_reference_list(
                &building.unlocks.room_ids,
                &room_ids,
                &format!("building '{}'.unlocks.room_ids", building.id),
            )?;
            validate_reference_list(
                &building.unlocks.floor_ids,
                &floor_ids,
                &format!("building '{}'.unlocks.floor_ids", building.id),
            )?;
            validate_reference_list(
                &building.unlocks.species_ids,
                &species_ids,
                &format!("building '{}'.unlocks.species_ids", building.id),
            )?;
            validate_reference_list(
                &building.unlocks.patron_tiers,
                &client_tier_ids,
                &format!("building '{}'.unlocks.patron_tiers", building.id),
            )?;
            if building.build_limit == 0 {
                return Err(format!(
                    "building '{}' must have a positive build_limit.",
                    building.id
                ));
            }
        }

        for floor in &self.floors.floors {
            validate_reference_list(
                &floor.requires_building_ids,
                &building_ids,
                &format!("floor '{}'.requires_building_ids", floor.id),
            )?;
            for requirement in &floor.required_roster {
                if !species_ids.contains(requirement.species_id.as_str()) {
                    return Err(format!(
                        "floor '{}'.required_roster references unknown species '{}'.",
                        floor.id, requirement.species_id
                    ));
                }
                if !(1..=3).contains(&requirement.minimum_quality_rank) {
                    return Err(format!(
                        "floor '{}'.required_roster for species '{}' must require 1 to 3 stars.",
                        floor.id, requirement.species_id
                    ));
                }
            }
            if floor.egg_species_entries.is_empty() {
                return Err(format!(
                    "floor '{}' must define at least one egg_species_entry.",
                    floor.id
                ));
            }
            for egg_entry in &floor.egg_species_entries {
                if !species_ids.contains(egg_entry.species_id.as_str()) {
                    return Err(format!(
                        "floor '{}'.egg_species_entries references unknown species '{}'.",
                        floor.id, egg_entry.species_id
                    ));
                }
                if egg_entry.weight == 0 {
                    return Err(format!(
                        "floor '{}'.egg_species_entries for species '{}' must have positive weight.",
                        floor.id, egg_entry.species_id
                    ));
                }
            }
            if floor.mission_ids.is_empty() {
                return Err(format!(
                    "floor '{}' must list at least one mission type.",
                    floor.id
                ));
            }
            validate_reference_list(
                &floor.mission_ids,
                &mission_ids,
                &format!("floor '{}'.mission_ids", floor.id),
            )?;
            if floor.difficulty == 0 {
                return Err(format!(
                    "floor '{}' must have a positive difficulty.",
                    floor.id
                ));
            }
        }

        for event in &self.events.events {
            validate_reference_list(
                &event.required_trait_ids,
                &trait_ids,
                &format!("event '{}'.required_trait_ids", event.id),
            )?;
            validate_reference_list(
                &event.required_species_ids,
                &species_ids,
                &format!("event '{}'.required_species_ids", event.id),
            )?;
            validate_reference_list(
                &event.required_building_ids,
                &building_ids,
                &format!("event '{}'.required_building_ids", event.id),
            )?;
            if event.text.trim().is_empty() {
                return Err(format!("event '{}' must contain text.", event.id));
            }
            if event.weight == Some(0) {
                return Err(format!(
                    "event '{}' weight must be greater than zero.",
                    event.id
                ));
            }
            if let Some(trigger_chance_pct) = event.trigger_chance_pct {
                if !(1..=100).contains(&trigger_chance_pct) {
                    return Err(format!(
                        "event '{}' trigger_chance_pct must be between 1 and 100.",
                        event.id
                    ));
                }
            }
            if event.reward.as_ref().is_some_and(|reward| reward.eggs > 0)
                || event.cost.as_ref().is_some_and(|cost| cost.eggs > 0)
            {
                return Err(format!(
                    "event '{}' cannot modify eggs directly; egg rewards must create inventory entries.",
                    event.id
                ));
            }
        }

        validate_reference_list(
            &self.config.new_game.starting_building_ids,
            &building_ids,
            "config.new_game.starting_building_ids",
        )?;
        validate_reference_list(
            &self.config.new_game.starting_room_ids,
            &room_ids,
            "config.new_game.starting_room_ids",
        )?;
        validate_reference_list(
            &self.config.new_game.starting_floor_ids,
            &floor_ids,
            "config.new_game.starting_floor_ids",
        )?;
        validate_reference_list(
            &self.config.new_game.starting_species_ids,
            &species_ids,
            "config.new_game.starting_species_ids",
        )?;

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
                &trait_ids,
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
            || self.config.day_cycle.girl_food_gold_per_day == 0
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

        self.ui_text.validate()?;
        validate_depth_systems(self)?;

        Ok(())
    }
}
fn validate_unique_ids<'a, I>(ids: I, domain_name: &str) -> Result<(), String>
where
    I: Iterator<Item = &'a str>,
{
    let mut seen_ids = HashSet::new();

    for id in ids {
        if id.trim().is_empty() {
            return Err(format!("{domain_name} contains an empty id."));
        }

        if !seen_ids.insert(id.to_owned()) {
            return Err(format!("{domain_name} contains duplicate id '{id}'."));
        }
    }

    Ok(())
}

fn is_valid_companion_skill_id(skill_id: &str) -> bool {
    matches!(
        skill_id,
        "scouting" | "guarding" | "hospitality" | "crafting" | "charm"
    )
}

fn companion_skill_progression_is_empty(progression: &CompanionSkillProgressionData) -> bool {
    progression.scouting == 0
        && progression.guarding == 0
        && progression.hospitality == 0
        && progression.crafting == 0
        && progression.charm == 0
}

fn work_history_progression_is_empty(progression: &CompanionWorkHistoryProgressionData) -> bool {
    progression.scouting_runs == 0
        && progression.guard_duties == 0
        && progression.hospitality_jobs == 0
        && progression.craft_jobs == 0
        && progression.contracts_completed == 0
        && progression.recovery_shifts == 0
        && progression.hatchery_assists == 0
}

fn validate_reference_list(
    ids: &[String],
    known_ids: &HashSet<&str>,
    label: &str,
) -> Result<(), String> {
    for id in ids {
        if !known_ids.contains(id.as_str()) {
            return Err(format!("{label} references unknown id '{id}'."));
        }
    }

    Ok(())
}

fn collect_ids<'a, I>(ids: I) -> HashSet<&'a str>
where
    I: Iterator<Item = &'a str>,
{
    ids.collect()
}

fn validate_non_negative_stats(stats: &StatBlockData, label: &str) -> Result<(), String> {
    if stats.power < 0 || stats.charm < 0 || stats.endurance < 0 || stats.instinct < 0 {
        return Err(format!("{label} contains a negative base stat."));
    }

    Ok(())
}
