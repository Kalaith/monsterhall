//! What a companion reads as, and what working outside it costs her.

use crate::data::GameData;
use crate::state::CompanionState;

/// How this companion reads to the tower: the single classifier behind both
/// `role_affinity`'s mission bonus and the label the profile screen shows.
///
/// The UI carried an independent copy of this branching with different label
/// strings. They agreed only by hand — updating one and not the other would have
/// told the player she is a performer while the engine scored her a delver and
/// quietly withheld the mission role bonus.
///
/// **The ladder is ordered, so its first branch decides for everybody it
/// catches.** That branch used to be `corruption >= 10`, and corruption is only
/// ever `saturating_add`ed — never reduced anywhere in the game. A companion
/// working the reception hall gains 1 a shift, so she crossed the line in ten
/// days of a 365-day campaign and was a `corruption_adept` for the rest of it
/// whatever her stats, skills, traits or bond said. Every branch below became
/// unreachable in play, including `versatile`, which is the whole mechanism the
/// roster-variety work added to make low-tier companions worth keeping.
///
/// Corruption still decides roles — through mutation, which is the game's own
/// statement that the tower has changed what a companion *is*, and which grants
/// or leads to `corruption_tuned` on most of its routes. A raw meter reading
/// cannot do that job: it only climbs, so any fixed threshold is a latch that
/// eventually swallows the entire roster. `corruption_adept_minimum` is
/// therefore `Option<u32>` and the shipped catalogue omits it.
pub fn monster_role(data: &GameData, monster: &CompanionState) -> &'static str {
    let stats = crate::engine::effective_stats(data, monster);
    let thresholds = &data.config.day_cycle.role_thresholds;
    let has = |trait_id: &str| monster.trait_ids.iter().any(|id| id == trait_id);

    let corruption_latched = thresholds
        .corruption_adept_minimum
        .is_some_and(|minimum| monster.corruption >= minimum);

    if corruption_latched || has("corruption_tuned") {
        "corruption_adept"
    } else if monster.work_history.hatchery_assists >= thresholds.hatchery_assist_minimum
        || has("hatchery_attuned")
    {
        "hatchery_specialist"
    } else if monster.skills.charm >= thresholds.performer_charm_skill_minimum
        || stats.charm >= stats.power + thresholds.performer_charm_margin as i32
    {
        "performer"
    } else if stats.power >= stats.charm + thresholds.delver_power_margin as i32 {
        "delver"
    } else if monster.bond >= thresholds.comfort_bond_minimum || has("calming_presence") {
        "comfort"
    } else {
        "versatile"
    }
}

pub(crate) fn role_affinity(data: &GameData, monster: &CompanionState, role: &str) -> i32 {
    if role.is_empty() {
        return 0;
    }

    let affinity = &data.config.day_cycle.role_affinity;
    let role_of_monster = monster_role(data, monster);
    if role_of_monster == role {
        affinity.matched_bonus
    } else if role_of_monster == "versatile" {
        affinity.versatile_bonus
    } else {
        -off_role_penalty(data, monster)
    }
}

/// What working outside her role costs a companion.
///
/// Off-role used to be a flat zero for everybody, which made a species' tier
/// pure upside: a `gargoyle_stairwarden` was exactly as flexible as a
/// `slime_companion` and strictly stronger, so there was never a reason to keep
/// a low tier once a high one was available. Capability is now paid for with
/// rigidity, scaled by the species' own base-stat total.
fn off_role_penalty(data: &GameData, monster: &CompanionState) -> i32 {
    let affinity = &data.config.day_cycle.role_affinity;
    let Some(species) = crate::engine::species_of(data, monster) else {
        return 0;
    };
    let stat_total = crate::engine::species_stat_total(species);
    let floor = affinity.flexibility_stat_floor;
    let ceiling = affinity.flexibility_stat_ceiling.max(floor + 1);

    if stat_total <= floor {
        return 0;
    }
    if stat_total >= ceiling {
        return affinity.off_role_penalty_max;
    }
    let above_floor = stat_total - floor;
    let span = ceiling - floor;
    affinity.off_role_penalty_max * above_floor as i32 / span as i32
}

#[cfg(test)]
mod tests {
    use super::monster_role;
    use crate::data::test_game_data;
    use crate::state::CompanionState;

    /// Stats live on the companion, written from her species when she hatches —
    /// `effective_stats` only adds the trait bonus on top. A fixture that leaves
    /// them at zero is testing a companion who does not exist.
    fn companion(
        data: &crate::data::GameData,
        species_id: &str,
        traits: &[&str],
    ) -> CompanionState {
        let species = data
            .species
            .species
            .iter()
            .find(|species| species.id == species_id)
            .expect("fixture names a real species");
        CompanionState {
            species_id: species_id.to_owned(),
            stats: species.base_stats.clone(),
            trait_ids: traits.iter().map(|id| (*id).to_owned()).collect(),
            ..Default::default()
        }
    }

    /// The property the old `corruption >= 10` branch broke.
    ///
    /// Corruption is `saturating_add`ed and never reduced, so a role that can be
    /// overwritten by a corruption reading is a role every companion loses and
    /// never gets back. A porter who has hauled for a year is still a delver.
    #[test]
    fn a_companion_keeps_her_role_however_much_corruption_she_carries() {
        let data = test_game_data();
        let mut porter = companion(&data, "minotaur_porter", &["resilient", "sharp_instinct"]);
        let role_when_fresh = monster_role(&data, &porter);
        assert_eq!(role_when_fresh, "delver");

        for corruption in [8, 10, 16, 100, 488, u32::MAX] {
            porter.corruption = corruption;
            assert_eq!(
                monster_role(&data, &porter),
                role_when_fresh,
                "corruption {corruption} rewrote the porter's role."
            );
        }
    }

    /// A `preferred_role` naming a role nothing produces is a bonus that never
    /// lands — the same shape as a preferred trait nobody can hold.
    #[test]
    fn every_role_a_mission_prefers_is_one_some_companion_can_read_as() {
        let data = test_game_data();
        let species_traits: Vec<(String, Vec<String>)> = data
            .species
            .species
            .iter()
            .map(|species| (species.id.clone(), species.starting_traits.clone()))
            .collect();

        for mission in &data.missions.missions {
            let Some(preferred) = mission.preferred_role.as_deref() else {
                continue;
            };
            let thresholds = &data.config.day_cycle.role_thresholds;
            let produced = species_traits.iter().any(|(species_id, traits)| {
                let refs: Vec<&str> = traits.iter().map(String::as_str).collect();
                let mut candidate = companion(&data, species_id, &refs);
                if monster_role(&data, &candidate) == preferred {
                    return true;
                }
                // Bond and hatchery work are earned, so a rung gated on them is
                // reachable even though a fresh companion does not show it.
                candidate.bond = thresholds.comfort_bond_minimum;
                candidate.work_history.hatchery_assists = thresholds.hatchery_assist_minimum;
                monster_role(&data, &candidate) == preferred
            });
            assert!(
                produced,
                "mission '{}' prefers role '{preferred}', which no companion reads as.",
                mission.id
            );
        }
    }
}
