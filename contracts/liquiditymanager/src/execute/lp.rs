use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};
use cw_utils::must_pay;

use crate::{
    state::{bond, DenomInfo, DENOM, PAUSED},
    ContractError,
};

pub fn bond_lp(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    let denom: DenomInfo = DENOM.load(deps.storage)?;
    let balance = must_pay(&info, &denom.lp_denom).unwrap();

    let bond_info = bond::bond(deps.storage, env, info.sender.clone(), balance)?;

    let respnose = Response::new().add_attributes(vec![
        attr("action", "bond"),
        attr("executor", info.sender),
        attr("amount", bond_info.amount),
    ]);

    Ok(respnose)
}
