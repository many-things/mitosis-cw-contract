use cosmwasm_std::{
    attr, to_binary, Addr, Coin, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg,
};
use cw_utils::one_coin;
use mitosis_interface::liquidity_manager;

use crate::{
    errors::ContractError,
    state::{assert_owned, context::set_withdraw_info, LIQUIDITY_MANAGER},
};

use super::consts::REPLY_WITHDRAW_SUBMESSAGE_SUCCESS;

pub fn send(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    to: String,
) -> Result<Response, ContractError> {
    let amount = one_coin(&info).map_err(|_| ContractError::MustPayOne {})?;

    let msg = liquidity_manager::ExecuteMsg::Deposit {
        depositor: Some(env.contract.address),
    };

    let lmgr = LIQUIDITY_MANAGER.load(deps.storage)?;

    let resp = Response::new()
        .add_message(WasmMsg::Execute {
            contract_addr: lmgr.into_string(),
            msg: to_binary(&msg)?,
            funds: info.funds,
        })
        .add_attributes(vec![
            attr("action", "send"),
            attr("executor", info.sender),
            attr("amount", amount.to_string()),
            attr("to", to),
        ]);
    Ok(resp)
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    to: Addr,
    amount: Coin,
) -> Result<Response, ContractError> {
    // Relayer call this method. To withdraw asset from liquidity manager.
    assert_owned(deps.storage, info.sender.clone())?;

    let lmgr = LIQUIDITY_MANAGER.load(deps.storage)?;
    let liquidity_msg = liquidity_manager::ExecuteMsg::Withdraw {
        withdrawer: Some(env.contract.address),
        amount: amount.clone(),
    };

    let msg = WasmMsg::Execute {
        contract_addr: lmgr.into_string(),
        msg: to_binary(&liquidity_msg)?,
        funds: vec![],
    };

    let submessage = SubMsg::reply_on_success(msg, REPLY_WITHDRAW_SUBMESSAGE_SUCCESS);
    set_withdraw_info(deps.storage, to.clone(), amount)?;

    Ok(Response::new()
        .add_submessage(submessage)
        .add_attributes(vec![
            attr("action", "execute"),
            attr("executor", info.sender),
            attr("receiver", to),
        ]))
}

#[cfg(test)]
mod test {
    use crate::state::OWNER;

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
        let mut deps = mock_dependencies();
        let env = mock_env();

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[]);

        let not_send_asset_err = send(deps.as_mut(), env, info, String::from(ADDR2)).unwrap_err();
        assert!(matches!(not_send_asset_err, ContractError::MustPayOne {}))
    }

    #[test]
    fn test_send_single_asset() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let addr = Addr::unchecked(ADDR1);
        let contract = Addr::unchecked("contract");
        let info = mock_info(addr.as_str(), &coins(200000, "uosmo"));
        let to = String::from(ADDR2);

        LIQUIDITY_MANAGER
            .save(deps.as_mut().storage, &contract)
            .unwrap();

        let result = send(deps.as_mut(), env.clone(), info.clone(), to.clone()).unwrap();
        assert_eq!(
            result.attributes,
            vec![
                attr("action", "send"),
                attr("executor", addr),
                attr("amount", info.funds[0].to_string()),
                attr("to", to)
            ]
        );

        let msg = liquidity_manager::ExecuteMsg::Deposit {
            depositor: Some(env.contract.address),
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
    fn test_send_multiple_assets_failure() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let sender = Addr::unchecked(ADDR1);
        let contract = Addr::unchecked("contract");
        let info = mock_info(
            sender.as_str(),
            &[coin(200000, "uosmo"), coin(100000, "uusdc")],
        );
        let to = String::from(ADDR2);

        LIQUIDITY_MANAGER
            .save(deps.as_mut().storage, &contract)
            .unwrap();

        let result = send(deps.as_mut(), env, info, to).unwrap_err();
        assert!(matches!(result, ContractError::MustPayOne {}))
    }

    #[test]
    fn test_execute_not_owned() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let sender = Addr::unchecked(ADDR2);

        let info = mock_info(sender.as_str(), &[]);

        OWNER.save(deps.as_mut().storage, &owner).unwrap();
        let result = execute(deps.as_mut(), env, info, sender, coin(100000, "uosmo")).unwrap_err();

        assert!(matches!(result, ContractError::Unauthorized {}))
    }

    #[test]
    fn test_execute_successfully() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let info = mock_info(owner.as_str(), &[]);
        let contract: Addr = Addr::unchecked("contract");

        OWNER.save(deps.as_mut().storage, &owner).unwrap();
        LIQUIDITY_MANAGER
            .save(deps.as_mut().storage, &contract)
            .unwrap();
        let result = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            owner.clone(),
            coin(100000, "uosmo"),
        )
        .unwrap();

        assert_eq!(
            result.attributes,
            vec![
                attr("action", "execute"),
                attr("executor", owner.clone()),
                attr("receiver", owner)
            ]
        );

        let msg = liquidity_manager::ExecuteMsg::Withdraw {
            withdrawer: Some(env.contract.address),
            amount: coin(100000, "uosmo"),
        };
        assert_eq!(
            result.messages,
            vec![SubMsg::reply_on_success(
                WasmMsg::Execute {
                    contract_addr: contract.into_string(),
                    msg: to_binary(&msg).unwrap(),
                    funds: info.funds,
                },
                REPLY_WITHDRAW_SUBMESSAGE_SUCCESS
            )]
        )
    }
}
