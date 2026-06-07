use macroquad::prelude::*;
use macroquad::time::get_time;

use crate::data::GameData;
use crate::state::{EggState, GameState, HatchRevealState, CompanionState};
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_backdrop, draw_egg_thumbnail, draw_species_portrait, BackdropKind};
use crate::ui::chrome::{
    draw_inline_status, draw_screen_header, draw_tier_panel, draw_top_utility_bar, PanelTier,
};
use crate::ui::core::{
    draw_body_text, draw_body_text_in_box, draw_heading_in_box, draw_wrapped_lines_in_box,
    primary_button,
};
use crate::ui::layout;
use crate::ui::theme;
use crate::ui::view_models::{
    egg_grade_label, monster_quality_label, species_name_by_id, trait_names_for_monster,
};

pub fn draw_hatch_reveal(
    data: &GameData,
    hatch_state: &HatchRevealState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    draw_backdrop(BackdropKind::Chamber);
    if let Some(action) = draw_top_utility_bar("Settings") {
        return Some(action);
    }
    draw_screen_header(
        "Hatchery Reveal",
        "Watch the egg open and confirm the companion joining the roster.",
    );

    let now = get_time();
    let elapsed = hatch_state.elapsed_seconds(now);
    let crack_progress = ease_out(((elapsed - 0.45) / 1.0).clamp(0.0, 1.0));
    let reveal_progress = ease_out(((elapsed - 1.35) / 0.85).clamp(0.0, 1.0));
    let is_complete = hatch_state.is_complete(now);
    let monster = game_state
        .monsters
        .iter()
        .find(|entry| entry.id == hatch_state.monster_id);
    let monster_name = monster
        .map(|entry| entry.name.as_str())
        .unwrap_or("The new companion");

    let panel_x = layout::OUTER_MARGIN;
    let panel_y = 96.0;
    let panel_w = (screen_width() - layout::OUTER_MARGIN * 2.0).max(520.0);
    let panel_h = (screen_height() - panel_y - 38.0).max(500.0);
    draw_tier_panel(
        panel_x,
        panel_y,
        panel_w,
        panel_h,
        Some("Hatchery reveal"),
        PanelTier::Primary,
        true,
    );

    draw_heading_in_box(
        &stage_title(elapsed, monster_name),
        panel_x + 28.0,
        panel_y + 34.0,
        panel_w - 56.0,
        42.0,
        31.0,
    );
    draw_body_text_in_box(
        &stage_body(elapsed, monster_name),
        panel_x + 42.0,
        panel_y + 80.0,
        panel_w - 84.0,
        48.0,
        18.0,
        theme::TEXT_BODY,
    );

    let button_y = panel_y + panel_h - 68.0;
    let info_y = button_y - 122.0;
    let media_y = panel_y + 148.0;
    let media_h = (info_y - media_y - 18.0).max(190.0);
    let gap = 24.0;
    let column_w = (panel_w - 64.0 - gap) * 0.5;
    let egg_x = panel_x + 32.0;
    let portrait_x = egg_x + column_w + gap;

    draw_hatching_egg(
        &hatch_state.egg,
        egg_x,
        media_y,
        column_w,
        media_h,
        elapsed,
        crack_progress,
        reveal_progress,
    );
    draw_emergence_portrait(
        data,
        monster,
        portrait_x,
        media_y,
        column_w,
        media_h,
        reveal_progress,
    );
    draw_outcome_panel(
        data,
        &hatch_state.egg,
        monster,
        panel_x + 32.0,
        info_y,
        panel_w - 64.0,
        104.0,
        reveal_progress,
    );

    if is_complete {
        if primary_button(panel_x + panel_w - 252.0, button_y, 220.0, 44.0, "Continue") {
            return Some(UiAction::ContinueAfterHatch);
        }
        draw_body_text(
            "Press Enter or Space to continue",
            panel_x + 36.0,
            button_y + 28.0,
            16.0,
            theme::TEXT_MUTED,
        );
    } else {
        draw_body_text_in_box(
            "Stabilizing hatchery output...",
            panel_x + 36.0,
            button_y + 8.0,
            panel_w - 72.0,
            28.0,
            16.0,
            theme::TEXT_MUTED,
        );
    }

    if let Some(error_message) = last_error {
        draw_inline_status(
            panel_x + 32.0,
            button_y - 42.0,
            panel_w - 64.0,
            error_message,
            theme::DANGER,
        );
    }

    None
}

