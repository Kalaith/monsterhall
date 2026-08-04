use macroquad::prelude::{screen_height, screen_width};

use crate::data::GameData;
use crate::engine::ContractServiceOutcome;
use crate::state::{CompanionState, ContractDeskState, ContractState, ContractStatus, GameState};
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_guest_silhouette, draw_room_thumbnail};
use crate::ui::chrome::{
    draw_inline_status, draw_screen_header, draw_standard_gameplay_footer, draw_tier_panel,
    draw_top_utility_bar, PanelTier,
};
use crate::ui::components::{
    draw_badge, draw_character_card, draw_empty_state, draw_metric_tile, CharacterCardSpec,
};
use crate::ui::core::{draw_body_text, primary_button, secondary_button, utility_button};
use crate::ui::feedback::draw_inline_error;
use crate::ui::layout;
use crate::ui::screens::roster_window::{draw_roster_pager, RosterWindow, ROSTER_COLUMNS};
use crate::ui::theme;
use crate::ui::view_models::{
    companion_skill_summary, evaluate_guest_candidate, fill_template, format_resources_state,
    guest_history_requirement_label, guest_skill_requirement_label,
    guest_species_requirement_label, guest_status_label, monster_name_by_id, monster_quality_label,
    quality_label, room_name_by_id, species_name_by_id, work_history_summary,
};

fn compact_text(text: &str, max_len: usize) -> String {
    let compact = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.chars().count() <= max_len {
        compact
    } else {
        let mut trimmed = compact
            .chars()
            .take(max_len.saturating_sub(3))
            .collect::<String>();
        if let Some(index) = trimmed.rfind(' ') {
            trimmed.truncate(index);
        }
        format!("{trimmed}...")
    }
}

/// Why this candidate cannot take this contract, as short gap badges.
///
/// Covered five of the ten skills and three of the seven work-history
/// categories — the same five-of-ten omission as the wage formula, the hatchery
/// replacement score and the policy's service score, making this the fifth copy
/// of that mistake. The caller only falls back to the engine's complete reason
/// list when this returns *nothing*, so a candidate blocked by both a covered
/// requirement and an uncovered one showed only the covered half: the player
/// trained the skill the card named, came back, and she was still blocked.
///
/// Labels come from the engine rather than a third naming — `format_skill_name`
/// and `work_history_label` are the same tables the refusal reasons and the day
/// log use, so the desk cannot drift from them.
fn blocked_candidate_summary(request: &ContractState, monster: &CompanionState) -> String {
    let mut parts = Vec::new();
    if monster.quality_rank < request.minimum_quality_rank.max(1) {
        parts.push(format!(
            "Star {}/{}",
            monster.quality_rank.max(1),
            request.minimum_quality_rank.max(1)
        ));
    }

    let skills = &request.required_skill_thresholds;
    for (skill_id, required, current) in [
        ("scouting", skills.scouting, monster.skills.scouting),
        ("guarding", skills.guarding, monster.skills.guarding),
        (
            "hospitality",
            skills.hospitality,
            monster.skills.hospitality,
        ),
        ("crafting", skills.crafting, monster.skills.crafting),
        ("charm", skills.charm, monster.skills.charm),
        ("recovery", skills.recovery, monster.skills.recovery),
        ("bargaining", skills.bargaining, monster.skills.bargaining),
        ("navigation", skills.navigation, monster.skills.navigation),
        ("arcana", skills.arcana, monster.skills.arcana),
        ("strength", skills.strength, monster.skills.strength),
    ] {
        push_requirement_gap(
            &mut parts,
            crate::engine::format_skill_name(skill_id),
            current,
            required,
        );
    }

    let history = &request.required_work_history_thresholds;
    let banked = &monster.work_history;
    for category in crate::engine::WORK_HISTORY_IDS {
        let required = crate::engine::required_work_history_value(history, category);
        let current = crate::engine::work_history_value(banked, category);
        push_requirement_gap(
            &mut parts,
            crate::engine::work_history_label(category),
            current,
            required,
        );
    }

    parts.join(" | ")
}

fn push_requirement_gap(parts: &mut Vec<String>, label: &str, current: u32, required: u32) {
    if required > 0 && current < required {
        parts.push(format!("{label} {current}/{required}"));
    }
}

