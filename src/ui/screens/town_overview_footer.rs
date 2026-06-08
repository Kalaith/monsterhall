use macroquad::prelude::*;

use crate::data::GameData;
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_ui_icon, UiIcon};
use crate::ui::core::draw_body_text_in_box;
use crate::ui::layout;
use crate::ui::theme;

use super::town_overview_sections::draw_organic_panel;

pub(super) fn draw_town_overview_footer(
    data: &GameData,
    x: f32,
    y: f32,
    w: f32,
) -> Option<UiAction> {
    let common = &data.ui_text.common;
    let actions: [(&str, UiAction); 8] = [
        ("Guild Hall", UiAction::ReturnToTownOverview),
        (
            common.town_planner_button.as_str(),
            UiAction::OpenTownManagement,
        ),
        (
            common.guild_jobs_button.as_str(),
            UiAction::OpenGuildHallManagement,
        ),
        (
            common.guest_desk_button.as_str(),
            UiAction::OpenContractDesk,
        ),
        (
            common.chamber_button.as_str(),
            UiAction::OpenHatcheryManagement,
        ),
        (
            common.expedition_desk_button.as_str(),
            UiAction::OpenExpeditionPlanning,
        ),
        (common.journal_button.as_str(), UiAction::OpenJournal),
        (common.end_day_button.as_str(), UiAction::ResolveDay),
    ];

    draw_organic_panel(x, y, w, layout::FOOTER_H, None, theme::BORDER_1, false);

    let gap = 8.0;
    let inner_x = x + 14.0;
    let inner_w = w - 28.0;
    let button_w = (inner_w - gap * (actions.len() as f32 - 1.0)) / actions.len() as f32;
    for (index, (label, action)) in actions.iter().enumerate() {
        let button_x = inner_x + index as f32 * (button_w + gap);
        let is_current_screen = same_footer_action(action, &UiAction::ReturnToTownOverview);
        if draw_footer_tile(
            button_x,
            y + 8.0,
            button_w,
            layout::FOOTER_H - 16.0,
            label,
            action,
            is_current_screen,
        ) {
            return Some(action.clone());
        }
    }

    None
}

fn draw_footer_tile(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    label: &str,
    action: &UiAction,
    highlighted: bool,
) -> bool {
    let (mouse_x, mouse_y) = mouse_position();
    let hovered = mouse_x >= x && mouse_x <= x + w && mouse_y >= y && mouse_y <= y + h;
    let pressed = hovered && is_mouse_button_down(MouseButton::Left);
    let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);
    let is_end_day = matches!(action, UiAction::ResolveDay);
    let accent = if highlighted {
        theme::GOLD
    } else if is_end_day {
        theme::WARNING
    } else {
        theme::BORDER_1
    };
    let fill_base = if highlighted {
        theme::PRIMARY
    } else {
        theme::PANEL_1
    };
    let fill_alpha = if pressed {
        0.98
    } else if hovered {
        0.88
    } else if highlighted {
        0.78
    } else if is_end_day {
        0.74
    } else {
        0.64
    };
    let fill = Color::new(fill_base.r, fill_base.g, fill_base.b, fill_alpha);

    draw_rectangle(x + 8.0, y + 2.0, w - 16.0, h - 4.0, fill);
    draw_rectangle(x + 3.0, y + 10.0, w - 6.0, h - 20.0, fill);
    draw_triangle(
        vec2(x + 4.0, y + 10.0),
        vec2(x + 18.0, y + 2.0),
        vec2(x + 12.0, y + h - 6.0),
        fill,
    );
    draw_triangle(
        vec2(x + w - 4.0, y + 9.0),
        vec2(x + w - 18.0, y + 2.0),
        vec2(x + w - 12.0, y + h - 6.0),
        fill,
    );
    draw_line(
        x + 10.0,
        y + 2.0,
        x + w - 15.0,
        y + 1.0,
        1.0,
        Color::new(
            accent.r,
            accent.g,
            accent.b,
            if highlighted || is_end_day {
                0.88
            } else {
                0.78
            },
        ),
    );
    draw_line(
        x + 9.0,
        y + h - 2.0,
        x + w - 18.0,
        y + h - 1.0,
        1.0,
        Color::new(accent.r, accent.g, accent.b, 0.42),
    );
    draw_ui_icon(
        icon_for_footer_action(action),
        x + 12.0,
        y + (h - 34.0) * 0.5,
        34.0,
        Color::new(
            1.0,
            1.0,
            1.0,
            if highlighted || is_end_day { 0.92 } else { 0.7 },
        ),
    );
    draw_body_text_in_box(
        label,
        x + 52.0,
        y + 10.0,
        w - 58.0,
        h - 18.0,
        14.0,
        theme::TEXT_STRONG,
    );

    clicked
}

fn same_footer_action(left: &UiAction, right: &UiAction) -> bool {
    std::mem::discriminant(left) == std::mem::discriminant(right)
}

pub(super) fn icon_for_footer_action(action: &UiAction) -> UiIcon {
    match action {
        UiAction::ReturnToTownOverview => UiIcon::NavTown,
        UiAction::OpenTownManagement => UiIcon::NavPlanner,
        UiAction::OpenGuildHallManagement => UiIcon::NavGuildJobs,
        UiAction::OpenContractDesk => UiIcon::NavGuest,
        UiAction::OpenHatcheryManagement => UiIcon::NavChamber,
        UiAction::OpenExpeditionPlanning => UiIcon::NavExpedition,
        UiAction::OpenJournal => UiIcon::NavJournal,
        UiAction::ResolveDay => UiIcon::NavEndDay,
        _ => UiIcon::StatusAssigned,
    }
}
