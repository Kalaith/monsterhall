use macroquad::prelude::*;

use crate::data::GameData;
use crate::engine::preview_upkeep;
use crate::state::{GameState, CompanionState, CompanionJobState, TownOverviewState};
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_species_portrait, draw_ui_icon, icon_for_metric_label};
use crate::ui::chrome::{draw_screen_title, draw_top_utility_bar};
use crate::ui::core::{draw_body_text, draw_body_text_in_box, secondary_button, utility_button};
use crate::ui::feedback::draw_inline_error;
use crate::ui::layout;
use crate::ui::theme;
use crate::ui::view_models::{
    action_from_action_hint, assignment_label, daily_priority_summary, companion_skill_summary,
    species_name_by_id,
};

use super::town_overview_footer::{draw_town_overview_footer, icon_for_footer_action};

pub(super) struct TownOverviewLayout {
    pub left_margin: f32,
    pub content_width: f32,
    pub priority_y: f32,
    pub priority_width: f32,
    pub summary_x: f32,
    pub summary_width: f32,
    pub roster_y: f32,
    pub roster_h: f32,
    pub footer_y: f32,
    pub compact_height: bool,
}

impl TownOverviewLayout {
    pub(super) fn new(_game_state: &GameState) -> Self {
        let left_margin = layout::OUTER_MARGIN;
        let content_width = screen_width() - left_margin * 2.0;
        let compact_height = screen_height() < 560.0;
        let priority_y = if compact_height { 86.0 } else { 92.0 };
        let priority_width = 446.0;
        let summary_x = left_margin + priority_width + layout::SECTION_GAP;
        let summary_width = content_width - priority_width - layout::SECTION_GAP;
        let roster_y = priority_y + 212.0;
        let footer_y = screen_height() - layout::FOOTER_BOTTOM_MARGIN - layout::FOOTER_H;
        let roster_h = if compact_height {
            0.0
        } else {
            (footer_y - roster_y - layout::SECTION_GAP).max(214.0)
        };

        Self {
            left_margin,
            content_width,
            priority_y,
            priority_width,
            summary_x,
            summary_width,
            roster_y,
            roster_h,
            footer_y,
            compact_height,
        }
    }
}

pub(super) fn draw_organic_panel(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    title: Option<&str>,
    accent: Color,
    emphasis: bool,
) {
    let fill = Color::new(theme::PANEL_0.r, theme::PANEL_0.g, theme::PANEL_0.b, 0.82);
    let shadow = Color::new(0.02, 0.01, 0.025, 0.52);
    let border = Color::new(
        accent.r,
        accent.g,
        accent.b,
        if emphasis { 0.88 } else { 0.62 },
    );
    let inner = Color::new(theme::PANEL_2.r, theme::PANEL_2.g, theme::PANEL_2.b, 0.34);

    draw_rectangle(x + 8.0, y + 10.0, w - 8.0, h - 8.0, shadow);
    draw_rectangle(x + 10.0, y + 3.0, w - 20.0, h - 6.0, fill);
    draw_rectangle(x + 4.0, y + 14.0, w - 8.0, h - 28.0, fill);
    draw_triangle(
        vec2(x + 10.0, y + 3.0),
        vec2(x + 42.0, y),
        vec2(x + 18.0, y + 24.0),
        fill,
    );
    draw_triangle(
        vec2(x + w - 12.0, y + 5.0),
        vec2(x + w - 46.0, y + 1.0),
        vec2(x + w - 16.0, y + 26.0),
        fill,
    );
    draw_triangle(
        vec2(x + 7.0, y + h - 8.0),
        vec2(x + 40.0, y + h - 2.0),
        vec2(x + 18.0, y + h - 30.0),
        fill,
    );
    draw_triangle(
        vec2(x + w - 10.0, y + h - 6.0),
        vec2(x + w - 42.0, y + h),
        vec2(x + w - 18.0, y + h - 28.0),
        fill,
    );

    draw_rectangle(x + 14.0, y + 38.0, w - 28.0, 1.0, inner);
    draw_line(
        x + 16.0,
        y + 4.0,
        x + w - 28.0,
        y + 2.0,
        if emphasis { 2.0 } else { 1.0 },
        border,
    );
    draw_line(
        x + w - 8.0,
        y + 16.0,
        x + w - 14.0,
        y + h - 22.0,
        1.0,
        border,
    );
    draw_line(
        x + 10.0,
        y + h - 6.0,
        x + w - 34.0,
        y + h - 3.0,
        1.0,
        border,
    );
    draw_line(x + 4.0, y + 24.0, x + 10.0, y + h - 26.0, 1.0, border);
    draw_line(
        x + 26.0,
        y + 10.0,
        x + w - 42.0,
        y + 12.0,
        1.0,
        Color::new(border.r, border.g, border.b, 0.28),
    );

    if let Some(title) = title {
        draw_banner_label(x + 16.0, y + 10.0, w - 32.0, title, accent);
    }
}

