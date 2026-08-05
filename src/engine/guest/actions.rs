use super::*;

pub fn assign_monster_to_contract(
    data: &GameData,
    game_state: &mut GameState,
    request_id: &str,
    monster_id: &str,
) -> Result<(), String> {
    let request_index = game_state
        .active_contracts
        .iter()
        .position(|request| request.request_id == request_id)
        .ok_or_else(|| format!("Unknown contract id '{request_id}'."))?;
    let monster = game_state
        .monsters
        .iter()
        .find(|monster| monster.id == monster_id)
        .ok_or_else(|| format!("Unknown monster id '{monster_id}'."))?;
    if contract_service_outcome(
        data,
        game_state,
        &game_state.active_contracts[request_index],
        monster,
    ) == ContractServiceOutcome::Refused
    {
        return Err(evaluate_contract_eligibility(
            data,
            game_state,
            &game_state.active_contracts[request_index],
            monster,
        )
        .failure_reasons
        .join(" "));
    }
    if !game_state.active_contracts[request_index].status.is_live() {
        return Err("That contract has already been resolved.".to_owned());
    }
    if game_state.active_contracts.iter().any(|request| {
        request.request_id != request_id
            && request.assigned_monster_id.as_deref() == Some(monster_id)
            && matches!(request.status, ContractStatus::Accepted)
    }) {
        return Err("That companion is already assigned to another contract.".to_owned());
    }

    let request = &mut game_state.active_contracts[request_index];
    request.assigned_monster_id = Some(monster_id.to_owned());
    request.status = ContractStatus::Accepted;

    // Taking a booking releases whatever she was rostered for, the same way
    // every other assignment releases her from an expedition.
    //
    // Blocking the reverse order was only half the fix: `assign_monster_to_room`
    // refuses a companion who is already booked, but booking a companion who is
    // already working the hall was still allowed — and `resolve_day` settles the
    // contract first and discards her shift, so the guild-job slot was held by
    // somebody whose work would never happen. Refusing here would be wrong; she
    // is perfectly able to take the contract. It is the slot that is wasted, so
    // the slot goes back.
    if let Some(monster) = game_state
        .monsters
        .iter_mut()
        .find(|monster| monster.id == monster_id)
    {
        if matches!(
            monster.current_job,
            CompanionJobState::GuildJob { .. } | CompanionJobState::Resting
        ) {
            monster.current_job = CompanionJobState::Idle;
        }
    }
    Ok(())
}

pub fn clear_contract_assignment(
    game_state: &mut GameState,
    request_id: &str,
) -> Result<(), String> {
    let request = game_state
        .active_contracts
        .iter_mut()
        .find(|request| request.request_id == request_id)
        .ok_or_else(|| format!("Unknown contract id '{request_id}'."))?;
    if !request.status.is_live() {
        return Err("That contract has already been resolved.".to_owned());
    }
    request.assigned_monster_id = None;
    request.status = ContractStatus::Pending;
    Ok(())
}

pub fn decline_contract(game_state: &mut GameState, request_id: &str) -> Result<(), String> {
    let request_index = game_state
        .active_contracts
        .iter()
        .position(|request| request.request_id == request_id)
        .ok_or_else(|| format!("Unknown contract id '{request_id}'."))?;
    if !game_state.active_contracts[request_index].status.is_live() {
        return Err("That contract has already been resolved.".to_owned());
    }

    let mut request = game_state.active_contracts.remove(request_index);
    request.assigned_monster_id = None;
    request.status = ContractStatus::Declined;
    game_state.resolved_contracts.push(request);
    Ok(())
}