pub(super) struct ContractDeskLayout {
    pub left_margin: f32,
    pub content_width: f32,
    pub requests_w: f32,
    pub requests_h: f32,
    pub detail_x: f32,
    pub detail_w: f32,
    pub candidates_y: f32,
    pub footer_y: f32,
}

impl ContractDeskLayout {
    pub(super) fn new() -> Self {
        let left_margin = layout::OUTER_MARGIN;
        let content_width = screen_width() - left_margin * 2.0;
        let requests_w = 290.0;
        let detail_x = left_margin + requests_w + layout::SECTION_GAP;
        let detail_w = content_width - requests_w - layout::SECTION_GAP;
        let candidates_y = 356.0;
        let footer_y = screen_height() - layout::FOOTER_BOTTOM_MARGIN - layout::FOOTER_H;
        let requests_h = (footer_y - 92.0 - layout::SECTION_GAP).max(228.0);

        Self {
            left_margin,
            content_width,
            requests_w,
            requests_h,
            detail_x,
            detail_w,
            candidates_y,
            footer_y,
        }
    }
}

pub(super) fn draw_header(data: &GameData) -> Option<UiAction> {
    if let Some(action) = draw_top_utility_bar(&data.ui_text.common.settings_button) {
        return Some(action);
    }
    draw_screen_header(
        &data.ui_text.contract_desk.title,
        &data.ui_text.contract_desk.subtitle,
    );
    None
}

pub(super) fn selected_request<'a>(
    guest_state: &ContractDeskState,
    game_state: &'a GameState,
) -> (
    Vec<&'a crate::state::ContractState>,
    Option<&'a crate::state::ContractState>,
) {
    let requests = game_state.active_contracts.iter().collect::<Vec<_>>();
    let selected_request = guest_state
        .selected_request_id
        .as_ref()
        .and_then(|request_id| {
            game_state
                .active_contracts
                .iter()
                .find(|request| &request.request_id == request_id)
        })
        .or_else(|| requests.first().copied());

    (requests, selected_request)
}

pub(super) fn draw_requests_panel(
    data: &GameData,
    guest_state: &ContractDeskState,
    requests: &[&crate::state::ContractState],
    resolved: &[crate::state::ContractState],
    layout: &ContractDeskLayout,
) -> Option<UiAction> {
    draw_tier_panel(
        layout.left_margin,
        92.0,
        layout.requests_w,
        layout.requests_h,
        Some(&data.ui_text.contract_desk.active_requests_panel_title),
        PanelTier::Support,
        false,
    );
    draw_inline_status(
        layout.left_margin + layout::PANEL_PADDING,
        132.0,
        layout.requests_w - layout::PANEL_PADDING * 2.0,
        &format!(
            "{}: {}",
            data.ui_text.contract_desk.status_label, guest_state.status_message
        ),
        theme::PRIMARY,
    );

    // Yesterday's outcomes sit under the live offers rather than vanishing at
    // day roll-over. `ContractStatus::Completed`/`Failed` had labels and colours
    // that nothing could ever display until now.
    let max_visible_requests = ((layout.requests_h - 74.0) / 36.0).floor().max(1.0) as usize;
    let resolved_rows = resolved.len().min(max_visible_requests.saturating_sub(1));
    let max_visible_requests = max_visible_requests - resolved_rows;

    for (index, request) in requests.iter().take(max_visible_requests).enumerate() {
        let y = 166.0 + index as f32 * 36.0;
        // The row is a 266px button and the full "… | Deadline: Day 4" ran past
        // it, so every offer in the list was clipped at the panel edge. The
        // detail panel beside it spells the deadline out; the picker only has
        // to say which day.
        let label = format!(
            "{} | {}",
            request.guest_name,
            fill_template(
                &data.ui_text.contract_desk.deadline_day_short_template,
                &[("{day}", request.deadline_day.to_string())],
            )
        );
        let is_selected = guest_state.selected_request_id.as_ref() == Some(&request.request_id);
        let pressed = if is_selected {
            primary_button(
                layout.left_margin + 12.0,
                y,
                layout.requests_w - 24.0,
                28.0,
                &label,
            )
        } else {
            secondary_button(
                layout.left_margin + 12.0,
                y,
                layout.requests_w - 24.0,
                28.0,
                &label,
            )
        };
        if pressed {
            return Some(UiAction::SelectContractRequest(request.request_id.clone()));
        }
    }

    for (index, request) in resolved.iter().take(resolved_rows).enumerate() {
        let y = 166.0 + (requests.len().min(max_visible_requests) + index) as f32 * 36.0;
        draw_inline_status(
            layout.left_margin + 12.0,
            y,
            layout.requests_w - 24.0,
            &format!(
                "{} | {}",
                request.guest_name,
                guest_status_label(data, &request.status)
            ),
            match request.status {
                ContractStatus::Completed => theme::POSITIVE,
                _ => theme::DANGER,
            },
        );
    }

    None
}

