use macroquad::prelude::*;

use crate::data::GameData;
use crate::state::{CompanionState, GameState, MonsterProfileState};
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_backdrop, draw_species_portrait, draw_trait_icons, BackdropKind};
use crate::ui::chrome::{
    draw_screen_header, draw_standard_gameplay_footer, draw_tier_panel, draw_top_utility_bar,
    PanelTier,
};
use crate::ui::components::draw_metric_tile;
use crate::ui::core::{draw_body_text, draw_body_text_in_box, utility_button};
use crate::ui::feedback::draw_inline_error;
use crate::ui::layout;
use crate::ui::theme;
use crate::ui::view_models::{
    assignment_label, fill_template, monster_quality_label, monster_role_summary,
    species_name_by_id, trait_names_for_monster,
};
use macroquad_toolkit::ui::{measure_text_size, TextStyle};

struct MonsterProfileLayout {
    left_margin: f32,
    content_width: f32,
    summary_y: f32,
    content_y: f32,
    content_h: f32,
    footer_y: f32,
}

impl MonsterProfileLayout {
    fn new() -> Self {
        let left_margin = layout::OUTER_MARGIN;
        let content_width = screen_width() - left_margin * 2.0;
        let footer_y = screen_height() - layout::FOOTER_BOTTOM_MARGIN - layout::FOOTER_H;
        let summary_y = 88.0;
        let content_y = 172.0;
        let content_h = (footer_y - content_y - 12.0).max(292.0);
        Self {
            left_margin,
            content_width,
            summary_y,
            content_y,
            content_h,
            footer_y,
        }
    }
}

pub fn draw_monster_profile(
    data: &GameData,
    profile_state: &MonsterProfileState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    draw_backdrop(BackdropKind::Profile);
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.025, 0.012, 0.035, 0.66),
    );

    let layout = MonsterProfileLayout::new();
    let ui = &data.ui_text.monster_profile;

    let Some(monster) = game_state
        .monsters
        .iter()
        .find(|monster| monster.id == profile_state.selected_monster_id)
    else {
        return Some(UiAction::ReturnToTownOverview);
    };

    if let Some(action) = draw_top_utility_bar(&data.ui_text.common.settings_button) {
        return Some(action);
    }

    draw_screen_header(
        &fill_template(&ui.title_template, &[("{name}", monster.name.clone())]),
        &ui.subtitle,
    );

    let role_summary = monster_role_summary(data, monster);
    let species_name = species_name_by_id(data, &monster.species_id);
    let assignment = assignment_label(data, &monster.current_job);

    draw_profile_summary(
        data,
        monster,
        &layout,
        &species_name,
        assignment,
        &role_summary,
    );

    let gap = 16.0;
    let identity_w = (layout.content_width * 0.29).clamp(248.0, 292.0);
    let right_x = layout.left_margin + identity_w + gap;
    let right_w = layout.content_width - identity_w - gap;
    let lower_y = layout.content_y + 166.0;
    let lower_h = (layout.content_h - 166.0).max(148.0);
    let lower_w = (right_w - gap) * 0.5;

    if let Some(action) = draw_identity_panel(
        data,
        monster,
        &layout,
        identity_w,
        &species_name,
        assignment,
        &role_summary,
    ) {
        return Some(action);
    }

    draw_best_use_panel(
        data,
        monster,
        right_x,
        layout.content_y,
        right_w,
        152.0,
        &role_summary,
    );
    draw_stats_panel(data, monster, right_x, lower_y, lower_w, lower_h);
    draw_traits_panel(
        data,
        monster,
        right_x + lower_w + gap,
        lower_y,
        lower_w,
        lower_h,
    );

    if let Some(error_message) = last_error {
        draw_inline_error(
            right_x,
            (layout.footer_y - 34.0).max(lower_y + lower_h - 28.0),
            right_w,
            error_message,
        );
    }

    draw_standard_gameplay_footer(
        data,
        layout.left_margin,
        layout.footer_y,
        layout.content_width,
        None,
    )
}

