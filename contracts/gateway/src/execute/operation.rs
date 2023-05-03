use cosmwasm_std::{
    attr, to_binary, Addr, Binary, Coin, CosmosMsg, DepsMut, Env, HexBinary, MessageInfo, Response,
    SubMsg, WasmMsg,
};
use cw_utils::one_coin;
use mitosis_interface::liquidity_manager;

use crate::{
    errors::ContractError,
    state::{assert_owned, context::set_withdraw_info, LIQUIDITY_MANAGER, PUBLIC_KEY},
    verify::sha256_digest,
};

use super::consts::REPLY_WITHDRAW_SUBMESSAGE_SUCCESS;

pub fn send(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    to: String,
    op_id: u64,
    op_args: Vec<Binary>,
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
            attr("op_id", op_id.to_string()),
            attr("op_args", serde_json::to_string(&op_args).unwrap()),
        ]);
    Ok(resp)
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msgs: Vec<CosmosMsg>,
    signature: HexBinary,
) -> Result<Response, ContractError> {
    // Relayer call this method. To withdraw asset from liquidity manager.
    assert_owned(deps.storage, info.sender.clone())?;

    let public_key = PUBLIC_KEY
        .load(deps.storage)
        .map_err(|_| ContractError::PublicKeyNotRegistered {})?;

    let hash = sha256_digest(to_binary(&msgs)?)?;
    let verify = deps
        .api
        .secp256k1_verify(
            &hash,
            signature.to_vec().as_slice(),
            public_key.to_vec().as_slice(),
        )
        .map_err(|_| ContractError::InvalidPubKey {})?;

    if !verify {
        return Err(ContractError::InvalidPubKey {}); // TODO: more specify contract err
    }

    let resp = Response::new().add_messages(msgs).add_attributes(vec![
        attr("action", "execute"),
        attr("executor", info.sender),
    ]);

    Ok(resp)
}

// #[cfg(test)]
// mod test {
//     use crate::state::OWNER;

//     use super::*;
//     use cosmwasm_std::{
//         coin, coins,
//         testing::{mock_dependencies, mock_env, mock_info},
//         Addr, SubMsg,
//     };

//     const ADDR1: &str = "ADDR1";
//     const ADDR2: &str = "ADDR2";

//     #[test]
//     fn test_not_send_assets() {
//         let mut deps = mock_dependencies();
//         let env = mock_env();

//         let addr = Addr::unchecked(ADDR1);
//         let info = mock_info(addr.as_str(), &[]);

//         let not_send_asset_err = send(deps.as_mut(), env, info, String::from(ADDR2)).unwrap_err();
//         assert!(matches!(not_send_asset_err, ContractError::MustPayOne {}))
//     }

//     #[test]
//     fn test_send_single_asset() {
//         let mut deps = mock_dependencies();
//         let env = mock_env();

//         let addr = Addr::unchecked(ADDR1);
//         let contract = Addr::unchecked("contract");
//         let info = mock_info(addr.as_str(), &coins(200000, "uosmo"));
//         let to = String::from(ADDR2);

//         LIQUIDITY_MANAGER
//             .save(deps.as_mut().storage, &contract)
//             .unwrap();

//         let result = send(deps.as_mut(), env.clone(), info.clone(), to.clone()).unwrap();
//         assert_eq!(
//             result.attributes,
//             vec![
//                 attr("action", "send"),
//                 attr("executor", addr),
//                 attr("amount", info.funds[0].to_string()),
//                 attr("to", to)
//             ]
//         );

//         let msg = liquidity_manager::ExecuteMsg::Deposit {
//             depositor: Some(env.contract.address),
//         };
//         assert_eq!(
//             result.messages,
//             vec![SubMsg::new(WasmMsg::Execute {
//                 contract_addr: contract.into_string(),
//                 msg: to_binary(&msg).unwrap(),
//                 funds: info.funds,
//             })]
//         )
//     }

//     #[test]
//     fn test_send_multiple_assets_failure() {
//         let mut deps = mock_dependencies();
//         let env = mock_env();

//         let sender = Addr::unchecked(ADDR1);
//         let contract = Addr::unchecked("contract");
//         let info = mock_info(
//             sender.as_str(),
//             &[coin(200000, "uosmo"), coin(100000, "uusdc")],
//         );
//         let to = String::from(ADDR2);

//         LIQUIDITY_MANAGER
//             .save(deps.as_mut().storage, &contract)
//             .unwrap();

//         let result = send(deps.as_mut(), env, info, to).unwrap_err();
//         assert!(matches!(result, ContractError::MustPayOne {}))
//     }

//     #[test]
//     fn test_execute_not_owned() {
//         let mut deps = mock_dependencies();
//         let env = mock_env();

//         let owner = Addr::unchecked(ADDR1);
//         let sender = Addr::unchecked(ADDR2);

//         let info = mock_info(sender.as_str(), &[]);

//         OWNER.save(deps.as_mut().storage, &owner).unwrap();
//         let result = execute(deps.as_mut(), env, info, sender, coin(100000, "uosmo")).unwrap_err();

//         assert!(matches!(result, ContractError::Unauthorized {}))
//     }

//     #[test]
//     fn test_execute_successfully() {
//         let mut deps = mock_dependencies();
//         let env = mock_env();

//         let owner = Addr::unchecked(ADDR1);
//         let info = mock_info(owner.as_str(), &[]);
//         let contract: Addr = Addr::unchecked("contract");

//         OWNER.save(deps.as_mut().storage, &owner).unwrap();
//         LIQUIDITY_MANAGER
//             .save(deps.as_mut().storage, &contract)
//             .unwrap();
//         let result = execute(
//             deps.as_mut(),
//             env.clone(),
//             info.clone(),
//             owner.clone(),
//             coin(100000, "uosmo"),
//         )
//         .unwrap();

//         assert_eq!(
//             result.attributes,
//             vec![
//                 attr("action", "execute"),
//                 attr("executor", owner.clone()),
//                 attr("receiver", owner)
//             ]
//         );

//         let msg = liquidity_manager::ExecuteMsg::Withdraw {
//             withdrawer: Some(env.contract.address),
//             amount: coin(100000, "uosmo"),
//         };
//         assert_eq!(
//             result.messages,
//             vec![SubMsg::reply_on_success(
//                 WasmMsg::Execute {
//                     contract_addr: contract.into_string(),
//                     msg: to_binary(&msg).unwrap(),
//                     funds: info.funds,
//                 },
//                 REPLY_WITHDRAW_SUBMESSAGE_SUCCESS
//             )]
//         )
//     }
// }
