//! Reusable higher-level UI pieces.

use macroquad::prelude::{
    draw_circle, draw_line, draw_rectangle, draw_rectangle_lines, draw_triangle, vec2, Color, Rect,
};

use crate::data::GameData;
use crate::state::CompanionState;
use crate::ui::art::{draw_species_portrait, draw_ui_icon, icon_for_metric_label};
use crate::ui::chrome::draw_inline_status;
use crate::ui::core::{draw_body_text, draw_body_text_in_box};
use crate::ui::theme;

pub fn draw_badge(x: f32, y: f32, w: f32, h: f32, text: &str, color: Color) {
    let fill = Color::new(color.r, color.g, color.b, 0.16);
    let style = macroquad_toolkit::ui::ChamferedSurfaceStyle::new(fill, color)
        .with_corner(8.0)
        .with_lower_alpha(0.48);
    macroquad_toolkit::ui::draw_chamfered_surface(Rect::new(x, y + 1.0, w, h - 2.0), &style);
    draw_body_text_in_box(
        text,
        x + 8.0,
        y + 3.0,
        w - 16.0,
        h - 6.0,
        12.0,
        theme::TEXT_BODY,
    );
}

pub fn draw_metric_tile(x: f32, y: f32, w: f32, h: f32, label: &str, value: &str, color: Color) {
    let fill = Color::new(theme::PANEL_1.r, theme::PANEL_1.g, theme::PANEL_1.b, 0.72);
    let style = macroquad_toolkit::ui::ChamferedSurfaceStyle::new(
        fill,
        Color::new(color.r, color.g, color.b, 0.78),
    )
    .with_corner(14.0)
    .with_lower_alpha(0.44);
    macroquad_toolkit::ui::draw_chamfered_surface(
        Rect::new(x + 3.0, y + 2.0, w - 6.0, h - 4.0),
        &style,
    );
    let text_x = if let Some(icon) = icon_for_metric_label(label) {
        let icon_size = (h - 16.0).clamp(26.0, 42.0);
        draw_ui_icon(
            icon,
            x + 12.0,
            y + (h - icon_size) * 0.5,
            icon_size,
            Color::new(1.0, 1.0, 1.0, 0.92),
        );
        x + icon_size + 24.0
    } else {
        x + 14.0
    };
    let text_w = (w - (text_x - x) - 8.0).max(24.0);
    draw_body_text_in_box(label, text_x, y + 8.0, text_w, 14.0, 12.0, color);
    draw_body_text_in_box(
        value,
        text_x,
        y + 28.0,
        text_w,
        20.0,
        18.0,
        theme::TEXT_STRONG,
    );
}

pub fn draw_empty_state(x: f32, y: f32, w: f32, h: f32, title: &str, text: &str) {
    draw_rectangle(
        x + 8.0,
        y + 8.0,
        w - 8.0,
        h - 8.0,
        Color::new(0.02, 0.01, 0.025, 0.42),
    );
    draw_rectangle(
        x + 8.0,
        y + 4.0,
        w - 16.0,
        h - 8.0,
        Color::new(theme::PANEL_0.r, theme::PANEL_0.g, theme::PANEL_0.b, 0.78),
    );
    draw_line(
        x + 16.0,
        y + 4.0,
        x + w - 24.0,
        y + 2.0,
        1.0,
        theme::BORDER_0,
    );
    draw_line(
        x + 14.0,
        y + h - 4.0,
        x + w - 30.0,
        y + h - 2.0,
        1.0,
        theme::BORDER_0,
    );
    draw_body_text(title, x + 16.0, y + 28.0, 18.0, theme::TEXT_STRONG);
    draw_body_text_in_box(
        text,
        x + 16.0,
        y + 40.0,
        w - 32.0,
        h - 56.0,
        16.0,
        theme::TEXT_MUTED,
    );
}

pub fn draw_entity_card_frame(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: Color,
    selected: bool,
    disabled: bool,
) {
    let fill = if disabled {
        theme::DISABLED_FILL
    } else if selected {
        theme::SELECTED_FILL
    } else {
        Color::new(theme::PANEL_0.r, theme::PANEL_0.g, theme::PANEL_0.b, 0.96)
    };
    let border = if disabled {
        theme::DISABLED_BORDER
    } else {
        color
    };
    let style = macroquad_toolkit::ui::ChamferedSurfaceStyle::new(fill, border)
        .with_corner(18.0)
        .with_border_width(if selected { 2.0 } else { 1.0 });
    macroquad_toolkit::ui::draw_chamfered_surface(
        Rect::new(x + 3.0, y + 3.0, w - 6.0, h - 6.0),
        &style,
    );
    if selected {
        draw_rectangle(x + 6.0, y + 16.0, 5.0, h - 32.0, color);
    }
}

pub struct CharacterCardLayout {
    pub action_x: f32,
    pub action_y: f32,
    pub action_w: f32,
}

pub struct CharacterCardSpec<'a> {
    pub name: &'a str,
    pub species: &'a str,
    pub state: &'a str,
    pub key_value: &'a str,
    pub color: Color,
    pub state_color: Color,
    pub selected: bool,
    pub disabled: bool,
}

pub fn draw_character_card(
    data: &GameData,
    monster: &CompanionState,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    spec: CharacterCardSpec<'_>,
) -> CharacterCardLayout {
    let action_w = 92.0_f32.min((w * 0.22).max(58.0));
    let portrait_w = 72.0_f32.min((w * 0.18).max(58.0));
    let portrait_h = (h - 20.0).clamp(56.0, 84.0);
    let text_x = x + portrait_w + 24.0;
    let action_x = x + w - action_w - 10.0;
    let text_w = (action_x - text_x - 12.0).max(96.0);
    let action_y = y + 8.0;

    draw_entity_card_frame(x, y, w, h, spec.color, spec.selected, spec.disabled);
    draw_species_portrait(data, monster, x + 10.0, y + 8.0, portrait_w, portrait_h);
    draw_body_text(spec.name, text_x, y + 18.0, 18.0, theme::TEXT_STRONG);
    draw_body_text(spec.species, text_x, y + 36.0, 13.0, theme::TEXT_BODY);
    draw_inline_status(
        text_x,
        y + 42.0,
        text_w.min(190.0),
        spec.state,
        spec.state_color,
    );
    draw_body_text_in_box(
        spec.key_value,
        text_x,
        y + 72.0,
        text_w,
        14.0,
        12.0,
        theme::TEXT_MUTED,
    );

    CharacterCardLayout {
        action_x,
        action_y,
        action_w,
    }
}

pub fn draw_status_marker(x: f32, y: f32, size: f32, color: Color, kind: u8) {
    let fill = Color::new(color.r, color.g, color.b, 0.18);
    draw_rectangle(x, y, size, size, fill);
    draw_rectangle_lines(x, y, size, size, 1.0, color);
    match kind {
        0 => {
            draw_circle(x + size * 0.5, y + size * 0.5, size * 0.22, color);
        }
        1 => {
            draw_rectangle(
                x + size * 0.28,
                y + size * 0.28,
                size * 0.44,
                size * 0.44,
                color,
            );
        }
        _ => {
            draw_triangle(
                vec2(x + size * 0.5, y + size * 0.24),
                vec2(x + size * 0.76, y + size * 0.72),
                vec2(x + size * 0.24, y + size * 0.72),
                color,
            );
        }
    }
}
