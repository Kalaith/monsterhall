use super::*;

pub(super) fn guild_job_instability_gain(
    room: &crate::data::GuildRoomData,
    monster: &CompanionState,
) -> u32 {
    let mut gain = 0;

    if room.id == "packroom_annex" {
        gain += 2;
    } else if room.service_tier >= 3 {
        gain += 1;
    }

    if monster
        .trait_ids
        .iter()
        .any(|trait_id| trait_id == "corruption_tuned")
    {
        gain += 1;
    }

    gain
}

pub(crate) fn apply_guild_job_progression(
    monster: &mut CompanionState,
    room: &crate::data::GuildRoomData,
    is_guest_booking: bool,
) -> Option<String> {
    let actual_work_history_gains = roll_work_history_gains(room);
    let mut eligible_skill_ids = skill_ids_from_work_history_gains(&actual_work_history_gains)
        .into_iter()
        .filter(|skill_id| {
            room.trained_skill_ids
                .iter()
                .any(|trained| trained == skill_id)
        })
        .collect::<Vec<_>>();
    if should_gain_charm(room, &actual_work_history_gains, is_guest_booking) {
        eligible_skill_ids.push("charm".to_owned());
    }
    let mut gained_skills = Vec::new();
    if !eligible_skill_ids.is_empty() {
        let primary_index = gen_range(0, eligible_skill_ids.len());
        let primary_skill_id = eligible_skill_ids.remove(primary_index);
        if increment_skill(&mut monster.skills, &primary_skill_id, 1) {
            gained_skills.push(format!("{} +1", format_skill_name(&primary_skill_id)));
        }
    }

    if !eligible_skill_ids.is_empty() && gen_range(0, 100) < 40 {
        let secondary_index = gen_range(0, eligible_skill_ids.len());
        let secondary_skill_id = eligible_skill_ids.remove(secondary_index);
        if increment_skill(&mut monster.skills, &secondary_skill_id, 1) {
            gained_skills.push(format!("{} +1", format_skill_name(&secondary_skill_id)));
        }
    }

    apply_work_history_gains(&mut monster.work_history, &actual_work_history_gains);
    let history_summary = summarize_work_history_gains(&actual_work_history_gains);

    if gained_skills.is_empty() && history_summary == "no tracked work" {
        None
    } else if gained_skills.is_empty() {
        Some(format!(
            "{} handled {} work. History: {}.",
            monster.name, room.name, history_summary
        ))
    } else {
        Some(format!(
            "{} handled {} work. Skills: {}. History: {}.",
            monster.name,
            room.name,
            gained_skills.join(", "),
            history_summary
        ))
    }
}

pub(super) fn should_gain_charm(
    room: &crate::data::GuildRoomData,
    gains: &crate::data::CompanionWorkHistoryProgressionData,
    is_guest_booking: bool,
) -> bool {
    let has_intimate_history = gains.scouting_runs > 0
        || gains.guard_duties > 0
        || gains.hospitality_jobs > 0
        || gains.craft_jobs > 0
        || gains.recovery_shifts > 0;
    if !has_intimate_history {
        return false;
    }

    let chance_pct = match room.id.as_str() {
        "vanilla_suite" if is_guest_booking => 12,
        "public_stage" => {
            if is_guest_booking {
                80
            } else {
                65
            }
        }
        "packroom_annex" | "nursery_wing" => {
            if is_guest_booking {
                45
            } else {
                25
            }
        }
        _ if !room.required_building_ids.is_empty() => {
            if is_guest_booking {
                35
            } else {
                20
            }
        }
        _ => 0,
    };

    chance_pct > 0 && gen_range(0, 100) < chance_pct
}

pub(super) fn skill_ids_from_work_history_gains(
    gains: &crate::data::CompanionWorkHistoryProgressionData,
) -> Vec<String> {
    let mut skill_ids = Vec::new();

    if gains.scouting_runs > 0 {
        skill_ids.push("scouting".to_owned());
    }
    if gains.guard_duties > 0 {
        skill_ids.push("guarding".to_owned());
    }
    if gains.hospitality_jobs > 0 {
        skill_ids.push("hospitality".to_owned());
    }
    if gains.craft_jobs > 0 {
        skill_ids.push("crafting".to_owned());
    }

    skill_ids
}

