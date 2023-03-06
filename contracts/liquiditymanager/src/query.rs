use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, Coin, Deps, Env, QueryResponse};

use crate::{
    state::{balances::inquiry_balance, rbac::OWNER, PAUSED},
    ContractError,
};

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
}

#[cw_serde]
pub struct PauseInfoResponse {
    pub paused: bool,
    pub expires_at: Option<u64>,
}

#[cw_serde]
pub struct GetBalanceResponse {
    pub depositor: Addr,
    pub assets: Vec<Coin>,
}

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
