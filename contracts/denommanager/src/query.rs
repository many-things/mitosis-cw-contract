use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, Deps, Env, QueryResponse};

use crate::{
    error::ContractError,
    state::{denoms::convert_denoms, rbac::OWNER},
};

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
}

#[cw_serde]
pub struct ConvertResponse {
    pub token: String,
    pub alias: String,
}

pub fn get_config(deps: Deps, _env: Env) -> Result<QueryResponse, ContractError> {
    let owner = OWNER.load(deps.storage)?;

    Ok(to_binary(&ConfigResponse { owner })?)
}

pub fn get_convert(deps: Deps, _env: Env, token: String) -> Result<QueryResponse, ContractError> {
    let alias = convert_denoms(deps.storage, token.clone())?;

    Ok(to_binary(&ConvertResponse { token, alias })?)
}
