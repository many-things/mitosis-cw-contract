use cosmwasm_std::{to_binary, Addr, Deps, Env, QueryResponse};
use mitosis_interface::liquidity_manager::{ConfigResponse, GetBalanceResponse, PauseInfoResponse};

use crate::{
    state::{balances::inquiry_balance, rbac::OWNER, ConfigInfo, CONFIG, PAUSED},
    ContractError,
};

pub fn get_config(deps: Deps, _env: Env) -> Result<QueryResponse, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    let config: ConfigInfo = CONFIG.load(deps.storage)?;

    Ok(to_binary(&ConfigResponse {
        owner,
        unbonding_period: config.unbonding_period,
    })?)
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
