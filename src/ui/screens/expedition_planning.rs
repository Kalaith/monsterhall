use macroquad::prelude::{screen_height, screen_width, Color};

use crate::data::GameData;
use crate::engine::preview_expedition_plan;
use crate::state::{CompanionJobState, ExpeditionPlanningState, ExpeditionPriority, GameState};
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_backdrop, draw_floor_preview, BackdropKind};
use crate::ui::chrome::{
    draw_inline_status, draw_screen_header, draw_standard_gameplay_footer, draw_tier_panel,
    draw_top_utility_bar, PanelTier,
};
use crate::ui::components::{
    draw_character_card, draw_empty_state, draw_metric_tile, CharacterCardSpec,
};
use crate::ui::core::{
    draw_body_text, draw_body_text_in_box, primary_button, secondary_button, utility_button,
};
use crate::ui::layout;
use crate::ui::screens::expedition_planning_sections::{
    draw_floor_list, floor_row_capacity, FloorListWindow, MissionGrid, MISSION_BUTTON_H,
};
use crate::ui::screens::roster_window::{
    draw_roster_pager, roster_panel_height, RosterWindow, ROSTER_COLUMNS,
};
use crate::ui::theme;
use crate::ui::view_models::{
    assignment_label, monster_depth_role_label, monster_quality_label, species_name_by_id,
};

/// Top of the first floor row, just below the floors panel title.
const FLOORS_FIRST_ROW_Y: f32 = 134.0;

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

fn expedition_prep_cost_label(mission: &crate::data::MissionData) -> String {
    let cost = &mission.prep_cost;
    let mut parts = Vec::new();
    if cost.gold > 0 {
        parts.push(format!("{}g", cost.gold));
    }
    if cost.tower_materials > 0 {
        parts.push(format!("{} materials", cost.tower_materials));
    }
    if cost.arcane_residue > 0 {
        parts.push(format!("{} residue", cost.arcane_residue));
    }
    if cost.relics > 0 {
        parts.push(format!("{} relics", cost.relics));
    }
    if parts.is_empty() {
        "Prep cost: none".to_owned()
    } else {
        format!("Prep cost: {}", parts.join(" / "))
    }
}

struct ExpeditionLayout {
    left_margin: f32,
    content_width: f32,
    floors_w: f32,
    detail_x: f32,
    detail_w: f32,
    detail_h: f32,
    team_y: f32,
    team_h: f32,
    footer_y: f32,
}

impl ExpeditionLayout {
    fn new() -> Self {
        let left_margin = layout::OUTER_MARGIN;
        let content_width = screen_width() - left_margin * 2.0;
        let floors_w = 278.0;
        let detail_x = left_margin + floors_w + layout::SECTION_GAP;
        let detail_w = content_width - floors_w - layout::SECTION_GAP;
        // Tall enough for the prep-cost caption under the metric row: tiles run
        // 328..378, caption baseline 394, panel closes at 92 + 306 = 398.
        let detail_h = 306.0;
        let team_y = 418.0;
        let footer_y = screen_height() - layout::FOOTER_BOTTOM_MARGIN - layout::FOOTER_H;
        let team_h = (footer_y - team_y - layout::SECTION_GAP).max(240.0);

        Self {
            left_margin,
            content_width,
            floors_w,
            detail_x,
            detail_w,
            detail_h,
            team_y,
            team_h,
            footer_y,
        }
    }
}

/// The score is how far past the injury threshold the most exposed companion
/// sits, so zero is not "safe enough" — it is the exact point where somebody
/// comes home hurt. Everything below it is margin.
fn risk_label(score: Option<i32>) -> &'static str {
    let Some(score) = score else {
        return "Risk: no party assigned";
    };
    if score >= 0 {
        "Risk: Injuries certain"
    } else if score >= -15 {
        "Risk: Narrow margin"
    } else {
        "Risk: Low"
    }
}

