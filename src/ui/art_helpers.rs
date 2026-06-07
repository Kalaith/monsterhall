use std::cell::RefCell;

use macroquad::prelude::*;

use super::art::{BackdropKind, UiIcon};
use crate::ui::theme;

const ICON_CELL: f32 = 64.0;

pub(super) struct UiTextures {
    pub(super) title_page_backdrop: Texture2D,
    pub(super) main_menu_backdrop: Texture2D,
    pub(super) town_backdrop: Texture2D,
    pub(super) town_overview_backdrop: Texture2D,
    pub(super) chamber_backdrop: Texture2D,
    pub(super) expedition_backdrop: Texture2D,
    pub(super) icon_atlas: Texture2D,
}

thread_local! {
    static UI_TEXTURES: RefCell<Option<UiTextures>> = RefCell::new(None);
}

pub(super) fn draw_tower_silhouette(x: f32, y: f32, w: f32, h: f32, accent: Color) {
    draw_rectangle(
        x,
        y + h * 0.2,
        w,
        h * 0.8,
        Color::new(0.05, 0.05, 0.06, 0.9),
    );
    draw_triangle(
        vec2(x - w * 0.12, y + h * 0.2),
        vec2(x + w * 1.12, y + h * 0.2),
        vec2(x + w * 0.5, y),
        Color::new(0.05, 0.05, 0.06, 0.95),
    );
    for i in 0..4 {
        draw_rectangle(
            x + w * (0.14 + i as f32 * 0.18),
            y + h * (0.34 + (i % 2) as f32 * 0.12),
            w * 0.08,
            h * 0.1,
            Color::new(accent.r, accent.g, accent.b, 0.35),
        );
    }
}

pub(super) fn draw_arches(w: f32, h: f32, accent: Color) {
    for i in 0..4 {
        let arch_w = w * 0.22;
        let x = i as f32 * (arch_w + 24.0) - 12.0;
        draw_rectangle(
            x,
            h * 0.68,
            arch_w,
            h * 0.4,
            Color::new(0.04, 0.04, 0.05, 0.24),
        );
        draw_ellipse_lines(
            x + arch_w * 0.5,
            h * 0.68,
            arch_w * 0.5,
            h * 0.18,
            0.0,
            2.0,
            Color::new(accent.r, accent.g, accent.b, 0.12),
        );
    }
}

pub(super) fn draw_scene_columns(x: f32, y: f32, w: f32, h: f32, accent: Color) {
    for i in 0..3 {
        let col_x = x + 28.0 + i as f32 * (w - 76.0) / 2.0;
        draw_rectangle(
            col_x,
            y + 24.0,
            22.0,
            h - 48.0,
            Color::new(0.18, 0.15, 0.14, 0.85),
        );
        draw_rectangle(col_x - 4.0, y + 20.0, 30.0, 10.0, accent);
        draw_rectangle(col_x - 4.0, y + h - 30.0, 30.0, 10.0, accent);
    }
}

pub(super) fn draw_figure_silhouette(x: f32, y: f32, w: f32, h: f32, accent: Color, alpha: f32) {
    let shade = Color::new(accent.r * 0.72, accent.g * 0.72, accent.b * 0.8, alpha);
    draw_circle(x + w * 0.5, y + h * 0.14, w * 0.14, shade);
    draw_ellipse(x + w * 0.5, y + h * 0.4, w * 0.22, h * 0.22, 0.0, shade);
    draw_ellipse(x + w * 0.5, y + h * 0.65, w * 0.18, h * 0.24, 0.0, shade);
    draw_line(
        x + w * 0.38,
        y + h * 0.52,
        x + w * 0.24,
        y + h * 0.8,
        8.0,
        shade,
    );
    draw_line(
        x + w * 0.62,
        y + h * 0.52,
        x + w * 0.76,
        y + h * 0.8,
        8.0,
        shade,
    );
    draw_line(
        x + w * 0.42,
        y + h * 0.3,
        x + w * 0.26,
        y + h * 0.54,
        8.0,
        shade,
    );
    draw_line(
        x + w * 0.58,
        y + h * 0.3,
        x + w * 0.74,
        y + h * 0.54,
        8.0,
        shade,
    );
}

