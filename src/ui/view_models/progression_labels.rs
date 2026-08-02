//! Labels for what a companion has learned and what a shift might teach her.
//!
//! Split out of `view_models.rs` when that file crossed the 800-line limit.

use crate::data::GameData;
use crate::ui::view_models::fill_template;

pub fn trained_skills_label(data: &GameData, skill_ids: &[String]) -> String {
    let common_text = &data.ui_text.common;
    let labels = skill_ids
        .iter()
        .map(|skill_id| match skill_id.as_str() {
            "scouting" => common_text.skill_label_scouting.as_str(),
            "guarding" => common_text.skill_label_guarding.as_str(),
            "hospitality" => common_text.skill_label_hospitality.as_str(),
            "crafting" => common_text.skill_label_crafting.as_str(),
            "charm" => common_text.skill_label_charm.as_str(),
            _ => common_text.unknown_label.as_str(),
        })
        .collect::<Vec<_>>();

    if labels.is_empty() {
        common_text.none_label.clone()
    } else {
        labels.join(", ")
    }
}

pub fn primary_skill_label<'a>(data: &'a GameData, skill_ids: &[String]) -> &'a str {
    let common_text = &data.ui_text.common;
    skill_ids
        .first()
        .map(|skill_id| match skill_id.as_str() {
            "scouting" => common_text.skill_label_scouting.as_str(),
            "guarding" => common_text.skill_label_guarding.as_str(),
            "hospitality" => common_text.skill_label_hospitality.as_str(),
            "crafting" => common_text.skill_label_crafting.as_str(),
            "charm" => common_text.skill_label_charm.as_str(),
            _ => common_text.unknown_label.as_str(),
        })
        .unwrap_or(&common_text.unknown_label)
}

pub fn companion_skill_summary(data: &GameData, monster: &crate::state::CompanionState) -> String {
    let skill_summary = fill_template(
        &data.ui_text.common.skill_summary_template,
        &[
            ("{scouting}", monster.skills.scouting.to_string()),
            ("{guarding}", monster.skills.guarding.to_string()),
            ("{hospitality}", monster.skills.hospitality.to_string()),
            ("{crafting}", monster.skills.crafting.to_string()),
            ("{charm}", monster.skills.charm.to_string()),
        ],
    );
    format!(
        "{} | Bond {} / Rep {}",
        skill_summary, monster.bond, monster.reputation
    )
}

pub fn work_history_summary(data: &GameData, monster: &crate::state::CompanionState) -> String {
    fill_template(
        &data.ui_text.common.work_history_summary_template,
        &[
            (
                "{scouting runs}",
                monster.work_history.scouting_runs.to_string(),
            ),
            ("{guarding}", monster.work_history.guard_duties.to_string()),
            (
                "{hospitality}",
                monster.work_history.hospitality_jobs.to_string(),
            ),
            ("{crafting}", monster.work_history.craft_jobs.to_string()),
            (
                "{completed contracts}",
                monster.work_history.contracts_completed.to_string(),
            ),
        ],
    )
}

pub fn history_gain_label(
    data: &GameData,
    history: &crate::state::CompanionWorkHistoryState,
) -> String {
    let mut parts = Vec::new();

    if history.scouting_runs > 0 {
        parts.push(format!("K+{}", history.scouting_runs));
    }
    if history.guard_duties > 0 {
        parts.push(format!("O+{}", history.guard_duties));
    }
    if history.hospitality_jobs > 0 {
        parts.push(format!("V+{}", history.hospitality_jobs));
    }
    if history.craft_jobs > 0 {
        parts.push(format!("A+{}", history.craft_jobs));
    }
    if history.contracts_completed > 0 {
        parts.push(format!("C+{}", history.contracts_completed));
    }
    if history.recovery_shifts > 0 {
        parts.push(format!("M+{}", history.recovery_shifts));
    }
    if history.hatchery_assists > 0 {
        parts.push(format!("B+{}", history.hatchery_assists));
    }

    if parts.is_empty() {
        data.ui_text.common.none_label.clone()
    } else {
        parts.join(" ")
    }
}

/// What a shift in this room might bank, and how often it does.
///
/// The plain gain label reads as a promise — `C+1` for a contract the room banks
/// twelve times in a hundred looks exactly like `K+1` for a scouting run it banks
/// seventy. Anything short of certain is quoted with its odds.
pub fn history_gain_chance_label(
    data: &GameData,
    gains: &crate::state::CompanionWorkHistoryState,
    chance_pct: &crate::data::CompanionWorkHistoryProgressionData,
) -> String {
    let entries = [
        ("K", gains.scouting_runs, chance_pct.scouting_runs),
        ("O", gains.guard_duties, chance_pct.guard_duties),
        ("V", gains.hospitality_jobs, chance_pct.hospitality_jobs),
        ("A", gains.craft_jobs, chance_pct.craft_jobs),
        (
            "C",
            gains.contracts_completed,
            chance_pct.contracts_completed,
        ),
        ("M", gains.recovery_shifts, chance_pct.recovery_shifts),
        ("B", gains.hatchery_assists, chance_pct.hatchery_assists),
    ];

    let parts = entries
        .iter()
        .filter(|(_, gain, chance)| *gain > 0 && *chance > 0)
        .map(|(code, gain, chance)| {
            if *chance >= 100 {
                format!("{code}+{gain}")
            } else {
                format!("{code}+{gain} @{chance}%")
            }
        })
        .collect::<Vec<_>>();

    if parts.is_empty() {
        data.ui_text.common.none_label.clone()
    } else {
        parts.join(" ")
    }
}

pub fn history_gain_label_from_progress(
    data: &GameData,
    history: &crate::data::CompanionWorkHistoryProgressionData,
) -> String {
    let state_like = crate::state::CompanionWorkHistoryState {
        scouting_runs: history.scouting_runs,
        guard_duties: history.guard_duties,
        hospitality_jobs: history.hospitality_jobs,
        craft_jobs: history.craft_jobs,
        contracts_completed: history.contracts_completed,
        recovery_shifts: history.recovery_shifts,
        hatchery_assists: history.hatchery_assists,
    };

    history_gain_label(data, &state_like)
}

pub fn opening_skill_gain_label(
    data: &GameData,
    skills: &crate::data::CompanionSkillProgressionData,
) -> String {
    let mut parts = Vec::new();

    if skills.scouting > 0 {
        parts.push(format!("K+{}", skills.scouting));
    }
    if skills.guarding > 0 {
        parts.push(format!("O+{}", skills.guarding));
    }
    if skills.hospitality > 0 {
        parts.push(format!("V+{}", skills.hospitality));
    }
    if skills.crafting > 0 {
        parts.push(format!("A+{}", skills.crafting));
    }
    if skills.charm > 0 {
        parts.push(format!("S+{}", skills.charm));
    }

    if parts.is_empty() {
        data.ui_text.common.none_label.clone()
    } else {
        parts.join(" ")
    }
}
