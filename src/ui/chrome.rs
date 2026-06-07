//! Shared screen chrome such as headers, utility bars, and footers.

use macroquad::prelude::*;

use crate::data::GameData;
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_ui_icon, UiIcon};
use crate::ui::core::{draw_body_text, draw_body_text_in_box, draw_heading, utility_button};
use crate::ui::layout;
use crate::ui::theme;

#[derive(Clone, Copy)]
pub enum PanelTier {
    Primary,
    Support,
    Utility,
}

#[derive(Clone, Copy)]
pub enum ChromeButtonKind {
    Primary,
    Secondary,
}

pub struct FooterAction<'a> {
    pub label: &'a str,
    pub action: UiAction,
    pub kind: ChromeButtonKind,
}

fn footer_kind_for(action: &UiAction, highlight_action: Option<&UiAction>) -> ChromeButtonKind {
    if let Some(highlight_action) = highlight_action {
        if same_footer_action(action, highlight_action) {
            return ChromeButtonKind::Primary;
        }
    }

    ChromeButtonKind::Secondary
}

fn same_footer_action(left: &UiAction, right: &UiAction) -> bool {
    matches!(
        (left, right),
        (
            UiAction::ReturnToTownOverview,
            UiAction::ReturnToTownOverview
        ) | (UiAction::OpenTownManagement, UiAction::OpenTownManagement)
            | (
                UiAction::OpenGuildHallManagement,
                UiAction::OpenGuildHallManagement
            )
            | (UiAction::OpenContractDesk, UiAction::OpenContractDesk)
            | (
                UiAction::OpenHatcheryManagement,
                UiAction::OpenHatcheryManagement
            )
            | (
                UiAction::OpenExpeditionPlanning,
                UiAction::OpenExpeditionPlanning
            )
            | (UiAction::OpenJournal, UiAction::OpenJournal)
            | (UiAction::ResolveDay, UiAction::ResolveDay)
    )
}

fn tier_color(tier: PanelTier) -> Color {
    match tier {
        PanelTier::Primary => theme::PANEL_PRIMARY,
        PanelTier::Support => theme::PANEL_SUPPORT,
        PanelTier::Utility => theme::PANEL_UTILITY,
    }
}

fn draw_ragged_panel_surface(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    fill: Color,
    accent: Color,
    emphasis: bool,
) {
    let shadow = Color::new(0.02, 0.01, 0.025, 0.50);
    let style = macroquad_toolkit::ui::RaggedSurfaceStyle::new(fill, accent)
        .with_shadow(shadow)
        .with_emphasis(emphasis);
    macroquad_toolkit::ui::draw_ragged_surface(Rect::new(x, y, w, h), &style);
}

fn draw_ribbon_label(x: f32, y: f32, max_w: f32, title: &str, accent: Color) {
    let banner_w = (title.len() as f32 * 8.6 + 56.0).clamp(118.0, max_w);
    let fill = Color::new(theme::PANEL_2.r, theme::PANEL_2.g, theme::PANEL_2.b, 0.82);

    draw_rectangle(x + 14.0, y, banner_w - 30.0, 28.0, fill);
    draw_triangle(
        vec2(x, y + 1.0),
        vec2(x + 18.0, y),
        vec2(x + 14.0, y + 29.0),
        fill,
    );
    draw_triangle(
        vec2(x + banner_w, y + 2.0),
        vec2(x + banner_w - 22.0, y),
        vec2(x + banner_w - 15.0, y + 29.0),
        fill,
    );
    draw_line(
        x + 18.0,
        y + 27.0,
        x + banner_w - 18.0,
        y + 25.0,
        2.0,
        Color::new(accent.r, accent.g, accent.b, 0.80),
    );
    draw_body_text(title, x + 28.0, y + 19.0, 14.0, theme::TEXT_MUTED);
}

