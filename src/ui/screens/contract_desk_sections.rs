use macroquad::prelude::{screen_height, screen_width};

use crate::data::GameData;
use crate::state::{
    GameState, ContractDeskState, ContractState, ContractStatus, CompanionState,
};
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
use crate::ui::theme;
use crate::ui::view_models::{
    evaluate_guest_candidate, fill_template, format_resources_state,
    guest_history_requirement_label, guest_skill_requirement_label,
    guest_species_requirement_label, guest_status_label, monster_name_by_id, monster_quality_label,
    quality_label, room_name_by_id, work_history_summary, companion_skill_summary, species_name_by_id,
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

fn blocked_candidate_summary(request: &ContractState, monster: &CompanionState) -> String {
    let mut parts = Vec::new();
    if monster.quality_rank < request.minimum_quality_rank.max(1) {
        parts.push(format!(
            "Star {}/{}",
            monster.quality_rank.max(1),
            request.minimum_quality_rank.max(1)
        ));
    }
    push_requirement_gap(
        &mut parts,
        "Kiss",
        monster.skills.scouting,
        request.required_skill_thresholds.scouting,
    );
    push_requirement_gap(
        &mut parts,
        "Guarding",
        monster.skills.guarding,
        request.required_skill_thresholds.guarding,
    );
    push_requirement_gap(
        &mut parts,
        "Vag",
        monster.skills.hospitality,
        request.required_skill_thresholds.hospitality,
    );
    push_requirement_gap(
        &mut parts,
        "Crafting",
        monster.skills.crafting,
        request.required_skill_thresholds.crafting,
    );
    push_requirement_gap(
        &mut parts,
        "Sed",
        monster.skills.charm,
        request.required_skill_thresholds.charm,
    );
    push_requirement_gap(
        &mut parts,
        "Kiss Hist",
        monster.work_history.scouting_runs,
        request.required_work_history_thresholds.scouting_runs,
    );
    push_requirement_gap(
        &mut parts,
        "Guarding Hist",
        monster.work_history.guard_duties,
        request.required_work_history_thresholds.guard_duties,
    );
    push_requirement_gap(
        &mut parts,
        "Vag Hist",
        monster.work_history.hospitality_jobs,
        request.required_work_history_thresholds.hospitality_jobs,
    );

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
        let candidates_y = 336.0;
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

    let max_visible_requests = ((layout.requests_h - 74.0) / 36.0).floor().max(1.0) as usize;
    for (index, request) in requests.iter().take(max_visible_requests).enumerate() {
        let y = 166.0 + index as f32 * 36.0;
        let label = format!(
            "{} | {}",
            request.guest_name,
            fill_template(
                &data.ui_text.contract_desk.deadline_day_template,
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
        228.0,
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
        &data
            .ui_text
            .contract_desk
            .context_accepted_requests_label,
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
        138.0,
        24.0,
        theme::TEXT_STRONG,
    );
    draw_inline_status(
        layout.detail_x + 150.0,
        150.0,
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
        196.0,
        15.0,
        theme::TEXT_BODY,
    );
    let patron_tier = request
        .patron_tier_id
        .as_deref()
        .unwrap_or(&data.ui_text.common.none_label);
    draw_body_text(
        &format!(
            "{}: {} | {}: {} | {}: {}",
            data.ui_text.contract_desk.category_label,
            request.category,
            data.ui_text.contract_desk.patron_tier_label,
            patron_tier,
            data.ui_text.contract_desk.preparation_quality_label,
            request.preparation_quality_required
        ),
        layout.detail_x + 150.0,
        180.0,
        13.0,
        theme::TEXT_MUTED,
    );
    draw_body_text(
        &format_resources_state(data, &request.reward),
        layout.detail_x + 150.0,
        214.0,
        15.0,
        theme::POSITIVE,
    );
    draw_body_text(
        &fill_template(
            &data.ui_text.contract_desk.penalty_gold_template,
            &[("{gold}", request.penalty_gold.to_string())],
        ),
        layout.detail_x + 150.0,
        234.0,
        14.0,
        theme::TEXT_MUTED,
    );
    draw_body_text(
        &fill_template(
            &data.ui_text.contract_desk.deadline_day_template,
            &[("{day}", request.deadline_day.to_string())],
        ),
        layout.detail_x + 150.0,
        252.0,
        14.0,
        theme::TEXT_MUTED,
    );

    draw_badge(
        layout.detail_x + 344.0,
        206.0,
        148.0,
        22.0,
        &guest_species_requirement_label(data, request),
        theme::PRIMARY,
    );
    draw_badge(
        layout.detail_x + 500.0,
        206.0,
        180.0,
        22.0,
        &guest_skill_requirement_label(data, &request.required_skill_thresholds),
        theme::INFO,
    );
    draw_badge(
        layout.detail_x + 688.0,
        206.0,
        116.0,
        22.0,
        &format!("Min {}", quality_label(request.minimum_quality_rank.max(1))),
        theme::WARNING,
    );
    draw_badge(
        layout.detail_x + 344.0,
        236.0,
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
        266.0,
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
            266.0,
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
            294.0,
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
) -> Option<UiAction> {
    draw_tier_panel(
        layout.left_margin,
        layout.candidates_y,
        layout.content_width,
        298.0,
        Some(&data.ui_text.contract_desk.eligible_girls_panel_title),
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
    for (index, monster) in game_state.monsters.iter().take(6).enumerate() {
        let col = index % 2;
        let row = index / 2;
        let x = layout.left_margin
            + layout::PANEL_PADDING
            + col as f32 * (card_w + layout::SECTION_GAP);
        let y = layout.candidates_y + 42.0 + row as f32 * 100.0;
        let report = evaluate_guest_candidate(data, game_state, request, monster);
        let state_color = if report.is_eligible {
            theme::POSITIVE
        } else {
            theme::WARNING
        };
        let detail = if report.is_eligible {
            fill_template(
                &data.ui_text.contract_desk.eligible_summary_template,
                &[
                    ("{skills}", companion_skill_summary(data, monster)),
                    ("{history}", work_history_summary(data, monster)),
                ],
            )
        } else {
            let blocked_summary = blocked_candidate_summary(request, monster);
            if blocked_summary.is_empty() {
                compact_text(&report.failure_reasons.join(" | "), 54)
            } else {
                blocked_summary
            }
        };
        let species_label = format!(
            "{} | {}",
            species_name_by_id(data, &monster.species_id),
            monster_quality_label(monster)
        );
        let state_label = if report.is_eligible {
            &data.ui_text.contract_desk.eligible_label
        } else {
            &data.ui_text.contract_desk.blocked_label
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

    None
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
