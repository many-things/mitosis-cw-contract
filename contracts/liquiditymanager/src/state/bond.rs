use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Env, StdResult, Storage, Uint128};
use cw_storage_plus::Map;

use crate::ContractError;

pub const BONDS_KEY: &str = "bonds";
pub const BONDS: Map<Addr, BondInfo> = Map::new(BONDS_KEY);

#[cw_serde]
pub struct BondInfo {
    pub amount: Uint128,
    pub bond_time: u64,
    pub unbond_time: Option<u64>,
}

pub fn bond(
    storage: &mut dyn Storage,
    env: Env,
    bonder: Addr,
    amount: Uint128,
) -> StdResult<BondInfo> {
    // TODO: consider more cases
    match BONDS.may_load(storage, bonder.clone())? {
        Some(mut bond) => {
            bond.amount = bond.amount.checked_add(amount)?;
            BONDS.save(storage, bonder, &bond)?;

            Ok(bond)
        }
        None => {
            let bond = BondInfo {
                amount,
                bond_time: env.block.time.seconds(),
                unbond_time: None,
            };
            BONDS.save(storage, bonder, &bond)?;

            Ok(bond)
        }
    }
}

pub fn start_unbond(
    storage: &mut dyn Storage,
    env: Env,
    bonder: Addr,
) -> Result<BondInfo, ContractError> {
    let mut bond_info = BONDS.load(storage, bonder.clone())?;

    if bond_info.unbond_time.is_some() {
        return Err(ContractError::UnbondingAlreadyStarted {});
    }

    bond_info.unbond_time = Some(env.block.time.seconds());
    BONDS.save(storage, bonder, &bond_info)?;

    Ok(bond_info)
}

pub fn finish_unbond(storage: &mut dyn Storage, bonder: Addr) -> StdResult<()> {
    BONDS.remove(storage, bonder);

    Ok(())
}

pub fn query_bond(storage: &dyn Storage, bonder: Addr) -> StdResult<BondInfo> {
    BONDS.load(storage, bonder)
}
