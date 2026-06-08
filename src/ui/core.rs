//! Shared UI drawing helpers.

use macroquad::prelude::*;
use macroquad::text::draw_text_ex;
use macroquad_toolkit::ui::{
    draw_text_block_ex, draw_text_centered_in_box, measure_text_size, ButtonStyle, TextStyle,
};

use crate::ui::theme;

const REFERENCE_WIDTH: f32 = 1920.0;
const REFERENCE_HEIGHT: f32 = 1080.0;
const BASE_FONT_BOOST: f32 = 1.24;
const MIN_SCALE: f32 = 0.92;
const MAX_SCALE: f32 = 1.18;

pub fn ui_scale_factor() -> f32 {
    let width_scale = screen_width() / REFERENCE_WIDTH;
    let height_scale = screen_height() / REFERENCE_HEIGHT;

    width_scale.min(height_scale).clamp(MIN_SCALE, MAX_SCALE)
}

pub fn scaled_font_size(font_size: f32) -> f32 {
    (font_size * ui_scale_factor() * BASE_FONT_BOOST)
        .round()
        .max(18.0)
}

pub fn scaled_spacing(spacing: f32) -> f32 {
    (spacing * ui_scale_factor()).round()
}

pub fn draw_heading(text: &str, x: f32, y: f32, font_size: f32) {
    draw_text_ex(
        text,
        x,
        y,
        TextStyle::new(scaled_font_size(font_size), theme::TEXT_STRONG).params(),
    );
}

pub fn draw_body_text(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    draw_text_ex(
        text,
        x,
        y,
        TextStyle::new(scaled_font_size(font_size), color).params(),
    );
}

fn primary_button_style() -> ButtonStyle {
    ButtonStyle {
        normal: Color::new(
            theme::PRIMARY.r * 0.92,
            theme::PRIMARY.g * 0.92,
            theme::PRIMARY.b * 0.92,
            0.98,
        ),
        hovered: Color::new(theme::INFO.r, theme::INFO.g, theme::INFO.b, 1.0),
        pressed: Color::new(
            theme::PRIMARY.r * 0.82,
            theme::PRIMARY.g * 0.82,
            theme::PRIMARY.b * 0.82,
            1.0,
        ),
        border: Color::new(
            theme::BORDER_HI.r,
            theme::BORDER_HI.g,
            theme::BORDER_HI.b,
            0.88,
        ),
        text_color: theme::TEXT_STRONG,
        disabled: theme::DISABLED_FILL,
    }
}

fn secondary_button_style() -> ButtonStyle {
    ButtonStyle {
        normal: Color::new(theme::PANEL_1.r, theme::PANEL_1.g, theme::PANEL_1.b, 0.98),
        hovered: Color::new(theme::PANEL_2.r, theme::PANEL_2.g, theme::PANEL_2.b, 1.0),
        pressed: Color::new(theme::PANEL_0.r, theme::PANEL_0.g, theme::PANEL_0.b, 1.0),
        border: Color::new(
            theme::BORDER_1.r,
            theme::BORDER_1.g,
            theme::BORDER_1.b,
            0.90,
        ),
        text_color: theme::TEXT_STRONG,
        disabled: theme::DISABLED_FILL,
    }
}

fn utility_button_style() -> ButtonStyle {
    ButtonStyle {
        normal: Color::new(theme::PANEL_1.r, theme::PANEL_1.g, theme::PANEL_1.b, 0.94),
        hovered: Color::new(theme::PANEL_2.r, theme::PANEL_2.g, theme::PANEL_2.b, 1.0),
        pressed: Color::new(theme::PANEL_0.r, theme::PANEL_0.g, theme::PANEL_0.b, 1.0),
        border: Color::new(
            theme::BORDER_1.r,
            theme::BORDER_1.g,
            theme::BORDER_1.b,
            0.76,
        ),
        text_color: Color::new(
            theme::TEXT_BODY.r,
            theme::TEXT_BODY.g,
            theme::TEXT_BODY.b,
            0.96,
        ),
        disabled: theme::DISABLED_FILL,
    }
}

fn draw_button_label(text: &str, x: f32, y: f32, w: f32, h: f32, color: Color, font_size: f32) {
    let size = scaled_font_size(font_size.max(h * 0.38));
    let dims = measure_text_size(text, TextStyle::new(size, color));
    let text_x = x + (w - dims.width) * 0.5;
    let text_y = y + (h + dims.height) * 0.5 - 2.0;
    draw_text_ex(
        text,
        text_x + 1.5,
        text_y + 2.0,
        TextStyle::new(size, Color::new(0.02, 0.01, 0.03, 0.88)).params(),
    );
    draw_text_ex(text, text_x, text_y, TextStyle::new(size, color).params());
}

