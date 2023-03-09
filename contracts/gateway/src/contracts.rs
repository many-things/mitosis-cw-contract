use cosmwasm_std::{entry_point, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response};
use cw2::set_contract_version;
use mitosis_interface::gateway::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::{
    errors::ContractError,
    state::{DENOM_MANAGER, LIQUIDITY_MANAGER, OWNER},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    OWNER.save(deps.storage, &info.sender)?;
    LIQUIDITY_MANAGER.save(deps.storage, &msg.liquidity_manager)?;
    DENOM_MANAGER.save(deps.storage, &msg.denom_manager)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::execute::{gov, managers, rbac};

    match msg {
        ExecuteMsg::ChangeOwner { new_owner } => rbac::change_owner(deps, env, info, new_owner),
        ExecuteMsg::ChangeLiquidityManager {
            new_liquidity_manager,
        } => managers::change_liquidity_manager(deps, env, info, new_liquidity_manager),
        ExecuteMsg::ChangeDenomManager { new_denom_manager } => {
            managers::change_denom_manager(deps, env, info, new_denom_manager)
        }
        ExecuteMsg::Pause { expires_at } => gov::pause(deps, env, info, expires_at),
        ExecuteMsg::Releaes {} => gov::release(deps, env, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use crate::query;

    match msg {
        QueryMsg::Config {} => query::get_config(deps, env),
    }
}
