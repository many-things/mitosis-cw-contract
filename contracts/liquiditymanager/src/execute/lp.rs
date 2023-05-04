use cosmwasm_std::{attr, coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};
use cw_utils::must_pay;
use osmosis_std::types::cosmos::bank::v1beta1::MsgSend;

use crate::{
    state::{
        bond::{self},
        DenomInfo, DENOM, PAUSED,
    },
    ContractError,
};

pub fn bond_lp(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    let denom: DenomInfo = DENOM.load(deps.storage)?;
    let balance = must_pay(&info, &denom.lp_denom).map_err(|_| ContractError::DenomNotFound {
        denom: denom.lp_denom.clone(),
    })?;

    let bond_info = bond::bond(deps.storage, env, info.sender.clone(), balance)?;

    let respnose = Response::new().add_attributes(vec![
        attr("action", "bond"),
        attr("executor", info.sender),
        attr("amount", bond_info.amount),
    ]);

    Ok(respnose)
}

pub fn start_unbond_lp(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    let unbond_info = bond::start_unbond(deps.storage, env, info.sender.clone(), amount)?;

    let response = Response::new().add_attributes(vec![
        attr("action", "start_unbond"),
        attr("executor", info.sender),
        attr("unbond_id", unbond_info.unbond_id.to_string()),
        attr("amount", unbond_info.amount),
    ]);

    Ok(response)
}

pub fn finish_unbond_lp(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    unbond_id: u64,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    let denom: DenomInfo = DENOM.load(deps.storage)?;

    let unbond_info =
        bond::finish_unbond(deps.storage, env.clone(), info.sender.clone(), unbond_id)?;

    let message: CosmosMsg = MsgSend {
        from_address: env.contract.address.into_string(),
        to_address: info.sender.clone().into_string(),
        amount: vec![coin(unbond_info.amount.u128(), denom.lp_denom).into()],
    }
    .into();

    let response = Response::new().add_message(message).add_attributes(vec![
        attr("action", "finish_unbond"),
        attr("executor", info.sender),
        attr("unbond_id", unbond_info.unbond_id.to_string()),
        attr("amount", unbond_info.amount),
    ]);

    Ok(response)
}
