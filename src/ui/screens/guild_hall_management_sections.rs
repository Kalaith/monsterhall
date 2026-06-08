use macroquad::prelude::{screen_height, screen_width};

use crate::data::GameData;
use crate::engine::preview_guild_job;
use crate::state::{CompanionJobState, GameState, GuildHallManagementState};
use crate::ui::actions::UiAction;
use crate::ui::art::draw_room_thumbnail;
use crate::ui::chrome::{
    draw_inline_status, draw_screen_header, draw_standard_gameplay_footer, draw_tier_panel,
    draw_top_utility_bar, PanelTier,
};
use crate::ui::components::{draw_badge, draw_character_card, draw_empty_state, CharacterCardSpec};
use crate::ui::core::{
    draw_body_text, draw_body_text_in_box, primary_button, secondary_button, utility_button,
};
use crate::ui::feedback::draw_inline_error;
use crate::ui::layout;
use crate::ui::theme;
use crate::ui::view_models::{
    assignment_label, history_gain_label, history_gain_label_from_progress, primary_skill_label,
    species_name_by_id, trained_skills_label, worker_decision_summary,
};

pub(super) struct GuildHallManagementLayout {
    pub left_margin: f32,
    pub content_width: f32,
    pub room_panel_y: f32,
    pub room_panel_w: f32,
    pub detail_x: f32,
    pub detail_w: f32,
    pub roster_y: f32,
    pub footer_y: f32,
}

impl GuildHallManagementLayout {
    pub(super) fn new() -> Self {
        let left_margin = layout::OUTER_MARGIN;
        let content_width = screen_width() - left_margin * 2.0;
        let room_panel_y = 92.0;
        let room_panel_w = 284.0;
        let detail_x = left_margin + room_panel_w + layout::SECTION_GAP;
        let detail_w = content_width - room_panel_w - layout::SECTION_GAP;
        let roster_y = 304.0;
        let footer_y = screen_height() - layout::FOOTER_BOTTOM_MARGIN - layout::FOOTER_H;
        Self {
            left_margin,
            content_width,
            room_panel_y,
            room_panel_w,
            detail_x,
            detail_w,
            roster_y,
            footer_y,
        }
    }
}

fn compact_sentence(text: &str, max_len: usize) -> String {
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

pub(super) fn draw_header(
    data: &GameData,
    _layout: &GuildHallManagementLayout,
) -> Option<UiAction> {
    if let Some(action) = draw_top_utility_bar(&data.ui_text.common.settings_button) {
        return Some(action);
    }
    draw_screen_header(
        &data.ui_text.guild_hall_management.title,
        &data.ui_text.guild_hall_management.subtitle,
    );
    None
}

pub(super) fn selected_room<'a>(
    data: &'a GameData,
    guild_jobs_state: &GuildHallManagementState,
    game_state: &'a GameState,
) -> (
    Vec<&'a crate::data::GuildRoomData>,
    Option<&'a crate::data::GuildRoomData>,
) {
    let available_rooms = data
        .guild_rooms
        .rooms
        .iter()
        .filter(|room| game_state.town.unlocked_room_ids.contains(&room.id))
        .collect::<Vec<_>>();

    let selected_room = available_rooms
        .iter()
        .find(|room| room.id == guild_jobs_state.selected_room_id)
        .copied()
        .or_else(|| available_rooms.first().copied());

    (available_rooms, selected_room)
}

