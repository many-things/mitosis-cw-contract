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

#[cfg(test)]
mod test {
    use crate::state::{balances::BALANCE, PauseInfo, PAUSED};
    use cosmwasm_std::{
        attr, coin,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Storage, Uint128,
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

    #[test]
    fn test_withdraw_paused() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[]);

        stop(deps.as_mut().storage, env.block.time.seconds());

        let response_error =
            withdraw(deps.as_mut(), env, info, Some(addr), coin(100000, "uosmo")).unwrap_err();
        assert!(matches!(response_error, ContractError::PausedError {}));
    }

    #[test]
    fn test_withdraw() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        resume(deps.as_mut().storage, env.block.time.seconds());

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[]);
        let amount = coin(50000, DENOM.to_string());

        mock_balances(deps.as_mut().storage);
        let resp = withdraw(deps.as_mut(), env, info, Some(addr.clone()), amount.clone()).unwrap();

        assert_eq!(
            resp.attributes,
            vec![
                attr("action", "withdraw"),
                attr("withdrawer", addr.to_string())
            ]
        );
        assert_eq!(
            resp.messages,
            vec![SubMsg::reply_on_error(
                BankMsg::Send {
                    to_address: addr.to_string(),
                    amount: vec![amount,]
                },
                REPLY_WITHDRAW_SUBMESSAGE_FAILURE
            )]
        );
    }
}
