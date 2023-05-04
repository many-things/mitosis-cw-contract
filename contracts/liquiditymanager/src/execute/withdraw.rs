use cosmwasm_std::{attr, Addr, BankMsg, Coin, DepsMut, Env, MessageInfo, Response};

use crate::{
    state::{balances::withdraw_balance, rbac::assert_owned, PAUSED},
    ContractError,
};

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

    assert_owned(deps.storage, info.sender.clone())?;

    // TODO: can be weakness point
    let withdrawer = match withdrawer {
        Some(withdrawer) => withdrawer,
        None => info.sender.clone(),
    };

    let withdraw_result =
        withdraw_balance(deps.storage, env, info.clone(), withdrawer.clone(), amount)?;

    let withdraw_message = BankMsg::Send {
        to_address: withdrawer.to_string(),
        amount: vec![withdraw_result],
    };

    let response = Response::new()
        .add_message(withdraw_message)
        .add_attributes(vec![
            attr("action", "withdraw"),
            attr("executor", info.sender),
            attr("withdrawer", withdrawer),
        ]);
    Ok(response)
}

#[cfg(test)]
mod test {
    use crate::state::{balances::BALANCE, rbac::OWNER, PauseInfo, PAUSED};
    use cosmwasm_std::{
        attr, coin,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Storage, SubMsg, Uint128,
    };

    use super::*;

    const ADDR1: &str = "addr1";
    const ADDR2: &str = "addr2";
    const DENOM: &str = "uosmo";

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

    fn mock_balances(storage: &mut dyn Storage) {
        let addr1 = Addr::unchecked(ADDR1);
        let addr2 = Addr::unchecked(ADDR2);

        BALANCE
            .save(storage, (addr1, DENOM.to_string()), &Uint128::new(100000))
            .unwrap();
        BALANCE
            .save(storage, (addr2, DENOM.to_string()), &Uint128::new(200000))
            .unwrap();
    }

    fn mock_owner(storage: &mut dyn Storage, owner: Addr) {
        OWNER.save(storage, &owner).unwrap();
    }

    #[test]
    fn test_withdraw_paused() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        resume(deps.as_mut().storage, env.block.time.seconds());

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[]);

        stop(deps.as_mut().storage, env.block.time.seconds());

        let response_error =
            withdraw(deps.as_mut(), env, info, Some(addr), coin(100000, "uosmo")).unwrap_err();
        assert!(matches!(response_error, ContractError::PausedError {}));
    }

    #[test]
    fn test_ownership_unauthorized() {
        let owner_addr = Addr::unchecked(ADDR1);
        let abuser_addr = Addr::unchecked(ADDR2);

        let mut deps = mock_dependencies();

        let info = mock_info(abuser_addr.as_str(), &[]);
        let env = mock_env();

        resume(deps.as_mut().storage, env.block.time.seconds());
        mock_owner(deps.as_mut().storage, owner_addr);

        let try_withdraw = withdraw(
            deps.as_mut(),
            env,
            info,
            None,
            coin(100000, DENOM.to_string()),
        )
        .unwrap_err();

        println!("{}", try_withdraw);

        assert!(matches!(try_withdraw, ContractError::Unauthorized {}));
    }

    #[test]
    fn test_withdraw() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        resume(deps.as_mut().storage, env.block.time.seconds());

        let sender = Addr::unchecked(ADDR1);
        let withdrawer = Addr::unchecked(ADDR2);
        let info = mock_info(sender.as_str(), &[]);
        let amount = coin(50000, DENOM.to_string());

        mock_owner(deps.as_mut().storage, sender.clone());
        mock_balances(deps.as_mut().storage);

        // Test unspecified accounts wallet
        let resp = withdraw(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            None,
            amount.clone(),
        )
        .unwrap();

        assert_eq!(
            resp.attributes,
            vec![
                attr("action", "withdraw"),
                attr("executor", sender.to_string()),
                attr("withdrawer", sender.to_string())
            ]
        );
        assert_eq!(
            resp.messages,
            vec![SubMsg::new(BankMsg::Send {
                to_address: sender.to_string(),
                amount: vec![amount.clone(),]
            })]
        );

        // Test speicifed account wallet

        let resp = withdraw(
            deps.as_mut(),
            env,
            info,
            Some(withdrawer.clone()),
            amount.clone(),
        )
        .unwrap();

        assert_eq!(
            resp.attributes,
            vec![
                attr("action", "withdraw"),
                attr("executor", sender.to_string()),
                attr("withdrawer", withdrawer.to_string())
            ]
        );
        assert_eq!(
            resp.messages,
            vec![SubMsg::new(BankMsg::Send {
                to_address: withdrawer.to_string(),
                amount: vec![amount,]
            })]
        );
    }

    #[test]
    fn test_withdraw_not_exist_balance() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        resume(deps.as_mut().storage, env.block.time.seconds());

        let sender = Addr::unchecked(ADDR1);
        let withdrawer = Addr::unchecked(ADDR2);
        let info = mock_info(sender.as_str(), &[]);
        let amount = coin(300000, DENOM.to_string());

        mock_owner(deps.as_mut().storage, sender);
        let resp = withdraw(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            None,
            amount.clone(),
        )
        .unwrap_err();
        assert!(matches!(resp, ContractError::DepositAssetNotFound { .. }));

        let resp = withdraw(deps.as_mut(), env, info, Some(withdrawer), amount).unwrap_err();
        assert!(matches!(resp, ContractError::DepositAssetNotFound { .. }));

        mock_balances(deps.as_mut().storage);
    }

    #[test]
    fn test_withdraw_insufficient_balance() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        resume(deps.as_mut().storage, env.block.time.seconds());

        let sender = Addr::unchecked(ADDR1);
        let withdrawer = Addr::unchecked(ADDR2);
        let info = mock_info(sender.as_str(), &[]);
        let amount = coin(300000, DENOM.to_string());

        mock_owner(deps.as_mut().storage, sender);
        mock_balances(deps.as_mut().storage);

        let resp = withdraw(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            None,
            amount.clone(),
        )
        .unwrap_err();
        assert!(matches!(
            resp,
            ContractError::InsufficientWithdrawableAsset { .. }
        ));

        let resp = withdraw(deps.as_mut(), env, info, Some(withdrawer), amount).unwrap_err();
        assert!(matches!(
            resp,
            ContractError::InsufficientWithdrawableAsset { .. }
        ));
    }
}