pub(super) fn draw_rooms_panel(
    data: &GameData,
    available_rooms: &[&crate::data::GuildRoomData],
    selected_room: Option<&crate::data::GuildRoomData>,
    layout: &GuildHallManagementLayout,
) -> Option<UiAction> {
    draw_tier_panel(
        layout.left_margin,
        layout.room_panel_y,
        layout.room_panel_w,
        196.0,
        Some(&data.ui_text.guild_hall_management.rooms_panel_title),
        PanelTier::Support,
        false,
    );
    let Some(selected_room) = selected_room else {
        draw_empty_state(
            layout.left_margin + 8.0,
            layout.room_panel_y + 40.0,
            layout.room_panel_w - 16.0,
            136.0,
            &data.ui_text.guild_hall_management.no_room_selected_title,
            &data.ui_text.guild_hall_management.no_rooms_message,
        );
        return None;
    };

    for (index, room) in available_rooms.iter().enumerate() {
        let y = layout.room_panel_y + 42.0 + index as f32 * 40.0;
        let is_selected = room.id == selected_room.id;
        let pressed = if is_selected {
            primary_button(
                layout.left_margin + 14.0,
                y,
                layout.room_panel_w - 28.0,
                32.0,
                &room.name,
            )
        } else {
            secondary_button(
                layout.left_margin + 14.0,
                y,
                layout.room_panel_w - 28.0,
                32.0,
                &room.name,
            )
        };
        if pressed {
            return Some(UiAction::SelectGuildRoom(room.id.clone()));
        }
    }
    None
}

pub(super) fn draw_selected_room_panel(
    data: &GameData,
    guild_jobs_state: &GuildHallManagementState,
    game_state: &GameState,
    selected_room: Option<&crate::data::GuildRoomData>,
    layout: &GuildHallManagementLayout,
) {
    let Some(selected_room) = selected_room else {
        return;
    };
    draw_tier_panel(
        layout.detail_x,
        layout.room_panel_y,
        layout.detail_w,
        196.0,
        Some(&data.ui_text.guild_hall_management.selected_room_panel_title),
        PanelTier::Primary,
        true,
    );
    draw_room_thumbnail(
        selected_room,
        layout.detail_x + 16.0,
        layout.room_panel_y + 18.0,
        232.0,
        156.0,
    );

    let assigned_workers = game_state
        .monsters
        .iter()
        .filter(|monster| matches!(&monster.current_job, CompanionJobState::GuildJob { room_id } if room_id == &selected_room.id))
        .collect::<Vec<_>>();
    let projected_gold = assigned_workers
        .iter()
        .filter_map(|monster| preview_guild_job(data, game_state, monster, &selected_room.id).ok())
        .map(|preview| preview.projected_gold)
        .sum::<u32>();
    let projected_residue = assigned_workers
        .iter()
        .filter_map(|monster| preview_guild_job(data, game_state, monster, &selected_room.id).ok())
        .map(|preview| preview.projected_arcane_residue)
        .sum::<u32>();
    let projected_materials = assigned_workers
        .iter()
        .filter_map(|monster| preview_guild_job(data, game_state, monster, &selected_room.id).ok())
        .map(|preview| preview.projected_materials)
        .sum::<u32>();
    let projected_prep = assigned_workers
        .iter()
        .filter_map(|monster| preview_guild_job(data, game_state, monster, &selected_room.id).ok())
        .map(|preview| preview.preparation_quality)
        .sum::<u32>();

    draw_body_text(
        &selected_room.name,
        layout.detail_x + 266.0,
        layout.room_panel_y + 40.0,
        26.0,
        theme::TEXT_STRONG,
    );
    draw_inline_status(
        layout.detail_x + 266.0,
        layout.room_panel_y + 52.0,
        layout.detail_w - 286.0,
        &format!(
            "{}: {}",
            data.ui_text.guild_hall_management.status_label, guild_jobs_state.status_message
        ),
        theme::PRIMARY,
    );
    draw_body_text_in_box(
        &compact_sentence(&selected_room.description, 150),
        layout.detail_x + 266.0,
        layout.room_panel_y + 84.0,
        layout.detail_w - 286.0,
        36.0,
        15.0,
        theme::TEXT_BODY,
    );
    let chip_y = layout.room_panel_y + 142.0;
    let chip_gap = 8.0;
    draw_badge(
        layout.detail_x + 266.0,
        chip_y,
        78.0,
        24.0,
        &format!("Tier {}", selected_room.service_tier),
        theme::PRIMARY,
    );
    draw_badge(
        layout.detail_x + 266.0 + 78.0 + chip_gap,
        chip_y,
        168.0,
        24.0,
        &format!(
            "Trains {}",
            compact_sentence(
                &trained_skills_label(data, &selected_room.trained_skill_ids),
                20
            )
        ),
        theme::INFO,
    );
    draw_badge(
        layout.detail_x + 266.0 + 254.0,
        chip_y,
        168.0,
        24.0,
        &format!(
            "History {}",
            compact_sentence(
                &history_gain_label_from_progress(data, &selected_room.work_history_gains),
                20
            )
        ),
        theme::WARNING,
    );
    draw_badge(
        layout.detail_x + 266.0,
        chip_y + 30.0,
        (layout.detail_w - 286.0).min(432.0),
        24.0,
        &if assigned_workers.is_empty() {
            "No workers assigned".to_owned()
        } else {
            format!(
                "Projected {} gold / {} residue | Focus {}",
                projected_gold,
                projected_residue,
                primary_skill_label(data, &selected_room.trained_skill_ids)
            )
        },
        if assigned_workers.is_empty() {
            theme::WARNING
        } else {
            theme::POSITIVE
        },
    );
    draw_badge(
        layout.detail_x + 266.0,
        chip_y + 60.0,
        (layout.detail_w - 286.0).min(432.0),
        24.0,
        &format!(
            "{} {} | {} {} | {} {}",
            data.ui_text.guild_hall_management.materials_label,
            projected_materials,
            data.ui_text.guild_hall_management.preparation_quality_label,
            projected_prep,
            data.ui_text.guild_hall_management.room_job_kind_label,
            selected_room.job_kind
        ),
        theme::INFO,
    );
}

