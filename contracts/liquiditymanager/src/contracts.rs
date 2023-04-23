#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{CosmosMsg, Deps, DepsMut, Env, MessageInfo, QueryResponse, Reply, Response};
use cw2::set_contract_version;
use mitosis_interface::liquidity_manager::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

use crate::{
    execute::consts::REPLY_WITHDRAW_SUBMESSAGE_FAILURE,
    state::{rbac::OWNER, PAUSED, SUBDENOM},
    ContractError, CONTRACT_NAME, CONTRACT_VERSION,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    OWNER.save(deps.storage, &info.sender)?;
    PAUSED.save(deps.storage, &Default::default())?;

    let subdenom = format!("factory/{}/{}", info.sender.to_string(), msg.lp_denom);
    SUBDENOM.save(deps.storage, &subdenom)?;

    // Only consider single asset.
    let msg_create_denom: CosmosMsg = MsgCreateDenom {
        sender: info.sender.clone().into(),
        subdenom: msg.lp_denom,
    }
    .into();

    Ok(Response::new()
        .add_message(msg_create_denom)
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::execute::{deposit::deposit, gov, rbac, withdraw::withdraw};

    match msg {
        ExecuteMsg::Deposit { depositor } => deposit(deps, env, info, depositor),
        ExecuteMsg::Withdraw { withdrawer, amount } => {
            withdraw(deps, env, info, withdrawer, amount)
        }
        ExecuteMsg::ChangeOwner { new_owner } => rbac::change_owner(deps, env, info, new_owner),
        ExecuteMsg::GrantRole { role, addr } => rbac::grant_role(deps, env, info, role, addr),
        ExecuteMsg::RevokeRole { role, addr } => rbac::revoke_role(deps, env, info, role, addr),
        ExecuteMsg::Pause { expires_at } => gov::pause(deps, env, info, expires_at),
        ExecuteMsg::Release {} => gov::release(deps, env, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_WITHDRAW_SUBMESSAGE_FAILURE => Ok(Response::new()),
        id => Err(ContractError::ReplyIdNotFound { id }),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use crate::query;

    match msg {
        QueryMsg::GetConfig {} => query::get_config(deps, env),
        QueryMsg::PauseInfo {} => query::get_paused_info(deps, env),
        QueryMsg::GetBalance { depositor } => query::get_balance(deps, env, depositor),
    }
}
