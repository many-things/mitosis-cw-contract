use cosmwasm_std::{to_binary, Addr, Deps, Env, QueryResponse};
use mitosis_interface::liquidity_manager::{
    ConfigResponse, GetBalanceResponse, GetBondResponse, GetUnbondListResponse, GetUnbondResponse,
    PauseInfoResponse,
};

use crate::{
    state::{
        balances::inquiry_balance,
        bond::{query_bond, query_unbond, query_unbonds_by_owner},
        rbac::OWNER,
        ConfigInfo, DenomInfo, CONFIG, DENOM, PAUSED,
    },
    ContractError,
};

pub fn get_config(deps: Deps, _env: Env) -> Result<QueryResponse, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    let config: ConfigInfo = CONFIG.load(deps.storage)?;
    let denom: DenomInfo = DENOM.load(deps.storage)?;

    Ok(to_binary(&ConfigResponse {
        owner,
        unbonding_period: config.unbonding_period,
        denom: denom.denom,
        lp_denom: denom.lp_denom,
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

pub fn get_bonds(deps: Deps, bonder: Addr) -> Result<QueryResponse, ContractError> {
    let result = query_bond(deps.storage, bonder)?;

    Ok(to_binary(&GetBondResponse {
        amount: result.amount,
        bond_time: result.bond_time,
    })?)
}

pub fn get_unbond(deps: Deps, unbond_id: u64) -> Result<QueryResponse, ContractError> {
    let result = query_unbond(deps.storage, unbond_id)?;

    Ok(to_binary(&GetUnbondResponse {
        unbond_id: result.unbond_id,
        owner: result.owner,
        amount: result.amount,
        unbond_time: result.unbond_time,
    })?)
}

pub fn get_unbonds_by_owner(deps: Deps, owner: Addr) -> Result<QueryResponse, ContractError> {
    let results = query_unbonds_by_owner(deps.storage, owner)?;
    let response_items = results
        .into_iter()
        .map(|r| GetUnbondResponse {
            unbond_id: r.unbond_id,
            owner: r.owner,
            amount: r.amount,
            unbond_time: r.unbond_time,
        })
        .collect();

    Ok(to_binary(&GetUnbondListResponse {
        items: response_items,
    })?)
}
