//! Shared local feedback rendering helpers.

use crate::ui::components::draw_badge;
use crate::ui::theme;

pub fn draw_inline_error(x: f32, y: f32, w: f32, text: &str) {
    draw_badge(x, y, w, 24.0, text, theme::DANGER);
}