fn draw_profile_summary(
    data: &GameData,
    monster: &CompanionState,
    layout: &MonsterProfileLayout,
    species_name: &str,
    assignment: &str,
    role_summary: &crate::ui::view_models::MonsterRoleSummary,
) {
    let ui = &data.ui_text.monster_profile;
    draw_tier_panel(
        layout.left_margin,
        layout.summary_y,
        layout.content_width,
        72.0,
        Some(&ui.profile_summary_panel_title),
        PanelTier::Primary,
        true,
    );

    let chip_y = layout.summary_y + 31.0;
    let chip_h = 34.0;
    let x = layout.left_margin + 18.0;
    let available_w = layout.content_width - 36.0;
    let gap = 12.0;
    let species_w = available_w * 0.28;
    let assignment_w = available_w * 0.20;
    let readiness_w = available_w * 0.29;
    let quality_w = available_w - species_w - assignment_w - readiness_w - gap * 3.0;
    draw_profile_chip(
        x,
        chip_y,
        species_w,
        chip_h,
        &ui.species_label,
        species_name,
        theme::TEXT_BODY,
    );
    draw_profile_chip(
        x + species_w + gap,
        chip_y,
        assignment_w,
        chip_h,
        "Assignment",
        assignment,
        role_summary.readiness_color,
    );
    draw_profile_chip(
        x + species_w + assignment_w + gap * 2.0,
        chip_y,
        readiness_w,
        chip_h,
        "Readiness",
        &role_summary.readiness_label,
        role_summary.readiness_color,
    );
    draw_profile_chip(
        x + species_w + assignment_w + readiness_w + gap * 3.0,
        chip_y,
        quality_w,
        chip_h,
        "Quality",
        &monster_quality_label(monster),
        theme::WARNING,
    );
}

fn draw_identity_panel(
    data: &GameData,
    monster: &CompanionState,
    layout: &MonsterProfileLayout,
    w: f32,
    species_name: &str,
    assignment: &str,
    role_summary: &crate::ui::view_models::MonsterRoleSummary,
) -> Option<UiAction> {
    let x = layout.left_margin;
    let y = layout.content_y;
    draw_tier_panel(
        x,
        y,
        w,
        layout.content_h,
        Some(&data.ui_text.monster_profile.portrait_panel_title),
        PanelTier::Support,
        false,
    );

    draw_species_portrait(data, monster, x + 18.0, y + 46.0, 82.0, 104.0);
    let text_x = x + 114.0;
    let text_w = w - 136.0;
    draw_body_text(&monster.name, text_x, y + 68.0, 23.0, theme::TEXT_STRONG);
    draw_body_text_in_box(
        species_name,
        text_x,
        y + 82.0,
        text_w,
        38.0,
        16.0,
        theme::TEXT_BODY,
    );
    draw_profile_chip(
        x + 18.0,
        y + 164.0,
        w - 40.0,
        30.0,
        "State",
        assignment,
        role_summary.readiness_color,
    );
    draw_profile_chip(
        x + 18.0,
        y + 200.0,
        w - 40.0,
        30.0,
        "Readiness",
        &role_summary.readiness_label,
        role_summary.readiness_color,
    );

    draw_line(
        x + 18.0,
        y + 240.0,
        x + w - 22.0,
        y + 238.0,
        1.0,
        Color::new(
            theme::BORDER_1.r,
            theme::BORDER_1.g,
            theme::BORDER_1.b,
            0.46,
        ),
    );
    draw_body_text("Today", x + 22.0, y + 264.0, 17.0, theme::TEXT_STRONG);
    draw_body_text_in_box(
        &role_summary.best_next_use,
        x + 22.0,
        y + 276.0,
        w - 44.0,
        66.0,
        17.0,
        theme::TEXT_BODY,
    );

    let release_y = y + layout.content_h - 58.0;
    draw_body_text("Danger Zone", x + 22.0, release_y, 15.0, theme::DANGER);
    if utility_button(
        x + 22.0,
        release_y + 8.0,
        w - 44.0,
        34.0,
        &data.ui_text.monster_profile.release_button,
    ) {
        return Some(UiAction::ReleaseMonster(monster.id.clone()));
    }
    None
}

