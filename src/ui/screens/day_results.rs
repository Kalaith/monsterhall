use macroquad::prelude::{draw_rectangle, screen_width, Color};

use crate::data::GameData;
use crate::state::DayResultsState;
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_backdrop, BackdropKind};
use crate::ui::chrome::{draw_inline_status, draw_tier_panel, draw_top_utility_bar, PanelTier};
use crate::ui::core::primary_button;
use crate::ui::core::{draw_body_text, draw_wrapped_lines, draw_wrapped_lines_in_box};
use crate::ui::theme;

pub fn draw_day_results(
    data: &GameData,
    results_state: &DayResultsState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    draw_backdrop(BackdropKind::Results);

    let summary = &results_state.summary;
    let common_text = &data.ui_text.common;
    let day_results_text = &data.ui_text.day_results;
    let left_margin = 34.0;
    let top_margin = 36.0;
    let content_width = screen_width() - left_margin * 2.0;
    let panel_width = (content_width - 36.0) / 4.0;

    crate::ui::core::draw_heading(
        &day_results_text
            .title_template
            .replace("{day}", &summary.resolved_day.to_string()),
        left_margin,
        top_margin,
        36.0,
    );
    if let Some(action) = draw_top_utility_bar(&common_text.settings_button) {
        return Some(action);
    }
    draw_body_text(
        &day_results_text
            .subtitle_template
            .replace("{game_title}", &data.config.title),
        left_margin,
        top_margin + 32.0,
        20.0,
        theme::TEXT_BODY,
    );

    let panel_y = top_margin + 56.0;
    draw_tier_panel(
        left_margin,
        panel_y,
        panel_width,
        188.0,
        Some(&day_results_text.guild_jobs_panel_title),
        PanelTier::Primary,
        false,
    );
    let guild_job_lines = vec![
        format!(
            "{}: {}",
            day_results_text.gold_earned_label, summary.guild_job_gold
        ),
        format!(
            "{}: {}",
            day_results_text.upkeep_paid_label, summary.upkeep_gold
        ),
        format!(
            "{}: {}",
            day_results_text.upkeep_shortfall_label, summary.upkeep_shortfall
        ),
        format!(
            "{}: {}",
            day_results_text.arcane_residue_earned_label, summary.guild_job_arcane_residue
        ),
    ];
    draw_wrapped_lines(
        &guild_job_lines,
        left_margin + 16.0,
        panel_y + 60.0,
        18.0,
        theme::TEXT_BODY,
    );

    let expedition_x = left_margin + panel_width + 12.0;
    draw_tier_panel(
        expedition_x,
        panel_y,
        panel_width,
        188.0,
        Some(&day_results_text.expedition_panel_title),
        PanelTier::Primary,
        false,
    );
    let expedition_lines = vec![
        format!(
            "{}: {}",
            day_results_text.materials_label, summary.expedition_materials
        ),
        format!(
            "{}: {}",
            day_results_text.arcane_residue_label, summary.expedition_arcane_residue
        ),
        format!(
            "{}: {}",
            day_results_text.eggs_label, summary.expedition_eggs
        ),
        format!(
            "{}: {}",
            day_results_text.relics_label, summary.expedition_relics
        ),
    ];
    draw_wrapped_lines(
        &expedition_lines,
        expedition_x + 16.0,
        panel_y + 60.0,
        18.0,
        theme::TEXT_BODY,
    );

    let debt_x = expedition_x + panel_width + 12.0;
    draw_tier_panel(
        debt_x,
        panel_y,
        panel_width,
        188.0,
        Some(&day_results_text.debt_panel_title),
        PanelTier::Support,
        false,
    );
    let debt_lines = if summary.debt_updates.is_empty() {
        vec![day_results_text.no_debt_change_message.clone()]
    } else {
        summary.debt_updates.clone()
    };
    draw_wrapped_lines_in_box(
        &debt_lines,
        debt_x + 16.0,
        panel_y + 60.0,
        panel_width - 32.0,
        110.0,
        16.0,
        theme::TEXT_BODY,
    );

    let guest_x = debt_x + panel_width + 12.0;
    draw_tier_panel(
        guest_x,
        panel_y,
        panel_width,
        188.0,
        Some(&day_results_text.guests_panel_title),
        PanelTier::Primary,
        false,
    );
    let guest_lines = if summary.contract_updates.is_empty() {
        vec![day_results_text.no_guest_contract_message.clone()]
    } else {
        summary.contract_updates.clone()
    };
    draw_wrapped_lines_in_box(
        &guest_lines,
        guest_x + 16.0,
        panel_y + 60.0,
        panel_width - 32.0,
        110.0,
        16.0,
        theme::TEXT_BODY,
    );

    let events_y = panel_y + 206.0;
    draw_tier_panel(
        left_margin,
        events_y,
        (content_width - 12.0) / 2.0,
        220.0,
        Some(&day_results_text.roster_updates_panel_title),
        PanelTier::Primary,
        false,
    );
    draw_wrapped_lines_in_box(
        &summary.roster_updates,
        left_margin + 16.0,
        events_y + 56.0,
        (content_width - 12.0) / 2.0 - 32.0,
        140.0,
        18.0,
        theme::TEXT_BODY,
    );

    let event_log_x = left_margin + (content_width - 12.0) / 2.0 + 12.0;
    draw_tier_panel(
        event_log_x,
        events_y,
        (content_width - 12.0) / 2.0,
        220.0,
        Some(&day_results_text.event_log_panel_title),
        PanelTier::Primary,
        false,
    );
    let mut log_y = events_y + 56.0;
    if !summary.special_event_lines.is_empty() {
        draw_rectangle(
            event_log_x + 12.0,
            log_y - 18.0,
            (content_width - 12.0) / 2.0 - 24.0,
            76.0,
            Color::new(theme::WARNING.r, theme::WARNING.g, theme::WARNING.b, 0.12),
        );
        draw_wrapped_lines_in_box(
            &summary.special_event_lines,
            event_log_x + 18.0,
            log_y,
            (content_width - 12.0) / 2.0 - 36.0,
            66.0,
            18.0,
            theme::WARNING,
        );
        log_y += 86.0;
    }
    draw_wrapped_lines_in_box(
        &summary.event_lines,
        event_log_x + 16.0,
        log_y,
        (content_width - 12.0) / 2.0 - 32.0,
        events_y + 210.0 - log_y,
        18.0,
        theme::TEXT_BODY,
    );

    if primary_button(
        left_margin,
        events_y + 238.0,
        220.0,
        44.0,
        &day_results_text.continue_button,
    ) {
        return Some(UiAction::ContinueAfterResults);
    }

    if let Some(error_message) = last_error {
        draw_inline_status(
            left_margin + 246.0,
            events_y + 247.0,
            360.0,
            error_message,
            theme::DANGER,
        );
    }

    None
}