fn draw_organic_stage_panel(x: f32, y: f32, w: f32, h: f32, title: &str, accent: Color) {
    let fill = Color::new(theme::PANEL_0.r, theme::PANEL_0.g, theme::PANEL_0.b, 0.56);
    let shadow = Color::new(0.02, 0.01, 0.025, 0.46);
    let border = Color::new(accent.r, accent.g, accent.b, 0.64);

    draw_rectangle(x + 10.0, y + 12.0, w - 12.0, h - 12.0, shadow);
    draw_rectangle(x + 8.0, y + 18.0, w - 16.0, h - 36.0, fill);
    draw_triangle(
        vec2(x + 8.0, y + 18.0),
        vec2(x + 54.0, y + 8.0),
        vec2(x + 26.0, y + 46.0),
        fill,
    );
    draw_triangle(
        vec2(x + w - 8.0, y + 18.0),
        vec2(x + w - 58.0, y + 9.0),
        vec2(x + w - 24.0, y + 46.0),
        fill,
    );
    draw_triangle(
        vec2(x + 8.0, y + h - 18.0),
        vec2(x + 64.0, y + h - 4.0),
        vec2(x + 28.0, y + h - 52.0),
        fill,
    );
    draw_triangle(
        vec2(x + w - 8.0, y + h - 18.0),
        vec2(x + w - 70.0, y + h - 4.0),
        vec2(x + w - 28.0, y + h - 54.0),
        fill,
    );
    draw_line(x + 18.0, y + 5.0, x + w - 42.0, y + 2.0, 1.0, border);
    draw_line(
        x + w - 8.0,
        y + 22.0,
        x + w - 12.0,
        y + h - 28.0,
        1.0,
        border,
    );
    draw_line(
        x + 12.0,
        y + h - 4.0,
        x + w - 52.0,
        y + h - 2.0,
        1.0,
        border,
    );
    draw_line(x + 3.0, y + 34.0, x + 8.0, y + h - 32.0, 1.0, border);

    draw_banner_label(x + 16.0, y + 10.0, w - 32.0, title, accent);
}

fn draw_banner_label(x: f32, y: f32, max_w: f32, label: &str, accent: Color) {
    let banner_w = (label.len() as f32 * 8.4 + 54.0).clamp(118.0, max_w);
    let fill = Color::new(theme::PANEL_2.r, theme::PANEL_2.g, theme::PANEL_2.b, 0.76);

    draw_rectangle(x + 11.0, y + 1.0, banner_w - 22.0, 24.0, fill);
    draw_triangle(
        vec2(x, y + 1.0),
        vec2(x + 16.0, y + 1.0),
        vec2(x + 12.0, y + 25.0),
        fill,
    );
    draw_triangle(
        vec2(x + banner_w, y + 2.0),
        vec2(x + banner_w - 18.0, y + 1.0),
        vec2(x + banner_w - 13.0, y + 25.0),
        fill,
    );
    draw_line(
        x + 14.0,
        y + 25.0,
        x + banner_w - 14.0,
        y + 23.0,
        2.0,
        Color::new(accent.r, accent.g, accent.b, 0.78),
    );
    draw_body_text(label, x + 26.0, y + 17.0, 13.0, theme::TEXT_MUTED);
}