fn risk_color(score: Option<i32>) -> Color {
    let Some(score) = score else {
        return theme::TEXT_MUTED;
    };
    if score >= 0 {
        theme::DANGER
    } else if score >= -15 {
        theme::WARNING
    } else {
        theme::POSITIVE
    }
}

/// How dangerous this floor reads before anybody is assigned to it.
///
/// The fallback used to be `difficulty >= 5`, from when the tower was three
/// floors deep. Authored difficulty now runs 20 to 104 against a validated
/// ceiling of 120, so that test was true for every floor in the game and the
/// header of the Expedition Desk — which opens with no party every single time —
/// was permanently red. Banded against the authored ceiling instead, so a
/// shallow floor reads shallow.
fn floor_difficulty_color(data: &GameData, difficulty: u32) -> Color {
    let ceiling = data.config.day_cycle.max_floor_difficulty.max(1);
    if difficulty * 3 >= ceiling * 2 {
        theme::DANGER
    } else if difficulty * 3 >= ceiling {
        theme::WARNING
    } else {
        theme::POSITIVE
    }
}

/// What the party's condition costs this run.
///
/// The bands were 95 and 75, which are numbers this screen invented. The engine
/// authors the two that matter: anything under 100 is already taking output off
/// the run, and `min_effectiveness_pct` is the floor it cannot fall past — the
/// same calm / strained / spent rule the roster badges and the profile tiles
/// use for the meters underneath it.
fn party_condition_color(data: &GameData, effectiveness_pct: u32) -> Color {
    let floor = data
        .config
        .day_cycle
        .condition_effects
        .min_effectiveness_pct;
    if effectiveness_pct >= 100 {
        theme::POSITIVE
    } else if effectiveness_pct > floor {
        theme::WARNING
    } else {
        theme::DANGER
    }
}

