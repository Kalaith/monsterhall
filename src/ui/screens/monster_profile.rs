use macroquad::prelude::{screen_height, screen_width};

use crate::data::GameData;
use crate::state::{GameState, MonsterProfileState};
use crate::ui::actions::UiAction;
use crate::ui::art::{draw_backdrop, draw_condition_badges, draw_trait_icons, BackdropKind};
use crate::ui::chrome::{
    draw_inline_status, draw_screen_header, draw_standard_gameplay_footer, draw_tier_panel,
    draw_top_utility_bar, PanelTier,
};
use crate::ui::components::{draw_badge, draw_character_card, draw_metric_tile, CharacterCardSpec};
use crate::ui::core::{draw_body_text, draw_body_text_in_box, utility_button};
use crate::ui::feedback::draw_inline_error;
use crate::ui::layout;
use crate::ui::theme;
use crate::ui::view_models::{
    assignment_label, companion_skill_summary, fill_template, monster_quality_label,
    monster_role_summary, species_name_by_id, species_portrait_key_by_id, trait_names_for_monster,
    work_history_summary,
};

struct MonsterProfileLayout {
    left_margin: f32,
    content_width: f32,
    footer_y: f32,
}

impl MonsterProfileLayout {
    fn new() -> Self {
        let left_margin = layout::OUTER_MARGIN;
        let content_width = screen_width() - left_margin * 2.0;
        let footer_y = screen_height() - layout::FOOTER_BOTTOM_MARGIN - layout::FOOTER_H;
        Self {
            left_margin,
            content_width,
            footer_y,
        }
    }
}