fn draw_organic_status(x: f32, y: f32, w: f32, h: f32, text: &str, accent: Color) {
    let fill = Color::new(theme::PANEL_1.r, theme::PANEL_1.g, theme::PANEL_1.b, 0.72);
    draw_rectangle(x + 6.0, y, w - 12.0, h, fill);
    draw_triangle(
        vec2(x, y + 3.0),
        vec2(x + 12.0, y),
        vec2(x + 8.0, y + h),
        fill,
    );
    draw_triangle(
        vec2(x + w, y + 2.0),
        vec2(x + w - 14.0, y),
        vec2(x + w - 8.0, y + h),
        fill,
    );
    draw_line(
        x + 10.0,
        y + 2.0,
        x + w - 16.0,
        y + 1.0,
        1.0,
        Color::new(accent.r, accent.g, accent.b, 0.72),
    );
    draw_line(
        x + 12.0,
        y + h - 2.0,
        x + w - 18.0,
        y + h - 1.0,
        1.0,
        Color::new(accent.r, accent.g, accent.b, 0.42),
    );
    draw_body_text_in_box(
        text,
        x + 14.0,
        y + 5.0,
        w - 28.0,
        h - 8.0,
        13.0,
        theme::TEXT_BODY,
    );
}

fn draw_resource_cell(x: f32, y: f32, w: f32, h: f32, label: &str, value: &str, accent: Color) {
    let fill = Color::new(theme::PANEL_1.r, theme::PANEL_1.g, theme::PANEL_1.b, 0.66);
    draw_rectangle(x + 8.0, y + 1.0, w - 16.0, h - 2.0, fill);
    draw_rectangle(x + 3.0, y + 7.0, w - 6.0, h - 14.0, fill);
    draw_line(
        x + 10.0,
        y + 2.0,
        x + w - 16.0,
        y + 1.0,
        1.0,
        Color::new(accent.r, accent.g, accent.b, 0.7),
    );
    draw_line(
        x + 8.0,
        y + h - 3.0,
        x + w - 24.0,
        y + h - 1.0,
        1.0,
        Color::new(accent.r, accent.g, accent.b, 0.28),
    );

    if let Some(icon) = icon_for_metric_label(label) {
        draw_ui_icon(
            icon,
            x + 12.0,
            y + (h - 30.0) * 0.5,
            30.0,
            Color::new(1.0, 1.0, 1.0, 0.78),
        );
    }
    draw_body_text(
        label,
        x + 50.0,
        y + 18.0,
        11.0,
        Color::new(accent.r, accent.g, accent.b, 0.86),
    );
    draw_body_text(value, x + 50.0, y + 40.0, 20.0, theme::TEXT_STRONG);
}

fn draw_organic_button(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    label: &str,
    accent: Color,
    primary: bool,
) -> bool {
    let (mouse_x, mouse_y) = mouse_position();
    let hovered = mouse_x >= x && mouse_x <= x + w && mouse_y >= y && mouse_y <= y + h;
    let pressed = hovered && is_mouse_button_down(MouseButton::Left);
    let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);
    let base = if primary {
        theme::PRIMARY
    } else {
        theme::PANEL_1
    };
    let alpha = if pressed {
        0.98
    } else if hovered {
        0.92
    } else {
        0.76
    };
    let fill = Color::new(base.r, base.g, base.b, alpha);
    let border_alpha = if hovered || primary { 0.9 } else { 0.58 };

    draw_rectangle(x + 6.0, y + 2.0, w - 12.0, h - 4.0, fill);
    draw_triangle(
        vec2(x, y + h * 0.5),
        vec2(x + 12.0, y + 2.0),
        vec2(x + 12.0, y + h - 2.0),
        fill,
    );
    draw_triangle(
        vec2(x + w, y + h * 0.5),
        vec2(x + w - 12.0, y + 2.0),
        vec2(x + w - 12.0, y + h - 2.0),
        fill,
    );
    draw_line(
        x + 10.0,
        y + 2.0,
        x + w - 14.0,
        y + 1.0,
        1.0,
        Color::new(accent.r, accent.g, accent.b, border_alpha),
    );
    draw_line(
        x + 10.0,
        y + h - 2.0,
        x + w - 16.0,
        y + h - 1.0,
        1.0,
        Color::new(accent.r, accent.g, accent.b, border_alpha * 0.6),
    );
    draw_body_text_in_box(
        label,
        x + 10.0,
        y + 6.0,
        w - 20.0,
        h - 10.0,
        13.0,
        theme::TEXT_STRONG,
    );

    clicked
}

