use cosmwasm_std::{
    attr, to_binary, Addr, Coin, Deps, Env, MessageInfo, Response, SubMsg, WasmMsg,
};
use mitosis_interface::liquidity_manager;

use crate::{
    errors::ContractError,
    state::{assert_owned, LIQUIDITY_MANAGER},
};

use super::consts::REPLY_WITHDRAW_SUBMESSAGE_SUCCESS;

pub fn send(
    deps: Deps,
    env: Env,
    info: MessageInfo,
    to: Option<Addr>,
) -> Result<Response, ContractError> {
    if info.funds.is_empty() {
        return Err(ContractError::MustPay {});
    }

    // TODO: can be weakness point
    let depositor = match to {
        Some(addr) => addr,
        None => info.sender.clone(),
    };

    let msg = liquidity_manager::ExecuteMsg::Deposit {
        depositor: Some(env.contract.address),
    };

    let lmgr = LIQUIDITY_MANAGER.load(deps.storage)?;
    let deposit_attributes = serde_json::to_string(&info.funds).unwrap();

    let resp = Response::new()
        .add_message(WasmMsg::Execute {
            contract_addr: lmgr.into_string(),
            msg: to_binary(&msg)?,
            funds: info.funds,
        })
        .add_attributes(vec![
            attr("action", "send"),
            attr("executor", info.sender),
            attr("assets", deposit_attributes),
        ]);
    Ok(resp)
}

pub fn receive(
    deps: Deps,
    _env: Env,
    info: MessageInfo,
    to: Addr,
    amount: Coin,
) -> Result<Response, ContractError> {
    // Relayer call this method. To withdraw asset from liquidity manager.
    assert_owned(deps.storage, info.sender.clone())?;

    let lmgr = LIQUIDITY_MANAGER.load(deps.storage)?;
    let liquidity_msg = liquidity_manager::ExecuteMsg::Withdraw {
        withdrawer: Some(info.sender.clone()),
        amount,
    };

    let msg = WasmMsg::Execute {
        contract_addr: lmgr.into_string(),
        msg: to_binary(&liquidity_msg)?,
        funds: vec![],
    };

    let submessage = SubMsg::reply_on_success(msg, REPLY_WITHDRAW_SUBMESSAGE_SUCCESS);

    Ok(Response::new()
        .add_submessage(submessage)
        .add_attributes(vec![
            attr("action", "receive"),
            attr("executor", info.sender),
            attr("receiver", to),
        ]))
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::{
        coin, coins,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, SubMsg,
    };

    const ADDR1: &str = "ADDR1";
    const ADDR2: &str = "ADDR2";

    #[test]
    fn test_not_send_assets() {
        let deps = mock_dependencies();
        let env = mock_env();

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[]);

        let not_send_asset_err = send(deps.as_ref(), env, info, None).unwrap_err();
        assert!(matches!(not_send_asset_err, ContractError::MustPay {}))
    }

    #[test]
    fn test_single_asset_to_sender() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let addr = Addr::unchecked(ADDR1);
        let contract = Addr::unchecked("contract");
        let info = mock_info(addr.as_str(), &coins(200000, "uosmo"));

        LIQUIDITY_MANAGER
            .save(deps.as_mut().storage, &contract)
            .unwrap();

        let result = send(deps.as_ref(), env, info.clone(), None).unwrap();
        assert_eq!(
            result.attributes,
            vec![
                attr("action", "send"),
                attr("executor", addr.clone()),
                attr("assets", serde_json::to_string(&info.funds).unwrap())
            ]
        );

        let msg = liquidity_manager::ExecuteMsg::Deposit {
            depositor: Some(addr),
        };
        assert_eq!(
            result.messages,
            vec![SubMsg::new(WasmMsg::Execute {
                contract_addr: contract.into_string(),
                msg: to_binary(&msg).unwrap(),
                funds: info.funds,
            })]
        )
    }

    #[test]
    fn test_single_asset_to_other() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let sender = Addr::unchecked(ADDR1);
        let receiver = Addr::unchecked(ADDR2);
        let contract = Addr::unchecked("contract");
        let info = mock_info(sender.as_str(), &coins(200000, "uosmo"));

        LIQUIDITY_MANAGER
            .save(deps.as_mut().storage, &contract)
            .unwrap();

        let result = send(deps.as_ref(), env, info.clone(), Some(receiver.clone())).unwrap();
        assert_eq!(
            result.attributes,
            vec![
                attr("action", "send"),
                attr("executor", sender),
                attr("assets", serde_json::to_string(&info.funds).unwrap())
            ]
        );

        let msg = liquidity_manager::ExecuteMsg::Deposit {
            depositor: Some(receiver),
        };
        assert_eq!(
            result.messages,
            vec![SubMsg::new(WasmMsg::Execute {
                contract_addr: contract.into_string(),
                msg: to_binary(&msg).unwrap(),
                funds: info.funds,
            })]
        )
    }

    #[test]
    fn test_multiple_asset_to_sender() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let sender = Addr::unchecked(ADDR1);
        let contract = Addr::unchecked("contract");
        let info = mock_info(
            sender.as_str(),
            &[coin(200000, "uosmo"), coin(100000, "uusdc")],
        );

        LIQUIDITY_MANAGER
            .save(deps.as_mut().storage, &contract)
            .unwrap();

        let result = send(deps.as_ref(), env, info.clone(), None).unwrap();
        assert_eq!(
            result.attributes,
            vec![
                attr("action", "send"),
                attr("executor", sender.clone()),
                attr("assets", serde_json::to_string(&info.funds).unwrap())
            ]
        );

        let msg = liquidity_manager::ExecuteMsg::Deposit {
            depositor: Some(sender),
        };
        assert_eq!(
            result.messages,
            vec![SubMsg::new(WasmMsg::Execute {
                contract_addr: contract.into_string(),
                msg: to_binary(&msg).unwrap(),
                funds: info.funds,
            })]
        )
    }

    #[test]
    fn test_multiple_asset_to_other() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let sender = Addr::unchecked(ADDR1);
        let receiver = Addr::unchecked(ADDR2);
        let contract = Addr::unchecked("contract");
        let info = mock_info(
            sender.as_str(),
            &[coin(200000, "uosmo"), coin(100000, "uusdc")],
        );

        LIQUIDITY_MANAGER
            .save(deps.as_mut().storage, &contract)
            .unwrap();

        let result = send(deps.as_ref(), env, info.clone(), Some(receiver.clone())).unwrap();
        assert_eq!(
            result.attributes,
            vec![
                attr("action", "send"),
                attr("executor", sender),
                attr("assets", serde_json::to_string(&info.funds).unwrap())
            ]
        );

        let msg = liquidity_manager::ExecuteMsg::Deposit {
            depositor: Some(receiver),
        };
        assert_eq!(
            result.messages,
            vec![SubMsg::new(WasmMsg::Execute {
                contract_addr: contract.into_string(),
                msg: to_binary(&msg).unwrap(),
                funds: info.funds,
            })]
        )
    }
}
