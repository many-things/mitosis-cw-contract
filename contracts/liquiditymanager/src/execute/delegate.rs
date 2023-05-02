use cosmwasm_std::{coin, CosmosMsg, DepsMut, Env, MessageInfo, Response};
use cw_utils::must_pay;
use osmosis_std::types::{
    cosmos::bank::v1beta1::MsgSend,
    osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint},
};

use crate::{
    state::PAUSED,
    state::{DenomInfo, DENOM},
    ContractError,
};

pub fn delegate(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    let denom: DenomInfo = DENOM.load(deps.storage)?;
    let balance = must_pay(&info, &denom.denom).unwrap();

    let lp_amount = coin(balance.into(), denom.lp_denom);

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
        .add_attribute("action", "delegate")
        .add_attribute("executor", info.sender)
        .add_attribute("amount", balance))
}

pub fn undelegate(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    let denom: DenomInfo = DENOM.load(deps.storage)?;
    let balance = must_pay(&info, &denom.lp_denom).unwrap();
    let burn_message: CosmosMsg = MsgBurn {
        sender: env.clone().contract.address.into_string(),
        amount: Some(coin(balance.into(), denom.lp_denom).into()),
    }
    .into();

    let send_message: CosmosMsg = MsgSend {
        from_address: env.contract.address.into_string(),
        to_address: info.clone().sender.into_string(),
        amount: vec![coin(balance.into(), denom.denom).into()],
    }
    .into();

    Ok(Response::new()
        .add_messages(vec![burn_message, send_message])
        .add_attribute("action", "undelegate")
        .add_attribute("executor", info.sender)
        .add_attribute("amount", balance))
}

#[cfg(test)]
mod test {
    use crate::state::{DenomInfo, PauseInfo, DENOM, PAUSED};
    use cosmwasm_std::{
        attr, coin,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Storage, SubMsg, Uint128,
    };

    use super::*;
    const ADDR1: &str = "addr1";

    fn mock_denom(storage: &mut dyn Storage, env: Env) -> DenomInfo {
        let denom_info = DenomInfo {
            denom: "uusdc".to_string(),
            lp_denom: format!("factory/{}/uusdc", env.contract.address),
        };

        DENOM.save(storage, &denom_info).unwrap();
        denom_info
    }

    fn resume(storage: &mut dyn Storage, now: u64) {
        PAUSED
            .save(
                storage,
                &PauseInfo {
                    paused: true,
                    expires_at: Some(now - 1000),
                },
            )
            .unwrap()
    }

    fn stop(storage: &mut dyn Storage, now: u64) {
        PAUSED
            .save(
                storage,
                &PauseInfo {
                    paused: true,
                    expires_at: Some(now + 1000),
                },
            )
            .unwrap()
    }

    #[test]
    fn test_delegate_paused() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[coin(200000, "uusdc")]);

        stop(deps.as_mut().storage, env.block.time.seconds());

        let response = delegate(deps.as_mut(), env, info).unwrap_err();
        assert!(matches!(response, ContractError::PausedError {}))
    }

    #[test]
    #[should_panic(expected = "uusdc")]
    fn test_delegate_wrong_coin() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let _denom = mock_denom(deps.as_mut().storage, env.clone());

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[coin(200000, "uosmo")]);

        resume(deps.as_mut().storage, env.block.time.seconds());
        delegate(deps.as_mut(), env, info).unwrap();
    }

    #[test]
    fn test_delegate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let denom = mock_denom(deps.as_mut().storage, env.clone());

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[coin(200000, "uusdc")]);

        resume(deps.as_mut().storage, env.block.time.seconds());

        let response = delegate(deps.as_mut(), env.clone(), info).unwrap();

        assert_eq!(
            response.messages,
            vec![
                SubMsg::new(MsgMint {
                    sender: env.contract.address.to_string(),
                    amount: Some(coin(Uint128::new(200000).into(), denom.clone().lp_denom).into()),
                }),
                SubMsg::new(MsgSend {
                    from_address: env.contract.address.into_string(),
                    to_address: addr.clone().into_string(),
                    amount: vec![coin(Uint128::new(200000).into(), denom.lp_denom).into()],
                })
            ]
        );
        assert_eq!(
            response.attributes,
            vec![
                attr("action", "delegate"),
                attr("executor", addr.to_string()),
                attr("amount", Uint128::new(200000)),
            ]
        )
    }

    #[test]
    fn test_undelegate_paused() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[coin(200000, "uusdc")]);

        stop(deps.as_mut().storage, env.block.time.seconds());

        let response = undelegate(deps.as_mut(), env, info).unwrap_err();
        assert!(matches!(response, ContractError::PausedError {}))
    }

    #[test]
    #[should_panic(expected = "factory/cosmos2contract/uusdc")]
    fn test_undelegate_wrong_coin() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let _denom = mock_denom(deps.as_mut().storage, env.clone());

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[coin(200000, "uosmo")]);

        resume(deps.as_mut().storage, env.block.time.seconds());

        let response = undelegate(deps.as_mut(), env, info).unwrap_err();
        assert!(matches!(
            response,
            ContractError::DelegateAssetNotMatches {}
        ))
    }

    #[test]
    fn test_undelegate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let denom = mock_denom(deps.as_mut().storage, env.clone());

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[coin(200000, denom.lp_denom.clone())]);

        resume(deps.as_mut().storage, env.block.time.seconds());

        let response = undelegate(deps.as_mut(), env.clone(), info).unwrap();

        assert_eq!(
            response.messages,
            vec![
                SubMsg::new(MsgBurn {
                    sender: env.contract.address.to_string(),
                    amount: Some(coin(Uint128::new(200000).into(), denom.clone().lp_denom).into()),
                }),
                SubMsg::new(MsgSend {
                    from_address: env.contract.address.into_string(),
                    to_address: addr.clone().into_string(),
                    amount: vec![coin(Uint128::new(200000).into(), denom.denom).into()],
                })
            ]
        );
        assert_eq!(
            response.attributes,
            vec![
                attr("action", "undelegate"),
                attr("executor", addr.to_string()),
                attr("amount", Uint128::new(200000)),
            ]
        )
    }
}