pub(super) fn apply_work_history_gains(
    history: &mut CompanionWorkHistoryState,
    gains: &crate::data::CompanionWorkHistoryProgressionData,
) {
    history.scouting_runs = history.scouting_runs.saturating_add(gains.scouting_runs);
    history.guard_duties = history.guard_duties.saturating_add(gains.guard_duties);
    history.hospitality_jobs = history.hospitality_jobs.saturating_add(gains.hospitality_jobs);
    history.craft_jobs = history.craft_jobs.saturating_add(gains.craft_jobs);
    history.contracts_completed = history.contracts_completed.saturating_add(gains.contracts_completed);
    history.recovery_shifts = history
        .recovery_shifts
        .saturating_add(gains.recovery_shifts);
    history.hatchery_assists = history.hatchery_assists.saturating_add(gains.hatchery_assists);
}

pub(super) fn roll_work_history_gains(
    room: &crate::data::GuildRoomData,
) -> crate::data::CompanionWorkHistoryProgressionData {
    let mut gains = crate::data::CompanionWorkHistoryProgressionData::default();

    match room.id.as_str() {
        "vanilla_suite" => {
            gains.scouting_runs = roll_binary_gain(room.work_history_gains.scouting_runs, 70);
            gains.hospitality_jobs = roll_binary_gain(room.work_history_gains.hospitality_jobs, 45);
            gains.contracts_completed = roll_binary_gain(room.work_history_gains.contracts_completed, 12);
        }
        "packroom_annex" => {
            gains.craft_jobs = roll_binary_gain(room.work_history_gains.craft_jobs, 55);
            gains.recovery_shifts = roll_binary_gain(room.work_history_gains.recovery_shifts, 60);
        }
        "nursery_wing" => {
            gains.hospitality_jobs = roll_binary_gain(room.work_history_gains.hospitality_jobs, 65);
            gains.contracts_completed = roll_binary_gain(room.work_history_gains.contracts_completed, 30);
            gains.hatchery_assists = roll_binary_gain(room.work_history_gains.hatchery_assists, 5);
        }
        "public_stage" => {
            gains.scouting_runs = roll_binary_gain(room.work_history_gains.scouting_runs, 35);
            gains.guard_duties = roll_binary_gain(room.work_history_gains.guard_duties, 55);
            gains.recovery_shifts = roll_binary_gain(room.work_history_gains.recovery_shifts, 65);
        }
        _ => {
            gains.scouting_runs = roll_binary_gain(room.work_history_gains.scouting_runs, 50);
            gains.guard_duties = roll_binary_gain(room.work_history_gains.guard_duties, 35);
            gains.hospitality_jobs = roll_binary_gain(room.work_history_gains.hospitality_jobs, 50);
            gains.craft_jobs = roll_binary_gain(room.work_history_gains.craft_jobs, 35);
            gains.contracts_completed = roll_binary_gain(room.work_history_gains.contracts_completed, 15);
            gains.recovery_shifts = roll_binary_gain(room.work_history_gains.recovery_shifts, 40);
            gains.hatchery_assists = roll_binary_gain(room.work_history_gains.hatchery_assists, 5);
        }
    }

    gains
}

pub(super) fn roll_binary_gain(max_gain: u32, chance_pct: i32) -> u32 {
    if max_gain == 0 {
        return 0;
    }

    if gen_range(0, 100) < chance_pct {
        1
    } else {
        0
    }
}