pub(super) fn draw_room_scene(room_id: &str, x: f32, y: f32, w: f32, h: f32, accent: Color) {
    draw_rectangle(
        x,
        y + h * 0.56,
        w,
        h * 0.44,
        Color::new(0.22, 0.12, 0.12, 0.85),
    );
    if room_id.contains("stage") {
        draw_rectangle(x + w * 0.14, y + h * 0.6, w * 0.72, h * 0.18, accent);
        draw_line(
            x + w * 0.2,
            y + h * 0.22,
            x + w * 0.2,
            y + h * 0.56,
            6.0,
            accent,
        );
        draw_line(
            x + w * 0.8,
            y + h * 0.22,
            x + w * 0.8,
            y + h * 0.56,
            6.0,
            accent,
        );
    } else if room_id.contains("golemkin") {
        for i in 0..5 {
            draw_line(
                x + w * (0.18 + i as f32 * 0.14),
                y + h * 0.86,
                x + w * (0.12 + i as f32 * 0.16),
                y + h * (0.28 + (i % 2) as f32 * 0.1),
                7.0,
                Color::new(accent.r, accent.g, accent.b, 0.82),
            );
        }
    } else if room_id.contains("nursery") {
        draw_rectangle(
            x + w * 0.18,
            y + h * 0.58,
            w * 0.64,
            h * 0.18,
            Color::new(0.44, 0.22, 0.24, 0.9),
        );
        draw_circle(x + w * 0.5, y + h * 0.42, 24.0, accent);
    } else {
        draw_rectangle(
            x + w * 0.18,
            y + h * 0.58,
            w * 0.64,
            h * 0.16,
            Color::new(0.46, 0.3, 0.28, 0.9),
        );
        draw_line(
            x + w * 0.16,
            y + h * 0.28,
            x + w * 0.84,
            y + h * 0.28,
            5.0,
            accent,
        );
    }
}

pub(super) fn draw_floor_scene(floor_id: &str, x: f32, y: f32, w: f32, h: f32, accent: Color) {
    draw_rectangle(
        x,
        y + h * 0.66,
        w,
        h * 0.34,
        Color::new(0.16, 0.14, 0.13, 0.9),
    );
    if floor_id.contains("slick") {
        for i in 0..4 {
            draw_circle(x + w * (0.18 + i as f32 * 0.18), y + h * 0.78, 18.0, accent);
        }
    } else if floor_id.contains("molten") {
        for i in 0..3 {
            draw_ellipse(
                x + w * (0.28 + i as f32 * 0.2),
                y + h * 0.76,
                34.0,
                16.0,
                0.0,
                accent,
            );
        }
    } else if floor_id.contains("kennel") {
        for i in 0..5 {
            let bar_x = x + w * 0.16 + i as f32 * w * 0.12;
            draw_line(bar_x, y + h * 0.2, bar_x, y + h * 0.76, 4.0, accent);
        }
    } else {
        draw_circle(
            x + w * 0.5,
            y + h * 0.45,
            48.0,
            Color::new(accent.r, accent.g, accent.b, 0.75),
        );
        for i in 0..5 {
            draw_line(
                x + w * 0.5,
                y + h * 0.45,
                x + w * (0.18 + i as f32 * 0.16),
                y + h * (0.18 + (i % 2) as f32 * 0.1),
                5.0,
                accent,
            );
        }
    }
}

pub(super) fn draw_building_scene(
    building_id: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    accent: Color,
) {
    draw_rectangle(
        x + w * 0.2,
        y + h * 0.46,
        w * 0.6,
        h * 0.38,
        Color::new(0.22, 0.14, 0.15, 0.9),
    );
    draw_triangle(
        vec2(x + w * 0.14, y + h * 0.48),
        vec2(x + w * 0.86, y + h * 0.48),
        vec2(x + w * 0.5, y + h * 0.22),
        accent,
    );
    if building_id.contains("pool") || building_id.contains("springs") {
        draw_ellipse(
            x + w * 0.5,
            y + h * 0.72,
            w * 0.22,
            h * 0.08,
            0.0,
            Color::new(0.45, 0.68, 0.76, 0.9),
        );
    } else if building_id.contains("forge") || building_id.contains("lab") {
        draw_circle(
            x + w * 0.68,
            y + h * 0.64,
            16.0,
            Color::new(0.92, 0.62, 0.28, 0.9),
        );
    } else if building_id.contains("archive") || building_id.contains("cartography") {
        draw_rectangle(x + w * 0.3, y + h * 0.58, w * 0.12, h * 0.18, accent);
        draw_rectangle(x + w * 0.44, y + h * 0.58, w * 0.12, h * 0.18, accent);
    }
}

