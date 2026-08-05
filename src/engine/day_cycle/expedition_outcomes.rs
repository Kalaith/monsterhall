use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RealizedExpeditionRewards {
    pub(super) materials: u32,
    pub(super) arcane_residue: u32,
    pub(super) eggs: u32,
    pub(super) relics: u32,
}

pub(super) fn realized_expedition_rewards(
    data: &GameData,
    preview: &ExpeditionPlanPreview,
    succeeded: bool,
) -> RealizedExpeditionRewards {
    let salvage_pct = if succeeded {
        100
    } else {
        data.config.day_cycle.expedition_failure_salvage_pct
    };
    RealizedExpeditionRewards {
        materials: scale_by_effectiveness(preview.projected_materials, salvage_pct),
        arcane_residue: scale_by_effectiveness(preview.projected_arcane_residue, salvage_pct),
        eggs: if succeeded { preview.projected_eggs } else { 0 },
        relics: if succeeded {
            preview.projected_relics
        } else {
            0
        },
    }
}

pub(super) fn expedition_condition_cost_pct(data: &GameData, priority: &ExpeditionPriority) -> u32 {
    if matches!(priority, ExpeditionPriority::RecoveryFocused) {
        data.config.day_cycle.recovery_focused_condition_cost_pct
    } else {
        100
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::test_game_data;

    #[test]
    fn a_failed_expedition_returns_only_salvage() {
        let data = test_game_data();
        let preview = ExpeditionPlanPreview {
            success_score: 40,
            success_chance_pct: 40,
            projected_materials: 50,
            projected_arcane_residue: 25,
            projected_eggs: 3,
            projected_relics: 2,
            injury_risk_score: Some(4),
            party_effectiveness_pct: 100,
        };

        assert_eq!(
            realized_expedition_rewards(&data, &preview, false),
            RealizedExpeditionRewards {
                materials: 10,
                arcane_residue: 5,
                eggs: 0,
                relics: 0,
            }
        );
        assert_eq!(
            realized_expedition_rewards(&data, &preview, true),
            RealizedExpeditionRewards {
                materials: 50,
                arcane_residue: 25,
                eggs: 3,
                relics: 2,
            }
        );
    }

    #[test]
    fn safe_protects_from_injury_while_recovery_reduces_the_condition_toll() {
        let data = test_game_data();

        assert!(
            priority_injury_risk(&ExpeditionPriority::Safe)
                < priority_injury_risk(&ExpeditionPriority::RecoveryFocused)
        );
        assert_eq!(
            expedition_condition_cost_pct(&data, &ExpeditionPriority::Safe),
            100
        );
        assert_eq!(
            expedition_condition_cost_pct(&data, &ExpeditionPriority::RecoveryFocused),
            50
        );
    }
}
