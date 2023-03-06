use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

use crate::{
    state::{rbac::assert_owned, PAUSED},
    ContractError,
};

pub fn pause(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    expires_at: u64,
) -> Result<Response, ContractError> {
    let mut pause_info = PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    assert_owned(deps.storage, info.sender.clone())?;

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
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_paused()?;

    assert_owned(deps.storage, info.sender.clone())?;

    PAUSED.save(deps.storage, &Default::default())?;

    let response = Response::new().add_attributes(vec![
        attr("action", "release"),
        attr("executor", info.sender),
    ]);

    Ok(response)
}
