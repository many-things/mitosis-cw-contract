use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{state::PAUSED, ContractError};

pub fn delegate(deps: DepsMut, env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    unimplemented!();
}

pub fn undelegate(deps: DepsMut, env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    unimplemented!();
}
