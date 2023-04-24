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

pub fn unbond(
    _storage: &mut dyn Storage,
    _env: Env,
    _bonder: Addr,
) -> Result<BondInfo, ContractError> {
    unimplemented!()
}

pub fn query_bond(_storage: &dyn Storage, _bonder: Addr) -> Result<BondInfo, ContractError> {
    unimplemented!()
}
