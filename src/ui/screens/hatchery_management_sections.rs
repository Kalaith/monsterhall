use macroquad::prelude::{screen_height, screen_width};

use crate::data::{GameData, ResourceAmountData};
use crate::engine::effective_population_cap;
use crate::state::{
    CompanionState, EggConversionKind, EggIncubationState, EggState, GameState,
    HatcheryManagementState,
};
use crate::ui::actions::UiAction;
use crate::ui::art::draw_egg_thumbnail;
use crate::ui::chrome::{
    draw_inline_status, draw_screen_header, draw_standard_gameplay_footer, draw_tier_panel,
    draw_top_utility_bar, PanelTier,
};
use crate::ui::components::{
    draw_badge, draw_empty_state, draw_entity_card_frame, draw_metric_tile,
};
use crate::ui::core::{draw_body_text, draw_body_text_in_box, primary_button, secondary_button};
use crate::ui::feedback::draw_inline_error;
use crate::ui::layout;
use crate::ui::theme;
use crate::ui::view_models::{
    egg_grade_label, egg_origin_summary, egg_outcome_count_label, egg_outcome_preview_label,
    egg_quality_label, egg_quality_rank, format_resource_cost, known_species_name_by_id,
    monster_quality_label, species_name_by_id,
};

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

pub(super) struct HatcheryManagementLayout {
    pub left_margin: f32,
    pub content_width: f32,
    pub inventory_w: f32,
    pub detail_x: f32,
    pub detail_w: f32,
    pub content_y: f32,
    pub content_h: f32,
    pub footer_y: f32,
}

impl HatcheryManagementLayout {
    pub(super) fn new() -> Self {
        let left_margin = layout::OUTER_MARGIN;
        let content_width = screen_width() - left_margin * 2.0;
        let inventory_w = 348.0;
        let detail_x = left_margin + inventory_w + layout::SECTION_GAP;
        let detail_w = content_width - inventory_w - layout::SECTION_GAP;
        let content_y = 176.0;
        let footer_y = screen_height() - layout::FOOTER_BOTTOM_MARGIN - layout::FOOTER_H;
        let content_h = (footer_y - content_y - layout::SECTION_GAP).max(420.0);

        Self {
            left_margin,
            content_width,
            inventory_w,
            detail_x,
            detail_w,
            content_y,
            content_h,
            footer_y,
        }
    }
}

pub(super) fn draw_header(data: &GameData) -> Option<UiAction> {
    if let Some(action) = draw_top_utility_bar(&data.ui_text.common.settings_button) {
        return Some(action);
    }
    draw_screen_header(
        &data.ui_text.hatchery_management.title,
        &data.ui_text.hatchery_management.subtitle,
    );
    None
}

pub(super) fn draw_status_panel(
    data: &GameData,
    chamber_state: &HatcheryManagementState,
    game_state: &GameState,
    layout: &HatcheryManagementLayout,
) {
    let chamber_text = &data.ui_text.hatchery_management;
    draw_tier_panel(
        layout.left_margin,
        92.0,
        layout.content_width,
        72.0,
        Some(&chamber_text.status_panel_title),
        PanelTier::Support,
        false,
    );
    let _status_message = &chamber_state.status_message;

    draw_metric_tile(
        layout.left_margin + layout::PANEL_PADDING,
        112.0,
        96.0,
        44.0,
        &data.ui_text.common.eggs_unit,
        &game_state.egg_inventory.len().to_string(),
        theme::PRIMARY,
    );

    draw_inline_status(
        layout.left_margin + 128.0,
        126.0,
        layout.content_width - 144.0,
        &format!(
            "{}: {}",
            chamber_text.sources_label,
            egg_origin_summary(game_state, data)
        ),
        theme::WARNING,
    );
}

pub(super) fn selected_egg<'a>(
    chamber_state: &HatcheryManagementState,
    game_state: &'a GameState,
) -> Option<&'a EggState> {
    chamber_state
        .selected_egg_id
        .as_ref()
        .and_then(|egg_id| {
            game_state
                .egg_inventory
                .iter()
                .find(|egg| &egg.id == egg_id)
        })
        .or_else(|| game_state.egg_inventory.first())
}

