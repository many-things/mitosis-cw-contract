use cosmwasm_std::{attr, Addr, DepsMut, Env, HexBinary, MessageInfo, Response};

use crate::{
    errors::ContractError,
    state::{assert_owned, OWNER, PAUSED, PUBLIC_KEY},
    verify::pub_to_addr,
};

pub fn change_owner(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_owner: Addr,
    new_pubkey: HexBinary,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    assert_owned(deps.storage, info.sender.clone())?;

    let public_key_addr = pub_to_addr(new_pubkey.clone().into(), "osmo")?;

    if public_key_addr != new_owner {
        return Err(ContractError::InvalidPubKey {});
    }

    OWNER.save(deps.storage, &new_owner)?;
    PUBLIC_KEY.save(deps.storage, &new_pubkey)?;

    let response = Response::new().add_attributes(vec![
        attr("action", "change_owner"),
        attr("executor", info.sender),
        attr("new_owner", new_owner),
        attr("new_public_key", new_pubkey.to_hex()),
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
        let dummy_pubkey = HexBinary::from_hex("12").unwrap();

        stop(deps.as_mut().storage, env.block.time.seconds());

        let change_owner_err =
            change_owner(deps.as_mut(), env, info, addr, dummy_pubkey).unwrap_err();
        assert!(matches!(change_owner_err, ContractError::PausedError {}));
    }

    #[test]
    fn test_abuser_change_owner() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let abuser = Addr::unchecked(ADDR2);
        let info = mock_info(abuser.as_str(), &[]);
        let dummy_pubkey = HexBinary::from_hex("12").unwrap();

        resume(deps.as_mut().storage, env.block.time.seconds());
        mock_owner(deps.as_mut().storage, owner);

        let unauthorized_err =
            change_owner(deps.as_mut(), env, info, abuser, dummy_pubkey).unwrap_err();
        assert!(matches!(unauthorized_err, ContractError::Unauthorized {}))
    }

    #[test]
    fn test_successful_change_owner() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let new_owner = Addr::unchecked("osmo134s3q9c56t93v96aksveuk9lp8ngljlnlupphd");
        let info = mock_info(owner.as_str(), &[]);
        let public_key = HexBinary::from(vec![
            2, 191, 219, 148, 192, 213, 90, 105, 81, 110, 121, 164, 102, 210, 194, 26, 140, 10, 19,
            2, 139, 176, 7, 14, 221, 13, 10, 7, 195, 19, 186, 83, 238,
        ]);

        resume(deps.as_mut().storage, env.block.time.seconds());
        mock_owner(deps.as_mut().storage, owner.clone());

        let changed_owner = change_owner(
            deps.as_mut(),
            env,
            info,
            new_owner.clone(),
            public_key.clone(),
        )
        .unwrap();
        assert_eq!(
            changed_owner.attributes,
            vec![
                attr("action", "change_owner"),
                attr("executor", owner.as_str()),
                attr("new_owner", new_owner.as_str()),
                attr("new_public_key", public_key.to_hex()),
            ]
        );
    }

    #[test]
    fn test_fail_pubkey_change_owner() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let owner = Addr::unchecked(ADDR1);
        let new_owner = Addr::unchecked(ADDR2);
        let info = mock_info(owner.as_str(), &[]);
        let public_key = HexBinary::from(vec![
            2, 191, 219, 148, 192, 213, 90, 105, 81, 110, 121, 164, 102, 210, 194, 26, 140, 10, 19,
            2, 139, 176, 7, 14, 221, 13, 10, 7, 195, 19, 186, 83, 238,
        ]);

        resume(deps.as_mut().storage, env.block.time.seconds());
        mock_owner(deps.as_mut().storage, owner);

        let changed_owner =
            change_owner(deps.as_mut(), env, info, new_owner, public_key).unwrap_err();
        assert!(matches!(changed_owner, ContractError::InvalidPubKey {}))
    }
}
