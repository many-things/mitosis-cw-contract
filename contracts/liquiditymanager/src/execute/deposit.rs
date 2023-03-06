use cosmwasm_std::{attr, Addr, Attribute, DepsMut, Env, MessageInfo, Response};

use crate::{
    state::{balances::deposit_balance, PAUSED},
    ContractError,
};

pub fn deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    depositor: Option<Addr>,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    if info.funds.is_empty() {
        return Err(ContractError::AssetNotFound {});
    }

    let depositor = match depositor {
        Some(depositor) => depositor,
        None => info.sender.clone(),
    };

    let deposit_result = deposit_balance(deps.storage, env, info.clone(), depositor.clone())?;
    let deposit_attributes = deposit_result
        .iter()
        .map(|x| Attribute {
            key: x.denom.to_string(),
            value: x.amount.to_string(),
        })
        .collect::<Vec<_>>();

    let response = Response::new()
        .add_attributes(vec![
            attr("action", "deposit"),
            attr("executor", info.sender),
            attr("depositor", depositor),
        ])
        .add_attributes(deposit_attributes);

    Ok(response)
}

#[cfg(test)]
mod test {
    use crate::state::{PauseInfo, PAUSED};
    use cosmwasm_std::{
        attr, coin,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Storage,
    };

    use super::*;

    const ADDR1: &str = "addr1";
    const ADDR2: &str = "addr2";

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
    fn test_deposit_paused() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[]);

        stop(deps.as_mut().storage, env.block.time.seconds());

        let response_error = deposit(deps.as_mut(), env, info, Some(addr)).unwrap_err();
        assert!(matches!(response_error, ContractError::PausedError {}));
    }

    #[test]
    fn test_deposit() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(
            addr.as_str(),
            &[coin(100000, "uosmo"), coin(200000, "uusdc")],
        );

        resume(deps.as_mut().storage, env.block.time.seconds());

        let response = deposit(deps.as_mut(), env, info, Some(addr.clone())).unwrap();
        assert_eq!(
            response.attributes,
            vec![
                attr("action", "deposit"),
                attr("executor", addr.to_string()),
                attr("depositor", addr.to_string()),
                attr("uosmo", "100000"),
                attr("uusdc", "200000"),
            ]
        )
    }

    #[test]
    fn test_deposit_to_other() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let sender = Addr::unchecked(ADDR1);
        let depositor = Addr::unchecked(ADDR2);
        let info = mock_info(
            sender.as_str(),
            &[coin(100000, "uosmo"), coin(200000, "uusdc")],
        );

        resume(deps.as_mut().storage, env.block.time.seconds());

        let response = deposit(deps.as_mut(), env, info, Some(depositor.clone())).unwrap();
        assert_eq!(
            response.attributes,
            vec![
                attr("action", "deposit"),
                attr("executor", sender.to_string()),
                attr("depositor", depositor.to_string()),
                attr("uosmo", "100000"),
                attr("uusdc", "200000"),
            ]
        )
    }
}