fn unlocked_species_ids_for_egg(game_state: &GameState, egg: &EggState) -> Vec<String> {
    egg.possible_species_ids
        .iter()
        .filter(|species_id| game_state.town.unlocked_species_ids.contains(*species_id))
        .cloned()
        .collect::<Vec<_>>()
}

fn is_roster_at_cap(data: &GameData, game_state: &GameState) -> bool {
    game_state.monsters.len() >= effective_population_cap(data, game_state)
}

fn species_count(game_state: &GameState, species_id: &str) -> usize {
    game_state
        .monsters
        .iter()
        .filter(|monster| monster.species_id == species_id)
        .count()
}

fn replacement_score(monster: &CompanionState) -> u32 {
    monster.quality_rank as u32 * 100
        + monster.skills.scouting
        + monster.skills.guarding
        + monster.skills.hospitality
        + monster.skills.crafting
        + monster.skills.charm
        + monster.stats.charm.max(0) as u32
}

fn recommended_replacement<'a>(
    game_state: &'a GameState,
    species_id: &str,
    new_quality_rank: u8,
) -> Option<&'a CompanionState> {
    game_state.monsters.iter().min_by_key(|monster| {
        let same_species_upgrade =
            monster.species_id == species_id && monster.quality_rank < new_quality_rank;
        let duplicate_type = species_count(game_state, &monster.species_id) > 1;
        let priority = if same_species_upgrade {
            0
        } else if duplicate_type {
            1
        } else {
            2
        };
        (priority, replacement_score(monster))
    })
}

pub(super) fn draw_inventory_panel(
    data: &GameData,
    chamber_state: &HatcheryManagementState,
    game_state: &GameState,
    layout: &HatcheryManagementLayout,
) -> Option<UiAction> {
    let chamber_text = &data.ui_text.hatchery_management;
    draw_tier_panel(
        layout.left_margin,
        layout.content_y,
        layout.inventory_w,
        layout.content_h,
        Some(&chamber_text.inventory_panel_title),
        PanelTier::Support,
        false,
    );

    if game_state.egg_inventory.is_empty() {
        draw_empty_state(
            layout.left_margin + 8.0,
            layout.content_y + 42.0,
            layout.inventory_w - 16.0,
            136.0,
            &chamber_text.inventory_empty_title,
            &chamber_text.no_eggs_message,
        );
        return None;
    }

    let row_height = 92.0;
    let visible_rows = 4usize;
    let max_scroll = game_state.egg_inventory.len().saturating_sub(visible_rows);
    let start_index = chamber_state.inventory_scroll.min(max_scroll);
    let current_selected = selected_egg(chamber_state, game_state).map(|egg| egg.id.as_str());

    for (visible_index, egg) in game_state
        .egg_inventory
        .iter()
        .skip(start_index)
        .take(visible_rows)
        .enumerate()
    {
        let row_y = layout.content_y + 42.0 + visible_index as f32 * row_height;
        let unlocked_species_ids = unlocked_species_ids_for_egg(game_state, egg);
        let is_selected = current_selected == Some(egg.id.as_str());
        draw_entity_card_frame(
            layout.left_margin + 12.0,
            row_y,
            layout.inventory_w - 24.0,
            78.0,
            if egg.incubation_state == EggIncubationState::ReadyToHatch {
                theme::POSITIVE
            } else {
                theme::PRIMARY
            },
            is_selected,
            false,
        );
        draw_egg_thumbnail(egg, layout.left_margin + 24.0, row_y + 10.0, 70.0, 58.0);
        draw_body_text(
            &egg.id,
            layout.left_margin + 108.0,
            row_y + 20.0,
            16.0,
            theme::TEXT_STRONG,
        );
        draw_body_text(
            &egg_grade_label(egg, data),
            layout.left_margin + 108.0,
            row_y + 40.0,
            13.0,
            theme::TEXT_BODY,
        );
        let preview_text = if egg.incubation_state == EggIncubationState::ReadyToHatch {
            format!(
                "{}: {}",
                chamber_text.prepared_outcome_label,
                egg_outcome_preview_label(egg, game_state, data)
            )
        } else if unlocked_species_ids.len() <= 1 {
            egg_outcome_preview_label(egg, game_state, data)
        } else {
            chamber_text.review_required_message.clone()
        };
        draw_body_text_in_box(
            &preview_text,
            layout.left_margin + 108.0,
            row_y + 42.0,
            layout.inventory_w - 238.0,
            22.0,
            12.0,
            theme::TEXT_MUTED,
        );
        if !is_selected
            && secondary_button(
                layout.left_margin + layout.inventory_w - 118.0,
                row_y + 24.0,
                94.0,
                24.0,
                &chamber_text.select_button,
            )
        {
            return Some(UiAction::SelectChamberEgg(egg.id.clone()));
        }
    }

    if start_index > 0 {
        draw_body_text(
            &chamber_text.scroll_up_message,
            layout.left_margin + 16.0,
            layout.content_y + 30.0,
            13.0,
            theme::TEXT_MUTED,
        );
    }
    if start_index < max_scroll {
        draw_body_text(
            &chamber_text.scroll_down_message,
            layout.left_margin + 16.0,
            layout.content_y + layout.content_h - 20.0,
            13.0,
            theme::TEXT_MUTED,
        );
    }

    None
}