pub(super) fn draw_no_requests_state(
    data: &GameData,
    game_state: &GameState,
    layout: &ContractDeskLayout,
) {
    draw_tier_panel(
        layout.left_margin,
        92.0,
        layout.content_width,
        layout.footer_y - 92.0 - layout::SECTION_GAP,
        Some(&data.ui_text.contract_desk.active_requests_panel_title),
        PanelTier::Primary,
        true,
    );

    let metric_y = 144.0;
    let metric_w = 136.0;
    draw_metric_tile(
        layout.left_margin + layout::PANEL_PADDING,
        metric_y,
        metric_w,
        54.0,
        &data.ui_text.contract_desk.context_gold_label,
        &game_state.resources.gold.to_string(),
        theme::POSITIVE,
    );
    draw_metric_tile(
        layout.left_margin + layout::PANEL_PADDING + metric_w + layout::SPACE_12,
        metric_y,
        metric_w,
        54.0,
        &data.ui_text.contract_desk.roster_label,
        &game_state.monsters.len().to_string(),
        theme::INFO,
    );

    draw_empty_state(
        layout.left_margin + layout::PANEL_PADDING,
        224.0,
        layout.content_width - layout::PANEL_PADDING * 2.0,
        154.0,
        &data.ui_text.contract_desk.no_requests_title,
        &data.ui_text.contract_desk.no_active_requests_message,
    );
}

