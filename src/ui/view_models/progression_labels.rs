//! Labels for what a companion has learned and what a shift might teach her.
//!
//! Split out of `view_models.rs` when that file crossed the 800-line limit.

use crate::data::GameData;
use crate::ui::view_models::fill_template;

/// A skill's authored name, or the unknown label if nothing authored it.
///
/// The lookup every screen shares. It used to be a five-arm `match` copied into
/// each label builder, so a room that trains `recovery` — three of the four do —
/// read to the player as training "Unknown".
pub fn skill_label<'a>(data: &'a GameData, skill_id: &str) -> &'a str {
    data.ui_text
        .common
        .skills
        .iter()
        .find(|skill| skill.id == skill_id)
        .map(|skill| skill.label.as_str())
        .unwrap_or(&data.ui_text.common.unknown_label)
}

/// The compact code for a skill, for lines that cannot fit ten full names.
pub fn skill_code<'a>(data: &'a GameData, skill_id: &str) -> &'a str {
    data.ui_text
        .common
        .skills
        .iter()
        .find(|skill| skill.id == skill_id)
        .map(|skill| skill.code.as_str())
        .unwrap_or(&data.ui_text.common.unknown_label)
}

/// The same list in compact codes, for the room card's badge.
///
/// That badge is 168px in a row that exactly fills its 432, so full names
/// cannot fit there and were being cut to a 20-character stub: the common room
/// trains Scouting, Hospitality and Charm, and the card read "Trains Scouting,"
/// — which does not look truncated, it looks like one skill. Choosing a room is
/// the whole decision this screen exists for. Codes say all of it.
pub fn trained_skill_codes_label(data: &GameData, skill_ids: &[String]) -> String {
    if skill_ids.is_empty() {
        return data.ui_text.common.none_label.clone();
    }
    skill_ids
        .iter()
        .map(|skill_id| skill_code(data, skill_id))
        .collect::<Vec<_>>()
        .join("/")
}

pub fn primary_skill_label<'a>(data: &'a GameData, skill_ids: &[String]) -> &'a str {
    skill_ids
        .first()
        .map(|skill_id| skill_label(data, skill_id))
        .unwrap_or(&data.ui_text.common.unknown_label)
}

/// The roster strip's skill line.
///
/// Lists only the skills a companion actually has. It used to print all five of
/// the skills the game shipped with, zeros included, and none of the five added
/// later — so a companion could train `recovery` at the packroom for fifty
/// shifts and her line never changed. Showing the non-zero ones keeps the line
/// the same length it was in practice while making all ten reachable.
pub fn companion_skill_summary(data: &GameData, monster: &crate::state::CompanionState) -> String {
    let parts = crate::engine::SKILL_IDS
        .iter()
        .filter_map(|skill_id| {
            let value = crate::engine::companion_skill_value(&monster.skills, skill_id);
            (value > 0).then(|| format!("{}{value}", skill_code(data, skill_id)))
        })
        .collect::<Vec<_>>();

    let skill_summary = if parts.is_empty() {
        data.ui_text.common.none_label.clone()
    } else {
        parts.join(" ")
    };

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
    let parts = crate::engine::SKILL_IDS
        .iter()
        .filter_map(|skill_id| {
            let value = crate::engine::progression_skill_value(skills, skill_id);
            (value > 0).then(|| format!("{}+{value}", skill_code(data, skill_id)))
        })
        .collect::<Vec<_>>();

    if parts.is_empty() {
        data.ui_text.common.none_label.clone()
    } else {
        parts.join(" ")
    }
}