fn button_rect(x: f32, y: f32, w: f32, h: f32, style: &ButtonStyle) -> bool {
    let (mouse_x, mouse_y) = mouse_position();
    let hovered = mouse_x >= x && mouse_x <= x + w && mouse_y >= y && mouse_y <= y + h;
    let pressed = hovered && is_mouse_button_down(MouseButton::Left);
    let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);
    let fill = if pressed {
        style.pressed
    } else if hovered {
        style.hovered
    } else {
        style.normal
    };

    draw_chipped_button_surface(x, y, w, h, fill, style.border, hovered || pressed);
    clicked
}

fn draw_chipped_button_surface(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    fill: Color,
    border: Color,
    emphasis: bool,
) {
    let chip = (h * 0.22).clamp(6.0, 12.0);
    let shadow = Color::new(0.02, 0.01, 0.025, 0.38);
    let line_alpha = if emphasis { 0.95 } else { 0.62 };

    draw_rectangle(x + 4.0, y + 5.0, w - 4.0, h - 4.0, shadow);
    draw_rectangle(x + chip, y + 2.0, w - chip * 2.0, h - 4.0, fill);
    draw_rectangle(x + 4.0, y + chip, w - 8.0, h - chip * 2.0, fill);
    draw_triangle(
        vec2(x + 4.0, y + chip),
        vec2(x + chip, y + 2.0),
        vec2(x + chip, y + h - 4.0),
        fill,
    );
    draw_triangle(
        vec2(x + w - 4.0, y + chip),
        vec2(x + w - chip, y + 2.0),
        vec2(x + w - chip, y + h - 4.0),
        fill,
    );
    draw_line(
        x + chip,
        y + 2.0,
        x + w - chip * 1.4,
        y + 1.0,
        if emphasis { 2.0 } else { 1.0 },
        Color::new(border.r, border.g, border.b, line_alpha),
    );
    draw_line(
        x + chip * 0.8,
        y + h - 2.0,
        x + w - chip * 1.6,
        y + h - 1.0,
        1.0,
        Color::new(border.r, border.g, border.b, line_alpha * 0.62),
    );
    draw_line(
        x + 3.0,
        y + chip,
        x + chip,
        y + h - chip * 0.7,
        1.0,
        Color::new(border.r, border.g, border.b, line_alpha * 0.5),
    );
    draw_line(
        x + w - 3.0,
        y + chip * 0.8,
        x + w - chip,
        y + h - chip,
        1.0,
        Color::new(border.r, border.g, border.b, line_alpha * 0.5),
    );
}

pub fn primary_button(x: f32, y: f32, w: f32, h: f32, text: &str) -> bool {
    let style = primary_button_style();
    let clicked = button_rect(x, y, w, h, &style);
    draw_button_label(text, x, y, w, h, style.text_color, 16.0);
    clicked
}

pub fn secondary_button(x: f32, y: f32, w: f32, h: f32, text: &str) -> bool {
    let style = secondary_button_style();
    let clicked = button_rect(x, y, w, h, &style);
    draw_button_label(text, x, y, w, h, style.text_color, 15.0);
    clicked
}

pub fn utility_button(x: f32, y: f32, w: f32, h: f32, text: &str) -> bool {
    let style = utility_button_style();
    let clicked = button_rect(x, y, w, h, &style);
    draw_button_label(text, x, y, w, h, style.text_color, 14.0);
    clicked
}

pub fn draw_wrapped_lines(lines: &[String], x: f32, mut y: f32, font_size: f32, color: Color) {
    let scaled_font_size = scaled_font_size(font_size);
    let line_gap = scaled_spacing(8.0);

    for line in lines {
        draw_text_ex(line, x, y, TextStyle::new(scaled_font_size, color).params());
        y += scaled_font_size + line_gap;
    }
}

pub fn draw_heading_in_box(text: &str, x: f32, y: f32, width: f32, height: f32, font_size: f32) {
    let _ = draw_text_centered_in_box(
        text,
        x,
        y,
        width,
        height,
        scaled_font_size(font_size),
        theme::TEXT_STRONG,
    );
}

pub fn draw_body_text_in_box(
    text: &str,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    font_size: f32,
    color: Color,
) {
    let _ = draw_text_block_ex(
        text,
        x,
        y,
        width,
        height,
        TextStyle::new(scaled_font_size(font_size), color).with_line_gap(scaled_spacing(6.0)),
        18.0,
    );
}

pub fn draw_wrapped_lines_in_box(
    lines: &[String],
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    font_size: f32,
    color: Color,
) {
    let joined = lines.join("\n");
    let _ = draw_text_block_ex(
        &joined,
        x,
        y,
        width,
        height,
        TextStyle::new(scaled_font_size(font_size), color).with_line_gap(scaled_spacing(6.0)),
        18.0,
    );
}