fn draw_title_mark(x: f32, y: f32, h: f32, accent: Color) {
    let tower_w = h * 0.42;
    let base_y = y + h;
    let fill = Color::new(accent.r, accent.g, accent.b, 0.26);
    let line = Color::new(accent.r, accent.g, accent.b, 0.72);

    draw_rectangle(
        x + tower_w * 0.32,
        y + h * 0.18,
        tower_w * 0.36,
        h * 0.72,
        fill,
    );
    draw_rectangle(
        x + tower_w * 0.18,
        y + h * 0.42,
        tower_w * 0.64,
        h * 0.20,
        fill,
    );
    draw_triangle(
        vec2(x + tower_w * 0.5, y),
        vec2(x + tower_w * 0.18, y + h * 0.22),
        vec2(x + tower_w * 0.82, y + h * 0.22),
        fill,
    );
    draw_line(x + tower_w * 0.5, y, x + tower_w * 0.5, y - 8.0, 1.0, line);
    draw_line(
        x + tower_w * 0.20,
        base_y - 2.0,
        x + tower_w * 0.86,
        base_y - 2.0,
        1.0,
        line,
    );
    draw_line(
        x + tower_w * 0.18,
        y + h * 0.22,
        x + tower_w * 0.82,
        y + h * 0.22,
        1.0,
        line,
    );
    draw_line(
        x + tower_w * 0.32,
        y + h * 0.18,
        x + tower_w * 0.32,
        base_y - 3.0,
        1.0,
        line,
    );
    draw_line(
        x + tower_w * 0.68,
        y + h * 0.18,
        x + tower_w * 0.68,
        base_y - 3.0,
        1.0,
        line,
    );
}

pub fn draw_screen_title(title: &str, subtitle: Option<&str>) {
    let x = layout::OUTER_MARGIN;
    let y = layout::OUTER_MARGIN - 2.0;
    draw_title_mark(x + 2.0, y + 3.0, 54.0, theme::PRIMARY);
    draw_heading(title, x + 54.0, y + 20.0, 40.0);
    draw_line(
        x + 54.0,
        y + 28.0,
        x + 54.0 + 330.0,
        y + 28.0,
        1.0,
        Color::new(theme::PRIMARY.r, theme::PRIMARY.g, theme::PRIMARY.b, 0.46),
    );
    if let Some(subtitle) = subtitle {
        draw_body_text(subtitle, x + 54.0, y + 52.0, 18.0, theme::TEXT_BODY);
    }
}

pub fn draw_tier_panel(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    title: Option<&str>,
    tier: PanelTier,
    emphasis: bool,
) {
    let fill = match tier {
        PanelTier::Primary => {
            Color::new(theme::PANEL_0.r, theme::PANEL_0.g, theme::PANEL_0.b, 0.84)
        }
        PanelTier::Support => {
            Color::new(theme::PANEL_1.r, theme::PANEL_1.g, theme::PANEL_1.b, 0.76)
        }
        PanelTier::Utility => Color::new(theme::BG_1.r, theme::BG_1.g, theme::BG_1.b, 0.78),
    };
    let color = tier_color(tier);
    draw_ragged_panel_surface(x, y, w, h, fill, color, emphasis);
    if let Some(title) = title {
        draw_ribbon_label(x + 16.0, y + 9.0, w - 32.0, title, color);
        draw_line(
            x + 30.0,
            y + 46.0,
            x + w - 34.0,
            y + 44.0,
            1.0,
            Color::new(color.r, color.g, color.b, 0.34),
        );
    }
}

pub fn draw_top_utility_bar(settings_label: &str) -> Option<UiAction> {
    let bar_y = 14.0;
    if utility_button(
        screen_width() - layout::OUTER_MARGIN - 116.0,
        bar_y + 2.0,
        116.0,
        layout::UTILITY_BUTTON_H,
        settings_label,
    ) {
        return Some(UiAction::OpenSettings);
    }
    draw_ui_icon(
        UiIcon::NavSettings,
        screen_width() - layout::OUTER_MARGIN - 108.0,
        bar_y + 3.0,
        20.0,
        Color::new(1.0, 1.0, 1.0, 0.76),
    );
    None
}

