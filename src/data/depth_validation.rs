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
            validate_role_or_niche(
                niche,
                &format!("guild room '{}'.strategic_niche", room.id),
            )?;
        }
    }

    for mission in &data.missions.missions {
        if let Some(role) = mission.preferred_role.as_deref() {
            validate_role_or_niche(role, &format!("mission '{}'.preferred_role", mission.id))?;
        }
    }

    for request in &data.contracts.requests {
        validate_known_ids(
            &request.preferred_trait_ids,
            &trait_ids,
            &format!("contract '{}'.preferred_trait_ids", request.id),
        )?;
        if let Some(role) = request.preferred_role.as_deref() {
            validate_role_or_niche(
                role,
                &format!("contract '{}'.preferred_role", request.id),
            )?;
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

fn validate_role_or_niche(value: &str, label: &str) -> Result<(), String> {
    if matches!(
        value,
        "comfort"
            | "performance"
            | "hatchery"
            | "corruption"
            | "corruption_adept"
            | "hatchery_specialist"
            | "performer"
            | "delver"
            | "versatile"
    ) {
        Ok(())
    } else {
        Err(format!("{label} contains unknown role or niche '{value}'."))
    }
}

fn id_set<'a, I>(ids: I) -> HashSet<&'a str>
where
    I: Iterator<Item = &'a str>,
{
    ids.collect()
}
