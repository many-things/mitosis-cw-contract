#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Reply, Response};
use cw2::set_contract_version;

use crate::{
    execute::consts::REPLY_WITHDRAW_SUBMESSAGE_FAILURE,
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg},
    state::OWNER,
    ContractError,
};

const CONTRACT_NAME: &str = "mitosis:liquiditymanager";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    OWNER.save(deps.storage, &info.sender)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::execute::{deposit::deposit, withdraw::withdraw};

    match msg {
        ExecuteMsg::Deposit { depositor } => deposit(deps, env, info, depositor),
        ExecuteMsg::Withdraw { withdrawer, amount } => {
            withdraw(deps, env, info, withdrawer, amount)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_WITHDRAW_SUBMESSAGE_FAILURE => Ok(Response::new()),
        id => Err(ContractError::ReplyIdNotFound { id }),
    }
}