pub(super) fn draw_selected_request_panel(
    data: &GameData,
    game_state: &GameState,
    requests: &[&crate::state::ContractState],
    request: Option<&crate::state::ContractState>,
    last_error: Option<&str>,
    layout: &ContractDeskLayout,
) -> Option<UiAction> {
    draw_tier_panel(
        layout.detail_x,
        92.0,
        layout.detail_w,
        248.0,
        Some(&data.ui_text.contract_desk.selected_request_panel_title),
        PanelTier::Primary,
        true,
    );

    let accepted_count = requests
        .iter()
        .filter(|entry| matches!(entry.status, ContractStatus::Accepted))
        .count()
        .to_string();
    let metric_x = layout.detail_x + layout.detail_w - 360.0;
    draw_metric_tile(
        metric_x,
        138.0,
        108.0,
        52.0,
        &data.ui_text.contract_desk.context_gold_label,
        &game_state.resources.gold.to_string(),
        theme::POSITIVE,
    );
    draw_metric_tile(
        metric_x + 116.0,
        138.0,
        108.0,
        52.0,
        &data.ui_text.contract_desk.roster_label,
        &game_state.monsters.len().to_string(),
        theme::INFO,
    );
    draw_metric_tile(
        metric_x + 232.0,
        138.0,
        108.0,
        52.0,
        &data.ui_text.contract_desk.context_accepted_requests_label,
        &accepted_count,
        theme::PRIMARY,
    );

    let Some(request) = request else {
        draw_empty_state(
            layout.detail_x + 8.0,
            132.0,
            layout.detail_w - 384.0,
            140.0,
            &data.ui_text.contract_desk.no_selected_request_title,
            &data.ui_text.contract_desk.no_selected_request_message,
        );
        return None;
    };

    draw_guest_silhouette(request, layout.detail_x + 16.0, 126.0, 118.0, 152.0);
    draw_body_text(
        &request.guest_name,
        layout.detail_x + 150.0,
        134.0,
        24.0,
        theme::TEXT_STRONG,
    );
    // One column, drawn top to bottom, spaced so nothing lands on anything
    // else: name 134 (24px, descends to ~154), status badge 158 (28 tall),
    // category 196, room 216, reward 236, penalty 256, deadline 276. The
    // original had the name at 138 with the badge starting at 150 and the
    // category at 180, so three of them overlapped.
    draw_inline_status(
        layout.detail_x + 150.0,
        158.0,
        190.0,
        guest_status_label(data, &request.status),
        match request.status {
            ContractStatus::Accepted => theme::POSITIVE,
            ContractStatus::Failed | ContractStatus::Declined => theme::DANGER,
            ContractStatus::Completed => theme::INFO,
            ContractStatus::Pending => theme::WARNING,
        },
    );
    draw_body_text(
        &room_name_by_id(data, &request.requested_room_id),
        layout.detail_x + 150.0,
        216.0,
        15.0,
        theme::TEXT_BODY,
    );
    let patron_tier = request
        .patron_tier_id
        .as_deref()
        .unwrap_or(&data.ui_text.common.none_label);
    // Both halves of the preparation figure, because the requirement now costs
    // the guild half the booking when the hall falls short of it, and a bare
    // "6" cannot tell the player whether they are short.
    let hall_preparation = crate::engine::hall_preparation_quality(data, game_state);
    draw_body_text(
        &format!(
            "{}: {} | {}: {} | {}: {}/{}",
            data.ui_text.contract_desk.category_label,
            request.category,
            data.ui_text.contract_desk.patron_tier_label,
            patron_tier,
            data.ui_text.contract_desk.preparation_quality_label,
            hall_preparation,
            request.preparation_quality_required
        ),
        layout.detail_x + 150.0,
        196.0,
        13.0,
        // A hall under the bar costs the guild half the booking, so the line
        // that says so should not read like the rest of the metadata.
        if hall_preparation < request.preparation_quality_required {
            theme::WARNING
        } else {
            theme::TEXT_MUTED
        },
    );
    draw_body_text(
        &format_resources_state(data, &request.reward),
        layout.detail_x + 150.0,
        236.0,
        15.0,
        theme::POSITIVE,
    );
    draw_body_text(
        &fill_template(
            &data.ui_text.contract_desk.penalty_gold_template,
            &[("{gold}", request.penalty_gold.to_string())],
        ),
        layout.detail_x + 150.0,
        256.0,
        14.0,
        theme::TEXT_MUTED,
    );
    draw_body_text(
        &fill_template(
            &data.ui_text.contract_desk.deadline_day_template,
            &[("{day}", request.deadline_day.to_string())],
        ),
        layout.detail_x + 150.0,
        276.0,
        14.0,
        theme::TEXT_MUTED,
    );

    draw_badge(
        layout.detail_x + 344.0,
        214.0,
        148.0,
        22.0,
        &guest_species_requirement_label(data, request),
        theme::PRIMARY,
    );
    draw_badge(
        layout.detail_x + 500.0,
        214.0,
        180.0,
        22.0,
        &guest_skill_requirement_label(data, &request.required_skill_thresholds),
        theme::INFO,
    );
    draw_badge(
        layout.detail_x + 688.0,
        214.0,
        116.0,
        22.0,
        &format!(
            "Min {}",
            quality_label(data, request.minimum_quality_rank.max(1))
        ),
        theme::WARNING,
    );
    draw_badge(
        layout.detail_x + 344.0,
        244.0,
        336.0,
        22.0,
        &guest_history_requirement_label(data, &request.required_work_history_thresholds),
        theme::WARNING,
    );

    let assigned_label = request
        .assigned_monster_id
        .as_ref()
        .map(|monster_id| monster_name_by_id(game_state, monster_id))
        .unwrap_or_else(|| data.ui_text.common.none_label.clone());
    draw_inline_status(
        layout.detail_x + 344.0,
        282.0,
        336.0,
        &format!(
            "{}: {assigned_label}",
            data.ui_text.contract_desk.assigned_label
        ),
        if request.assigned_monster_id.is_some() {
            theme::POSITIVE
        } else {
            theme::WARNING
        },
    );

    if let Some(room) = data
        .guild_rooms
        .rooms
        .iter()
        .find(|room| room.id == request.requested_room_id)
    {
        draw_room_thumbnail(
            room,
            layout.detail_x + layout.detail_w - 148.0,
            126.0,
            132.0,
            92.0,
        );
    }

    if request.assigned_monster_id.is_some()
        && utility_button(
            layout.detail_x + layout.detail_w - 180.0,
            282.0,
            156.0,
            26.0,
            &data.ui_text.contract_desk.clear_assignment_button,
        )
    {
        return Some(UiAction::ClearGuestAssignment(request.request_id.clone()));
    }

    if let Some(error_message) = last_error {
        draw_inline_error(
            layout.detail_x + 344.0,
            314.0,
            layout.detail_w - 368.0,
            error_message,
        );
    }

    None
}

