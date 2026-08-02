use std::collections::HashSet;

use super::types::GameData;

pub(super) fn validate_depth_systems(data: &GameData) -> Result<(), String> {
    let building_ids = id_set(
        data.buildings
            .buildings
            .iter()
            .map(|entry| entry.id.as_str()),
    );
    let trait_ids = id_set(data.traits.traits.iter().map(|entry| entry.id.as_str()));
    let request_ids = id_set(
        data.contracts
            .requests
            .iter()
            .map(|entry| entry.id.as_str()),
    );

    for room in &data.guild_rooms.rooms {
        validate_known_ids(
            &room.upgrade_building_ids,
            &building_ids,
            &format!("guild room '{}'.upgrade_building_ids", room.id),
        )?;
        if let Some(niche) = room.strategic_niche.as_deref() {
            validate_room_niche(niche, &format!("guild room '{}'.strategic_niche", room.id))?;
        }
    }

    for mission in &data.missions.missions {
        if let Some(role) = mission.preferred_role.as_deref() {
            validate_companion_role(role, &format!("mission '{}'.preferred_role", mission.id))?;
        }
        // The engine matches this string to decide the mission's multipliers,
        // its inferred party role and which reward bar it relieves. An unknown
        // focus is not a warning at runtime — it silently drops all of them, and
        // the mission just quietly stops being about anything.
        if !matches!(
            mission.reward_focus.as_str(),
            "materials" | "eggs" | "relics" | "residue"
        ) {
            return Err(format!(
                "mission '{}' has unknown reward_focus '{}'.",
                mission.id, mission.reward_focus
            ));
        }
    }

    for request in &data.contracts.requests {
        validate_known_ids(
            &request.preferred_trait_ids,
            &trait_ids,
            &format!("contract '{}'.preferred_trait_ids", request.id),
        )?;
        if let Some(role) = request.preferred_role.as_deref() {
            validate_companion_role(role, &format!("contract '{}'.preferred_role", request.id))?;
        }
        if let Some(follow_up_id) = request.follow_up_request_id.as_deref() {
            if follow_up_id == request.id {
                return Err(format!(
                    "contract '{}' cannot follow up with itself.",
                    request.id
                ));
            }
            if !request_ids.contains(follow_up_id) {
                return Err(format!(
                    "contract '{}' references unknown follow_up_request_id '{}'.",
                    request.id, follow_up_id
                ));
            }
        }
    }

    for floor in &data.floors.floors {
        if floor.hazard_tags.iter().any(|tag| tag.trim().is_empty()) {
            return Err(format!("floor '{}' contains a blank hazard tag.", floor.id));
        }
    }

    for event in &data.events.events {
        if event.situation_days == 0
            && (event.situation_upkeep_pressure_pct > 0 || event.situation_guest_bonus > 0)
        {
            return Err(format!(
                "event '{}' defines situation pressure without situation_days.",
                event.id
            ));
        }
    }

    Ok(())
}

fn validate_known_ids(
    ids: &[String],
    known_ids: &HashSet<&str>,
    label: &str,
) -> Result<(), String> {
    for id in ids {
        if !known_ids.contains(id.as_str()) {
            return Err(format!("{label} references unknown id '{id}'."));
        }
    }
    Ok(())
}

/// The niches a guild room can have. `room_depth_profile_for_town` matches on
/// exactly these; anything else falls through to a generic bias.
const ROOM_NICHES: [&str; 4] = ["comfort", "performance", "hatchery", "corruption"];

/// The roles `monster_role` can return. `role_affinity` compares against exactly
/// these, and a companion who matches none is charged the off-role penalty.
const COMPANION_ROLES: [&str; 6] = [
    "corruption_adept",
    "hatchery_specialist",
    "performer",
    "delver",
    "comfort",
    "versatile",
];

