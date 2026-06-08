use macroquad::prelude::*;
use macroquad::text::draw_text_ex;
use macroquad_toolkit::ui::TextStyle;

use super::art_helpers::{
    accent_from_seed, draw_arches, draw_backdrop_texture, draw_building_scene, draw_cover_texture,
    draw_figure_silhouette, draw_floor_scene, draw_icon_glyph, draw_room_scene, draw_round_panel,
    draw_scene_columns, draw_text_center, draw_tower_silhouette, hash, icon_source, mix,
    with_ui_textures,
};
use crate::data::{BuildingData, GameData, GuildRoomData, TowerFloorData};
use crate::state::{CompanionState, ContractState, EggState};
use crate::ui::theme;
use crate::ui::view_models::species_name_by_id;

#[derive(Clone, Copy)]
pub enum BackdropKind {
    MainMenu,
    Opening,
    Town,
    TownManagement,
    Chamber,
    GuildJobs,
    GuestDesk,
    Expedition,
    Profile,
    Results,
    Settings,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum UiIcon {
    ResourceGold,
    ResourceMaterials,
    ResourceEgg,
    ResourceRelic,
    ResourceResidue,
    StatPower,
    StatCharm,
    StatEndurance,
    StatInstinct,
    ConditionFatigue,
    ConditionStress,
    ConditionInjury,
    ConditionCorruption,
    AssignIdle,
    AssignGuildJob,
    AssignResting,
    AssignExpedition,
    AssignGuestRequest,
    NavTown,
    NavPlanner,
    NavGuildJobs,
    NavGuest,
    NavChamber,
    NavExpedition,
    NavJournal,
    NavSettings,
    NavEndDay,
    StatusAvailable,
    StatusBuilt,
    StatusLocked,
    StatusAssigned,
    StatusBlocked,
    StatusWarning,
    StatusAccepted,
    StatusCompleted,
    StatusFailed,
    MissionResourceRun,
    MissionEggHunt,
    MissionRelicRaid,
    MissionCorruptionDive,
    ActionSave,
    ActionClose,
    ActionQuit,
}

pub fn draw_backdrop(kind: BackdropKind) {
    let (top, bottom, accent) = match kind {
        BackdropKind::MainMenu => (
            color_u8!(18, 10, 18, 255),
            color_u8!(66, 26, 44, 255),
            theme::GOLD,
        ),
        BackdropKind::Opening => (
            color_u8!(20, 12, 20, 255),
            color_u8!(58, 30, 45, 255),
            theme::ROSE,
        ),
        BackdropKind::Town => (
            color_u8!(19, 12, 22, 255),
            color_u8!(54, 30, 52, 255),
            theme::PRIMARY,
        ),
        BackdropKind::TownManagement => (
            color_u8!(22, 14, 20, 255),
            color_u8!(63, 34, 40, 255),
            theme::GOLD,
        ),
        BackdropKind::Chamber => (
            color_u8!(18, 10, 24, 255),
            color_u8!(53, 24, 67, 255),
            theme::INFO,
        ),
        BackdropKind::GuildJobs => (
            color_u8!(24, 10, 18, 255),
            color_u8!(84, 28, 44, 255),
            theme::ROSE,
        ),
        BackdropKind::GuestDesk => (
            color_u8!(20, 12, 22, 255),
            color_u8!(65, 31, 51, 255),
            theme::ROSE,
        ),
        BackdropKind::Expedition => (
            color_u8!(16, 11, 19, 255),
            color_u8!(54, 30, 58, 255),
            theme::WARNING,
        ),
        BackdropKind::Profile => (
            color_u8!(18, 10, 18, 255),
            color_u8!(62, 26, 50, 255),
            theme::ROSE,
        ),
        BackdropKind::Results => (
            color_u8!(20, 12, 18, 255),
            color_u8!(70, 38, 32, 255),
            theme::GOLD,
        ),
        BackdropKind::Settings => (
            color_u8!(18, 10, 18, 255),
            color_u8!(48, 26, 52, 255),
            theme::PRIMARY,
        ),
    };

    let width = screen_width();
    let height = screen_height();
    if draw_backdrop_texture(kind, width, height) {
        if matches!(kind, BackdropKind::MainMenu) {
            return;
        }

        draw_rectangle(
            0.0,
            0.0,
            width,
            height,
            Color::new(top.r, top.g, top.b, 0.32),
        );
        draw_rectangle(
            0.0,
            0.0,
            width,
            92.0,
            Color::new(theme::BG_1.r, theme::BG_1.g, theme::BG_1.b, 0.42),
        );
        draw_rectangle(
            0.0,
            height - 96.0,
            width,
            96.0,
            Color::new(theme::BG_1.r, theme::BG_1.g, theme::BG_1.b, 0.34),
        );
        draw_arches(width, height, accent);
        return;
    }

    let steps = 16;
    for i in 0..steps {
        let t = i as f32 / (steps - 1) as f32;
        let band_y = height * t;
        let color = mix(top, bottom, t);
        draw_rectangle(0.0, band_y, width, height / steps as f32 + 1.0, color);
    }

    for i in 0..5 {
        let alpha = 0.12 - i as f32 * 0.018;
        draw_circle(
            width * (0.15 + i as f32 * 0.18),
            height * (0.15 + (i % 2) as f32 * 0.08),
            160.0 + i as f32 * 44.0,
            Color::new(accent.r, accent.g, accent.b, alpha.max(0.03)),
        );
    }

    draw_tower_silhouette(
        width * 0.78,
        height * 0.16,
        width * 0.16,
        height * 0.6,
        accent,
    );
    draw_arches(width, height, accent);
}

pub fn draw_town_overview_backdrop() {
    let width = screen_width();
    let height = screen_height();

    with_ui_textures(|textures| {
        draw_cover_texture(
            &textures.town_overview_backdrop,
            0.0,
            0.0,
            width,
            height,
            WHITE,
        );
    });

    draw_rectangle(0.0, 0.0, width, height, Color::new(0.06, 0.03, 0.07, 0.28));
    draw_rectangle(
        0.0,
        0.0,
        width,
        92.0,
        Color::new(theme::BG_1.r, theme::BG_1.g, theme::BG_1.b, 0.58),
    );
    draw_rectangle(
        0.0,
        height - 88.0,
        width,
        88.0,
        Color::new(theme::BG_1.r, theme::BG_1.g, theme::BG_1.b, 0.54),
    );
    draw_rectangle(
        0.0,
        84.0,
        width * 0.52,
        height - 162.0,
        Color::new(theme::BG_1.r, theme::BG_1.g, theme::BG_1.b, 0.16),
    );
}

pub fn draw_ui_icon(icon: UiIcon, x: f32, y: f32, size: f32, tint: Color) {
    with_ui_textures(|textures| {
        draw_texture_ex(
            &textures.icon_atlas,
            x,
            y,
            tint,
            DrawTextureParams {
                source: Some(icon_source(icon)),
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    });
}

pub fn icon_for_metric_label(label: &str) -> Option<UiIcon> {
    let normalized = label.to_ascii_lowercase();
    if normalized.contains("gold") {
        Some(UiIcon::ResourceGold)
    } else if normalized.contains("material") {
        Some(UiIcon::ResourceMaterials)
    } else if normalized.contains("egg") {
        Some(UiIcon::ResourceEgg)
    } else if normalized.contains("relic") {
        Some(UiIcon::ResourceRelic)
    } else if normalized.contains("residue") {
        Some(UiIcon::ResourceResidue)
    } else if normalized.contains("debt") || normalized.contains("risk") {
        Some(UiIcon::StatusWarning)
    } else if normalized.contains("power") {
        Some(UiIcon::StatPower)
    } else if normalized.contains("charm") {
        Some(UiIcon::StatCharm)
    } else if normalized.contains("endurance") {
        Some(UiIcon::StatEndurance)
    } else if normalized.contains("instinct") {
        Some(UiIcon::StatInstinct)
    } else if normalized.contains("fatigue") {
        Some(UiIcon::ConditionFatigue)
    } else if normalized.contains("stress") {
        Some(UiIcon::ConditionStress)
    } else if normalized.contains("injury") {
        Some(UiIcon::ConditionInjury)
    } else if normalized.contains("corruption") {
        Some(UiIcon::ConditionCorruption)
    } else if normalized.contains("success") {
        Some(UiIcon::StatusCompleted)
    } else if normalized == "built" || normalized.contains("built count") {
        Some(UiIcon::StatusBuilt)
    } else {
        None
    }
}

pub fn draw_story_cg_placeholder(title: &str, x: f32, y: f32, w: f32, h: f32, seed: &str) {
    let seed = hash(seed);
    let accent = accent_from_seed(seed);
    draw_round_panel(
        x,
        y,
        w,
        h,
        Color::new(theme::BG_1.r, theme::BG_1.g, theme::BG_1.b, 0.92),
        accent,
    );
    draw_scene_columns(x, y, w, h, accent);
    draw_figure_silhouette(x + w * 0.26, y + h * 0.18, w * 0.18, h * 0.64, accent, 0.85);
    draw_figure_silhouette(
        x + w * 0.56,
        y + h * 0.12,
        w * 0.16,
        h * 0.7,
        mix(accent, WHITE, 0.2),
        0.8,
    );
    draw_text_center(
        title,
        x + 16.0,
        y + h - 36.0,
        w - 32.0,
        22.0,
        20.0,
        theme::TEXT_STRONG,
    );
}

pub fn draw_species_portrait(
    data: &GameData,
    monster: &CompanionState,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    let seed_key = format!("{}:{}", monster.species_id, monster.name);
    let seed = hash(&seed_key);
    let accent = accent_from_seed(seed);
    let species_name = species_name_by_id(data, &monster.species_id);

    draw_round_panel(
        x,
        y,
        w,
        h,
        Color::new(theme::BG_1.r, theme::BG_1.g, theme::BG_1.b, 0.94),
        accent,
    );
    for band in 0..6 {
        let t = band as f32 / 5.0;
        let band_color = mix(Color::new(0.12, 0.1, 0.14, 0.95), accent, t * 0.5);
        draw_rectangle(
            x + 10.0,
            y + 10.0 + (h - 20.0) * t,
            w - 20.0,
            (h - 20.0) / 6.0 + 1.0,
            band_color,
        );
    }

    draw_figure_silhouette(x + w * 0.22, y + h * 0.1, w * 0.56, h * 0.72, accent, 0.9);
    if monster.species_id.contains("harpy") || species_name.to_ascii_lowercase().contains("harpy") {
        draw_triangle(
            vec2(x + w * 0.24, y + h * 0.46),
            vec2(x + w * 0.08, y + h * 0.68),
            vec2(x + w * 0.28, y + h * 0.64),
            Color::new(accent.r, accent.g, accent.b, 0.72),
        );
        draw_triangle(
            vec2(x + w * 0.76, y + h * 0.46),
            vec2(x + w * 0.92, y + h * 0.68),
            vec2(x + w * 0.72, y + h * 0.64),
            Color::new(accent.r, accent.g, accent.b, 0.72),
        );
    }
    if monster.species_id.contains("lamia") || monster.species_id.contains("golemkin") {
        for i in 0..4 {
            let sway = (i as f32 * 31.0 + (seed % 17) as f32).sin() * 12.0;
            draw_line(
                x + w * 0.42 + i as f32 * 18.0,
                y + h * 0.62,
                x + w * 0.28 + i as f32 * 28.0 + sway,
                y + h * 0.9,
                6.0,
                Color::new(accent.r, accent.g, accent.b, 0.68),
            );
        }
    }
}

pub fn draw_guest_silhouette(request: &ContractState, x: f32, y: f32, w: f32, h: f32) {
    let seed = hash(&request.guest_name);
    let accent = accent_from_seed(seed);
    draw_round_panel(
        x,
        y,
        w,
        h,
        Color::new(theme::BG_1.r, theme::BG_1.g, theme::BG_1.b, 0.92),
        accent,
    );
    draw_figure_silhouette(x + w * 0.26, y + h * 0.12, w * 0.48, h * 0.72, accent, 0.78);
    draw_line(
        x + w * 0.6,
        y + h * 0.34,
        x + w * 0.77,
        y + h * 0.84,
        5.0,
        Color::new(accent.r, accent.g, accent.b, 0.8),
    );
    draw_text_center(
        &request.guest_name,
        x + 10.0,
        y + h - 28.0,
        w - 20.0,
        18.0,
        16.0,
        theme::TEXT_BODY,
    );
}

pub fn draw_room_thumbnail(room: &GuildRoomData, x: f32, y: f32, w: f32, h: f32) {
    let accent = accent_from_seed(hash(&room.id));
    draw_round_panel(
        x,
        y,
        w,
        h,
        Color::new(theme::PANEL_0.r, theme::PANEL_0.g, theme::PANEL_0.b, 0.92),
        accent,
    );
    draw_room_scene(&room.id, x + 8.0, y + 8.0, w - 16.0, h - 16.0, accent);
}

pub fn draw_floor_preview(floor: &TowerFloorData, x: f32, y: f32, w: f32, h: f32) {
    let accent = accent_from_seed(hash(&floor.id));
    draw_round_panel(
        x,
        y,
        w,
        h,
        Color::new(theme::PANEL_0.r, theme::PANEL_0.g, theme::PANEL_0.b, 0.92),
        accent,
    );
    draw_floor_scene(&floor.id, x + 8.0, y + 8.0, w - 16.0, h - 16.0, accent);
}

pub fn draw_building_thumbnail(building: &BuildingData, x: f32, y: f32, w: f32, h: f32) {
    let accent = accent_from_seed(hash(&building.id));
    draw_round_panel(
        x,
        y,
        w,
        h,
        Color::new(theme::PANEL_0.r, theme::PANEL_0.g, theme::PANEL_0.b, 0.92),
        accent,
    );
    draw_building_scene(&building.id, x + 8.0, y + 8.0, w - 16.0, h - 16.0, accent);
}

pub fn draw_egg_thumbnail(egg: &EggState, x: f32, y: f32, w: f32, h: f32) {
    let seed = hash(&egg.id);
    let accent = accent_from_seed(seed);
    draw_round_panel(
        x,
        y,
        w,
        h,
        Color::new(theme::BG_1.r, theme::BG_1.g, theme::BG_1.b, 0.94),
        accent,
    );
    let shell = Color::new(
        accent.r * 0.9 + 0.1,
        accent.g * 0.9 + 0.1,
        accent.b * 0.95 + 0.05,
        1.0,
    );
    draw_ellipse(x + w * 0.5, y + h * 0.5, w * 0.24, h * 0.3, 0.0, shell);
    draw_ellipse_lines(x + w * 0.5, y + h * 0.5, w * 0.24, h * 0.3, 0.0, 4.0, WHITE);
    for i in 0..4 {
        draw_circle(
            x + w * (0.39 + i as f32 * 0.08),
            y + h * (0.34 + (i % 2) as f32 * 0.1),
            4.0 + (seed % 3) as f32,
            Color::new(1.0, 1.0, 1.0, 0.45),
        );
    }
}

pub fn draw_trait_icons(data: &GameData, trait_ids: &[String], x: f32, y: f32, w: f32) {
    if trait_ids.is_empty() {
        draw_text_ex(
            "No traits",
            x,
            y + 18.0,
            TextStyle::new(18.0, theme::TEXT_DIM).params(),
        );
        return;
    }

    let cell = 34.0;
    let per_row = (w / (cell + 8.0)).max(1.0) as usize;
    for (index, trait_id) in trait_ids.iter().enumerate() {
        let Some(trait_data) = data
            .traits
            .traits
            .iter()
            .find(|entry| entry.id == *trait_id)
        else {
            continue;
        };
        let col = index % per_row;
        let row = index / per_row;
        let cell_x = x + col as f32 * (cell + 8.0);
        let cell_y = y + row as f32 * (cell + 8.0);
        let accent = accent_from_seed(hash(&trait_data.icon_key));
        draw_round_panel(
            cell_x,
            cell_y,
            cell,
            cell,
            Color::new(theme::PANEL_0.r, theme::PANEL_0.g, theme::PANEL_0.b, 0.94),
            accent,
        );
        draw_icon_glyph(cell_x + 8.0, cell_y + 8.0, 18.0, accent, index as u32 + 20);
    }
}

pub fn draw_condition_badges(monster: &CompanionState, x: f32, y: f32, w: f32) {
    let badges = [
        ("F", monster.fatigue, color_u8!(205, 175, 90, 255)),
        ("S", monster.stress, color_u8!(205, 122, 111, 255)),
        ("I", monster.injury, color_u8!(191, 92, 92, 255)),
        ("C", monster.corruption, color_u8!(161, 98, 185, 255)),
    ];
    let badge_w = (w - 18.0) / 4.0;
    for (index, (label, value, accent)) in badges.iter().enumerate() {
        let badge_x = x + index as f32 * (badge_w + 6.0);
        draw_round_panel(
            badge_x,
            y,
            badge_w,
            30.0,
            Color::new(theme::PANEL_0.r, theme::PANEL_0.g, theme::PANEL_0.b, 0.92),
            *accent,
        );
        draw_text_ex(
            label,
            badge_x + 8.0,
            y + 19.0,
            TextStyle::new(18.0, theme::TEXT_STRONG).params(),
        );
        draw_text_ex(
            &format!("{value}"),
            badge_x + 24.0,
            y + 19.0,
            TextStyle::new(18.0, theme::TEXT_BODY).params(),
        );
    }
}