pub(super) fn draw_eligible_panel(
    data: &GameData,
    game_state: &GameState,
    request: Option<&crate::state::ContractState>,
    layout: &ContractDeskLayout,
    roster_page: usize,
) -> Option<UiAction> {
    let panel_h = (layout.footer_y - layout.candidates_y - layout::SECTION_GAP).max(238.0);
    // Rows of cards the panel can hold, which is what decides whether the roster
    // needs paging. The panel keeps its full height either way — it is anchored
    // to the footer — so the pager sits under the cards rather than growing it.
    let roster = RosterWindow::from_panel(
        game_state.monsters.len(),
        roster_page,
        panel_h - 42.0,
        100.0,
        ROSTER_COLUMNS,
    );
    draw_tier_panel(
        layout.left_margin,
        layout.candidates_y,
        layout.content_width,
        panel_h,
        Some(&data.ui_text.contract_desk.eligible_companions_panel_title),
        PanelTier::Support,
        false,
    );

    let Some(request) = request else {
        draw_empty_state(
            layout.left_margin + 8.0,
            layout.candidates_y + 40.0,
            layout.content_width - 16.0,
            120.0,
            &data.ui_text.contract_desk.no_selected_request_title,
            &data.ui_text.contract_desk.no_selected_request_message,
        );
        return None;
    };

    if game_state.monsters.is_empty() {
        draw_empty_state(
            layout.left_margin + 8.0,
            layout.candidates_y + 40.0,
            layout.content_width - 16.0,
            120.0,
            &data.ui_text.contract_desk.no_roster_title,
            &data.ui_text.contract_desk.no_selected_request_message,
        );
        return None;
    }

    let card_w = (layout.content_width - layout::PANEL_PADDING * 2.0 - layout::SECTION_GAP) / 2.0;
    for (index, monster) in game_state
        .monsters
        .iter()
        .skip(roster.first_index)
        .take(roster.visible_count)
        .enumerate()
    {
        let col = index % ROSTER_COLUMNS;
        let row = index / ROSTER_COLUMNS;
        let x = layout.left_margin
            + layout::PANEL_PADDING
            + col as f32 * (card_w + layout::SECTION_GAP);
        let y = layout.candidates_y + 42.0 + row as f32 * 100.0;
        let report = evaluate_guest_candidate(data, game_state, request, monster);
        // Three states, not two. A companion the booking refuses may still be
        // close enough to send for half, and drawing her as "Blocked" beside a
        // live Assign button meant the player found the halving out a day later
        // in the report. The engine's own answer, so the two cannot disagree.
        let outcome = crate::engine::contract_service_outcome(data, game_state, request, monster);
        let state_color = match outcome {
            ContractServiceOutcome::Full => theme::POSITIVE,
            ContractServiceOutcome::Partial => theme::WARNING,
            ContractServiceOutcome::Refused => theme::DANGER,
        };
        let gap_summary = || {
            let blocked_summary = blocked_candidate_summary(request, monster);
            if blocked_summary.is_empty() {
                compact_text(&report.failure_reasons.join(" | "), 54)
            } else {
                blocked_summary
            }
        };
        let detail = match outcome {
            ContractServiceOutcome::Full => fill_template(
                &data.ui_text.contract_desk.eligible_summary_template,
                &[
                    ("{skills}", companion_skill_summary(data, monster)),
                    ("{history}", work_history_summary(data, monster)),
                ],
            ),
            // Both short states show the same thing — what she is short of.
            // The badge above is what separates them, because a sentence
            // explaining half pay costs the whole line: at the card's width it
            // truncated to "Short of the terms, but close enough to send l..."
            // and the gaps, which are the actionable half, never appeared.
            ContractServiceOutcome::Partial | ContractServiceOutcome::Refused => gap_summary(),
        };
        let species_label = format!(
            "{} | {}",
            species_name_by_id(data, &monster.species_id),
            monster_quality_label(data, monster)
        );
        let state_label = match outcome {
            ContractServiceOutcome::Full => &data.ui_text.contract_desk.eligible_label,
            ContractServiceOutcome::Partial => &data.ui_text.contract_desk.half_pay_label,
            ContractServiceOutcome::Refused => &data.ui_text.contract_desk.blocked_label,
        };
        let card = draw_character_card(
            data,
            monster,
            x,
            y,
            card_w,
            92.0,
            CharacterCardSpec {
                name: &monster.name,
                species: &species_label,
                state: state_label,
                key_value: &detail,
                color: state_color,
                state_color,
                selected: request.assigned_monster_id.as_ref() == Some(&monster.id),
                disabled: false,
            },
        );

        let is_assigned = request.assigned_monster_id.as_ref() == Some(&monster.id);
        if primary_button(
            card.action_x,
            card.action_y,
            card.action_w,
            24.0,
            if is_assigned {
                &data.ui_text.contract_desk.assigned_button
            } else {
                &data.ui_text.common.assign_button
            },
        ) {
            return Some(UiAction::AssignMonsterToGuest(
                request.request_id.clone(),
                monster.id.clone(),
            ));
        }
    }

    draw_roster_pager(
        &roster,
        game_state.monsters.len(),
        layout.left_margin + layout::PANEL_PADDING,
        layout.candidates_y + 42.0 + roster.card_rows() as f32 * 100.0,
        layout.content_width - layout::PANEL_PADDING * 2.0,
        UiAction::ShowRosterPage,
    )
}

