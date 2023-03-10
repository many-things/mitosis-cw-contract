use cosmwasm_std::{to_binary, Addr, Deps, Env, QueryResponse};
use mitosis_interface::{
    denom_manager::ConfigResponse,
    liquidity_manager::{GetBalanceResponse, PauseInfoResponse},
};

use crate::{
    state::{balances::inquiry_balance, rbac::OWNER, PAUSED},
    ContractError,
};

pub fn get_config(deps: Deps, _env: Env) -> Result<QueryResponse, ContractError> {
    let owner = OWNER.load(deps.storage)?;

    Ok(to_binary(&ConfigResponse { owner })?)
}

pub fn get_paused_info(deps: Deps, _env: Env) -> Result<QueryResponse, ContractError> {
    let pause = PAUSED.load(deps.storage)?;

    Ok(to_binary(&PauseInfoResponse {
        paused: pause.paused,
        expires_at: pause.expires_at,
    })?)
}

pub fn get_balance(deps: Deps, env: Env, depositor: Addr) -> Result<QueryResponse, ContractError> {
    let result = inquiry_balance(deps.storage, env, depositor.clone())?;

    Ok(to_binary(&GetBalanceResponse {
        depositor,
        assets: result,
    })?)
}
