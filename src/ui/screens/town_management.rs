use macroquad::prelude::{screen_height, screen_width};

use crate::data::GameData;
use crate::state::{GameState, TownManagementState};
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_backdrop, draw_building_thumbnail, BackdropKind};
use crate::ui::chrome::{
    draw_inline_status, draw_screen_header, draw_standard_gameplay_footer, draw_tier_panel,
    draw_top_utility_bar, PanelTier,
};
use crate::ui::components::{draw_badge, draw_metric_tile, draw_status_marker};
use crate::ui::core::{draw_body_text, draw_body_text_in_box, primary_button, secondary_button};
use crate::ui::feedback::draw_inline_error;
use crate::ui::layout;
use crate::ui::theme;
use crate::ui::view_models::{building_decision_summary, format_resource_cost};

fn compact_text(text: &str, max_len: usize) -> String {
    let compact = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.chars().count() <= max_len {
        compact
    } else {
        let trimmed = compact
            .chars()
            .take(max_len.saturating_sub(1))
            .collect::<String>();
        format!("{trimmed}...")
    }
}

fn status_marker(
    status_label: &str,
    available_label: &str,
    built_out_label: &str,
) -> (u8, macroquad::prelude::Color) {
    if status_label == available_label {
        (0, theme::POSITIVE)
    } else if status_label == built_out_label {
        (1, theme::INFO)
    } else {
        (2, theme::WARNING)
    }
}

fn building_group_id(category: &str) -> &str {
    match category {
        "project" | "prestige" => "projects",
        _ => "core",
    }
}

struct TownManagementLayout {
    left_margin: f32,
    content_width: f32,
    list_w: f32,
    detail_x: f32,
    detail_w: f32,
    footer_y: f32,
}

impl TownManagementLayout {
    fn new() -> Self {
        let left_margin = layout::OUTER_MARGIN;
        let content_width = screen_width() - left_margin * 2.0;
        let list_w = 318.0;
        let detail_x = left_margin + list_w + layout::SECTION_GAP;
        let detail_w = content_width - list_w - layout::SECTION_GAP;
        let footer_y = screen_height() - layout::FOOTER_BOTTOM_MARGIN - layout::FOOTER_H;

        Self {
            left_margin,
            content_width,
            list_w,
            detail_x,
            detail_w,
            footer_y,
        }
    }
}

