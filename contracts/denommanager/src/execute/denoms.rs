use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

use crate::{
    error::ContractError,
    state::{self, rbac::assert_owned, PAUSED},
};

pub fn add_alias(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token: String,
    alias: String,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    assert_owned(deps.storage, info.sender.clone())?;

    let (token, alias) = state::denoms::add_alias(deps.storage, token, alias)?;

    let response = Response::new().add_attributes(vec![
        attr("action", "add_alias"),
        attr("executor", info.sender),
        attr("token", token),
        attr("alias", alias),
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

    use crate::state::{rbac::OWNER, PauseInfo};

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
    fn test_add_alias_paused() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let addr = Addr::unchecked(ADDR1);
        let info = mock_info(addr.as_str(), &[]);

        stop(deps.as_mut().storage, env.block.time.seconds());

        let token = "token".to_string();
        let alias = "alias".to_string();
        let response_error = add_alias(deps.as_mut(), env, info, token, alias).unwrap_err();
        assert!(matches!(response_error, ContractError::PausedError {}));
    }

    #[test]
    fn test_add_alias_not_owned() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let abuser = Addr::unchecked(ADDR2);
        let info = mock_info(abuser.as_str(), &[]);

        resume(deps.as_mut().storage, env.block.time.seconds());
        mock_owner(deps.as_mut().storage, owner);

        let token = "token".to_string();
        let alias = "alias".to_string();
        let response_error = add_alias(deps.as_mut(), env, info, token, alias).unwrap_err();
        assert!(matches!(response_error, ContractError::Unauthorized {}));
    }

    #[test]
    fn test_add_alias() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let info = mock_info(owner.as_str(), &[]);

        resume(deps.as_mut().storage, env.block.time.seconds());
        mock_owner(deps.as_mut().storage, owner.clone());

        let token = "token".to_string();
        let alias = "alias".to_string();
        let response = add_alias(deps.as_mut(), env, info, token.clone(), alias.clone()).unwrap();
        assert_eq!(
            response.attributes,
            vec![
                attr("action", "add_alias"),
                attr("executor", owner),
                attr("token", token),
                attr("alias", alias),
            ]
        )
    }
}
