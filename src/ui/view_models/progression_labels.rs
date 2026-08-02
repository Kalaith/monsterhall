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

/// The badge code for each kind of banked work, in category order.
///
/// These read `K`, `O`, `V`, `A`, `C`, `M`, `B` — initials of the premise this
/// game was reskinned from, with `K` standing for a scouting run and `B` for a
/// hatchery assist. They are the last of that vocabulary on a player-facing
/// surface, and being one letter wide is what let them survive the rename pass
/// unread. Two letters costs a few pixels and says what the work is.
///
/// One table, so the plain label and the with-odds label cannot name the same
/// category two different ways.
fn work_history_codes(
    history: &crate::state::CompanionWorkHistoryState,
) -> [(&'static str, u32); 7] {
    [
        ("Sc", history.scouting_runs),
        ("Gd", history.guard_duties),
        ("Hs", history.hospitality_jobs),
        ("Cf", history.craft_jobs),
        ("Ct", history.contracts_completed),
        ("Rc", history.recovery_shifts),
        ("Ht", history.hatchery_assists),
    ]
}

pub fn history_gain_label(
    data: &GameData,
    history: &crate::state::CompanionWorkHistoryState,
) -> String {
    let parts = work_history_codes(history)
        .into_iter()
        .filter(|(_, gain)| *gain > 0)
        .map(|(code, gain)| format!("{code}+{gain}"))
        .collect::<Vec<_>>();

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
    let odds = work_history_codes(&crate::state::CompanionWorkHistoryState {
        scouting_runs: chance_pct.scouting_runs,
        guard_duties: chance_pct.guard_duties,
        hospitality_jobs: chance_pct.hospitality_jobs,
        craft_jobs: chance_pct.craft_jobs,
        contracts_completed: chance_pct.contracts_completed,
        recovery_shifts: chance_pct.recovery_shifts,
        hatchery_assists: chance_pct.hatchery_assists,
    });

    let parts = work_history_codes(gains)
        .into_iter()
        .zip(odds)
        .filter(|((_, gain), (_, chance))| *gain > 0 && *chance > 0)
        .map(|((code, gain), (_, chance))| {
            if chance >= 100 {
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

/// Room-side variant of [`history_gain_chance_label`]: what a shift here can
/// bank and how often, from the room's own ceilings and odds.
///
/// Charm rides along because it is the one lesson with no work-history category
/// behind it — its odds were a `match` on room id in Rust until they became
/// authored data, so this badge could not have shown them before. A room that
/// lists charm and teaches it two days in three is a different proposition from
/// one that only teaches it while a patron is in the room.
pub fn history_gain_chance_label_from_progress(
    data: &GameData,
    room: &crate::data::GuildRoomData,
) -> String {
    let gains = &room.work_history_gains;
    let state_like = crate::state::CompanionWorkHistoryState {
        scouting_runs: gains.scouting_runs,
        guard_duties: gains.guard_duties,
        hospitality_jobs: gains.hospitality_jobs,
        craft_jobs: gains.craft_jobs,
        contracts_completed: gains.contracts_completed,
        recovery_shifts: gains.recovery_shifts,
        hatchery_assists: gains.hatchery_assists,
    };
    let banked = history_gain_chance_label(data, &state_like, &room.work_history_gain_chance_pct);

    let shift = crate::engine::charm_training_chance_pct(room, false);
    let booking = crate::engine::charm_training_chance_pct(room, true);
    if shift == 0 && booking == 0 {
        return banked;
    }
    let charm = if shift == booking {
        format!("Ch @{shift}%")
    } else {
        format!("Ch @{shift}/{booking}%")
    };

    if banked == data.ui_text.common.none_label {
        charm
    } else {
        format!("{banked} {charm}")
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