fn draw_worker_cards(
    data: &GameData,
    game_state: &GameState,
    selected_room: &crate::data::GuildRoomData,
    workers: &[&crate::state::CompanionState],
    x: f32,
    y: f32,
    w: f32,
    title: &str,
    collapse_empty: bool,
) -> Option<UiAction> {
    let rows = ((workers.len().max(1) + 1) / 2) as f32;
    let panel_h = if workers.is_empty() {
        if collapse_empty {
            118.0
        } else {
            136.0
        }
    } else {
        (52.0 + rows * 104.0).min(330.0)
    };
    draw_tier_panel(x, y, w, panel_h, Some(title), PanelTier::Support, false);
    if workers.is_empty() {
        draw_empty_state(
            x + 8.0,
            y + 40.0,
            w - 16.0,
            if collapse_empty { 62.0 } else { 120.0 },
            &data.ui_text.guild_hall_management.empty_bucket_title,
            &data.ui_text.guild_hall_management.empty_bucket_detail,
        );
        return None;
    }

    let card_w = (w - layout::PANEL_PADDING * 2.0 - layout::SECTION_GAP) / 2.0;
    for (index, monster) in workers.iter().enumerate() {
        let col = index % 2;
        let row = index / 2;
        let card_x = x + layout::PANEL_PADDING + col as f32 * (card_w + layout::SECTION_GAP);
        let card_y = y + 42.0 + row as f32 * 104.0;
        let preview = preview_guild_job(data, game_state, monster, &selected_room.id).ok();
        let prediction = preview
            .as_ref()
            .map(|value| {
                format!(
                    "{} gold / {} mat / {} residue | Prep {} | Score {} | {}",
                    value.projected_gold,
                    value.projected_materials,
                    value.projected_arcane_residue,
                    value.preparation_quality,
                    value.success_score,
                    history_gain_label(data, &value.projected_work_history_gains)
                )
            })
            .unwrap_or_else(|| {
                data.ui_text
                    .guild_hall_management
                    .no_preview_message
                    .clone()
            });
        let summary = worker_decision_summary(data, monster, prediction);

        let species_label = species_name_by_id(data, &monster.species_id);
        let card = draw_character_card(
            data,
            monster,
            card_x,
            card_y,
            card_w,
            92.0,
            CharacterCardSpec {
                name: &monster.name,
                species: &species_label,
                state: assignment_label(data, &monster.current_job),
                key_value: &summary.prediction_line,
                color: summary.highlight,
                state_color: summary.highlight,
                selected: false,
                disabled: false,
            },
        );

        let assigned_to_selected = matches!(&monster.current_job, CompanionJobState::GuildJob { room_id } if room_id == &selected_room.id);
        if !assigned_to_selected
            && primary_button(
                card.action_x,
                card.action_y,
                card.action_w,
                22.0,
                &data.ui_text.guild_hall_management.assign_button,
            )
        {
            return Some(UiAction::AssignMonsterToRoom(
                monster.id.clone(),
                selected_room.id.clone(),
            ));
        }
        let mut support_y = if assigned_to_selected {
            card.action_y
        } else {
            card.action_y + 24.0
        };
        if !matches!(monster.current_job, CompanionJobState::Resting)
            && secondary_button(
                card.action_x,
                support_y,
                card.action_w,
                22.0,
                &data.ui_text.guild_hall_management.rest_button,
            )
        {
            return Some(UiAction::AssignMonsterToRest(monster.id.clone()));
        }
        if matches!(monster.current_job, CompanionJobState::Resting) {
            support_y = if assigned_to_selected {
                card.action_y
            } else {
                card.action_y + 24.0
            };
        } else {
            support_y += 24.0;
        }
        if !matches!(monster.current_job, CompanionJobState::Idle)
            && utility_button(
                card.action_x,
                support_y,
                card.action_w,
                22.0,
                &data.ui_text.guild_hall_management.idle_button,
            )
        {
            return Some(UiAction::AssignMonsterToIdle(monster.id.clone()));
        }
    }
    None
}