pub(super) fn draw_icon_glyph(x: f32, y: f32, size: f32, accent: Color, variant: u32) {
    match variant % 6 {
        0 => draw_circle(x + size * 0.5, y + size * 0.5, size * 0.4, accent),
        1 => draw_rectangle(x + 2.0, y + 2.0, size - 4.0, size - 4.0, accent),
        2 => draw_triangle(
            vec2(x + size * 0.5, y + 1.0),
            vec2(x + size - 1.0, y + size - 1.0),
            vec2(x + 1.0, y + size - 1.0),
            accent,
        ),
        3 => {
            draw_line(
                x + size * 0.5,
                y + 2.0,
                x + size * 0.5,
                y + size - 2.0,
                3.0,
                accent,
            );
            draw_line(
                x + 2.0,
                y + size * 0.5,
                x + size - 2.0,
                y + size * 0.5,
                3.0,
                accent,
            );
        }
        4 => {
            draw_circle_lines(x + size * 0.5, y + size * 0.5, size * 0.34, 3.0, accent);
            draw_circle(x + size * 0.5, y + size * 0.5, size * 0.1, accent);
        }
        _ => {
            draw_line(
                x + 2.0,
                y + 2.0,
                x + size - 2.0,
                y + size - 2.0,
                3.0,
                accent,
            );
            draw_line(
                x + size - 2.0,
                y + 2.0,
                x + 2.0,
                y + size - 2.0,
                3.0,
                accent,
            );
        }
    }
}

pub(super) fn draw_round_panel(x: f32, y: f32, w: f32, h: f32, fill: Color, border: Color) {
    draw_rectangle(x, y, w, h, fill);
    draw_rectangle_lines(x, y, w, h, 2.0, border);
    draw_rectangle_lines(
        x + 4.0,
        y + 4.0,
        w - 8.0,
        h - 8.0,
        1.0,
        Color::new(border.r, border.g, border.b, 0.5),
    );
}

pub(super) fn draw_text_center(
    text: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    size: f32,
    color: Color,
) {
    let dims = measure_text(text, None, size as u16, 1.0);
    let text_x = x + (w - dims.width) * 0.5;
    let text_y = y + (h + dims.height) * 0.5 - 4.0;
    draw_text(text, text_x, text_y, size, color);
}

pub(super) fn draw_backdrop_texture(kind: BackdropKind, w: f32, h: f32) -> bool {
    with_ui_textures(|textures| {
        let texture = match kind {
            BackdropKind::MainMenu => &textures.title_page_backdrop,
            BackdropKind::Opening => &textures.main_menu_backdrop,
            BackdropKind::Chamber => &textures.chamber_backdrop,
            BackdropKind::Expedition => &textures.expedition_backdrop,
            BackdropKind::Town
            | BackdropKind::TownManagement
            | BackdropKind::GuildJobs
            | BackdropKind::GuestDesk
            | BackdropKind::Profile
            | BackdropKind::Results
            | BackdropKind::Settings => &textures.town_backdrop,
        };
        draw_cover_texture(texture, 0.0, 0.0, w, h, WHITE);
        true
    })
}

pub(super) fn draw_cover_texture(texture: &Texture2D, x: f32, y: f32, w: f32, h: f32, tint: Color) {
    let texture_w = texture.width();
    let texture_h = texture.height();
    let texture_aspect = texture_w / texture_h;
    let target_aspect = w / h;
    let source = if texture_aspect > target_aspect {
        let source_w = texture_h * target_aspect;
        Rect::new((texture_w - source_w) * 0.5, 0.0, source_w, texture_h)
    } else {
        let source_h = texture_w / target_aspect;
        Rect::new(0.0, (texture_h - source_h) * 0.5, texture_w, source_h)
    };

    draw_texture_ex(
        texture,
        x,
        y,
        tint,
        DrawTextureParams {
            source: Some(source),
            dest_size: Some(vec2(w, h)),
            ..Default::default()
        },
    );
}

pub(super) fn with_ui_textures<R>(draw: impl FnOnce(&UiTextures) -> R) -> R {
    UI_TEXTURES.with(|cell| {
        if cell.borrow().is_none() {
            *cell.borrow_mut() = Some(UiTextures::load());
        }
        let textures = cell.borrow();
        draw(
            textures
                .as_ref()
                .expect("UI textures should be initialized"),
        )
    })
}