pub fn draw_town_management(
    data: &GameData,
    town_state: &TownManagementState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    draw_backdrop(BackdropKind::TownManagement);
    let layout = TownManagementLayout::new();
    let ui = &data.ui_text.town_management;

    if let Some(action) = draw_top_utility_bar(&data.ui_text.common.settings_button) {
        return Some(action);
    }
    draw_screen_header(&ui.title, &ui.subtitle);

    let selected_group_id = if town_state.selected_group_id == "projects" {
        "projects"
    } else {
        "core"
    };
    let visible_buildings = data
        .buildings
        .buildings
        .iter()
        .filter(|building| building_group_id(&building.category) == selected_group_id)
        .collect::<Vec<_>>();
    let selected_building = visible_buildings
        .iter()
        .copied()
        .find(|building| building.id == town_state.selected_building_id)
        .or_else(|| visible_buildings.first().copied())
        .or_else(|| data.buildings.buildings.first())?;
    let summary = building_decision_summary(data, game_state, selected_building);

    let list_h = (layout.footer_y - 92.0 - layout::SECTION_GAP).max(420.0);
    draw_tier_panel(
        layout.left_margin,
        92.0,
        layout.list_w,
        list_h,
        Some(&ui.buildings_panel_title),
        PanelTier::Support,
        false,
    );
    let core_pressed = if selected_group_id == "core" {
        primary_button(layout.left_margin + 12.0, 104.0, 128.0, 28.0, "Core")
    } else {
        secondary_button(layout.left_margin + 12.0, 104.0, 128.0, 28.0, "Core")
    };
    if core_pressed {
        return Some(UiAction::SelectTownBuildingGroup("core".to_owned()));
    }
    let projects_pressed = if selected_group_id == "projects" {
        primary_button(layout.left_margin + 148.0, 104.0, 140.0, 28.0, "Projects")
    } else {
        secondary_button(layout.left_margin + 148.0, 104.0, 140.0, 28.0, "Projects")
    };
    if projects_pressed {
        return Some(UiAction::SelectTownBuildingGroup("projects".to_owned()));
    }

    for (index, building) in visible_buildings.iter().take(10).enumerate() {
        let y = 144.0 + index as f32 * 38.0;
        let other_summary = building_decision_summary(data, game_state, building);
        let (marker_kind, marker_color) = status_marker(
            &other_summary.status_label,
            &ui.available_label,
            &ui.built_out_label,
        );
        let pressed = if building.id == selected_building.id {
            primary_button(
                layout.left_margin + 12.0,
                y,
                layout.list_w - 52.0,
                30.0,
                &compact_text(&building.name, 28),
            )
        } else {
            secondary_button(
                layout.left_margin + 12.0,
                y,
                layout.list_w - 52.0,
                30.0,
                &compact_text(&building.name, 28),
            )
        };
        draw_status_marker(
            layout.left_margin + layout.list_w - 34.0,
            y + 6.0,
            18.0,
            marker_color,
            marker_kind,
        );
        if pressed {
            return Some(UiAction::SelectTownBuilding(building.id.clone()));
        }
    }

    let legend_y = (layout.footer_y - 76.0).min(560.0);
    draw_status_marker(
        layout.left_margin + 16.0,
        legend_y,
        16.0,
        theme::POSITIVE,
        0,
    );
    draw_body_text(
        &ui.available_label,
        layout.left_margin + 40.0,
        legend_y + 13.0,
        12.0,
        theme::TEXT_MUTED,
    );
    draw_status_marker(layout.left_margin + 114.0, legend_y, 16.0, theme::INFO, 1);
    draw_body_text(
        &ui.built_out_label,
        layout.left_margin + 138.0,
        legend_y + 13.0,
        12.0,
        theme::TEXT_MUTED,
    );
    draw_status_marker(
        layout.left_margin + 16.0,
        legend_y + 22.0,
        16.0,
        theme::WARNING,
        2,
    );
    draw_body_text(
        &ui.locked_by_cost_label,
        layout.left_margin + 40.0,
        legend_y + 35.0,
        12.0,
        theme::TEXT_MUTED,
    );

    draw_tier_panel(
        layout.detail_x,
        92.0,
        layout.detail_w,
        318.0,
        Some(&ui.selected_building_panel_title),
        PanelTier::Primary,
        true,
    );
    let detail_top_y = 138.0;
    draw_building_thumbnail(
        selected_building,
        layout.detail_x + 16.0,
        detail_top_y,
        220.0,
        176.0,
    );
    draw_body_text(
        &selected_building.name,
        layout.detail_x + 252.0,
        detail_top_y + 12.0,
        24.0,
        theme::TEXT_STRONG,
    );
    draw_inline_status(
        layout.detail_x + 252.0,
        detail_top_y + 28.0,
        188.0,
        &summary.status_label,
        summary.status_color,
    );
    draw_body_text_in_box(
        &compact_text(&selected_building.description, 138),
        layout.detail_x + 252.0,
        detail_top_y + 52.0,
        layout.detail_w - 268.0,
        40.0,
        15.0,
        theme::TEXT_BODY,
    );
    let is_built_out = summary.status_label == ui.built_out_label;
    if summary.next_destination != data.ui_text.common.return_to_town_button {
        draw_badge(
            layout.detail_x + 252.0,
            detail_top_y + 98.0,
            220.0,
            24.0,
            &format!("Unlock route: {}", summary.next_destination),
            theme::WARNING,
        );
    }

    if !is_built_out
        && primary_button(
            layout.detail_x + layout.detail_w - 192.0,
            detail_top_y + 96.0,
            168.0,
            28.0,
            &ui.build_selected_button,
        )
    {
        return Some(UiAction::PurchaseBuilding(selected_building.id.clone()));
    }
    draw_metric_tile(
        layout.detail_x + 252.0,
        detail_top_y + 134.0,
        96.0,
        54.0,
        &ui.built_count_label,
        &format!("{}/{}", summary.build_count, selected_building.build_limit),
        theme::PRIMARY,
    );
    draw_metric_tile(
        layout.detail_x + 356.0,
        detail_top_y + 134.0,
        308.0,
        54.0,
        &ui.cost_panel_title,
        &compact_text(
            &format_resource_cost(&data.ui_text, &selected_building.cost),
            46,
        ),
        if summary.can_afford {
            theme::POSITIVE
        } else {
            theme::WARNING
        },
    );
    draw_metric_tile(
        layout.detail_x + 672.0,
        detail_top_y + 134.0,
        112.0,
        54.0,
        &ui.category_label,
        &selected_building.category,
        theme::INFO,
    );

    let effects_y = 424.0;
    let effects_h = (layout.footer_y - effects_y - layout::SECTION_GAP).max(168.0);
    draw_tier_panel(
        layout.detail_x,
        effects_y,
        layout.detail_w,
        effects_h,
        Some(&ui.effects_panel_title),
        PanelTier::Support,
        false,
    );
    let effect_w = ((layout.detail_w - 48.0) * 0.48).max(280.0);
    for (index, effect) in summary.effect_lines.iter().take(5).enumerate() {
        draw_badge(
            layout.detail_x + 16.0,
            466.0 + index as f32 * 30.0,
            effect_w,
            22.0,
            &compact_text(effect, 58),
            theme::INFO,
        );
    }
    let unlock_x = layout.detail_x + effect_w + 32.0;
    let has_unlocks = !summary.unlock_labels.iter().any(|unlock| {
        unlock.trim() == data.ui_text.common.none_label && summary.unlock_labels.len() == 1
    });
    let visible_unlocks = summary
        .unlock_labels
        .iter()
        .filter(|unlock| unlock.trim() != data.ui_text.common.none_label)
        .take(3)
        .collect::<Vec<_>>();
    if has_unlocks && !visible_unlocks.is_empty() {
        draw_body_text(
            &ui.progression_panel_title,
            unlock_x,
            456.0,
            15.0,
            theme::TEXT_MUTED,
        );
    }
    for (index, unlock) in visible_unlocks.iter().enumerate() {
        draw_badge(
            unlock_x,
            466.0 + index as f32 * 28.0,
            layout.detail_w - effect_w - 48.0,
            22.0,
            &compact_text(unlock, 68),
            theme::WARNING,
        );
    }
    let metric_x = unlock_x;
    draw_metric_tile(
        metric_x,
        548.0,
        96.0,
        50.0,
        &ui.built_label,
        &game_state.town.constructed_building_ids.len().to_string(),
        theme::PRIMARY,
    );
    draw_metric_tile(
        metric_x + 104.0,
        548.0,
        96.0,
        50.0,
        &ui.rooms_label,
        &game_state.town.unlocked_room_ids.len().to_string(),
        theme::POSITIVE,
    );
    draw_metric_tile(
        metric_x + 208.0,
        548.0,
        96.0,
        50.0,
        &ui.floors_label,
        &game_state.town.unlocked_floor_ids.len().to_string(),
        theme::WARNING,
    );
    draw_metric_tile(
        metric_x + 312.0,
        548.0,
        96.0,
        50.0,
        &ui.species_label,
        &game_state.town.unlocked_species_ids.len().to_string(),
        theme::INFO,
    );

    let _status_message = &town_state.status_message;
    if let Some(error_message) = last_error {
        draw_inline_error(
            layout.detail_x + 260.0,
            338.0,
            layout.detail_w - 276.0,
            error_message,
        );
    }

    draw_standard_gameplay_footer(
        data,
        layout.left_margin,
        layout.footer_y,
        layout.content_width,
        Some(UiAction::OpenTownManagement),
    )
}
