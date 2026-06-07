//! Shared semantic colors and panel-tier colors.

use macroquad::prelude::{color_u8, Color};

pub const BG_1: Color = color_u8!(24, 17, 29, 255);
pub const PANEL_0: Color = color_u8!(33, 24, 39, 255);
pub const PANEL_1: Color = color_u8!(42, 29, 49, 255);
pub const PANEL_2: Color = color_u8!(53, 35, 62, 255);

pub const BORDER_0: Color = color_u8!(74, 55, 85, 255);
pub const BORDER_1: Color = color_u8!(106, 77, 121, 255);
pub const BORDER_HI: Color = color_u8!(199, 154, 59, 255);

pub const PRIMARY: Color = color_u8!(106, 58, 126, 255);
pub const POSITIVE: Color = color_u8!(95, 163, 110, 255);
pub const WARNING: Color = color_u8!(211, 155, 69, 255);
pub const DANGER: Color = color_u8!(184, 74, 90, 255);
pub const INFO: Color = color_u8!(122, 99, 184, 255);
pub const GOLD: Color = color_u8!(199, 154, 59, 255);
pub const ROSE: Color = color_u8!(181, 106, 122, 255);

pub const PANEL_PRIMARY: Color = PRIMARY;
pub const PANEL_SUPPORT: Color = GOLD;
pub const PANEL_UTILITY: Color = BORDER_1;

pub const TEXT_STRONG: Color = color_u8!(241, 232, 244, 255);
pub const TEXT_BODY: Color = color_u8!(201, 184, 206, 255);
pub const TEXT_MUTED: Color = color_u8!(143, 125, 151, 255);
pub const TEXT_DIM: Color = color_u8!(100, 88, 102, 255);

pub const SELECTED_FILL: Color = Color::new(0.21, 0.14, 0.25, 0.78);
pub const DISABLED_FILL: Color = Color::new(0.10, 0.07, 0.11, 0.82);
pub const DISABLED_BORDER: Color = Color::new(0.27, 0.22, 0.30, 0.92);
