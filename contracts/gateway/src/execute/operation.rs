use cosmwasm_std::{
    attr, to_binary, CosmosMsg, DepsMut, Env, HexBinary, MessageInfo, Response, WasmMsg,
};
use cw_utils::one_coin;
use mitosis_interface::liquidity_manager;

use crate::{
    errors::ContractError,
    state::{assert_owned, LIQUIDITY_MANAGER, PUBLIC_KEY},
    verify::sha256_digest,
};

pub fn send(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    op_id: u64,
    op_args: Vec<String>,
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
    req_op_id: u64,
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
        attr("req_op_id", req_op_id.to_string()),
    ]);

    Ok(resp)
}

#[cfg(test)]
mod test {
    use crate::state::OWNER;

    use super::*;
    use cosmwasm_std::{
        coin, coins,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, BankMsg, SubMsg,
    };

    const ADDR1: &str = "ADDR1";
    const ADDR2: &str = "ADDR2";

    #[test]
    fn test_not_send_assets() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[]);

        let not_send_asset_err = send(deps.as_mut(), env, info, 1u64, vec![]).unwrap_err();
        assert!(matches!(not_send_asset_err, ContractError::MustPayOne {}))
    }

    #[test]
    fn test_send_single_asset() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let addr = Addr::unchecked(ADDR1);
        let contract = Addr::unchecked("contract");
        let info = mock_info(addr.as_str(), &coins(200000, "uosmo"));

        LIQUIDITY_MANAGER
            .save(deps.as_mut().storage, &contract)
            .unwrap();

        let result = send(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            1u64,
            vec![String::from("hello")],
        )
        .unwrap();

        assert_eq!(
            result.attributes,
            vec![
                attr("action", "send"),
                attr("executor", addr),
                attr("amount", info.funds[0].to_string()),
                attr("op_id", "1"),
                attr(
                    "op_args",
                    serde_json::to_string(&vec![String::from("hello")]).unwrap()
                )
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

        LIQUIDITY_MANAGER
            .save(deps.as_mut().storage, &contract)
            .unwrap();

        let result = send(deps.as_mut(), env, info, 1u64, vec![]).unwrap_err();
        assert!(matches!(result, ContractError::MustPayOne {}))
    }

    #[test]
    fn test_execute_failure() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let sender = Addr::unchecked(ADDR2);

        let info = mock_info(sender.as_str(), &[]);

        PUBLIC_KEY
            .save(deps.as_mut().storage, &HexBinary::from(vec![1u8, 2u8]))
            .unwrap();
        OWNER.save(deps.as_mut().storage, &owner).unwrap();
        let result = execute(
            deps.as_mut(),
            env.clone(),
            info,
            vec![],
            0,
            HexBinary::from_hex("12").unwrap(),
        )
        .unwrap_err();

        assert!(matches!(result, ContractError::Unauthorized {}));

        let info = mock_info(owner.as_str(), &[]);
        let result = execute(
            deps.as_mut(),
            env,
            info,
            vec![],
            0,
            HexBinary::from_hex("12").unwrap(),
        )
        .unwrap_err();

        assert!(matches!(result, ContractError::InvalidPubKey {}))
    }

    #[test]
    fn test_execute_successfully() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let info = mock_info(owner.as_str(), &[]);
        let contract: Addr = Addr::unchecked("contract");
        let public_key = HexBinary::from(vec![
            2, 191, 219, 148, 192, 213, 90, 105, 81, 110, 121, 164, 102, 210, 194, 26, 140, 10, 19,
            2, 139, 176, 7, 14, 221, 13, 10, 7, 195, 19, 186, 83, 238,
        ]);

        OWNER.save(deps.as_mut().storage, &owner).unwrap();
        LIQUIDITY_MANAGER
            .save(deps.as_mut().storage, &contract)
            .unwrap();
        PUBLIC_KEY.save(deps.as_mut().storage, &public_key).unwrap();

        let msgs: Vec<CosmosMsg> = vec![BankMsg::Send {
            to_address: owner.to_string(),
            amount: coins(100000, "uosmo"),
        }
        .into()];

        let result = execute(
            deps.as_mut(),
            env,
            info,
            msgs,
            0,
            HexBinary::from(vec![
                245, 203, 35, 74, 190, 205, 192, 228, 239, 109, 138, 172, 195, 248, 157, 251, 142,
                80, 76, 247, 112, 108, 193, 156, 235, 191, 2, 26, 29, 49, 146, 83, 113, 7, 13, 45,
                130, 54, 230, 228, 81, 193, 132, 88, 49, 202, 195, 198, 46, 13, 152, 54, 178, 68,
                151, 170, 97, 88, 240, 183, 245, 51, 243, 164,
            ]),
        )
        .unwrap();

        assert_eq!(
            result.attributes,
            vec![
                attr("action", "execute"),
                attr("executor", owner.clone()),
                attr("req_op_id", "0")
            ]
        );
        assert_eq!(
            result.messages,
            vec![SubMsg::new(BankMsg::Send {
                to_address: owner.to_string(),
                amount: coins(100000, "uosmo"),
            })]
        )
    }
}