pub fn draw_expedition_planning(
    data: &GameData,
    expedition_state: &ExpeditionPlanningState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    draw_backdrop(BackdropKind::Expedition);
    let layout = ExpeditionLayout::new();
    let expedition_text = &data.ui_text.expedition_planning;

    if let Some(action) = draw_top_utility_bar(&data.ui_text.common.settings_button) {
        return Some(action);
    }
    draw_screen_header(&expedition_text.title, &expedition_text.subtitle);

    let available_floors = data
        .floors
        .floors
        .iter()
        .filter(|floor| game_state.town.unlocked_floor_ids.contains(&floor.id))
        .collect::<Vec<_>>();

    draw_tier_panel(
        layout.left_margin,
        92.0,
        layout.floors_w,
        layout.detail_h,
        Some(&expedition_text.floors_panel_title),
        PanelTier::Support,
        false,
    );

    if available_floors.is_empty() {
        draw_empty_state(
            layout.left_margin + 8.0,
            132.0,
            layout.floors_w - 16.0,
            124.0,
            &expedition_text.no_floor_title,
            &expedition_text.no_floor_message,
        );
        return draw_standard_gameplay_footer(
            data,
            layout.left_margin,
            layout.footer_y,
            layout.content_width,
            Some(UiAction::OpenExpeditionPlanning),
        );
    }

    let selected_floor = available_floors
        .iter()
        .find(|floor| floor.id == expedition_state.selected_floor_id)
        .copied()
        .or_else(|| available_floors.first().copied())?;
    let selected_mission_id = selected_floor
        .mission_ids
        .iter()
        .find(|mission_id| **mission_id == expedition_state.selected_mission_id)
        .cloned()
        .unwrap_or_else(|| {
            selected_floor
                .mission_ids
                .first()
                .cloned()
                .unwrap_or_default()
        });
    let selected_mission = data
        .missions
        .missions
        .iter()
        .find(|mission| mission.id == selected_mission_id);

    let selected_index = available_floors
        .iter()
        .position(|floor| floor.id == selected_floor.id)
        .unwrap_or_default();
    let floor_window = FloorListWindow::new(
        available_floors.len(),
        selected_index,
        floor_row_capacity(92.0, layout.detail_h, FLOORS_FIRST_ROW_Y),
    );
    if let Some(action) = draw_floor_list(
        &available_floors,
        &selected_floor.id,
        floor_window,
        &expedition_text.floor_depth_template,
        layout.left_margin + 12.0,
        FLOORS_FIRST_ROW_Y,
        layout.floors_w - 24.0,
    ) {
        return Some(action);
    }

    let preview = preview_expedition_plan(
        data,
        game_state,
        &selected_floor.id,
        &selected_mission_id,
        &expedition_state.priority,
    )
    .ok();

    draw_tier_panel(
        layout.detail_x,
        92.0,
        layout.detail_w,
        layout.detail_h,
        Some(&expedition_text.floor_details_panel_title),
        PanelTier::Primary,
        true,
    );
    draw_floor_preview(selected_floor, layout.detail_x + 16.0, 136.0, 220.0, 112.0);
    draw_body_text(
        &selected_floor.name,
        layout.detail_x + 252.0,
        146.0,
        24.0,
        theme::TEXT_STRONG,
    );
    let assigned_count = game_state
        .monsters
        .iter()
        .filter(|monster| matches!(monster.current_job, CompanionJobState::OnExpedition { .. }))
        .count();
    // With nobody assigned the risk clause only restates the count, and it is
    // long enough that the status strip truncated it away mid-word, leaving the
    // header ending on a bare separator.
    let plan_status = preview
        .as_ref()
        .and_then(|preview| preview.injury_risk_score)
        .map_or_else(
            || format!("{assigned_count} assigned"),
            |score| format!("{} assigned | {}", assigned_count, risk_label(Some(score))),
        );
    draw_inline_status(
        layout.detail_x + 252.0,
        160.0,
        (layout.detail_w - 268.0).min(360.0),
        &format!(
            "{} | {} {} | {}",
            expedition_state.status_message,
            expedition_text.difficulty_label,
            selected_floor.difficulty,
            plan_status
        ),
        preview
            .as_ref()
            .map(|preview| risk_color(preview.injury_risk_score))
            .unwrap_or_else(|| floor_difficulty_color(data, selected_floor.difficulty)),
    );
    draw_body_text_in_box(
        &compact_text(&selected_floor.description, 160),
        layout.detail_x + 252.0,
        190.0,
        layout.detail_w - 268.0,
        34.0,
        15.0,
        theme::TEXT_BODY,
    );
    let control_x = layout.detail_x + 252.0;
    let control_y = 236.0;
    let control_w = layout.detail_w - 268.0;
    let control_gap = 12.0;
    let mission_panel_w = 292.0_f32.min(control_w * 0.42);
    let priority_panel_x = control_x + mission_panel_w + control_gap;
    let priority_panel_w = (control_w - mission_panel_w - control_gap).max(240.0);
    let mission_inner_w = mission_panel_w - 24.0;
    let mission_grid = MissionGrid::new(selected_floor.mission_ids.len(), mission_inner_w);
    let mission_panel_h = mission_grid.panel_height();
    let priority_panel_h = 94.0;
    draw_tier_panel(
        control_x,
        control_y,
        mission_panel_w,
        mission_panel_h,
        None,
        PanelTier::Utility,
        false,
    );
    draw_tier_panel(
        priority_panel_x,
        control_y,
        priority_panel_w,
        priority_panel_h,
        None,
        PanelTier::Utility,
        false,
    );

    let mission_inner_x = control_x + 12.0;
    let priority_inner_x = priority_panel_x + 12.0;
    let priority_inner_w = priority_panel_w - 24.0;
    let priority_button_w = ((priority_inner_w - 8.0) / 2.0).max(112.0);
    let top_button_y = control_y + 10.0;
    let priority_button_h = 24.0;
    for (index, mission_id) in selected_floor.mission_ids.iter().enumerate() {
        let Some(mission) = data
            .missions
            .missions
            .iter()
            .find(|entry| entry.id == *mission_id)
        else {
            continue;
        };
        let (offset_x, offset_y) = mission_grid.cell(index);
        let x = mission_inner_x + offset_x;
        let y = top_button_y + offset_y;
        let pressed = if *mission_id == selected_mission_id {
            primary_button(
                x,
                y,
                mission_grid.button_w,
                MISSION_BUTTON_H,
                &compact_text(&mission.name, 12),
            )
        } else {
            secondary_button(
                x,
                y,
                mission_grid.button_w,
                MISSION_BUTTON_H,
                &compact_text(&mission.name, 12),
            )
        };
        if pressed {
            return Some(UiAction::SelectExpeditionMission(mission.id.clone()));
        }
    }

    let priority_buttons = [
        (
            &expedition_text.balanced_button,
            ExpeditionPriority::Balanced,
        ),
        (
            &expedition_text.aggressive_button,
            ExpeditionPriority::Aggressive,
        ),
        (&expedition_text.safe_button, ExpeditionPriority::Safe),
        (
            &expedition_text.recovery_button,
            ExpeditionPriority::RecoveryFocused,
        ),
        (
            &expedition_text.curiosity_button,
            ExpeditionPriority::Curiosity,
        ),
    ];
    for (index, (label, value)) in priority_buttons.iter().enumerate() {
        let col = index % 2;
        let row = index / 2;
        let x = priority_inner_x + col as f32 * (priority_button_w + 8.0);
        let y = top_button_y + row as f32 * 24.0;
        let is_selected =
            std::mem::discriminant(&expedition_state.priority) == std::mem::discriminant(value);
        let pressed = if is_selected {
            primary_button(x, y, priority_button_w, priority_button_h, label)
        } else {
            secondary_button(x, y, priority_button_w, priority_button_h, label)
        };
        if pressed {
            return Some(UiAction::SetExpeditionPriority(value.clone()));
        }
    }

    if let Some(preview) = &preview {
        let metric_y = 328.0;
        let metric_gap = 8.0;
        let mut metrics = vec![
            (
                "Success".to_owned(),
                preview.success_score.to_string(),
                theme::POSITIVE,
            ),
            (
                "Injury Risk".to_owned(),
                preview
                    .injury_risk_score
                    .map_or_else(|| "—".to_owned(), |score| score.to_string()),
                risk_color(preview.injury_risk_score),
            ),
            (
                "Party Condition".to_owned(),
                format!("{}%", preview.party_effectiveness_pct),
                party_condition_color(data, preview.party_effectiveness_pct),
            ),
        ];
        if preview.projected_materials > 0 {
            metrics.push((
                "Materials".to_owned(),
                preview.projected_materials.to_string(),
                theme::INFO,
            ));
        }
        if preview.projected_eggs > 0 {
            metrics.push((
                "Eggs".to_owned(),
                preview.projected_eggs.to_string(),
                theme::WARNING,
            ));
        }
        if preview.projected_arcane_residue > 0 {
            metrics.push((
                "Residue".to_owned(),
                preview.projected_arcane_residue.to_string(),
                theme::PRIMARY,
            ));
        }
        if preview.projected_relics > 0 {
            metrics.push((
                "Relics".to_owned(),
                preview.projected_relics.to_string(),
                theme::INFO,
            ));
        }
        let metric_count = metrics.len().min(5);
        let metric_w = (layout.detail_w - 32.0 - metric_gap * (metric_count as f32 - 1.0))
            / metric_count as f32;
        for (index, (label, value, color)) in metrics.iter().take(metric_count).enumerate() {
            draw_metric_tile(
                layout.detail_x + 16.0 + (metric_w + metric_gap) * index as f32,
                metric_y,
                metric_w,
                50.0,
                label,
                value,
                *color,
            );
        }
        if let Some(mission) = selected_mission {
            draw_body_text(
                // The metric tiles end at 378 and the panel now closes at 398, so
                // a 13px baseline at 394 clears both. It used to sit at 386,
                // which drew the caption through the bottom of the Success tile.
                &expedition_prep_cost_label(mission),
                layout.detail_x + 16.0,
                394.0,
                13.0,
                theme::TEXT_MUTED,
            );
        }
    } else {
        draw_body_text(
            &expedition_text.no_preview_message,
            layout.detail_x + 16.0,
            316.0,
            14.0,
            theme::TEXT_MUTED,
        );
    }

    // How many card rows the panel can physically hold, which is what decides
    // whether the roster needs paging at all.
    let roster = RosterWindow::from_panel(
        game_state.monsters.len(),
        expedition_state.roster_page,
        layout.team_h - 58.0,
        104.0,
        ROSTER_COLUMNS,
    );
    let visible_team_h = if game_state.monsters.is_empty() {
        150.0
    } else {
        roster_panel_height(&roster, 58.0, 104.0).min(layout.team_h)
    };
    draw_tier_panel(
        layout.left_margin,
        layout.team_y,
        layout.content_width,
        visible_team_h,
        Some(&expedition_text.team_panel_title),
        PanelTier::Support,
        false,
    );

    if game_state.monsters.is_empty() {
        draw_empty_state(
            layout.left_margin + 8.0,
            layout.team_y + 40.0,
            layout.content_width - 16.0,
            (visible_team_h - 56.0).max(92.0),
            &expedition_text.team_empty_title,
            &expedition_text.team_empty_message,
        );
    } else {
        let card_w =
            (layout.content_width - layout::PANEL_PADDING * 2.0 - layout::SECTION_GAP) / 2.0;
        for (index, monster) in game_state
            .monsters
            .iter()
            .skip(roster.first_index)
            .take(roster.visible_count)
            .enumerate()
        {
            let col = index % ROSTER_COLUMNS;
            let row = index / ROSTER_COLUMNS;
            let x = layout.left_margin
                + layout::PANEL_PADDING
                + col as f32 * (card_w + layout::SECTION_GAP);
            let y = layout.team_y + 46.0 + row as f32 * 104.0;
            let assigned = matches!(monster.current_job, CompanionJobState::OnExpedition { .. });
            // Booked companions cannot join a run — day resolution settles the
            // contract first — so the party preview must not invite the player
            // to count her stats into it.
            let is_booked = crate::engine::is_booked_for_contract(game_state, &monster.id);
            let state_label = if is_booked {
                &data.ui_text.common.assignment_booked_label
            } else {
                assignment_label(data, &monster.current_job)
            };
            let species_label = format!(
                "{} | {}",
                compact_text(&species_name_by_id(data, &monster.species_id), 18),
                monster_quality_label(data, monster)
            );
            // This card carried one figure — her instability — on the screen
            // whose entire job is choosing who goes down the tower, and
            // instability is the one number a run does not turn on. Twenty
            // companions read as twenty identical cards.
            //
            // What a run actually turns on: her role against the mission's
            // preferred one, power for materials, endurance for coming back
            // unhurt, instinct for residue, and how worn she already is. Her
            // rank moved up beside the species, because a floor's
            // `required_roster` is written in stars.
            let stats = crate::engine::effective_stats(data, monster);
            let effectiveness = crate::engine::companion_effectiveness(data, monster);
            let condition_note = if effectiveness < 100 {
                format!(" | Worn {effectiveness}%")
            } else {
                String::new()
            };
            let key_value = format!(
                "{} | Pow {} End {} Ins {}{} | Instability {}",
                monster_depth_role_label(data, monster),
                stats.power,
                stats.endurance,
                stats.instinct,
                condition_note,
                monster.corruption,
            );
            let card = draw_character_card(
                data,
                monster,
                x,
                y,
                card_w,
                92.0,
                CharacterCardSpec {
                    name: &monster.name,
                    species: &species_label,
                    state: state_label,
                    key_value: &key_value,
                    color: if assigned {
                        theme::POSITIVE
                    } else {
                        theme::PRIMARY
                    },
                    state_color: if assigned {
                        theme::POSITIVE
                    } else {
                        theme::WARNING
                    },
                    selected: assigned,
                    disabled: false,
                },
            );
            let mut action_y = card.action_y;
            if !assigned
                && !is_booked
                && primary_button(
                    card.action_x,
                    action_y,
                    card.action_w,
                    22.0,
                    &data.ui_text.common.assign_button,
                )
            {
                return Some(UiAction::AssignMonsterToExpedition(
                    monster.id.clone(),
                    selected_floor.id.clone(),
                ));
            }
            if !assigned {
                action_y += 24.0;
            }
            // Same reason as the Assign button above: a booked companion is
            // skipped by day resolution, so resting her recovers nothing.
            if !matches!(monster.current_job, CompanionJobState::Resting)
                && !is_booked
                && secondary_button(
                    card.action_x,
                    action_y,
                    card.action_w,
                    22.0,
                    &data.ui_text.common.rest_button,
                )
            {
                return Some(UiAction::AssignMonsterToRest(monster.id.clone()));
            }
            if !matches!(monster.current_job, CompanionJobState::Resting) {
                action_y += 24.0;
            }
            if !matches!(monster.current_job, CompanionJobState::Idle)
                && utility_button(
                    card.action_x,
                    action_y,
                    card.action_w,
                    22.0,
                    &data.ui_text.common.idle_button,
                )
            {
                return Some(UiAction::AssignMonsterToIdle(monster.id.clone()));
            }
        }

        if let Some(action) = draw_roster_pager(
            &roster,
            game_state.monsters.len(),
            layout.left_margin + layout::PANEL_PADDING,
            layout.team_y + 46.0 + roster.card_rows() as f32 * 104.0,
            layout.content_width - layout::PANEL_PADDING * 2.0,
            UiAction::ShowRosterPage,
        ) {
            return Some(action);
        }
    }

    if let Some(action) = draw_standard_gameplay_footer(
        data,
        layout.left_margin,
        layout.footer_y,
        layout.content_width,
        Some(UiAction::OpenExpeditionPlanning),
    ) {
        return Some(action);
    }

    if let Some(error_message) = last_error {
        draw_body_text(
            error_message,
            layout.left_margin + 16.0,
            layout.footer_y - 8.0,
            16.0,
            theme::DANGER,
        );
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{floor_difficulty_color, party_condition_color};
    use crate::data::test_game_data;
    use crate::ui::theme;

    /// The Expedition Desk opens with nobody assigned, so the floor's own colour
    /// is the first thing a player reads about it — and `difficulty >= 5`, from
    /// when the tower was three floors deep, made every floor in a 20-to-104
    /// catalogue read as lethal.
    #[test]
    fn a_shallow_floor_does_not_read_as_lethal() {
        let data = test_game_data();
        let shallowest = data
            .floors
            .floors
            .iter()
            .map(|floor| floor.difficulty)
            .min()
            .expect("the tower should have floors");
        let deepest = data
            .floors
            .floors
            .iter()
            .map(|floor| floor.difficulty)
            .max()
            .expect("the tower should have floors");

        assert_eq!(
            floor_difficulty_color(&data, shallowest),
            theme::POSITIVE,
            "the first floor of the tower should not be painted as its worst"
        );
        assert_eq!(
            floor_difficulty_color(&data, deepest),
            theme::DANGER,
            "the bottom of the tower should be"
        );
    }

    /// Condition costs output the moment it is under 100, and cannot fall past
    /// the authored floor. Those are the two numbers; 95 and 75 were not.
    #[test]
    fn party_condition_turns_where_the_engine_says_it_costs_something() {
        let data = test_game_data();
        let floor = data
            .config
            .day_cycle
            .condition_effects
            .min_effectiveness_pct;

        assert_eq!(party_condition_color(&data, 100), theme::POSITIVE);
        assert_eq!(party_condition_color(&data, 99), theme::WARNING);
        assert_eq!(party_condition_color(&data, floor + 1), theme::WARNING);
        assert_eq!(party_condition_color(&data, floor), theme::DANGER);
    }
}
