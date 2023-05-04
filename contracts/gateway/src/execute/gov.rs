use cosmwasm_std::{attr, DepsMut, Env, HexBinary, MessageInfo, Response};

use crate::{
    errors::ContractError,
    state::{assert_owned, PAUSED, PUBLIC_KEY},
    verify::pub_to_addr,
};

pub fn pause(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    expires_at: u64,
) -> Result<Response, ContractError> {
    assert_owned(deps.storage, info.sender.clone())?;

    let mut pause_info = PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    if env.block.time.seconds() >= expires_at {
        return Err(ContractError::InvalidArgument {
            msg: "expires_at must be in the future".to_string(),
        });
    }

    pause_info.paused = true;
    pause_info.expires_at = Some(expires_at);

    PAUSED.save(deps.storage, &pause_info)?;

    let response = Response::new().add_attributes(vec![
        attr("action", "pause"),
        attr("executor", info.sender),
        attr("expires_at", pause_info.expires_at.unwrap().to_string()),
    ]);

    Ok(response)
}

pub fn release(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    assert_owned(deps.storage, info.sender.clone())?;

    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_paused()?;

    PAUSED.save(deps.storage, &Default::default())?;

    let response = Response::new().add_attributes(vec![
        attr("action", "release"),
        attr("executor", info.sender),
    ]);

    Ok(response)
}

pub fn change_public_key(
    deps: DepsMut,
    info: MessageInfo,
    public_key: HexBinary,
) -> Result<Response, ContractError> {
    assert_owned(deps.storage, info.sender.clone())?;

    let public_key_addr = pub_to_addr(public_key.clone().into(), "osmo")?;

    if public_key_addr != info.sender {
        return Err(ContractError::InvalidPubKey {});
    }

    PUBLIC_KEY.save(deps.storage, &public_key)?;

    let response = Response::new().add_attributes(vec![
        attr("action", "change_public_key"),
        attr("executor", info.sender),
        attr("public_key", public_key.to_string()),
    ]);
    Ok(response)
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Storage,
    };

    use crate::state::{PauseInfo, OWNER, PAUSED};

    use super::*;

    const ADDR1: &str = "ADDR1";
    const ADDR2: &str = "ADDR2";

    fn mock_owner(storage: &mut dyn Storage, owner: Addr) {
        OWNER.save(storage, &owner).unwrap();
    }

    #[test]
    fn test_check_authority() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let abuser = Addr::unchecked(ADDR2);

        mock_owner(deps.as_mut().storage, owner);
        let info = mock_info(abuser.as_str(), &[]);

        let unauth_pause = pause(deps.as_mut(), env.clone(), info.clone(), 0).unwrap_err();
        assert!(matches!(unauth_pause, ContractError::Unauthorized {}));

        let unauth_release = release(deps.as_mut(), env, info).unwrap_err();
        assert!(matches!(unauth_release, ContractError::Unauthorized {}));
    }

    #[test]
    fn test_check_paused_state() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let info = mock_info(owner.as_str(), &[]);
        mock_owner(deps.as_mut().storage, owner);

        PAUSED
            .save(
                deps.as_mut().storage,
                &PauseInfo {
                    paused: true,
                    expires_at: Some(env.block.time.seconds() + 1),
                },
            )
            .unwrap();
        let paused_err = pause(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            env.block.time.seconds() + 1,
        )
        .unwrap_err();

        assert!(matches!(paused_err, ContractError::PausedError {}));

        PAUSED
            .save(deps.as_mut().storage, &Default::default())
            .unwrap();
        let unpaused_err = release(deps.as_mut(), env, info).unwrap_err();

        assert!(matches!(unpaused_err, ContractError::NotPausedError {}));
    }

    #[test]
    fn test_pause_expires_at_past() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let info = mock_info(owner.as_str(), &[]);
        mock_owner(deps.as_mut().storage, owner);
        PAUSED
            .save(deps.as_mut().storage, &Default::default())
            .unwrap();

        let response = pause(
            deps.as_mut(),
            env.clone(),
            info,
            env.block.time.seconds() - 1,
        )
        .unwrap_err();

        assert!(matches!(response, ContractError::InvalidArgument { .. }));
    }

    #[test]
    fn test_successfully_pause() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let info = mock_info(owner.as_str(), &[]);

        mock_owner(deps.as_mut().storage, owner.clone());
        PAUSED
            .save(deps.as_mut().storage, &Default::default())
            .unwrap();

        let expires_at: u64 = env.block.time.seconds() + 1;

        let response = pause(deps.as_mut(), env, info, expires_at).unwrap();

        assert_eq!(
            response.attributes,
            vec![
                attr("action", "pause"),
                attr("executor", owner.to_string()),
                attr("expires_at", expires_at.to_string())
            ]
        )
    }

    #[test]
    fn test_successfully_release() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let info = mock_info(owner.as_str(), &[]);

        mock_owner(deps.as_mut().storage, owner.clone());
        let expires_at: u64 = env.block.time.seconds() + 1;

        PAUSED
            .save(
                deps.as_mut().storage,
                &PauseInfo {
                    paused: true,
                    expires_at: Some(expires_at),
                },
            )
            .unwrap();

        let response = release(deps.as_mut(), env, info).unwrap();

        assert_eq!(
            response.attributes,
            vec![
                attr("action", "release"),
                attr("executor", owner.to_string()),
            ]
        )
    }

    #[test]
    fn test_change_public_key_success() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked("osmo134s3q9c56t93v96aksveuk9lp8ngljlnlupphd");
        let info = mock_info(owner.as_str(), &[]);

        mock_owner(deps.as_mut().storage, owner);
        let public_key = HexBinary::from(vec![
            2, 191, 219, 148, 192, 213, 90, 105, 81, 110, 121, 164, 102, 210, 194, 26, 140, 10, 19,
            2, 139, 176, 7, 14, 221, 13, 10, 7, 195, 19, 186, 83, 238,
        ]);

        let result = change_public_key(deps.as_mut(), info.clone(), public_key.clone()).unwrap();

        assert_eq!(
            result.attributes,
            vec![
                attr("action", "change_public_key"),
                attr("executor", info.sender),
                attr("public_key", public_key.to_string()),
            ]
        )
    }

    #[test]
    fn test_change_public_key_failure() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked(ADDR1);
        let not_owner = Addr::unchecked(ADDR2);
        let info = mock_info(not_owner.as_str(), &[]);

        mock_owner(deps.as_mut().storage, owner.clone());
        let public_key = HexBinary::from_hex("ffd265b795c0e3c45f7c362a2bb3b6a7").unwrap();

        let result = change_public_key(deps.as_mut(), info, public_key.clone()).unwrap_err();
        assert!(matches!(result, ContractError::Unauthorized {}));

        // right owner but wrong pubkey
        let right_user_info: MessageInfo = mock_info(owner.as_str(), &[]);
        let result: ContractError =
            change_public_key(deps.as_mut(), right_user_info, public_key).unwrap_err();
        assert!(matches!(result, ContractError::InvalidPubKey {}))
    }
}
