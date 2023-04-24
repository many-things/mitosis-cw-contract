use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Env, Storage, Uint128};
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
    _storage: &mut dyn Storage,
    _env: Env,
    _bonder: Addr,
    _amount: Uint128,
) -> Result<BondInfo, ContractError> {
    unimplemented!()
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