pub(super) fn draw_header(data: &GameData, _layout: &TownOverviewLayout) -> Option<UiAction> {
    if let Some(action) = draw_top_utility_bar(&data.ui_text.common.settings_button) {
        return Some(action);
    }
    draw_screen_title(&data.config.title, None);
    None
}

pub(super) fn draw_priority_panel(
    data: &GameData,
    town_state: &TownOverviewState,
    game_state: &GameState,
    layout: &TownOverviewLayout,
) -> Option<UiAction> {
    let priority = daily_priority_summary(data, game_state);
    let priority_action = action_from_action_hint(data, &priority.action_hint);
    let priority_icon = icon_for_footer_action(&priority_action);
    draw_organic_panel(
        layout.left_margin,
        layout.priority_y,
        layout.priority_width,
        196.0,
        Some(&data.ui_text.town_overview.priority_panel_title),
        priority.color,
        true,
    );
    draw_ui_icon(
        priority_icon,
        layout.left_margin + layout::PANEL_PADDING,
        layout.priority_y + 56.0,
        44.0,
        Color::new(1.0, 1.0, 1.0, 0.78),
    );
    draw_body_text(
        &priority.title,
        layout.left_margin + layout::PANEL_PADDING + 58.0,
        layout.priority_y + 62.0,
        28.0,
        theme::TEXT_STRONG,
    );
    draw_body_text_in_box(
        &priority.detail,
        layout.left_margin + layout::PANEL_PADDING,
        layout.priority_y + 86.0,
        layout.priority_width - layout::PANEL_PADDING * 2.0,
        48.0,
        16.0,
        theme::TEXT_BODY,
    );
    let _status_message = &town_state.status_message;
    draw_organic_status(
        layout.left_margin + layout::PANEL_PADDING,
        layout.priority_y + 142.0,
        layout.priority_width - layout::PANEL_PADDING * 2.0,
        30.0,
        &format!("Priority route: {}", priority.action_hint),
        priority.color,
    );
    None
}

