use macroquad::prelude::{draw_rectangle, screen_height, screen_width, Color};

use crate::data::GameData;
use crate::state::DayResultsState;
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_backdrop, BackdropKind};
use crate::ui::chrome::{draw_inline_status, draw_tier_panel, draw_top_utility_bar, PanelTier};
use crate::ui::core::primary_button;
use crate::ui::core::{
    draw_body_text, draw_wrapped_lines, draw_wrapped_lines_in_box, scaled_font_size, scaled_spacing,
};
use crate::ui::theme;
use crate::ui::view_models::fill_template;

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
    // Six lines at roughly 30px from `panel_y + 60`. The panels were 188 tall
    // for the four lines they carried before the cost breakdown, prep spend and
    // offer counts were added, so the last line of Town Jobs and Expedition was
    // sliced by the frame and spilled into the row beneath.
    const SUMMARY_PANEL_H: f32 = 248.0;
    draw_tier_panel(
        left_margin,
        panel_y,
        panel_width,
        SUMMARY_PANEL_H,
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
        fill_template(
            &day_results_text.upkeep_breakdown_template,
            &[
                ("{wages}", summary.upkeep_wage_gold.to_string()),
                ("{cleaning}", summary.upkeep_cleaning_gold.to_string()),
                ("{maintenance}", summary.upkeep_maintenance_gold.to_string()),
            ],
        ),
        format!(
            "{}: {}",
            day_results_text.upkeep_shortfall_label, summary.upkeep_shortfall
        ),
        format!(
            "{}: {}",
            day_results_text.arcane_residue_earned_label, summary.guild_job_arcane_residue
        ),
        fill_template(
            &day_results_text.special_events_template,
            &[
                ("{count}", summary.special_event_count.to_string()),
                ("{gold}", summary.special_event_gold_delta.to_string()),
            ],
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
        SUMMARY_PANEL_H,
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
        fill_template(
            &day_results_text.expedition_prep_template,
            &[
                ("{gold}", summary.expedition_prep_gold.to_string()),
                ("{materials}", summary.expedition_prep_materials.to_string()),
                (
                    "{residue}",
                    summary.expedition_prep_arcane_residue.to_string(),
                ),
            ],
        ),
        format!(
            "{}: {}",
            day_results_text.expedition_prep_shortfall_label, summary.expedition_prep_shortfall
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
        SUMMARY_PANEL_H,
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
        170.0,
        16.0,
        theme::TEXT_BODY,
    );

    let guest_x = debt_x + panel_width + 12.0;
    draw_tier_panel(
        guest_x,
        panel_y,
        panel_width,
        SUMMARY_PANEL_H,
        Some(&day_results_text.guests_panel_title),
        PanelTier::Primary,
        false,
    );
    // The offer flow was entirely invisible: the desk shows what is on it, and
    // nothing said how many came in or how many the guild had no room for.
    let mut guest_lines = if summary.contract_updates.is_empty() {
        vec![day_results_text.no_guest_contract_message.clone()]
    } else {
        summary.contract_updates.clone()
    };
    guest_lines.push(fill_template(
        &day_results_text.contract_offers_template,
        &[
            ("{generated}", summary.contracts_generated.to_string()),
            ("{rejected}", summary.contracts_rejected.to_string()),
        ],
    ));
    draw_wrapped_lines_in_box(
        &guest_lines,
        guest_x + 16.0,
        panel_y + 60.0,
        panel_width - 32.0,
        170.0,
        16.0,
        theme::TEXT_BODY,
    );

    let events_y = panel_y + SUMMARY_PANEL_H + 18.0;
    // Both narrative panels were a fixed 220 tall with a 140px text box, in the
    // top half of a screen whose bottom half is empty backdrop. A day where the
    // whole guild works produces two lines per companion, so a roster of twenty
    // filled six of them and the other thirty-odd were dropped — with nothing on
    // screen to say they existed. `roster_updates` is not written to the event
    // log either, so those lines were gone for good. The panels now take the
    // height that was already there, and whatever still does not fit is counted
    // out loud below.
    let events_h = (screen_height() - events_y - CONTINUE_BUTTON_H - 56.0).max(220.0);
    let panel_w = (content_width - 12.0) / 2.0;
    let text_h = events_h - EVENTS_PANEL_CHROME_H;

    draw_tier_panel(
        left_margin,
        events_y,
        panel_w,
        events_h,
        Some(&day_results_text.roster_updates_panel_title),
        PanelTier::Primary,
        false,
    );
    draw_overflow_aware_lines(
        &summary.roster_updates,
        left_margin + 16.0,
        events_y + 56.0,
        panel_w - 32.0,
        text_h,
        events_y + events_h - 18.0,
        &day_results_text.more_lines_template,
    );

    let event_log_x = left_margin + panel_w + 12.0;
    draw_tier_panel(
        event_log_x,
        events_y,
        panel_w,
        events_h,
        Some(&day_results_text.event_log_panel_title),
        PanelTier::Primary,
        false,
    );
    let mut log_y = events_y + 56.0;
    if !summary.special_event_lines.is_empty() {
        draw_rectangle(
            event_log_x + 12.0,
            log_y - 18.0,
            panel_w - 24.0,
            76.0,
            Color::new(theme::WARNING.r, theme::WARNING.g, theme::WARNING.b, 0.12),
        );
        draw_wrapped_lines_in_box(
            &summary.special_event_lines,
            event_log_x + 18.0,
            log_y,
            panel_w - 36.0,
            66.0,
            18.0,
            theme::WARNING,
        );
        log_y += 86.0;
    }
    draw_overflow_aware_lines(
        &summary.event_lines,
        event_log_x + 16.0,
        log_y,
        panel_w - 32.0,
        events_y + 56.0 + text_h - log_y,
        events_y + events_h - 18.0,
        &day_results_text.more_lines_template,
    );

    if primary_button(
        left_margin,
        events_y + events_h + 12.0,
        220.0,
        CONTINUE_BUTTON_H,
        &day_results_text.continue_button,
    ) {
        return Some(UiAction::ContinueAfterResults);
    }

    if let Some(error_message) = last_error {
        draw_inline_status(
            left_margin + 246.0,
            events_y + events_h + 21.0,
            360.0,
            error_message,
            theme::DANGER,
        );
    }

    None
}

/// Height of the Continue button, and the chrome each narrative panel spends on
/// its title and its overflow note.
const CONTINUE_BUTTON_H: f32 = 44.0;
const EVENTS_PANEL_CHROME_H: f32 = 92.0;
/// Font size and line gap the narrative panels draw with.
const EVENTS_FONT_SIZE: f32 = 18.0;
const EVENTS_LINE_GAP: f32 = 6.0;

/// Draws as many lines as the box holds and says out loud how many it could not.
///
/// `draw_wrapped_lines_in_box` clips whatever runs past the bottom, silently. On
/// a day when the whole guild works that meant the report showed three
/// companions out of twenty and gave no sign the other seventeen had a day at
/// all — and `roster_updates` is never written to the event log, so there was
/// nowhere else to find them. Bounding coverage is fine; bounding it without
/// saying so is what turns a full box into a wrong report.
fn draw_overflow_aware_lines(
    lines: &[String],
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    note_y: f32,
    more_template: &str,
) {
    let row_h = scaled_font_size(EVENTS_FONT_SIZE) + scaled_spacing(EVENTS_LINE_GAP);
    let capacity = ((height / row_h).floor() as usize).max(1);

    if lines.len() <= capacity {
        draw_wrapped_lines_in_box(
            lines,
            x,
            y,
            width,
            height,
            EVENTS_FONT_SIZE,
            theme::TEXT_BODY,
        );
        return;
    }

    draw_wrapped_lines_in_box(
        &lines[..capacity],
        x,
        y,
        width,
        height,
        EVENTS_FONT_SIZE,
        theme::TEXT_BODY,
    );
    draw_body_text(
        &fill_template(
            more_template,
            &[("{count}", (lines.len() - capacity).to_string())],
        ),
        x,
        note_y,
        14.0,
        theme::TEXT_MUTED,
    );
}