pub(super) fn draw_selected_egg_panel(
    data: &GameData,
    game_state: &GameState,
    selected_egg: Option<&EggState>,
    last_error: Option<&str>,
    layout: &HatcheryManagementLayout,
) -> Option<UiAction> {
    let chamber_text = &data.ui_text.hatchery_management;
    draw_tier_panel(
        layout.detail_x,
        layout.content_y,
        layout.detail_w,
        layout.content_h,
        Some(&chamber_text.selected_egg_panel_title),
        PanelTier::Primary,
        true,
    );

    let Some(egg) = selected_egg else {
        draw_empty_state(
            layout.detail_x + 8.0,
            layout.content_y + 42.0,
            layout.detail_w - 16.0,
            140.0,
            &chamber_text.no_selected_egg_title,
            &chamber_text.no_selected_egg_message,
        );
        return None;
    };

    let unlocked_species_ids = unlocked_species_ids_for_egg(game_state, egg);
    let preview = egg_outcome_preview_label(egg, game_state, data);
    let top_section_y = layout.content_y + 56.0;
    draw_egg_thumbnail(egg, layout.detail_x + 18.0, top_section_y, 140.0, 112.0);
    draw_body_text(
        &egg.id,
        layout.detail_x + 176.0,
        top_section_y + 18.0,
        24.0,
        theme::TEXT_STRONG,
    );
    draw_inline_status(
        layout.detail_x + 176.0,
        top_section_y + 30.0,
        230.0,
        if egg.incubation_state == EggIncubationState::ReadyToHatch {
            &chamber_text.prepared_outcome_label
        } else {
            &chamber_text.locked_outcome_label
        },
        if egg.incubation_state == EggIncubationState::ReadyToHatch {
            theme::POSITIVE
        } else {
            theme::WARNING
        },
    );
    draw_body_text_in_box(
        &preview,
        layout.detail_x + 176.0,
        top_section_y + 72.0,
        layout.detail_w - 450.0,
        34.0,
        14.0,
        theme::TEXT_BODY,
    );

    draw_badge(
        layout.detail_x + 176.0,
        top_section_y + 112.0,
        142.0,
        24.0,
        &egg_grade_label(egg, data),
        theme::PRIMARY,
    );
    draw_badge(
        layout.detail_x + 326.0,
        top_section_y + 112.0,
        196.0,
        24.0,
        &egg_outcome_count_label(data, egg),
        theme::INFO,
    );
    draw_badge(
        layout.detail_x + 530.0,
        top_section_y + 112.0,
        116.0,
        24.0,
        &egg_quality_label(egg),
        theme::WARNING,
    );

    draw_body_text(
        &chamber_text.possible_outcomes_heading,
        layout.detail_x + 18.0,
        top_section_y + 148.0,
        18.0,
        theme::TEXT_STRONG,
    );

    if unlocked_species_ids.len() <= 1 {
        let row_y = top_section_y + 174.0;
        let hatch_target = unlocked_species_ids.first().cloned();
        if let Some(species_id) = hatch_target.as_ref() {
            let result_w = (layout.detail_w - 24.0).min(684.0);
            let known_name = known_species_name_by_id(data, game_state, species_id);
            let species = data
                .species
                .species
                .iter()
                .find(|entry| &entry.id == species_id);
            let detail = species
                .map(|entry| compact_text(&entry.description, 120))
                .unwrap_or_else(|| chamber_text.unknown_outcome_message.clone());
            let action_cost = species
                .map(|entry| ResourceAmountData {
                    gold: entry.hatching_cost.gold,
                    tower_materials: entry.hatching_cost.tower_materials,
                    eggs: 0,
                    relics: entry.hatching_cost.relics,
                    arcane_residue: entry.hatching_cost.arcane_residue,
                })
                .unwrap_or_default();
            let cost_text = format_resource_cost(&data.ui_text, &action_cost);

            draw_entity_card_frame(
                layout.detail_x + 12.0,
                row_y,
                result_w,
                96.0,
                if egg.incubation_state == EggIncubationState::ReadyToHatch {
                    theme::POSITIVE
                } else {
                    theme::PRIMARY
                },
                true,
                false,
            );
            draw_body_text(
                &known_name,
                layout.detail_x + 24.0,
                row_y + 24.0,
                20.0,
                theme::TEXT_STRONG,
            );
            draw_body_text_in_box(
                &detail,
                layout.detail_x + 24.0,
                row_y + 32.0,
                result_w - 226.0,
                28.0,
                12.0,
                theme::TEXT_MUTED,
            );
            if egg.incubation_state == EggIncubationState::ReadyToHatch {
                draw_inline_status(
                    layout.detail_x + 24.0,
                    row_y + 66.0,
                    result_w - 226.0,
                    &chamber_text.bound_message,
                    theme::POSITIVE,
                );
            }
            let hatch_label = if egg.incubation_state == EggIncubationState::ReadyToHatch {
                chamber_text.hatch_button.clone()
            } else {
                format!("{} ({})", chamber_text.hatch_button, cost_text)
            };
            let at_cap = is_roster_at_cap(data, game_state);
            let replacement = if at_cap {
                recommended_replacement(game_state, species_id, egg_quality_rank(egg))
            } else {
                None
            };
            if let Some(replacement) = replacement {
                draw_inline_status(
                    layout.detail_x + 24.0,
                    row_y + 66.0,
                    result_w - 226.0,
                    &format!(
                        "At cap: replaces {} ({}, {})",
                        replacement.name,
                        species_name_by_id(data, &replacement.species_id),
                        monster_quality_label(replacement)
                    ),
                    theme::WARNING,
                );
            }
            if primary_button(
                layout.detail_x + 12.0 + result_w - 182.0,
                row_y + 34.0,
                170.0,
                28.0,
                if at_cap { "Replace" } else { &hatch_label },
            ) {
                if let Some(replacement) = replacement {
                    return Some(UiAction::ReplaceMonsterWithEgg(
                        egg.id.clone(),
                        hatch_target,
                        replacement.id.clone(),
                    ));
                }
                return Some(UiAction::HatchSelectedEgg(egg.id.clone(), hatch_target));
            }
        }
    } else {
        let list_top = top_section_y + 168.0;
        let list_bottom = if last_error.is_some() {
            layout.content_y + layout.content_h - 96.0
        } else {
            layout.content_y + layout.content_h - 64.0
        };
        let row_count = unlocked_species_ids.len().max(1) as f32;
        let row_gap = 8.0;
        let row_height = (((list_bottom - list_top) - row_gap * (row_count - 1.0)) / row_count)
            .floor()
            .clamp(40.0, 58.0);
        for (index, species_id) in unlocked_species_ids.iter().enumerate() {
            let Some(species) = data
                .species
                .species
                .iter()
                .find(|entry| entry.id == *species_id)
            else {
                continue;
            };
            let row_y = list_top + index as f32 * (row_height + row_gap);
            let known_name = known_species_name_by_id(data, game_state, species_id);
            let detail = if known_name == data.ui_text.common.unknown_label {
                chamber_text.unknown_outcome_message.clone()
            } else {
                compact_text(
                    &species.description,
                    if row_height <= 46.0 { 56 } else { 92 },
                )
            };
            let action_cost = if egg.incubation_state == EggIncubationState::ReadyToHatch {
                ResourceAmountData::default()
            } else {
                ResourceAmountData {
                    gold: species.hatching_cost.gold,
                    tower_materials: species.hatching_cost.tower_materials,
                    eggs: 0,
                    relics: species.hatching_cost.relics,
                    arcane_residue: species.hatching_cost.arcane_residue,
                }
            };
            let cost_text = format_resource_cost(&data.ui_text, &action_cost);
            let info_line = detail;
            let button_h = if row_height <= 46.0 { 22.0 } else { 24.0 };

            draw_entity_card_frame(
                layout.detail_x + 12.0,
                row_y,
                layout.detail_w - 24.0,
                row_height,
                theme::PRIMARY,
                egg.selected_species_id.as_ref() == Some(species_id),
                false,
            );
            draw_body_text(
                &known_name,
                layout.detail_x + 24.0,
                row_y + 18.0,
                17.0,
                theme::TEXT_STRONG,
            );
            draw_body_text_in_box(
                &info_line,
                layout.detail_x + 24.0,
                row_y + 22.0,
                layout.detail_w - 238.0,
                (row_height - 18.0).max(16.0),
                11.0,
                theme::TEXT_MUTED,
            );
            let hatch_label = if egg.incubation_state == EggIncubationState::ReadyToHatch {
                chamber_text.hatch_button.clone()
            } else {
                format!("{} ({})", chamber_text.hatch_button, cost_text)
            };
            let at_cap = is_roster_at_cap(data, game_state);
            let replacement = if at_cap {
                recommended_replacement(game_state, species_id, egg_quality_rank(egg))
            } else {
                None
            };
            let action_label = replacement
                .map(|monster| format!("Replace {}", compact_text(&monster.name, 10)))
                .unwrap_or(hatch_label);
            if primary_button(
                layout.detail_x + layout.detail_w - 194.0,
                row_y + (row_height - button_h) * 0.5,
                170.0,
                button_h,
                &action_label,
            ) {
                if let Some(replacement) = replacement {
                    return Some(UiAction::ReplaceMonsterWithEgg(
                        egg.id.clone(),
                        Some(species.id.clone()),
                        replacement.id.clone(),
                    ));
                }
                return Some(UiAction::HatchSelectedEgg(
                    egg.id.clone(),
                    Some(species.id.clone()),
                ));
            }
        }
    }

    if unlocked_species_ids.is_empty() {
        draw_empty_state(
            layout.detail_x + 8.0,
            top_section_y + 174.0,
            layout.detail_w - 16.0,
            120.0,
            &chamber_text.no_selected_egg_title,
            &chamber_text.no_eggs_message,
        );
    }

    let conversion_y = if last_error.is_some() {
        layout.content_y + layout.content_h - 64.0
    } else {
        layout.content_y + layout.content_h - 34.0
    };
    if secondary_button(layout.detail_x + 18.0, conversion_y, 96.0, 24.0, "Sell Egg") {
        return Some(UiAction::ConvertEgg(
            egg.id.clone(),
            EggConversionKind::Sell,
        ));
    }
    if secondary_button(
        layout.detail_x + 122.0,
        conversion_y,
        118.0,
        24.0,
        "Dissolve",
    ) {
        return Some(UiAction::ConvertEgg(
            egg.id.clone(),
            EggConversionKind::Dissolve,
        ));
    }
    if game_state.egg_inventory.len() >= 2
        && secondary_button(layout.detail_x + 248.0, conversion_y, 116.0, 24.0, "Refine")
    {
        return Some(UiAction::ConvertEgg(
            egg.id.clone(),
            EggConversionKind::Refine,
        ));
    }

    if let Some(error_message) = last_error {
        draw_inline_error(
            layout.detail_x + 18.0,
            layout.content_y + layout.content_h - 28.0,
            layout.detail_w - 36.0,
            error_message,
        );
    }

    None
}

pub(super) fn draw_footer(data: &GameData, layout: &HatcheryManagementLayout) -> Option<UiAction> {
    draw_standard_gameplay_footer(
        data,
        layout.left_margin,
        layout.footer_y,
        layout.content_width,
        Some(UiAction::OpenHatcheryManagement),
    )
}