fn draw_best_use_panel(
    data: &GameData,
    monster: &CompanionState,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    role_summary: &crate::ui::view_models::MonsterRoleSummary,
) {
    let ui = &data.ui_text.monster_profile;
    draw_tier_panel(
        x,
        y,
        w,
        h,
        Some(&ui.best_next_use_panel_title),
        PanelTier::Primary,
        true,
    );
    draw_body_text_in_box(
        &role_summary.best_next_use,
        x + 20.0,
        y + 44.0,
        w * 0.36,
        58.0,
        18.0,
        theme::TEXT_STRONG,
    );
    draw_profile_chip(
        x + 20.0,
        y + 108.0,
        w * 0.36,
        32.0,
        "Readiness",
        &role_summary.readiness_label,
        role_summary.readiness_color,
    );

    let tile_w = ((w * 0.60) - 18.0) / 4.0;
    let tile_x = x + w * 0.40;
    draw_metric_tile(
        tile_x,
        y + 42.0,
        tile_w,
        58.0,
        &ui.fatigue_label,
        &monster.fatigue.to_string(),
        condition_color(monster.fatigue, 2, false),
    );
    draw_metric_tile(
        tile_x + tile_w + 6.0,
        y + 42.0,
        tile_w,
        58.0,
        &ui.stress_label,
        &monster.stress.to_string(),
        condition_color(monster.stress, 2, false),
    );
    draw_metric_tile(
        tile_x + (tile_w + 6.0) * 2.0,
        y + 42.0,
        tile_w,
        58.0,
        &ui.injury_label,
        &monster.injury.to_string(),
        condition_color(monster.injury, 0, true),
    );
    draw_metric_tile(
        tile_x + (tile_w + 6.0) * 3.0,
        y + 42.0,
        tile_w,
        58.0,
        "Instability",
        &monster.corruption.to_string(),
        theme::INFO,
    );

    let strength_y = y + 108.0;
    let strength_w = ((w * 0.60) - 24.0) / 3.0;
    draw_profile_chip(
        tile_x,
        strength_y,
        strength_w,
        30.0,
        &ui.power_label,
        &monster.stats.power.to_string(),
        theme::INFO,
    );
    draw_profile_chip(
        tile_x + strength_w + 8.0,
        strength_y,
        strength_w,
        30.0,
        &ui.charm_label,
        &monster.stats.charm.to_string(),
        theme::POSITIVE,
    );
    draw_profile_chip(
        tile_x + (strength_w + 8.0) * 2.0,
        strength_y,
        strength_w,
        30.0,
        &ui.endurance_label,
        &monster.stats.endurance.to_string(),
        theme::WARNING,
    );
}

