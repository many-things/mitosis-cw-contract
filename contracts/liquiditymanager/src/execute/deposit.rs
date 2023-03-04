use cosmwasm_std::{Addr, Attribute, DepsMut, Env, MessageInfo, Response};

use crate::{
    state::{balances::deposit_balance, PAUSED},
    ContractError,
};

pub fn deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    depositor: Option<Addr>,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    if info.funds.is_empty() {
        return Err(ContractError::AssetNotFound {});
    }

    let depositor = match depositor {
        Some(depositor) => depositor,
        None => info.sender.clone(),
    };

    let deposit_result = deposit_balance(deps.storage, env, info, depositor.clone())?;
    let deposit_attributes = deposit_result
        .iter()
        .map(|x| Attribute {
            key: x.denom.to_string(),
            value: x.amount.to_string(),
        })
        .collect::<Vec<_>>();

    let response = Response::new()
        .add_attribute("action", "deposit")
        .add_attribute("depositor", depositor)
        .add_attributes(deposit_attributes);
    Ok(response)
}
