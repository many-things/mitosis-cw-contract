use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

use crate::{
    errors::ContractError,
    state::{assert_owned, DENOM_MANAGER, LIQUIDITY_MANAGER, PAUSED},
};

pub fn change_liquidity_manager(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_liquidity_manager: Addr,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    assert_owned(deps.storage, info.sender.clone())?;
    LIQUIDITY_MANAGER.save(deps.storage, &new_liquidity_manager)?;

    let response = Response::new().add_attributes(vec![
        attr("action", "change_liquidity_manager"),
        attr("executor", info.sender),
        attr("new_liquidity_manager", new_liquidity_manager),
    ]);

    Ok(response)
}

pub fn change_denom_manager(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_denom_manager: Addr,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    assert_owned(deps.storage, info.sender.clone())?;
    DENOM_MANAGER.save(deps.storage, &new_denom_manager)?;

    let response = Response::new().add_attributes(vec![
        attr("action", "change_denom_manager"),
        attr("executor", info.sender),
        attr("new_liquidity_manager", new_denom_manager),
    ]);

    Ok(response)
}