pub fn draw_screen_header(title: &str, subtitle: &str) {
    draw_screen_title(title, Some(subtitle));
}

pub fn draw_modal_panel(x: f32, y: f32, w: f32, h: f32, title: &str, subtitle: Option<&str>) {
    draw_tier_panel(x, y, w, h, Some(title), PanelTier::Primary, true);
    if let Some(subtitle) = subtitle {
        draw_body_text_in_box(
            subtitle,
            x + layout::PANEL_PADDING,
            y + 44.0,
            w - layout::PANEL_PADDING * 2.0,
            28.0,
            16.0,
            theme::TEXT_MUTED,
        );
    }
}

pub fn draw_footer_bar(x: f32, y: f32, w: f32, actions: &[FooterAction<'_>]) -> Option<UiAction> {
    if actions.is_empty() {
        return None;
    }

    draw_ragged_panel_surface(
        x,
        y,
        w,
        layout::FOOTER_H,
        Color::new(theme::BG_1.r, theme::BG_1.g, theme::BG_1.b, 0.72),
        theme::BORDER_1,
        false,
    );
    let gap = layout::SPACE_8;
    let inner_x = x + 14.0;
    let inner_w = w - 28.0;
    let button_w = (inner_w - gap * (actions.len() as f32 - 1.0)) / actions.len() as f32;
    for (index, action) in actions.iter().enumerate() {
        let button_x = inner_x + index as f32 * (button_w + gap);
        let pressed = draw_footer_tile(
            button_x,
            y + 8.0,
            button_w,
            layout::FOOTER_H - 16.0,
            action.label,
            &action.action,
            matches!(action.kind, ChromeButtonKind::Primary),
        );
        if pressed {
            return Some(action.action.clone());
        }
    }

    None
}

fn draw_footer_tile(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    label: &str,
    action: &UiAction,
    primary: bool,
) -> bool {
    let (mouse_x, mouse_y) = mouse_position();
    let hovered = mouse_x >= x && mouse_x <= x + w && mouse_y >= y && mouse_y <= y + h;
    let pressed = hovered && is_mouse_button_down(MouseButton::Left);
    let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);
    let is_end_day = matches!(action, UiAction::ResolveDay);
    let accent = if primary {
        theme::GOLD
    } else if is_end_day {
        theme::WARNING
    } else {
        theme::BORDER_1
    };
    let fill_base = if primary {
        theme::PRIMARY
    } else {
        theme::PANEL_1
    };
    let fill_alpha = if pressed {
        0.98
    } else if hovered {
        0.88
    } else if primary {
        0.82
    } else if is_end_day {
        0.74
    } else {
        0.66
    };
    let fill = Color::new(fill_base.r, fill_base.g, fill_base.b, fill_alpha);

    let accent_color = Color::new(
        accent.r,
        accent.g,
        accent.b,
        if primary || hovered || is_end_day {
            0.92
        } else {
            0.56
        },
    );
    let surface = macroquad_toolkit::ui::ChamferedSurfaceStyle::new(fill, accent_color)
        .with_corner(14.0)
        .with_border_width(if primary || hovered { 2.0 } else { 1.0 })
        .with_lower_alpha(if primary || hovered || is_end_day {
            0.78
        } else {
            0.60
        });
    macroquad_toolkit::ui::draw_chamfered_surface(
        Rect::new(x + 3.0, y + 2.0, w - 6.0, h - 4.0),
        &surface,
    );
    draw_ui_icon(
        icon_for_footer_action(action),
        x + 12.0,
        y + (h - 34.0) * 0.5,
        34.0,
        Color::new(
            1.0,
            1.0,
            1.0,
            if primary || hovered || is_end_day {
                0.92
            } else {
                0.66
            },
        ),
    );
    draw_body_text_in_box(
        label,
        x + 52.0,
        y + 10.0,
        w - 58.0,
        h - 18.0,
        14.0,
        theme::TEXT_STRONG,
    );

    clicked
}