pub fn draw_monster_profile(
    data: &GameData,
    profile_state: &MonsterProfileState,
    game_state: &GameState,
    last_error: Option<&str>,
) -> Option<UiAction> {
    draw_backdrop(BackdropKind::Profile);
    let layout = MonsterProfileLayout::new();
    let ui = &data.ui_text.monster_profile;

    let Some(monster) = game_state
        .monsters
        .iter()
        .find(|monster| monster.id == profile_state.selected_monster_id)
    else {
        return Some(UiAction::ReturnToTownOverview);
    };

    if let Some(action) = draw_top_utility_bar(&data.ui_text.common.settings_button) {
        return Some(action);
    }
    draw_screen_header(
        &fill_template(&ui.title_template, &[("{name}", monster.name.clone())]),
        &ui.subtitle,
    );

    let role_summary = monster_role_summary(data, monster);

    draw_tier_panel(
        layout.left_margin,
        92.0,
        layout.content_width,
        90.0,
        Some(&ui.profile_summary_panel_title),
        PanelTier::Primary,
        true,
    );
    draw_inline_status(
        layout.left_margin + layout::PANEL_PADDING,
        132.0,
        220.0,
        &ui.profile_status_label,
        theme::PRIMARY,
    );
    let _status_message = &profile_state.status_message;
    draw_inline_status(
        layout.left_margin + 260.0,
        132.0,
        210.0,
        assignment_label(data, &monster.current_job),
        role_summary.readiness_color,
    );
    draw_body_text(
        &format!(
            "{}: {}",
            ui.species_label,
            species_name_by_id(data, &monster.species_id)
        ),
        layout.left_margin + 16.0,
        170.0,
        15.0,
        theme::TEXT_BODY,
    );
    draw_body_text(
        &format!(
            "{}: {}",
            ui.portrait_key_label,
            species_portrait_key_by_id(data, &monster.species_id)
        ),
        layout.left_margin + 360.0,
        170.0,
        15.0,
        theme::TEXT_MUTED,
    );
    draw_body_text(
        &format!("Quality: {}", monster_quality_label(monster)),
        layout.left_margin + 690.0,
        170.0,
        15.0,
        theme::WARNING,
    );
    draw_tier_panel(
        layout.left_margin,
        196.0,
        310.0,
        430.0,
        Some(&ui.portrait_panel_title),
        PanelTier::Support,
        false,
    );
    let profile_key_line = format!(
        "{} | {}",
        role_summary.readiness_label,
        companion_skill_summary(data, monster)
    );
    let profile_card = draw_character_card(
        data,
        monster,
        layout.left_margin + 16.0,
        218.0,
        278.0,
        142.0,
        CharacterCardSpec {
            name: &monster.name,
            species: &species_name_by_id(data, &monster.species_id),
            state: assignment_label(data, &monster.current_job),
            key_value: &profile_key_line,
            color: role_summary.readiness_color,
            state_color: role_summary.readiness_color,
            selected: true,
            disabled: false,
        },
    );
    if utility_button(
        profile_card.action_x,
        profile_card.action_y,
        profile_card.action_w,
        24.0,
        &data.ui_text.monster_profile.release_button,
    ) {
        return Some(UiAction::ReleaseMonster(monster.id.clone()));
    }
    draw_condition_badges(monster, layout.left_margin + 16.0, 382.0, 278.0);

    draw_tier_panel(
        layout.left_margin + 326.0,
        196.0,
        layout.content_width - 326.0,
        120.0,
        Some(&ui.best_next_use_panel_title),
        PanelTier::Support,
        false,
    );
    draw_inline_status(
        layout.left_margin + 342.0,
        238.0,
        180.0,
        &role_summary.readiness_label,
        role_summary.readiness_color,
    );
    draw_metric_tile(
        layout.left_margin + 342.0,
        270.0,
        104.0,
        44.0,
        &ui.fatigue_label,
        &monster.fatigue.to_string(),
        if monster.fatigue > 2 {
            theme::WARNING
        } else {
            theme::POSITIVE
        },
    );
    draw_metric_tile(
        layout.left_margin + 454.0,
        270.0,
        104.0,
        44.0,
        &ui.stress_label,
        &monster.stress.to_string(),
        if monster.stress > 2 {
            theme::WARNING
        } else {
            theme::POSITIVE
        },
    );
    draw_metric_tile(
        layout.left_margin + 566.0,
        270.0,
        104.0,
        44.0,
        &ui.injury_label,
        &monster.injury.to_string(),
        if monster.injury > 0 {
            theme::DANGER
        } else {
            theme::POSITIVE
        },
    );
    draw_metric_tile(
        layout.left_margin + 678.0,
        270.0,
        124.0,
        44.0,
        &data.ui_text.common.corruption_label,
        &monster.corruption.to_string(),
        theme::INFO,
    );
    draw_metric_tile(
        layout.left_margin + 826.0,
        232.0,
        118.0,
        56.0,
        &ui.power_label,
        &monster.stats.power.to_string(),
        theme::INFO,
    );
    draw_metric_tile(
        layout.left_margin + 952.0,
        232.0,
        118.0,
        56.0,
        &ui.charm_label,
        &monster.stats.charm.to_string(),
        theme::POSITIVE,
    );
    draw_metric_tile(
        layout.left_margin + 1078.0,
        232.0,
        118.0,
        56.0,
        &ui.endurance_label,
        &monster.stats.endurance.to_string(),
        theme::WARNING,
    );
    draw_body_text(
        &role_summary.best_next_use,
        layout.left_margin + 570.0,
        272.0,
        13.0,
        theme::TEXT_BODY,
    );

    draw_tier_panel(
        layout.left_margin + 326.0,
        332.0,
        370.0,
        294.0,
        Some(&ui.core_stats_panel_title),
        PanelTier::Support,
        false,
    );
    draw_badge(
        layout.left_margin + 342.0,
        374.0,
        338.0,
        24.0,
        &companion_skill_summary(data, monster),
        theme::INFO,
    );
    draw_badge(
        layout.left_margin + 342.0,
        406.0,
        338.0,
        24.0,
        &work_history_summary(data, monster),
        theme::WARNING,
    );
    draw_body_text(
        &fill_template(
            &data.ui_text.town_overview.monster_tower_stats_template,
            &[
                ("{power}", monster.stats.power.to_string()),
                ("{charm}", monster.stats.charm.to_string()),
                ("{endurance}", monster.stats.endurance.to_string()),
                ("{instinct}", monster.stats.instinct.to_string()),
            ],
        ),
        layout.left_margin + 342.0,
        456.0,
        15.0,
        theme::TEXT_BODY,
    );

    draw_tier_panel(
        layout.left_margin + 712.0,
        332.0,
        layout.content_width - 712.0,
        184.0,
        Some(&ui.traits_panel_title),
        PanelTier::Support,
        false,
    );
    draw_trait_icons(
        data,
        &monster.trait_ids,
        layout.left_margin + 728.0,
        374.0,
        layout.content_width - 744.0,
    );
    draw_body_text_in_box(
        &trait_names_for_monster(data, monster),
        layout.left_margin + 728.0,
        462.0,
        layout.content_width - 744.0,
        38.0,
        15.0,
        theme::TEXT_BODY,
    );

    if let Some(error_message) = last_error {
        draw_inline_error(
            layout.left_margin + 326.0,
            606.0,
            layout.content_width - 342.0,
            error_message,
        );
    }

    draw_standard_gameplay_footer(
        data,
        layout.left_margin,
        layout.footer_y,
        layout.content_width,
        None,
    )
}
