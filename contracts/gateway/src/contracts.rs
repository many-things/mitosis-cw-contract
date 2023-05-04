use cosmwasm_std::{
    attr, entry_point, BankMsg, Deps, DepsMut, Env, MessageInfo, QueryResponse, Reply, Response,
};
use cw2::set_contract_version;
use mitosis_interface::gateway::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::{
    errors::ContractError,
    execute::consts::REPLY_WITHDRAW_SUBMESSAGE_SUCCESS,
    state::{context::WITHDRAW, DENOM_MANAGER, LIQUIDITY_MANAGER, OWNER, PUBLIC_KEY},
    verify::pub_to_addr,
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

    let public_key_addr = pub_to_addr(msg.public_key.clone().into(), "osmo")?;

    if public_key_addr != info.sender {
        return Err(ContractError::InvalidPubKey {});
    }

    PUBLIC_KEY.save(deps.storage, &msg.public_key)?;
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
    use crate::execute::{gov, managers, operation, rbac};

    match msg {
        ExecuteMsg::ChangeOwner {
            new_owner,
            new_public_key,
        } => rbac::change_owner(deps, env, info, new_owner, new_public_key),
        ExecuteMsg::ChangeLiquidityManager {
            new_liquidity_manager,
        } => managers::change_liquidity_manager(deps, env, info, new_liquidity_manager),
        ExecuteMsg::ChangeDenomManager { new_denom_manager } => {
            managers::change_denom_manager(deps, env, info, new_denom_manager)
        }
        ExecuteMsg::Send { to, op_id, op_args } => {
            operation::send(deps, env, info, to, op_id, op_args)
        }
        ExecuteMsg::Execute { msgs, signature } => {
            operation::execute(deps, env, info, msgs, signature)
        }
        ExecuteMsg::Pause { expires_at } => gov::pause(deps, env, info, expires_at),
        ExecuteMsg::Release {} => gov::release(deps, env, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_WITHDRAW_SUBMESSAGE_SUCCESS => {
            let withdraw_context = WITHDRAW.load(deps.storage)?;

            let send_msg = BankMsg::Send {
                to_address: withdraw_context.to_address.clone().into_string(),
                amount: vec![withdraw_context.amount.clone()],
            };

            let resp = Response::new().add_message(send_msg).add_attributes(vec![
                attr("action", "reply_withdraw"),
                attr("to", withdraw_context.to_address),
                attr("amount", withdraw_context.amount.to_string()),
            ]);
            Ok(resp)
        }
        id => Err(ContractError::ReplyIdNotFound { id }),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use crate::query;

    match msg {
        QueryMsg::GetConfig {} => query::get_config(deps, env),
    }
}
