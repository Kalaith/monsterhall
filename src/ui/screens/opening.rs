use macroquad::prelude::{screen_height, screen_width};

use crate::data::GameData;
use crate::state::{GameState, OpeningChapterState, OpeningChapterStep};
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_backdrop, draw_story_cg_placeholder, BackdropKind};
use crate::ui::chrome::{draw_inline_status, draw_tier_panel, PanelTier};
use crate::ui::core::{draw_heading_in_box, draw_wrapped_lines_in_box};
use crate::ui::core::{primary_button, secondary_button};
use crate::ui::theme;
use crate::ui::view_models::{
    format_resource_cost, history_gain_label_from_progress, opening_skill_gain_label,
};

pub fn draw_opening_chapter(
    data: &GameData,
    opening_state: &OpeningChapterState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    draw_backdrop(BackdropKind::Opening);

    let opening_text = &data.ui_text.opening;
    let step_id = match opening_state.step {
        OpeningChapterStep::Camp => "camp",
        OpeningChapterStep::Discovery => "discovery",
        OpeningChapterStep::Incubation => "incubation",
        OpeningChapterStep::Hatch => "hatch",
        OpeningChapterStep::BuildRoom => "build_room",
        OpeningChapterStep::FirstClient => "first_client",
        OpeningChapterStep::Complete => "first_client",
    };

    let step_data = data
        .story_events
        .opening_steps
        .iter()
        .find(|step| step.id == step_id)?;

    let panel_width = (screen_width() - 44.0).min(980.0);
    let panel_height = (screen_height() - 44.0).min(600.0);
    let panel_x = screen_width() * 0.5 - panel_width * 0.5;
    let panel_y = screen_height() * 0.5 - panel_height * 0.5;
    let body_y = panel_y + 100.0;
    let button_y = panel_y + panel_height - 70.0;
    let gain_h = if opening_state.step == OpeningChapterStep::FirstClient {
        54.0
    } else {
        0.0
    };
    let gain_y = button_y - gain_h - 12.0;
    let body_bottom = if gain_h > 0.0 {
        gain_y - 14.0
    } else {
        button_y - 14.0
    };
    let body_h = (body_bottom - body_y).max(150.0);
    let show_art = panel_width >= 900.0 && body_h >= 170.0;
    let art_size = 190.0_f32.min(panel_width * 0.26).min(body_h);
    let text_width = if show_art {
        panel_width - art_size - 86.0
    } else {
        panel_width - 48.0
    };
    let button_width = match opening_state.step {
        OpeningChapterStep::BuildRoom => (panel_width - 48.0).min(480.0),
        OpeningChapterStep::FirstClient => (panel_width - 48.0).min(540.0),
        _ => (panel_width - 48.0).min(340.0),
    };

    draw_tier_panel(
        panel_x,
        panel_y,
        panel_width,
        panel_height,
        Some(&opening_text.panel_title),
        PanelTier::Primary,
        true,
    );
    draw_heading_in_box(
        &step_data.title,
        panel_x + 24.0,
        panel_y + 34.0,
        panel_width - 48.0,
        48.0,
        38.0,
    );
    draw_wrapped_lines_in_box(
        &step_data.body_lines,
        panel_x + 24.0,
        body_y,
        text_width,
        body_h,
        29.0,
        theme::TEXT_STRONG,
    );
    if show_art {
        draw_story_cg_placeholder(
            &step_data.title,
            panel_x + panel_width - art_size - 24.0,
            body_y + 4.0,
            art_size,
            art_size,
            &step_data.id,
        );
    }

    let status_x = panel_x + 24.0 + button_width + 24.0;
    let status_w = panel_x + panel_width - 24.0 - status_x;
    if status_w >= 180.0 {
        let status_lines = vec![format!(
            "{} {}  {} {}  {} {}  {} {}  {} {}",
            opening_text.status_day_label,
            game_state.current_day,
            opening_text.status_gold_label,
            game_state.resources.gold,
            opening_text.status_materials_label,
            game_state.resources.tower_materials,
            opening_text.status_eggs_label,
            game_state.resources.eggs,
            opening_text.status_roster_label,
            game_state.monsters.len()
        )];
        draw_wrapped_lines_in_box(
            &status_lines,
            status_x,
            button_y + 2.0,
            status_w,
            48.0,
            18.0,
            theme::TEXT_BODY,
        );
    }

    if opening_state.step == OpeningChapterStep::BuildRoom {
        let cost_label = format!(
            "{} ({})",
            step_data.primary_action_label,
            format_resource_cost(&data.ui_text, &data.story_events.first_room_cost)
        );
        if primary_button(panel_x + 24.0, button_y, button_width, 50.0, &cost_label) {
            return Some(UiAction::BuildOpeningRoom);
        }
    } else if opening_state.step == OpeningChapterStep::FirstClient {
        let reward_label = opening_text
            .first_client_reward_template
            .replace("{label}", &step_data.primary_action_label)
            .replace(
                "{gold}",
                &data.story_events.first_client_reward.gold.to_string(),
            )
            .replace(
                "{residue}",
                &data
                    .story_events
                    .first_client_reward
                    .arcane_residue
                    .to_string(),
            );
        let gain_lines = vec![
            format!(
                "{}: {}",
                opening_text.skill_gains_label,
                opening_skill_gain_label(data, &data.story_events.first_client_skill_gains)
            ),
            format!(
                "{}: {}",
                opening_text.work_history_gains_label,
                history_gain_label_from_progress(
                    data,
                    &data.story_events.first_client_work_history_gains,
                )
            ),
        ];
        draw_wrapped_lines_in_box(
            &gain_lines,
            panel_x + 24.0,
            gain_y,
            panel_width - 48.0,
            gain_h,
            20.0,
            theme::TEXT_BODY,
        );
        if primary_button(panel_x + 24.0, button_y, button_width, 50.0, &reward_label) {
            return Some(UiAction::ResolveOpeningClient);
        }
    } else if secondary_button(
        panel_x + 24.0,
        button_y,
        button_width,
        50.0,
        &step_data.primary_action_label,
    ) {
        return Some(UiAction::ContinueOpening);
    }

    if let Some(error_message) = last_error {
        draw_inline_status(
            panel_x + 24.0,
            button_y - 44.0,
            panel_width - 48.0,
            error_message,
            theme::DANGER,
        );
    }

    None
}