impl UiTextures {
    fn load() -> Self {
        Self {
            title_page_backdrop: embedded_texture(
                include_bytes!("../../assets/images/backdrops/title_page.png"),
                FilterMode::Linear,
            ),
            main_menu_backdrop: embedded_texture(
                include_bytes!("../../assets/images/backdrops/main_menu.png"),
                FilterMode::Linear,
            ),
            town_backdrop: embedded_texture(
                include_bytes!("../../assets/images/backdrops/town.png"),
                FilterMode::Linear,
            ),
            town_overview_backdrop: embedded_texture(
                include_bytes!("../../assets/images/backdrops/town_overview.png"),
                FilterMode::Linear,
            ),
            chamber_backdrop: embedded_texture(
                include_bytes!("../../assets/images/backdrops/chamber.png"),
                FilterMode::Linear,
            ),
            expedition_backdrop: embedded_texture(
                include_bytes!("../../assets/images/backdrops/expedition.png"),
                FilterMode::Linear,
            ),
            icon_atlas: embedded_texture(
                include_bytes!("../../assets/images/icons/ui_icon_atlas.png"),
                FilterMode::Linear,
            ),
        }
    }
}

pub(super) fn embedded_texture(bytes: &[u8], filter: FilterMode) -> Texture2D {
    let texture = Texture2D::from_file_with_format(bytes, None);
    texture.set_filter(filter);
    texture
}

pub(super) fn icon_source(icon: UiIcon) -> Rect {
    let index = icon_index(icon);
    let col = (index % 8) as f32;
    let row = (index / 8) as f32;
    Rect::new(col * ICON_CELL, row * ICON_CELL, ICON_CELL, ICON_CELL)
}

pub(super) fn icon_index(icon: UiIcon) -> u32 {
    match icon {
        UiIcon::ResourceGold => 0,
        UiIcon::ResourceMaterials => 1,
        UiIcon::ResourceEgg => 2,
        UiIcon::ResourceRelic => 3,
        UiIcon::ResourceResidue => 4,
        UiIcon::StatPower => 5,
        UiIcon::StatCharm => 6,
        UiIcon::StatEndurance => 7,
        UiIcon::StatInstinct => 8,
        UiIcon::ConditionFatigue => 9,
        UiIcon::ConditionStress => 10,
        UiIcon::ConditionInjury => 11,
        UiIcon::ConditionCorruption => 12,
        UiIcon::AssignIdle => 13,
        UiIcon::AssignGuildJob => 14,
        UiIcon::AssignResting => 15,
        UiIcon::AssignExpedition => 16,
        UiIcon::AssignGuestRequest => 17,
        UiIcon::NavTown => 18,
        UiIcon::NavPlanner => 19,
        UiIcon::NavGuildJobs => 20,
        UiIcon::NavGuest => 21,
        UiIcon::NavChamber => 22,
        UiIcon::NavExpedition => 23,
        UiIcon::NavJournal => 24,
        UiIcon::NavSettings => 25,
        UiIcon::NavEndDay => 26,
        UiIcon::StatusAvailable => 27,
        UiIcon::StatusBuilt => 28,
        UiIcon::StatusLocked => 29,
        UiIcon::StatusAssigned => 30,
        UiIcon::StatusBlocked => 31,
        UiIcon::StatusWarning => 32,
        UiIcon::StatusAccepted => 33,
        UiIcon::StatusCompleted => 34,
        UiIcon::StatusFailed => 35,
        UiIcon::MissionResourceRun => 36,
        UiIcon::MissionEggHunt => 37,
        UiIcon::MissionRelicRaid => 38,
        UiIcon::MissionCorruptionDive => 39,
        UiIcon::ActionSave => 40,
        UiIcon::ActionClose => 41,
        UiIcon::ActionQuit => 42,
    }
}

pub(super) fn accent_from_seed(seed: u32) -> Color {
    let hue_band = seed % 6;
    match hue_band {
        0 => theme::ROSE,
        1 => theme::INFO,
        2 => theme::GOLD,
        3 => theme::PRIMARY,
        4 => theme::POSITIVE,
        _ => theme::DANGER,
    }
}

pub(super) fn hash(input: &str) -> u32 {
    let mut value = 2166136261u32;
    for byte in input.as_bytes() {
        value ^= u32::from(*byte);
        value = value.wrapping_mul(16777619);
    }
    value
}

pub(super) fn mix(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}