fn stage_title(elapsed: f32, monster_name: &str) -> String {
    if elapsed < 0.7 {
        "Egg hatching".to_owned()
    } else if elapsed < 1.45 {
        "Shell fracture".to_owned()
    } else {
        format!("{monster_name} emerged")
    }
}

fn stage_body(elapsed: f32, monster_name: &str) -> String {
    if elapsed < 0.7 {
        "The hatchery egg shivers as heat and arcane residue collect beneath the shell.".to_owned()
    } else if elapsed < 1.45 {
        "Hairline cracks split across the shell and a bright pulse leaks through.".to_owned()
    } else {
        format!("{monster_name} is now registered to the keep roster.")
    }
}

fn draw_hatching_egg(
    egg: &EggState,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    elapsed: f32,
    crack_progress: f32,
    reveal_progress: f32,
) {
    let wobble = (elapsed * 18.0).sin() * 4.0 * (1.0 - reveal_progress);
    let glow = 0.12 + 0.16 * (elapsed * 4.0).sin().abs() + reveal_progress * 0.18;
    draw_circle(
        x + w * 0.5,
        y + h * 0.52,
        (w.min(h) * (0.34 + reveal_progress * 0.12)).max(60.0),
        Color::new(theme::GOLD.r, theme::GOLD.g, theme::GOLD.b, glow),
    );
    draw_egg_thumbnail(egg, x + wobble, y, w, h);
    draw_cracks(x + wobble, y, w, h, crack_progress);
    draw_shell_burst(x + wobble, y, w, h, reveal_progress);
}

fn draw_cracks(x: f32, y: f32, w: f32, h: f32, progress: f32) {
    if progress <= 0.02 {
        return;
    }

    let center_x = x + w * 0.5;
    let top_y = y + h * 0.29;
    let mid_y = y + h * 0.5;
    let bottom_y = y + h * 0.68;
    let line = Color::new(0.08, 0.05, 0.1, 0.9);
    let light = Color::new(theme::GOLD.r, theme::GOLD.g, theme::GOLD.b, 0.7 * progress);
    let length = 54.0 * progress;

    draw_line(center_x, top_y, center_x - length * 0.28, mid_y, 4.0, line);
    draw_line(
        center_x - length * 0.28,
        mid_y,
        center_x + length * 0.18,
        bottom_y,
        4.0,
        line,
    );
    draw_line(
        center_x + 2.0,
        top_y,
        center_x - length * 0.2,
        mid_y,
        1.5,
        light,
    );
    draw_line(
        center_x - length * 0.2,
        mid_y,
        center_x + length * 0.16,
        bottom_y,
        1.5,
        light,
    );

    if progress > 0.35 {
        let branch = (progress - 0.35) / 0.65;
        draw_line(
            center_x - length * 0.18,
            mid_y,
            center_x - 42.0 * branch,
            mid_y - 30.0 * branch,
            3.0,
            line,
        );
        draw_line(
            center_x + length * 0.12,
            bottom_y - 8.0,
            center_x + 44.0 * branch,
            bottom_y - 30.0 * branch,
            3.0,
            line,
        );
    }
}

fn draw_shell_burst(x: f32, y: f32, w: f32, h: f32, reveal_progress: f32) {
    if reveal_progress <= 0.05 {
        return;
    }

    let burst = ((reveal_progress - 0.05) / 0.95).clamp(0.0, 1.0);
    let center_x = x + w * 0.5;
    let center_y = y + h * 0.48;
    let color = Color::new(1.0, 0.91, 0.62, 0.32 * (1.0 - burst * 0.4));
    draw_circle(center_x, center_y, w.min(h) * (0.18 + burst * 0.18), color);

    let shell = Color::new(0.88, 0.76, 0.94, 0.76 * (1.0 - burst * 0.5));
    for index in 0..5 {
        let angle = -2.2 + index as f32 * 1.1;
        let distance = 28.0 + 54.0 * burst;
        let shard_x = center_x + angle.cos() * distance;
        let shard_y = center_y + angle.sin() * distance * 0.72;
        draw_triangle(
            vec2(shard_x, shard_y - 10.0),
            vec2(shard_x + 14.0, shard_y + 8.0),
            vec2(shard_x - 12.0, shard_y + 8.0),
            shell,
        );
    }
}

