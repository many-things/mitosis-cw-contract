use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use mitosis_interface::gateway::InstantiateMsg;

use crate::{errors::ContractError, state::OWNER, CONTRACT_NAME, CONTRACT_VERSION};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    OWNER.save(deps.storage, &info.sender)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}
