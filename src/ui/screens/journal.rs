use macroquad::prelude::{screen_height, screen_width};

use crate::data::GameData;
use crate::state::{GameState, JournalState};
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_backdrop, BackdropKind};
use crate::ui::chrome::{
    draw_inline_status, draw_screen_header, draw_standard_gameplay_footer, draw_tier_panel,
    draw_top_utility_bar, PanelTier,
};
use crate::ui::core::{
    draw_body_text, draw_body_text_in_box, draw_wrapped_lines_in_box, primary_button,
};
use crate::ui::feedback::draw_inline_error;
use crate::ui::layout;
use crate::ui::theme;
use crate::ui::view_models::{action_from_action_hint, daily_priority_summary, onboarding_lines};

pub fn draw_journal(
    data: &GameData,
    journal_state: &JournalState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    draw_backdrop(BackdropKind::Town);

    let ui = &data.ui_text.journal;
    let left_margin = layout::OUTER_MARGIN;
    let content_width = screen_width() - left_margin * 2.0;
    let top_y = 92.0;
    let upper_panel_h = 212.0;
    let panel_gap = layout::SECTION_GAP;
    let left_panel_w = 448.0;
    let right_panel_w = content_width - left_panel_w - panel_gap;
    let footer_y = screen_height() - layout::FOOTER_BOTTOM_MARGIN - layout::FOOTER_H;
    let log_y = top_y + upper_panel_h + panel_gap;
    let log_h = (footer_y - log_y - panel_gap).max(260.0);

    if let Some(action) = draw_top_utility_bar(&data.ui_text.common.settings_button) {
        return Some(action);
    }
    draw_screen_header(&ui.title, &ui.subtitle);

    let priority = daily_priority_summary(data, game_state);
    draw_tier_panel(
        left_margin,
        top_y,
        left_panel_w,
        upper_panel_h,
        Some(&ui.current_priority_panel_title),
        PanelTier::Primary,
        true,
    );
    draw_inline_status(
        left_margin + layout::PANEL_PADDING,
        top_y + 50.0,
        left_panel_w - layout::PANEL_PADDING * 2.0,
        &format!("{}: {}", ui.priority_label, priority.title),
        priority.color,
    );
    draw_body_text_in_box(
        &priority.detail,
        left_margin + layout::PANEL_PADDING,
        top_y + 92.0,
        left_panel_w - layout::PANEL_PADDING * 2.0,
        64.0,
        17.0,
        theme::TEXT_BODY,
    );
    if primary_button(
        left_margin + layout::PANEL_PADDING,
        top_y + upper_panel_h - 54.0,
        168.0,
        layout::PRIMARY_BUTTON_H,
        &priority.action_hint,
    ) {
        return Some(action_from_action_hint(data, &priority.action_hint));
    }

    draw_tier_panel(
        left_margin + left_panel_w + panel_gap,
        top_y,
        right_panel_w,
        upper_panel_h,
        Some(&ui.guidance_panel_title),
        PanelTier::Support,
        false,
    );
    draw_wrapped_lines_in_box(
        &onboarding_lines(data, game_state),
        left_margin + left_panel_w + panel_gap + layout::PANEL_PADDING,
        top_y + 52.0,
        right_panel_w - layout::PANEL_PADDING * 2.0,
        upper_panel_h - 72.0,
        16.0,
        theme::TEXT_BODY,
    );

    draw_tier_panel(
        left_margin,
        log_y,
        content_width,
        log_h,
        Some(&ui.event_log_panel_title),
        PanelTier::Support,
        false,
    );

    let recent_events = game_state
        .event_log
        .iter()
        .rev()
        .cloned()
        .collect::<Vec<_>>();
    let visible_rows = 12usize;
    let max_scroll = recent_events.len().saturating_sub(visible_rows);
    let start_index = journal_state.event_log_scroll.min(max_scroll);
    let visible_events = if recent_events.is_empty() {
        vec![ui.recent_events_empty_message.clone()]
    } else {
        recent_events
            .iter()
            .skip(start_index)
            .take(visible_rows)
            .cloned()
            .collect::<Vec<_>>()
    };
    draw_wrapped_lines_in_box(
        &visible_events,
        left_margin + layout::PANEL_PADDING,
        log_y + 52.0,
        content_width - layout::PANEL_PADDING * 2.0,
        log_h - 92.0,
        16.0,
        theme::TEXT_BODY,
    );

    if start_index > 0 {
        draw_body_text(
            &ui.scroll_up_message,
            left_margin + layout::PANEL_PADDING,
            log_y + 28.0,
            14.0,
            theme::TEXT_MUTED,
        );
    }
    if start_index < max_scroll {
        draw_body_text(
            &ui.scroll_down_message,
            left_margin + layout::PANEL_PADDING,
            log_y + log_h - 16.0,
            14.0,
            theme::TEXT_MUTED,
        );
    }

    if let Some(action) = draw_standard_gameplay_footer(
        data,
        left_margin,
        footer_y,
        content_width,
        Some(UiAction::OpenJournal),
    ) {
        return Some(action);
    }

    if let Some(error_message) = last_error {
        draw_inline_error(left_margin, footer_y - 30.0, content_width, error_message);
    }

    None
}
