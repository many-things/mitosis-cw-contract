use cosmwasm_std::{coin, CosmosMsg, DepsMut, Env, MessageInfo, Response};
use cw_utils::one_coin;
use osmosis_std::types::{
    cosmos::bank::v1beta1::MsgSend,
    osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint},
};

use crate::{
    state::{
        delegates::{delegate_balance, undelegate_balance},
        PAUSED,
    },
    state::{DenomInfo, DENOM},
    ContractError,
};

pub fn delegate(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    let denom: DenomInfo = DENOM.load(deps.storage)?;
    let balance = match one_coin(&info) {
        Ok(coin) => {
            if coin.denom != denom.denom {
                return Err(ContractError::DelegateAssetNotMatches {});
            }
            coin
        }
        Err(_) => return Err(ContractError::DelegateAssetNotMatches {}),
    };

    delegate_balance(deps.storage, env.clone(), info.clone(), balance.clone())?;
    let lp_amount = coin(balance.amount.into(), denom.sub_denom);

    let mint_message: CosmosMsg = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(lp_amount.clone().into()),
    }
    .into();

    let send_message: CosmosMsg = MsgSend {
        from_address: env.contract.address.to_string(),
        to_address: info.sender.to_string(),
        amount: vec![lp_amount.into()],
    }
    .into();

    Ok(Response::new()
        .add_messages(vec![mint_message, send_message])
        .add_attribute("method", "delegate")
        .add_attribute("executor", info.sender)
        .add_attribute("amount", balance.amount))
}

pub fn undelegate(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    let denom: DenomInfo = DENOM.load(deps.storage)?;
    let balance = match one_coin(&info) {
        Ok(coin) => {
            if coin.denom != denom.sub_denom {
                return Err(ContractError::DelegateAssetNotMatches {});
            }
            coin
        }
        Err(_) => return Err(ContractError::DelegateAssetNotMatches {}),
    };

    match undelegate_balance(deps.storage, env.clone(), info.clone(), balance.clone()) {
        Ok(amount) => amount,
        Err(_) => return Err(ContractError::InsufficientDelegateAsset {}),
    };

    let burn_message: CosmosMsg = MsgBurn {
        sender: env.clone().contract.address.into_string(),
        amount: Some(balance.clone().into()),
    }
    .into();

    let send_message: CosmosMsg = MsgSend {
        from_address: env.contract.address.into_string(),
        to_address: info.clone().sender.into_string(),
        amount: vec![coin(balance.amount.into(), denom.denom).into()],
    }
    .into();

    Ok(Response::new()
        .add_messages(vec![burn_message, send_message])
        .add_attribute("method", "undelegate")
        .add_attribute("executor", info.sender)
        .add_attribute("amount", balance.amount))
}