/// A room's niche.
///
/// Rooms and companions used to share one validator over the union of both
/// vocabularies, which let each side pass with a value only the other side
/// understands — and both failures are silent. A room authored `delver` takes
/// the generic gold and residue bias with nothing to say why it earns less than
/// its neighbours.
fn validate_room_niche(value: &str, label: &str) -> Result<(), String> {
    if ROOM_NICHES.contains(&value) {
        Ok(())
    } else {
        Err(format!(
            "{label} is '{value}', which is not a room niche. Expected one of {ROOM_NICHES:?}."
        ))
    }
}

/// A mission's or contract's preferred companion role.
///
/// The worse half of the shared validator: `role_affinity` gives the matched
/// bonus only to a companion whose `monster_role` equals this string, so a
/// mission asking for `performance` — a room niche, and one the old validator
/// accepted — matches nobody and charges the **whole party** the off-role
/// penalty instead of rewarding anyone.
fn validate_companion_role(value: &str, label: &str) -> Result<(), String> {
    if COMPANION_ROLES.contains(&value) {
        Ok(())
    } else {
        Err(format!(
            "{label} is '{value}', which is not a companion role. Expected one of {COMPANION_ROLES:?}."
        ))
    }
}

fn id_set<'a, I>(ids: I) -> HashSet<&'a str>
where
    I: Iterator<Item = &'a str>,
{
    ids.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The validator's two closed sets, checked against the code that consumes
    /// them rather than against each other.
    ///
    /// They were one list over the union of both vocabularies, so each side
    /// accepted values only the other understood — and both failures are silent
    /// at runtime. This is the assertion that keeps them apart: a role the
    /// validator accepts must be one `monster_role` can return, or `role_affinity`
    /// will match nobody and penalise the whole party.
    #[test]
    fn every_accepted_role_is_a_role_the_engine_can_infer() {
        let data = crate::data::test_game_data();
        let mut monster = crate::state::CompanionState {
            quality_rank: 1,
            ..crate::state::CompanionState::default()
        };

        let mut inferable = std::collections::HashSet::new();
        // Sweep the branches of `monster_role` by driving the inputs it reads.
        // Traits are one of those inputs: `corruption_adept` is reached by
        // carrying `corruption_tuned`, not by a meter reading, because
        // corruption only ever climbs and a threshold on it latches the whole
        // roster into one role for the rest of the campaign.
        for trait_ids in [vec![], vec!["corruption_tuned".to_owned()]] {
            for hatchery_assists in [0, 1] {
                for charm_skill in [0, 3] {
                    for (power, charm) in [(1, 1), (9, 1), (1, 9)] {
                        for bond in [0, 9] {
                            monster.trait_ids.clone_from(&trait_ids);
                            monster.work_history.hatchery_assists = hatchery_assists;
                            monster.skills.charm = charm_skill;
                            monster.stats.power = power;
                            monster.stats.charm = charm;
                            monster.bond = bond;
                            inferable.insert(crate::engine::monster_role(&data, &monster));
                        }
                    }
                }
            }
        }

        for role in COMPANION_ROLES {
            assert!(
                inferable.contains(role),
                "'{role}' is accepted on missions and contracts but no companion can ever hold it"
            );
        }
        for role in &inferable {
            assert!(
                COMPANION_ROLES.contains(role),
                "the engine infers '{role}' but the validator would reject content asking for it"
            );
        }
        assert!(
            !COMPANION_ROLES.contains(&"performance"),
            "'performance' is a room niche; a mission asking for it matches nobody"
        );
    }

    /// And the niches, against the profile that actually branches on them.
    #[test]
    fn every_room_authors_a_niche_the_profile_understands() {
        let data = crate::data::test_game_data();

        for room in &data.guild_rooms.rooms {
            let niche = room.strategic_niche.as_deref().unwrap_or_default();
            assert!(
                ROOM_NICHES.contains(&niche),
                "room '{}' has niche '{niche}', which falls through to the generic bias",
                room.id
            );
        }
    }
}
