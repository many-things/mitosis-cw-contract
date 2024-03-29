use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

use crate::{
    error::ContractError,
    state::{rbac, PAUSED},
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

    rbac::assert_owned(deps.storage, info.sender.clone())?;

    rbac::change_owner(deps.storage, new_owner.clone())?;

    let response = Response::new().add_attributes(vec![
        attr("action", "change_owner"),
        attr("executor", info.sender),
        attr("new_owner", new_owner),
    ]);

    Ok(response)
}

pub fn grant_role(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    role: String,
    approval_addr: Addr,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    rbac::assert_owned(deps.storage, info.sender.clone())?;

    let (role, addr) = rbac::grant_role(deps.storage, role, approval_addr)?;

    let response = Response::new().add_attributes(vec![
        attr("action", "grant_role"),
        attr("executor", info.sender),
        attr("role", role),
        attr("addr", addr),
    ]);

    Ok(response)
}

pub fn revoke_role(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    role: String,
    revoked_addr: Addr,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    rbac::assert_owned(deps.storage, info.sender.clone())?;

    let (role, addr) = rbac::revoke_role(deps.storage, role, revoked_addr)?;

    let response = Response::new().add_attributes(vec![
        attr("action", "revoke_role"),
        attr("executor", info.sender),
        attr("role", role),
        attr("addr", addr),
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

    use crate::state::{
        rbac::{ADDR_ROLE, GATEWAY_ROLE, OWNER},
        PauseInfo,
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

    fn mock_owner(storage: &mut dyn Storage, owner: Addr) {
        OWNER.save(storage, &owner).unwrap();
    }

    #[test]
    fn test_rbac_paused() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[]);
        let role = GATEWAY_ROLE.to_string();

        stop(deps.as_mut().storage, env.block.time.seconds());

        let change_owner_err =
            change_owner(deps.as_mut(), env.clone(), info.clone(), addr.clone()).unwrap_err();
        assert!(matches!(change_owner_err, ContractError::PausedError {}));

        let grant_role_err = grant_role(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            role.clone(),
            addr.clone(),
        )
        .unwrap_err();
        assert!(matches!(grant_role_err, ContractError::PausedError {}));

        let revoke_role_err = revoke_role(deps.as_mut(), env, info, role, addr).unwrap_err();
        assert!(matches!(revoke_role_err, ContractError::PausedError {}));
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
                attr("executor", owner),
                attr("new_owner", new_owner.as_str())
            ]
        );

        // is it actually changed?
        let current_owner = OWNER.load(&deps.storage).unwrap();
        assert_eq!(new_owner, current_owner);
    }

    #[test]
    fn test_abuser_grant_role() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let abuser = Addr::unchecked(ADDR2);
        let info = mock_info(abuser.as_str(), &[]);
        let role = GATEWAY_ROLE.to_string();

        resume(deps.as_mut().storage, env.block.time.seconds());
        mock_owner(deps.as_mut().storage, owner);

        let unauthorized_err = grant_role(deps.as_mut(), env, info, role, abuser).unwrap_err();
        assert!(matches!(unauthorized_err, ContractError::Unauthorized {}))
    }

    #[test]
    fn test_abuser_revoke_role() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let abuser = Addr::unchecked(ADDR2);
        let info = mock_info(abuser.as_str(), &[]);
        let role = GATEWAY_ROLE.to_string();

        resume(deps.as_mut().storage, env.block.time.seconds());
        mock_owner(deps.as_mut().storage, owner);

        let unauthorized_err = revoke_role(deps.as_mut(), env, info, role, abuser).unwrap_err();
        assert!(matches!(unauthorized_err, ContractError::Unauthorized {}))
    }

    #[test]
    fn test_owner_try_revoke_not_granted_role() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let revoke_addr = Addr::unchecked(ADDR2);
        let info = mock_info(owner.as_str(), &[]);
        let role = GATEWAY_ROLE.to_string();

        resume(deps.as_mut().storage, env.block.time.seconds());
        mock_owner(deps.as_mut().storage, owner);

        let role_not_exist = revoke_role(deps.as_mut(), env, info, role, revoke_addr).unwrap_err();
        assert!(matches!(role_not_exist, ContractError::RoleNotExist { .. }))
    }

    #[test]
    fn test_successfully_grant_role() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let approval_addr = Addr::unchecked(ADDR2);
        let info = mock_info(owner.as_str(), &[]);
        let role = GATEWAY_ROLE.to_string();

        resume(deps.as_mut().storage, env.block.time.seconds());
        mock_owner(deps.as_mut().storage, owner.clone());

        let response = grant_role(
            deps.as_mut(),
            env,
            info,
            role.clone(),
            approval_addr.clone(),
        )
        .unwrap();
        assert_eq!(
            response.attributes,
            vec![
                attr("action", "grant_role"),
                attr("executor", owner),
                attr("role", role),
                attr("addr", approval_addr.to_string())
            ]
        );
    }

    #[test]
    fn test_successfully_revoke_role() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let revoke_addr = Addr::unchecked(ADDR2);
        let info = mock_info(owner.as_str(), &[]);
        let role = GATEWAY_ROLE.to_string();

        resume(deps.as_mut().storage, env.block.time.seconds());
        mock_owner(deps.as_mut().storage, owner.clone());

        ADDR_ROLE
            .save(
                deps.as_mut().storage,
                (role.clone(), revoke_addr.clone()),
                &true,
            )
            .unwrap();

        let response =
            revoke_role(deps.as_mut(), env, info, role.clone(), revoke_addr.clone()).unwrap();
        assert_eq!(
            response.attributes,
            vec![
                attr("action", "revoke_role"),
                attr("executor", owner),
                attr("role", role),
                attr("addr", revoke_addr.to_string())
            ]
        );
    }
}
