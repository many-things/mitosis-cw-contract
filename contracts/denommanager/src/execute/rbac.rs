use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};

use crate::{
    error::ContractError,
    state::{rbac, PAUSED},
};

pub fn change_owner(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_owner: Addr,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    rbac::assert_owned(deps.storage, info.sender)?;

    rbac::change_owner(deps.storage, new_owner.clone())?;

    let response = Response::new()
        .add_attribute("action", "change_owner")
        .add_attribute("new_owner", new_owner);

    Ok(response)
}

pub fn grant_role(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    role: String,
    approval_addr: Addr,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    rbac::assert_owned(deps.storage, info.sender)?;

    let result = rbac::grant_role(deps.storage, role, approval_addr)?;

    let response = Response::new()
        .add_attribute("action", "grant_role")
        .add_attribute("role", result.0)
        .add_attribute("addr", result.1);

    Ok(response)
}

pub fn revoke_role(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    role: String,
    revoked_addr: Addr,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    rbac::assert_owned(deps.storage, info.sender)?;

    let result = rbac::revoke_role(deps.storage, role, revoked_addr)?;

    let response = Response::new()
        .add_attribute("action", "revoke_role")
        .add_attribute("role", result.0)
        .add_attribute("addr", result.1);

    Ok(response)
}
