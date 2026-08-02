//! Labels for what a companion has learned and what a shift might teach her.
//!
//! Split out of `view_models.rs` when that file crossed the 800-line limit.

use crate::data::GameData;

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

/// A work-history category's authored name.
pub fn work_history_text_label<'a>(data: &'a GameData, category_id: &str) -> &'a str {
    data.ui_text
        .common
        .work_history
        .iter()
        .find(|entry| entry.id == category_id)
        .map(|entry| entry.label.as_str())
        .unwrap_or(&data.ui_text.common.unknown_label)
}

/// Its compact badge code.
///
/// These read `K`, `O`, `V`, `A`, `C`, `M`, `B` — initials of the premise this
/// game was reskinned from, with `K` standing for a scouting run and `B` for a
/// hatchery assist. They were the last of that vocabulary on a player-facing
/// surface, and being one letter wide is what let them survive the rename pass
/// unread. They are authored now, so every line that badges banked work reads
/// the same table.
pub fn work_history_code<'a>(data: &'a GameData, category_id: &str) -> &'a str {
    data.ui_text
        .common
        .work_history
        .iter()
        .find(|entry| entry.id == category_id)
        .map(|entry| entry.code.as_str())
        .unwrap_or(&data.ui_text.common.unknown_label)
}

/// What a companion has actually banked.
///
/// This filled a five-placeholder template against seven categories, so
/// `recovery_shifts` and `hatchery_assists` — both of which rooms bank, and the
/// second of which is what turns a companion into a `hatchery_specialist` —
/// never appeared on the one line that reports her work. It also spelled the
/// five category names inline in the template while seven authored labels sat
/// beside it unused.
///
/// Codes rather than names, and only what she has. This sits on the contract
/// desk's candidate line beside `Skills Sc1 Ho3`, which is already coded — full
/// names overran the card and left a dangling entry, which reads as a companion
/// with fewer banked shifts than she has, on the screen where banked shifts are
/// what qualifies her.
pub fn work_history_summary(data: &GameData, monster: &crate::state::CompanionState) -> String {
    let parts = crate::engine::WORK_HISTORY_IDS
        .iter()
        .filter_map(|category_id| {
            let value = crate::engine::work_history_value(&monster.work_history, category_id);
            (value > 0).then(|| format!("{}{value}", work_history_code(data, category_id)))
        })
        .collect::<Vec<_>>();

    if parts.is_empty() {
        data.ui_text.common.none_label.clone()
    } else {
        parts.join(" ")
    }
}

/// What a shift banks, with no odds attached — used where the gain is certain.
pub fn history_gain_label_from_progress(
    data: &GameData,
    history: &crate::data::CompanionWorkHistoryProgressionData,
) -> String {
    let parts = crate::engine::WORK_HISTORY_IDS
        .iter()
        .filter_map(|category_id| {
            let gain = crate::engine::progression_work_history_value(history, category_id);
            (gain > 0).then(|| format!("{}+{gain}", work_history_code(data, category_id)))
        })
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
fn history_gain_chance_label(
    data: &GameData,
    gains: &crate::data::CompanionWorkHistoryProgressionData,
    chance_pct: &crate::data::CompanionWorkHistoryProgressionData,
) -> String {
    let parts = crate::engine::WORK_HISTORY_IDS
        .iter()
        .filter_map(|category_id| {
            let gain = crate::engine::progression_work_history_value(gains, category_id);
            let chance = crate::engine::progression_work_history_value(chance_pct, category_id);
            if gain == 0 || chance == 0 {
                return None;
            }
            let code = work_history_code(data, category_id);
            Some(if chance >= 100 {
                format!("{code}+{gain}")
            } else {
                format!("{code}+{gain} @{chance}%")
            })
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
    let banked = history_gain_chance_label(
        data,
        &room.work_history_gains,
        &room.work_history_gain_chance_pct,
    );

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