fn draw_stats_panel(data: &GameData, monster: &CompanionState, x: f32, y: f32, w: f32, h: f32) {
    draw_tier_panel(
        x,
        y,
        w,
        h,
        Some(&data.ui_text.monster_profile.core_stats_panel_title),
        PanelTier::Support,
        false,
    );

    draw_gold_separator(x + 18.0, y + 46.0, w - 36.0);
    let chip_w = (w - 58.0) / 3.0;
    let chip_h = 30.0;
    let mut chip_x = x + 18.0;
    let mut chip_y = y + 62.0;
    for (index, (label, value, color)) in [
        ("Scout", monster.skills.scouting.to_string(), theme::INFO),
        ("Guard", monster.skills.guarding.to_string(), theme::WARNING),
        (
            "Hosp.",
            monster.skills.hospitality.to_string(),
            theme::POSITIVE,
        ),
        ("Craft", monster.skills.crafting.to_string(), theme::GOLD),
        ("Charm", monster.skills.charm.to_string(), theme::ROSE),
        ("Bond", monster.bond.to_string(), theme::PRIMARY),
    ]
    .iter()
    .enumerate()
    {
        draw_profile_chip(chip_x, chip_y, chip_w, chip_h, label, value, *color);
        chip_x += chip_w + 6.0;
        if index == 2 {
            chip_x = x + 18.0;
            chip_y += chip_h + 10.0;
        }
    }

    draw_gold_separator(x + 18.0, y + 148.0, w - 36.0);
    let tower_y = y + 164.0;
    let tower_w = (w - 60.0) / 4.0;
    draw_profile_chip(
        x + 18.0,
        tower_y,
        tower_w,
        chip_h,
        "Power",
        &monster.stats.power.to_string(),
        theme::INFO,
    );
    draw_profile_chip(
        x + 24.0 + tower_w,
        tower_y,
        tower_w,
        chip_h,
        "Charm",
        &monster.stats.charm.to_string(),
        theme::POSITIVE,
    );
    draw_profile_chip(
        x + 30.0 + tower_w * 2.0,
        tower_y,
        tower_w,
        chip_h,
        "Endure",
        &monster.stats.endurance.to_string(),
        theme::WARNING,
    );
    draw_profile_chip(
        x + 36.0 + tower_w * 3.0,
        tower_y,
        tower_w,
        chip_h,
        "Instinct",
        &monster.stats.instinct.to_string(),
        theme::ROSE,
    );
}

fn draw_gold_separator(x: f32, y: f32, w: f32) {
    draw_line(
        x,
        y,
        x + w,
        y,
        1.0,
        Color::new(theme::GOLD.r, theme::GOLD.g, theme::GOLD.b, 0.58),
    );
}

fn draw_traits_panel(data: &GameData, monster: &CompanionState, x: f32, y: f32, w: f32, h: f32) {
    draw_tier_panel(
        x,
        y,
        w,
        h,
        Some(&data.ui_text.monster_profile.traits_panel_title),
        PanelTier::Support,
        false,
    );
    draw_trait_icons(data, &monster.trait_ids, x + 18.0, y + 48.0, w - 36.0);
    draw_body_text_in_box(
        &trait_names_for_monster(data, monster),
        x + 18.0,
        y + 96.0,
        w - 36.0,
        (h - 112.0).max(46.0),
        18.0,
        theme::TEXT_BODY,
    );
}

fn draw_profile_chip(x: f32, y: f32, w: f32, h: f32, label: &str, value: &str, color: Color) {
    let fill = Color::new(theme::PANEL_1.r, theme::PANEL_1.g, theme::PANEL_1.b, 0.90);
    let border = Color::new(color.r, color.g, color.b, 0.72);
    let style = macroquad_toolkit::ui::ChamferedSurfaceStyle::new(fill, border)
        .with_corner(8.0)
        .with_lower_alpha(0.32);
    macroquad_toolkit::ui::draw_chamfered_surface(Rect::new(x, y, w, h), &style);
    let label_color = Color::new(color.r, color.g, color.b, 0.96);
    let baseline = y + (h * 0.5) + 6.0;
    draw_body_text(label, x + 8.0, baseline, 12.0, label_color);

    let value_style = TextStyle::new(12.0, theme::TEXT_STRONG);
    let value_width = measure_text_size(value, value_style).width;
    draw_body_text(
        value,
        x + w - value_width - 8.0,
        baseline,
        12.0,
        theme::TEXT_STRONG,
    );
}

fn condition_color(value: u32, warning_threshold: u32, any_is_danger: bool) -> Color {
    if any_is_danger && value > 0 {
        theme::DANGER
    } else if value > warning_threshold {
        theme::WARNING
    } else {
        theme::POSITIVE
    }
}