pub(super) fn draw_footer_actions(
    data: &GameData,
    layout: &ContractDeskLayout,
) -> Option<UiAction> {
    draw_standard_gameplay_footer(
        data,
        layout.left_margin,
        layout.footer_y,
        layout.content_width,
        Some(UiAction::OpenContractDesk),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ContractHistoryRequirementState, ContractSkillRequirementState};

    /// The gap badges must name every requirement that is blocking, not the
    /// first five skills and first three histories.
    #[test]
    fn a_blocked_candidate_shows_every_requirement_she_is_short_of() {
        let request = ContractState {
            required_skill_thresholds: ContractSkillRequirementState {
                charm: 2,
                arcana: 3,
                strength: 1,
                ..ContractSkillRequirementState::default()
            },
            required_work_history_thresholds: ContractHistoryRequirementState {
                hatchery_assists: 3,
                craft_jobs: 2,
                contracts_completed: 1,
                ..ContractHistoryRequirementState::default()
            },
            ..ContractState::default()
        };
        let monster = CompanionState {
            quality_rank: 1,
            ..CompanionState::default()
        };

        let summary = blocked_candidate_summary(&request, &monster);

        for expected in [
            "Charm",
            "Arcana",
            "Strength",
            "Hatchery Assists",
            "Crafting Jobs",
            "Contracts Completed",
        ] {
            assert!(
                summary.contains(expected),
                "'{expected}' is blocking her and the desk does not say so: {summary}"
            );
        }
    }

    /// And the labels are the engine's, so a badge cannot call a requirement one
    /// thing while the refusal reason calls it another.
    #[test]
    fn gap_badges_use_the_same_names_as_the_refusal_reasons() {
        for category in crate::engine::WORK_HISTORY_IDS {
            let label = crate::engine::work_history_label(category);
            let request = ContractState {
                required_work_history_thresholds: history_requiring(category),
                ..ContractState::default()
            };
            let monster = CompanionState {
                quality_rank: 1,
                ..CompanionState::default()
            };

            let summary = blocked_candidate_summary(&request, &monster);
            assert!(
                summary.contains(label),
                "'{category}' should appear as '{label}': {summary}"
            );
        }
    }

    fn history_requiring(category: &str) -> ContractHistoryRequirementState {
        let mut history = ContractHistoryRequirementState::default();
        match category {
            "scouting_runs" => history.scouting_runs = 1,
            "guard_duties" => history.guard_duties = 1,
            "hospitality_jobs" => history.hospitality_jobs = 1,
            "craft_jobs" => history.craft_jobs = 1,
            "contracts_completed" => history.contracts_completed = 1,
            "recovery_shifts" => history.recovery_shifts = 1,
            "hatchery_assists" => history.hatchery_assists = 1,
            other => panic!("unknown work-history category '{other}'"),
        }
        history
    }
}