pub(super) fn summarize_work_history_gains(gains: &crate::data::CompanionWorkHistoryProgressionData) -> String {
    let mut parts = Vec::new();

    if gains.scouting_runs > 0 {
        parts.push(format!("scouting runs +{}", gains.scouting_runs));
    }
    if gains.guard_duties > 0 {
        parts.push(format!("guarding +{}", gains.guard_duties));
    }
    if gains.hospitality_jobs > 0 {
        parts.push(format!("hospitality +{}", gains.hospitality_jobs));
    }
    if gains.craft_jobs > 0 {
        parts.push(format!("crafting +{}", gains.craft_jobs));
    }
    if gains.contracts_completed > 0 {
        parts.push(format!("completed contracts +{}", gains.contracts_completed));
    }
    if gains.recovery_shifts > 0 {
        parts.push(format!("recovery +{}", gains.recovery_shifts));
    }
    if gains.hatchery_assists > 0 {
        parts.push(format!("hatchery assists +{}", gains.hatchery_assists));
    }

    if parts.is_empty() {
        "no tracked work".to_owned()
    } else {
        parts.join(", ")
    }
}

pub(super) fn increment_skill(skills: &mut CompanionSkillState, skill_id: &str, gain: u32) -> bool {
    match skill_id {
        "scouting" => {
            skills.scouting = skills.scouting.saturating_add(gain);
            true
        }
        "guarding" => {
            skills.guarding = skills.guarding.saturating_add(gain);
            true
        }
        "hospitality" => {
            skills.hospitality = skills.hospitality.saturating_add(gain);
            true
        }
        "crafting" => {
            skills.crafting = skills.crafting.saturating_add(gain);
            true
        }
        "charm" => {
            skills.charm = skills.charm.saturating_add(gain);
            true
        }
        _ => false,
    }
}

pub(super) fn companion_skill_value(skills: &CompanionSkillState, skill_id: &str) -> u32 {
    match skill_id {
        "scouting" => skills.scouting,
        "guarding" => skills.guarding,
        "hospitality" => skills.hospitality,
        "crafting" => skills.crafting,
        "charm" => skills.charm,
        _ => 0,
    }
}

pub(super) fn format_skill_name(skill_id: &str) -> &'static str {
    match skill_id {
        "scouting" => "Scouting",
        "guarding" => "Guarding",
        "hospitality" => "Hospitality",
        "crafting" => "Crafting",
        "charm" => "Charm",
        _ => "Unknown",
    }
}

pub(super) fn expedition_corruption_gain(
    floor: &crate::data::TowerFloorData,
    mission: &crate::data::MissionData,
    monster: &CompanionState,
) -> u32 {
    let mut gain = 0;

    if mission.reward_focus == "residue" {
        gain += 6;
    } else if floor.depth >= 3 {
        gain += 1;
    }

    if monster
        .trait_ids
        .iter()
        .any(|trait_id| trait_id == "corruption_tuned")
    {
        gain += 1;
    }

    gain
}

pub(super) fn try_apply_mutation(
    data: &GameData,
    monster: &mut CompanionState,
) -> Option<String> {
    let mutation = data.mutations.mutations.iter().find(|mutation| {
        mutation.source_species_id == monster.species_id
            && monster.corruption >= mutation.minimum_corruption
            && mutation
                .required_trait_ids
                .iter()
                .all(|trait_id| monster.trait_ids.contains(trait_id))
    })?;

    let source_species = data
        .species
        .species
        .iter()
        .find(|species| species.id == mutation.source_species_id)?;
    let target_species = data
        .species
        .species
        .iter()
        .find(|species| species.id == mutation.target_species_id)?;

    monster.stats.power += target_species.base_stats.power - source_species.base_stats.power;
    monster.stats.charm += target_species.base_stats.charm - source_species.base_stats.charm;
    monster.stats.endurance +=
        target_species.base_stats.endurance - source_species.base_stats.endurance;
    monster.stats.instinct +=
        target_species.base_stats.instinct - source_species.base_stats.instinct;
    monster.species_id = target_species.id.clone();
    add_missing_traits(&mut monster.trait_ids, &target_species.starting_traits);
    add_missing_traits(&mut monster.trait_ids, &mutation.granted_trait_ids);

    Some(
        mutation
            .event_text
            .replace("{name}", &monster.name)
            .replace("{target_species}", &target_species.name),
    )
}

pub(super) fn add_missing_traits(target: &mut Vec<String>, source: &[String]) {
    for trait_id in source {
        if !target.contains(trait_id) {
            target.push(trait_id.clone());
        }
    }
}
