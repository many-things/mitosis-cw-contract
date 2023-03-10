use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

use crate::{
    errors::ContractError,
    state::{assert_owned, OWNER, PAUSED},
};

pub fn change_owner(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_owner: Addr,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    assert_owned(deps.storage, info.sender.clone())?;
    OWNER.save(deps.storage, &new_owner)?;

    let response = Response::new().add_attributes(vec![
        attr("action", "change_owner"),
        attr("executor", info.sender),
        attr("new_owner", new_owner),
    ]);

    Ok(response)
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        attr,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Storage,
    };

    use crate::state::PauseInfo;

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

    fn mock_owner(storage: &mut dyn Storage, owner: Addr) {
        OWNER.save(storage, &owner).unwrap();
    }

    #[test]
    fn test_rbac_paused() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[]);

        stop(deps.as_mut().storage, env.block.time.seconds());

        let change_owner_err = change_owner(deps.as_mut(), env, info, addr).unwrap_err();
        assert!(matches!(change_owner_err, ContractError::PausedError {}));
    }

    #[test]
    fn test_abuser_change_owner() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let abuser = Addr::unchecked(ADDR2);
        let info = mock_info(abuser.as_str(), &[]);

        resume(deps.as_mut().storage, env.block.time.seconds());
        mock_owner(deps.as_mut().storage, owner);

        let unauthorized_err = change_owner(deps.as_mut(), env, info, abuser).unwrap_err();
        assert!(matches!(unauthorized_err, ContractError::Unauthorized {}))
    }

    #[test]
    fn test_successful_change_owner() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let new_owner = Addr::unchecked(ADDR2);
        let info = mock_info(owner.as_str(), &[]);

        resume(deps.as_mut().storage, env.block.time.seconds());
        mock_owner(deps.as_mut().storage, owner.clone());

        let changed_owner = change_owner(deps.as_mut(), env, info, new_owner.clone()).unwrap();
        assert_eq!(
            changed_owner.attributes,
            vec![
                attr("action", "change_owner"),
                attr("executor", owner.as_str()),
                attr("new_owner", new_owner.as_str())
            ]
        );

        let current_owner = OWNER.load(&deps.storage).unwrap();
        assert_eq!(new_owner, current_owner);
    }
}