pub(super) fn draw_summary_strip(
    data: &GameData,
    game_state: &GameState,
    layout: &TownOverviewLayout,
) -> Option<UiAction> {
    draw_organic_panel(
        layout.summary_x,
        layout.priority_y,
        layout.summary_width,
        196.0,
        Some(&data.ui_text.town_overview.snapshot_panel_title),
        theme::GOLD,
        false,
    );

    if layout.compact_height {
        let debt_value = game_state
            .debt
            .as_ref()
            .map(|debt| format!("{} in {}d", debt.current_balance_due, debt.days_until_due))
            .unwrap_or_else(|| "Clear".to_owned());
        let upkeep = preview_upkeep(data, game_state);
        let compact_summary = format!(
            "Gold {}   Materials {}\nEggs {}   Relics {}\nResidue {}   Debt {}\nNext upkeep {}g",
            game_state.resources.gold,
            game_state.resources.tower_materials,
            game_state.resources.eggs,
            game_state.resources.relics,
            game_state.resources.arcane_residue,
            debt_value,
            upkeep.total_gold
        );
        draw_body_text_in_box(
            &compact_summary,
            layout.summary_x + layout::PANEL_PADDING,
            layout.priority_y + 54.0,
            layout.summary_width - layout::PANEL_PADDING * 2.0,
            118.0,
            18.0,
            theme::TEXT_BODY,
        );
        return None;
    }

    let due_value = game_state
        .debt
        .as_ref()
        .map(|debt| format!("{} in {}d", debt.current_balance_due, debt.days_until_due))
        .unwrap_or_else(|| "Clear".to_owned());
    let debt_color = game_state
        .debt
        .as_ref()
        .map(|debt| {
            if debt.days_until_due <= 2 {
                theme::DANGER
            } else {
                theme::WARNING
            }
        })
        .unwrap_or(theme::POSITIVE);
    let metrics = [
        (
            "Gold",
            game_state.resources.gold.to_string(),
            theme::POSITIVE,
        ),
        (
            "Materials",
            game_state.resources.tower_materials.to_string(),
            theme::INFO,
        ),
        (
            &data.ui_text.town_overview.resources_eggs_label,
            game_state.resources.eggs.to_string(),
            theme::PRIMARY,
        ),
        (
            &data.ui_text.town_overview.resources_relics_label,
            game_state.resources.relics.to_string(),
            theme::ROSE,
        ),
        (
            &data.ui_text.town_overview.resources_arcane_residue_label,
            game_state.resources.arcane_residue.to_string(),
            theme::DANGER,
        ),
        ("Debt", due_value, debt_color),
    ];
    let tile_gap = 12.0;
    let tile_w = (layout.summary_width - layout::PANEL_PADDING * 2.0 - tile_gap * 2.0) / 3.0;
    for (index, (label, value, color)) in metrics.iter().enumerate() {
        let col = (index % 3) as f32;
        let row = (index / 3) as f32;
        let tile_x = layout.summary_x + layout::PANEL_PADDING + col * (tile_w + tile_gap);
        let tile_y = layout.priority_y + 50.0 + row * 62.0;
        draw_resource_cell(tile_x, tile_y, tile_w, 54.0, label, value, *color);
    }

    let upkeep = preview_upkeep(data, game_state);
    let debt_button_w = 132.0;
    let debt_button_gap = 12.0;
    let can_pay_debt = game_state
        .debt
        .as_ref()
        .is_some_and(|debt| game_state.resources.gold >= debt.current_balance_due);
    let upkeep_width = if can_pay_debt {
        layout.summary_width - layout::PANEL_PADDING * 2.0 - debt_button_w - debt_button_gap
    } else {
        layout.summary_width - layout::PANEL_PADDING * 2.0
    };
    draw_body_text_in_box(
        &format!(
            "Next upkeep {}g: wages {}, supplies {}, repairs {}. Band {} companions / {} patron tiers. Companion {}g (+{}), building {}g (+{}).",
            upkeep.total_gold,
            upkeep.food_gold,
            upkeep.cleaning_gold,
            upkeep.maintenance_gold,
            upkeep.active_band_min_girls,
            upkeep.active_band_min_patron_tiers,
            upkeep.next_girl_total_gold,
            upkeep.next_girl_delta_gold,
            upkeep.next_building_total_gold,
            upkeep.next_building_delta_gold
        ),
        layout.summary_x + layout::PANEL_PADDING,
        layout.priority_y + 172.0,
        upkeep_width,
        20.0,
        13.0,
        theme::TEXT_MUTED,
    );
    if can_pay_debt
        && draw_organic_button(
            layout.summary_x + layout.summary_width - layout::PANEL_PADDING - debt_button_w,
            layout.priority_y + 166.0,
            debt_button_w,
            28.0,
            "Pay Debt",
            theme::WARNING,
            false,
        )
    {
        return Some(UiAction::PayDebtNow);
    }

    None
}

