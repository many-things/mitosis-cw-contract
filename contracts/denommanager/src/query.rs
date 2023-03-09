use cosmwasm_std::{to_binary, Deps, Env, QueryResponse};
use mitosis_interface::denommanager::{ConfigResponse, ConvertResponse};

use crate::{
    error::ContractError,
    state::{denoms::convert_denoms, rbac::OWNER},
};

pub fn get_config(deps: Deps, _env: Env) -> Result<QueryResponse, ContractError> {
    let owner = OWNER.load(deps.storage)?;

    Ok(to_binary(&ConfigResponse { owner })?)
}

pub fn get_convert(deps: Deps, _env: Env, token: String) -> Result<QueryResponse, ContractError> {
    let alias = convert_denoms(deps.storage, token.clone())?;

    Ok(to_binary(&ConvertResponse { token, alias })?)
}
