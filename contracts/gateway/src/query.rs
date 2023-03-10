use cosmwasm_std::{to_binary, Deps, Env, QueryResponse};
use mitosis_interface::gateway::ConfigResponse;

use crate::{
    errors::ContractError,
    state::{DENOM_MANAGER, LIQUIDITY_MANAGER, OWNER},
};

pub fn get_config(deps: Deps, _env: Env) -> Result<QueryResponse, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    let liquidity_manager = LIQUIDITY_MANAGER.load(deps.storage)?;
    let denom_manager = DENOM_MANAGER.load(deps.storage)?;

    Ok(to_binary(&ConfigResponse {
        owner,
        liquidity_manager,
        denom_manager,
    })?)
}
