use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

use crate::{
    errors::ContractError,
    state::{assert_owned, DENOM_MANAGER, LIQUIDITY_MANAGER, PAUSED},
};

pub fn change_liquidity_manager(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_liquidity_manager: Addr,
) -> Result<Response, ContractError> {
    assert_owned(deps.storage, info.sender.clone())?;

    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    LIQUIDITY_MANAGER.save(deps.storage, &new_liquidity_manager)?;

    let response = Response::new().add_attributes(vec![
        attr("action", "change_liquidity_manager"),
        attr("executor", info.sender),
        attr("new_liquidity_manager", new_liquidity_manager),
    ]);

    Ok(response)
}

pub fn change_denom_manager(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_denom_manager: Addr,
) -> Result<Response, ContractError> {
    assert_owned(deps.storage, info.sender.clone())?;

    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    DENOM_MANAGER.save(deps.storage, &new_denom_manager)?;

    let response = Response::new().add_attributes(vec![
        attr("action", "change_denom_manager"),
        attr("executor", info.sender),
        attr("new_liquidity_manager", new_denom_manager),
    ]);

    Ok(response)
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Storage,
    };

    use crate::state::{PauseInfo, OWNER};

    use super::*;

    const ADDR1: &str = "ADDR1";
    const ADDR2: &str = "ADDR2";

    fn release(storage: &mut dyn Storage) {
        PAUSED.save(storage, &Default::default()).unwrap()
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

    fn mock_owner(storage: &mut dyn Storage, owner: Addr) {
        OWNER.save(storage, &owner).unwrap();
    }

    #[test]
    fn test_authority() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let abuser = Addr::unchecked(ADDR2);
        let contract = Addr::unchecked("contract");

        mock_owner(deps.as_mut().storage, owner);
        let info = mock_info(abuser.as_str(), &[]);

        let change_liquidity =
            change_liquidity_manager(deps.as_mut(), env.clone(), info.clone(), contract.clone())
                .unwrap_err();
        assert!(matches!(change_liquidity, ContractError::Unauthorized {}));

        let change_denom = change_denom_manager(deps.as_mut(), env, info, contract).unwrap_err();
        assert!(matches!(change_denom, ContractError::Unauthorized {}));
    }

    #[test]
    fn test_paused() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let contract = Addr::unchecked("contract");

        mock_owner(deps.as_mut().storage, owner.clone());
        let info = mock_info(owner.as_str(), &[]);

        stop(deps.as_mut().storage, env.block.time.seconds());

        let change_liquidity =
            change_liquidity_manager(deps.as_mut(), env.clone(), info.clone(), contract.clone())
                .unwrap_err();
        assert!(matches!(change_liquidity, ContractError::PausedError {}));

        let change_denom = change_denom_manager(deps.as_mut(), env, info, contract).unwrap_err();
        assert!(matches!(change_denom, ContractError::PausedError {}));
    }

    #[test]
    fn test_change_liquidity_manager() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let contract = Addr::unchecked("contract");

        mock_owner(deps.as_mut().storage, owner.clone());
        let info = mock_info(owner.as_str(), &[]);

        release(deps.as_mut().storage);

        let change_liquidity =
            change_liquidity_manager(deps.as_mut(), env, info, contract.clone()).unwrap();
        assert_eq!(
            change_liquidity.attributes,
            vec![
                attr("action", "change_liquidity_manager"),
                attr("executor", owner),
                attr("new_liquidity_manager", contract.clone()),
            ]
        );
        assert_eq!(contract, LIQUIDITY_MANAGER.load(&deps.storage).unwrap())
    }

    #[test]
    fn test_change_denom_manager() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let contract = Addr::unchecked("contract");

        mock_owner(deps.as_mut().storage, owner.clone());
        let info = mock_info(owner.as_str(), &[]);

        release(deps.as_mut().storage);

        let change_denom =
            change_denom_manager(deps.as_mut(), env, info, contract.clone()).unwrap();
        assert_eq!(
            change_denom.attributes,
            vec![
                attr("action", "change_denom_manager"),
                attr("executor", owner),
                attr("new_liquidity_manager", contract.clone()),
            ]
        );
        assert_eq!(contract, DENOM_MANAGER.load(&deps.storage).unwrap())
    }
}
