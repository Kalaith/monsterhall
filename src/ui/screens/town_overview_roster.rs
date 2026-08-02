//! The Town Overview roster strip.
//!
//! Split out of `town_overview_sections.rs` when that file crossed the 800-line
//! limit, and the natural seam: this panel is the guild's roster view, and the
//! only route to a companion's profile — and therefore to releasing her, which
//! is what a guild at its population cap has to do to keep hatching.

use macroquad::prelude::*;

use crate::data::GameData;
use crate::state::{CompanionJobState, CompanionState, GameState};
use crate::ui::actions::UiAction;
use crate::ui::art::draw_condition_badges;
use crate::ui::art::draw_species_portrait;
use crate::ui::core::{draw_body_text, draw_body_text_in_box, secondary_button, utility_button};
use crate::ui::layout;
use crate::ui::screens::roster_window::{draw_roster_pager, RosterWindow, PAGER_RESERVE_H};
use crate::ui::theme;

use crate::ui::view_models::{assignment_label, companion_skill_summary, species_name_by_id};

use super::town_overview_sections::{
    draw_organic_panel, draw_organic_stage_panel, draw_organic_status, TownOverviewLayout,
};

/// Companion cards across the Town Overview roster strip.
const ROSTER_STRIP_COLUMNS: usize = 3;

pub(super) fn draw_monster_roster(
    data: &GameData,
    game_state: &GameState,
    layout: &TownOverviewLayout,
    roster_page: usize,
) -> Option<UiAction> {
    if game_state.monsters.is_empty() || layout.compact_height {
        return None;
    }

    draw_organic_stage_panel(
        layout.left_margin,
        layout.roster_y,
        layout.content_width,
        layout.roster_h,
        &data.ui_text.town_overview.roster_panel_title,
        theme::GOLD,
    );

    // This strip is the only route to a companion's profile, and the profile is
    // the only place she can be released — so a flat `.min(3)` against a
    // twenty-companion cap meant seventeen companions had no profile screen and
    // could never be let go, which is exactly the wall a guild hits at capacity.
    let roster = RosterWindow::new(
        game_state.monsters.len(),
        roster_page,
        1,
        ROSTER_STRIP_COLUMNS,
    );
    let visible_count = roster.visible_count;
    let card_width = if visible_count <= 1 {
        layout.content_width - layout::PANEL_PADDING * 2.0
    } else {
        let total_gap = layout::SECTION_GAP * (visible_count as f32 - 1.0);
        (layout.content_width - layout::PANEL_PADDING * 2.0 - total_gap) / visible_count as f32
    };
    // One row of cards, so the pager comes out of the card height rather than
    // getting a row of its own.
    let pager_reserve = if roster.needs_pager() {
        PAGER_RESERVE_H
    } else {
        0.0
    };
    let card_h = 162.0_f32.min((layout.roster_h - 62.0 - pager_reserve).max(120.0));
    let card_y = layout.roster_y + 44.0;

    for (index, monster) in game_state
        .monsters
        .iter()
        .skip(roster.first_index)
        .take(visible_count)
        .enumerate()
    {
        let card_x = layout.left_margin
            + layout::PANEL_PADDING
            + index as f32 * (card_width + layout::SECTION_GAP);
        if let Some(action) =
            draw_roster_card_organic(data, monster, card_x, card_y, card_width, card_h)
        {
            return Some(action);
        }
    }

    if let Some(action) = draw_roster_pager(
        &roster,
        game_state.monsters.len(),
        layout.left_margin + layout::PANEL_PADDING,
        card_y + card_h + 4.0,
        layout.content_width - layout::PANEL_PADDING * 2.0,
        UiAction::ShowRosterPage,
    ) {
        return Some(action);
    }

    None
}

fn draw_roster_card_organic(
    data: &GameData,
    monster: &CompanionState,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Option<UiAction> {
    let state_label = assignment_label(data, &monster.current_job);
    let species_label = species_name_by_id(data, &monster.species_id);
    let key_value = format!("Skills {}", companion_skill_summary(data, monster));
    let accent = job_color(&monster.current_job);

    draw_organic_panel(x, y, w, h, None, accent, false);

    let portrait_w = 76.0_f32.min((w * 0.22).max(64.0));
    let portrait_h = (h - 28.0).max(92.0);
    draw_species_portrait(data, monster, x + 14.0, y + 14.0, portrait_w, portrait_h);

    let action_w = 88.0_f32.min((w * 0.25).max(76.0));
    let action_x = x + w - action_w - 14.0;
    let action_y = y + 18.0;
    let text_x = x + portrait_w + 30.0;
    let text_w = (action_x - text_x - 14.0).max(90.0);

    // This card carries the Rest button, so it is where the decision to rest
    // someone is actually made — and until now it said nothing about her
    // condition. Fatigue and stress cost real output since the condition wiring
    // landed, so the numbers belong next to the button that fixes them.
    let show_condition = h >= 150.0;
    draw_body_text(&monster.name, text_x, y + 30.0, 20.0, theme::TEXT_STRONG);
    draw_body_text(&species_label, text_x, y + 52.0, 13.0, theme::TEXT_BODY);
    draw_organic_status(text_x, y + 66.0, text_w, 28.0, state_label, accent);
    draw_body_text_in_box(
        &key_value,
        text_x,
        if show_condition { y + 98.0 } else { y + 106.0 },
        text_w,
        24.0,
        12.0,
        theme::TEXT_MUTED,
    );
    if show_condition {
        // Capped: the roster card is full-width, so an uncapped strip gives four
        // enormous boxes holding two characters each.
        draw_condition_badges(monster, text_x, y + h - 40.0, text_w.min(300.0));
    }

    if secondary_button(
        action_x,
        action_y,
        action_w,
        24.0,
        &data.ui_text.town_overview.monster_profile_button,
    ) {
        return Some(UiAction::OpenMonsterProfile(monster.id.clone()));
    }
    if secondary_button(
        action_x,
        action_y + 30.0,
        action_w,
        24.0,
        &data.ui_text.common.rest_button,
    ) {
        return Some(UiAction::AssignMonsterToRest(monster.id.clone()));
    }
    if !matches!(monster.current_job, CompanionJobState::Idle)
        && utility_button(
            action_x,
            action_y + 60.0,
            action_w,
            24.0,
            &data.ui_text.common.idle_button,
        )
    {
        return Some(UiAction::AssignMonsterToIdle(monster.id.clone()));
    }

    None
}

fn job_color(job: &CompanionJobState) -> Color {
    match job {
        CompanionJobState::Idle => theme::TEXT_MUTED,
        CompanionJobState::GuildJob { .. } => theme::ROSE,
        CompanionJobState::Resting => theme::INFO,
        CompanionJobState::OnExpedition { .. } => theme::WARNING,
    }
}
