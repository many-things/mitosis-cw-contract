use cosmwasm_std::{attr, coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg};
use cw_utils::must_pay;
use osmosis_std::types::cosmos::bank::v1beta1::MsgSend;

use crate::{
    state::{
        bond::{self, BONDS},
        ConfigInfo, DenomInfo, CONFIG, DENOM, PAUSED,
    },
    ContractError,
};

pub fn bond_lp(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    let denom: DenomInfo = DENOM.load(deps.storage)?;
    let balance = must_pay(&info, &denom.lp_denom).unwrap();

    let bond_info = bond::bond(deps.storage, env, info.sender.clone(), balance)?;

    let respnose = Response::new().add_attributes(vec![
        attr("action", "bond_lp"),
        attr("executor", info.sender),
        attr("amount", bond_info.amount),
    ]);

    Ok(respnose)
}

pub fn start_unbond_lp(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    let bond_info = bond::start_unbond(deps.storage, env, info.sender.clone())?;

    let response = Response::new().add_attributes(vec![
        attr("action", "start_unbond_lp"),
        attr("executor", info.sender),
        attr("amount", bond_info.amount),
        attr("unbond_time", bond_info.unbond_time.unwrap().to_string()),
    ]);

    Ok(response)
}

pub fn finish_unbond_lp(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    let config: ConfigInfo = CONFIG.load(deps.storage)?;
    let denom: DenomInfo = DENOM.load(deps.storage)?;
    let bond_info = BONDS.load(deps.storage, info.sender.clone())?;

    // Not unbond yet
    if bond_info.unbond_time.is_none() {
        return Err(ContractError::UnbondingNotStarted {});
    }

    let unbonding_finished_time = bond_info
        .unbond_time
        .unwrap()
        .checked_add(config.unbonding_period)
        .unwrap();

    if unbonding_finished_time > env.block.time.seconds() {
        return Err(ContractError::UnbondingNotFinished {});
    }

    bond::finish_unbond(deps.storage, info.sender.clone())?;

    let message: CosmosMsg = MsgSend {
        from_address: env.contract.address.into_string(),
        to_address: info.sender.clone().into_string(),
        amount: vec![coin(bond_info.amount.u128(), denom.lp_denom).into()],
    }
    .into();

    let response = Response::new().add_message(message).add_attributes(vec![
        attr("action", "finish_unbond_pl"),
        attr("executor", info.sender),
        attr("amount", bond_info.amount),
    ]);

    Ok(response)
}