fn icon_for_footer_action(action: &UiAction) -> UiIcon {
    match action {
        UiAction::ReturnToTownOverview => UiIcon::NavTown,
        UiAction::OpenTownManagement => UiIcon::NavPlanner,
        UiAction::OpenGuildHallManagement => UiIcon::NavGuildJobs,
        UiAction::OpenContractDesk => UiIcon::NavGuest,
        UiAction::OpenHatcheryManagement => UiIcon::NavChamber,
        UiAction::OpenExpeditionPlanning => UiIcon::NavExpedition,
        UiAction::OpenJournal => UiIcon::NavJournal,
        UiAction::ResolveDay => UiIcon::NavEndDay,
        _ => UiIcon::StatusAssigned,
    }
}

pub fn draw_standard_gameplay_footer(
    data: &GameData,
    x: f32,
    y: f32,
    w: f32,
    highlight_action: Option<UiAction>,
) -> Option<UiAction> {
    let common = &data.ui_text.common;
    let highlight_ref = highlight_action.as_ref();
    let actions = [
        FooterAction {
            label: &common.return_to_town_button,
            action: UiAction::ReturnToTownOverview,
            kind: footer_kind_for(&UiAction::ReturnToTownOverview, highlight_ref),
        },
        FooterAction {
            label: &common.town_planner_button,
            action: UiAction::OpenTownManagement,
            kind: footer_kind_for(&UiAction::OpenTownManagement, highlight_ref),
        },
        FooterAction {
            label: &common.guild_jobs_button,
            action: UiAction::OpenGuildHallManagement,
            kind: footer_kind_for(&UiAction::OpenGuildHallManagement, highlight_ref),
        },
        FooterAction {
            label: &common.guest_desk_button,
            action: UiAction::OpenContractDesk,
            kind: footer_kind_for(&UiAction::OpenContractDesk, highlight_ref),
        },
        FooterAction {
            label: &common.chamber_button,
            action: UiAction::OpenHatcheryManagement,
            kind: footer_kind_for(&UiAction::OpenHatcheryManagement, highlight_ref),
        },
        FooterAction {
            label: &common.expedition_desk_button,
            action: UiAction::OpenExpeditionPlanning,
            kind: footer_kind_for(&UiAction::OpenExpeditionPlanning, highlight_ref),
        },
        FooterAction {
            label: &common.journal_button,
            action: UiAction::OpenJournal,
            kind: footer_kind_for(&UiAction::OpenJournal, highlight_ref),
        },
        FooterAction {
            label: &common.end_day_button,
            action: UiAction::ResolveDay,
            kind: footer_kind_for(&UiAction::ResolveDay, highlight_ref),
        },
    ];

    draw_footer_bar(x, y, w, &actions)
}

pub fn draw_inline_status(x: f32, y: f32, w: f32, text: &str, color: Color) {
    let h = 28.0;
    let fill = Color::new(theme::PANEL_2.r, theme::PANEL_2.g, theme::PANEL_2.b, 0.74);
    let surface = macroquad_toolkit::ui::ChamferedSurfaceStyle::new(
        fill,
        Color::new(color.r, color.g, color.b, 0.72),
    )
    .with_corner(10.0)
    .with_lower_alpha(0.58);
    macroquad_toolkit::ui::draw_chamfered_surface(Rect::new(x, y + 1.0, w, h - 2.0), &surface);
    draw_body_text_in_box(
        text,
        x + 14.0,
        y + 4.0,
        w - 28.0,
        h - 8.0,
        13.0,
        theme::TEXT_BODY,
    );
}