pub(super) fn draw_worker_lists(
    data: &GameData,
    game_state: &GameState,
    selected_room: Option<&crate::data::GuildRoomData>,
    layout: &GuildHallManagementLayout,
) -> Option<UiAction> {
    let selected_room = selected_room?;
    let assigned = game_state
        .monsters
        .iter()
        .filter(|monster| matches!(&monster.current_job, CompanionJobState::GuildJob { room_id } if room_id == &selected_room.id))
        .collect::<Vec<_>>();
    let available = game_state
        .monsters
        .iter()
        .filter(|monster| !matches!(&monster.current_job, CompanionJobState::GuildJob { room_id } if room_id == &selected_room.id))
        .collect::<Vec<_>>();

    let column_w = (layout.content_width - layout::SECTION_GAP) / 2.0;
    if let Some(action) = draw_worker_cards(
        data,
        game_state,
        selected_room,
        &assigned,
        layout.left_margin,
        layout.roster_y,
        column_w,
        &data.ui_text.guild_hall_management.assigned_here_panel_title,
        true,
    ) {
        return Some(action);
    }
    draw_worker_cards(
        data,
        game_state,
        selected_room,
        &available,
        layout.left_margin + column_w + layout::SECTION_GAP,
        layout.roster_y,
        column_w,
        &data.ui_text.guild_hall_management.available_panel_title,
        false,
    )
}

pub(super) fn draw_footer(
    data: &GameData,
    layout: &GuildHallManagementLayout,
    last_error: Option<&str>,
) -> Option<UiAction> {
    if let Some(error_message) = last_error {
        draw_inline_error(
            layout.left_margin,
            layout.footer_y - 30.0,
            layout.content_width,
            error_message,
        );
    }
    draw_standard_gameplay_footer(
        data,
        layout.left_margin,
        layout.footer_y,
        layout.content_width,
        Some(UiAction::OpenGuildHallManagement),
    )
}
