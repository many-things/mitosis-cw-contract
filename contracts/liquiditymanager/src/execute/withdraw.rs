use cosmwasm_std::{Addr, BankMsg, Coin, DepsMut, Env, MessageInfo, Response, SubMsg};

use crate::{
    state::{balances::withdraw_balance, PAUSED},
    ContractError,
};

use super::consts::REPLY_WITHDRAW_SUBMESSAGE_FAILURE;

pub fn withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    withdrawer: Option<Addr>,
    amount: Coin,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    let withdrawer = match withdrawer {
        Some(withdrawer) => withdrawer,
        None => info.sender.clone(),
    };

    let withdraw_result = withdraw_balance(deps.storage, env, info, withdrawer.clone(), amount)?;

    let withdraw_message = BankMsg::Send {
        to_address: withdrawer.to_string(),
        amount: vec![withdraw_result],
    };
    let withdraw_submessage =
        SubMsg::reply_on_error(withdraw_message, REPLY_WITHDRAW_SUBMESSAGE_FAILURE);

    let response = Response::new()
        .add_submessage(withdraw_submessage)
        .add_attribute("action", "withdraw")
        .add_attribute("withdrawer", withdrawer);
    Ok(response)
}