pub(super) fn draw_monster_roster(
    data: &GameData,
    game_state: &GameState,
    layout: &TownOverviewLayout,
) -> Option<UiAction> {
    if game_state.monsters.is_empty() || layout.compact_height {
        return None;
    }

    draw_organic_stage_panel(
        layout.left_margin,
        layout.roster_y,
        layout.content_width,
        layout.roster_h,
        &data.ui_text.town_overview.roster_panel_title,
        theme::GOLD,
    );

    let visible_count = game_state.monsters.len().min(3);
    let card_width = if visible_count <= 1 {
        layout.content_width - layout::PANEL_PADDING * 2.0
    } else {
        let total_gap = layout::SECTION_GAP * (visible_count as f32 - 1.0);
        (layout.content_width - layout::PANEL_PADDING * 2.0 - total_gap) / visible_count as f32
    };

    for (index, monster) in game_state.monsters.iter().take(visible_count).enumerate() {
        let card_x = layout.left_margin
            + layout::PANEL_PADDING
            + index as f32 * (card_width + layout::SECTION_GAP);
        let card_y = layout.roster_y + 44.0;
        let card_h = 162.0_f32.min((layout.roster_h - 62.0).max(132.0));
        if let Some(action) =
            draw_roster_card_organic(data, monster, card_x, card_y, card_width, card_h)
        {
            return Some(action);
        }
    }

    None
}

fn draw_roster_card_organic(
    data: &GameData,
    monster: &CompanionState,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Option<UiAction> {
    let state_label = assignment_label(data, &monster.current_job);
    let species_label = species_name_by_id(data, &monster.species_id);
    let key_value = format!("Skills {}", companion_skill_summary(data, monster));
    let accent = job_color(&monster.current_job);

    draw_organic_panel(x, y, w, h, None, accent, false);

    let portrait_w = 76.0_f32.min((w * 0.22).max(64.0));
    let portrait_h = (h - 28.0).max(92.0);
    draw_species_portrait(data, monster, x + 14.0, y + 14.0, portrait_w, portrait_h);

    let action_w = 88.0_f32.min((w * 0.25).max(76.0));
    let action_x = x + w - action_w - 14.0;
    let action_y = y + 18.0;
    let text_x = x + portrait_w + 30.0;
    let text_w = (action_x - text_x - 14.0).max(90.0);

    draw_body_text(&monster.name, text_x, y + 30.0, 20.0, theme::TEXT_STRONG);
    draw_body_text(&species_label, text_x, y + 52.0, 13.0, theme::TEXT_BODY);
    draw_organic_status(text_x, y + 66.0, text_w, 28.0, &state_label, accent);
    draw_body_text_in_box(
        &key_value,
        text_x,
        y + 106.0,
        text_w,
        24.0,
        12.0,
        theme::TEXT_MUTED,
    );

    if secondary_button(
        action_x,
        action_y,
        action_w,
        24.0,
        &data.ui_text.town_overview.monster_profile_button,
    ) {
        return Some(UiAction::OpenMonsterProfile(monster.id.clone()));
    }
    if secondary_button(
        action_x,
        action_y + 30.0,
        action_w,
        24.0,
        &data.ui_text.common.rest_button,
    ) {
        return Some(UiAction::AssignMonsterToRest(monster.id.clone()));
    }
    if !matches!(monster.current_job, CompanionJobState::Idle)
        && utility_button(
            action_x,
            action_y + 60.0,
            action_w,
            24.0,
            &data.ui_text.common.idle_button,
        )
    {
        return Some(UiAction::AssignMonsterToIdle(monster.id.clone()));
    }

    None
}

fn job_color(job: &CompanionJobState) -> Color {
    match job {
        CompanionJobState::Idle => theme::TEXT_MUTED,
        CompanionJobState::GuildJob { .. } => theme::ROSE,
        CompanionJobState::Resting => theme::INFO,
        CompanionJobState::OnExpedition { .. } => theme::WARNING,
    }
}

pub(super) fn draw_footer_actions(
    data: &GameData,
    _game_state: &GameState,
    layout: &TownOverviewLayout,
) -> Option<UiAction> {
    draw_town_overview_footer(
        data,
        layout.left_margin,
        layout.footer_y,
        layout.content_width,
    )
}

pub(super) fn draw_error_panel(layout: &TownOverviewLayout, last_error: Option<&str>) {
    if let Some(error_message) = last_error {
        draw_inline_error(
            layout.left_margin,
            layout.footer_y - 30.0,
            layout.content_width,
            error_message,
        );
    }
}