fn draw_emergence_portrait(
    data: &GameData,
    monster: Option<&CompanionState>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    reveal_progress: f32,
) {
    draw_tier_panel(
        x,
        y,
        w,
        h,
        Some("Generated companion"),
        PanelTier::Support,
        reveal_progress > 0.8,
    );
    let inner_x = x + 16.0;
    let inner_y = y + 52.0;
    let inner_w = w - 32.0;
    let inner_h = h - 68.0;

    if let Some(monster) = monster {
        if reveal_progress > 0.0 {
            draw_species_portrait(data, monster, inner_x, inner_y, inner_w, inner_h);
            let cover_alpha = 1.0 - reveal_progress;
            draw_rectangle(
                inner_x,
                inner_y,
                inner_w,
                inner_h,
                Color::new(theme::BG_1.r, theme::BG_1.g, theme::BG_1.b, cover_alpha),
            );
        } else {
            draw_waiting_silhouette(inner_x, inner_y, inner_w, inner_h);
        }
    } else {
        draw_body_text_in_box(
            "The hatch succeeded, but this roster record is no longer available.",
            inner_x,
            inner_y + 32.0,
            inner_w,
            inner_h - 64.0,
            17.0,
            theme::TEXT_BODY,
        );
    }
}

fn draw_waiting_silhouette(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(
        x,
        y,
        w,
        h,
        Color::new(theme::PANEL_0.r, theme::PANEL_0.g, theme::PANEL_0.b, 0.9),
    );
    draw_circle(
        x + w * 0.5,
        y + h * 0.34,
        w.min(h) * 0.12,
        Color::new(theme::INFO.r, theme::INFO.g, theme::INFO.b, 0.24),
    );
    draw_ellipse(
        x + w * 0.5,
        y + h * 0.62,
        w * 0.2,
        h * 0.25,
        0.0,
        Color::new(theme::INFO.r, theme::INFO.g, theme::INFO.b, 0.20),
    );
    draw_rectangle_lines(
        x,
        y,
        w,
        h,
        1.0,
        Color::new(theme::BORDER_1.r, theme::BORDER_1.g, theme::BORDER_1.b, 0.6),
    );
}

fn draw_outcome_panel(
    data: &GameData,
    egg: &EggState,
    monster: Option<&CompanionState>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    reveal_progress: f32,
) {
    draw_tier_panel(
        x,
        y,
        w,
        h,
        Some("Result"),
        PanelTier::Utility,
        reveal_progress > 0.9,
    );
    let lines = outcome_lines(data, egg, monster, reveal_progress);
    draw_wrapped_lines_in_box(
        &lines,
        x + 18.0,
        y + 42.0,
        w - 36.0,
        h - 50.0,
        16.0,
        theme::TEXT_BODY,
    );
}

fn outcome_lines(
    data: &GameData,
    egg: &EggState,
    monster: Option<&CompanionState>,
    reveal_progress: f32,
) -> Vec<String> {
    if reveal_progress < 0.9 {
        return vec![format!(
            "Egg grade: {}. Outcome stabilizing...",
            egg_grade_label(egg, data)
        )];
    }

    if let Some(monster) = monster {
        vec![
            format!(
                "{} emerged from a {} egg.",
                monster.name,
                egg_grade_label(egg, data)
            ),
            format!("Species: {}", species_name_by_id(data, &monster.species_id)),
            format!("Quality: {}", monster_quality_label(monster)),
            format!("Bond {} / Reputation {}", monster.bond, monster.reputation),
            format!("Traits: {}", trait_names_for_monster(data, monster)),
        ]
    } else {
        vec![
            format!("Egg grade: {}", egg_grade_label(egg, data)),
            "Roster profile unavailable.".to_owned(),
        ]
    }
}

fn ease_out(value: f32) -> f32 {
    1.0 - (1.0 - value).powi(3)
}
