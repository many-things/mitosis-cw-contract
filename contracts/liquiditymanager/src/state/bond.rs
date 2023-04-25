use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Env, Order, StdResult, Storage, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};

use crate::ContractError;

use super::CONFIG;

pub const BONDS_KEY: &str = "bonds";
pub const BONDS: Map<Addr, BondInfo> = Map::new(BONDS_KEY);

pub const UNBONDS_KEY: &str = "unbonds";
pub const UNBONDS_ID_KEY: &str = "unbond_index";
pub const UNBONDS_ID: Item<u64> = Item::new(UNBONDS_ID_KEY);

#[cw_serde]
pub struct BondInfo {
    pub amount: Uint128,
    pub bond_time: u64,
}

#[cw_serde]
pub struct UnbondInfo {
    pub unbond_id: u64,
    pub owner: Addr,
    pub amount: Uint128,
    pub unbond_time: u64,
}

pub struct UnbondsIndexes<'a> {
    pub owner: MultiIndex<'a, Addr, UnbondInfo, u64>,
}

impl<'a> IndexList<UnbondInfo> for UnbondsIndexes<'a> {
    fn get_indexes(
        &'_ self,
    ) -> Box<dyn Iterator<Item = &'_ dyn cw_storage_plus::Index<UnbondInfo>> + '_> {
        let v: Vec<&dyn Index<UnbondInfo>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

pub fn unbonds<'a>() -> IndexedMap<'a, u64, UnbondInfo, UnbondsIndexes<'a>> {
    let indexes = UnbondsIndexes {
        owner: MultiIndex::new(|u| u.owner.clone(), UNBONDS_KEY, "UNBOND_OWNER"),
    };

    IndexedMap::new(UNBONDS_KEY, indexes)
}

pub fn init_unbonds_id(storage: &mut dyn Storage) -> StdResult<()> {
    UNBONDS_ID.save(storage, &0u64)?;
    Ok(())
}

pub fn get_unbonds_by_bonder(storage: &dyn Storage, bonder: Addr) -> StdResult<Vec<UnbondInfo>> {
    let result = unbonds()
        .idx
        .owner
        .prefix(bonder)
        .range(storage, None, None, Order::Ascending)
        .map(|r| r.unwrap().1)
        .collect();

    Ok(result)
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
    amount: Uint128,
) -> Result<UnbondInfo, ContractError> {
    let bonds = BONDS.load(storage, bonder.clone())?;
    let current_unbonds = get_unbonds_by_bonder(storage, bonder.clone())?;

    let unbond_amount: Uint128 = current_unbonds
        .into_iter()
        .map(|x| x.amount)
        .reduce(|a, b| a.checked_add(b).unwrap())
        .unwrap_or(Uint128::new(0));

    let available_amount: Uint128 = bonds.amount.checked_sub(unbond_amount).unwrap();

    if amount > available_amount {
        return Err(ContractError::InsufficientBondAmount {});
    }

    let unbond_id = UNBONDS_ID.load(storage)?;
    let new_unbond = UnbondInfo {
        unbond_id,
        amount,
        owner: bonder,
        unbond_time: env.block.time.seconds(),
    };

    unbonds().save(storage, unbond_id, &new_unbond)?;
    UNBONDS_ID.save(storage, &(unbond_id + 1))?;

    Ok(new_unbond)
}

pub fn finish_unbond(
    storage: &mut dyn Storage,
    env: Env,
    bonder: Addr,
    unbond_id: u64,
) -> Result<UnbondInfo, ContractError> {
    let unbond = unbonds().load(storage, unbond_id)?;
    let config = CONFIG.load(storage)?;

    if unbond.owner != bonder {
        return Err(ContractError::Unauthorized {});
    } else if unbond.unbond_time + config.unbonding_period > env.block.time.seconds() {
        return Err(ContractError::UnbondingNotFinished {});
    }

    unbonds().remove(storage, unbond_id)?;
    Ok(unbond)
}

pub fn query_bond(storage: &dyn Storage, bonder: Addr) -> StdResult<BondInfo> {
    BONDS.load(storage, bonder)
}

pub fn query_unbond(storage: &dyn Storage, unbond_id: u64) -> StdResult<UnbondInfo> {
    unbonds().load(storage, unbond_id)
}

pub fn query_unbonds_by_bonder(storage: &dyn Storage, bonder: Addr) -> StdResult<Vec<UnbondInfo>> {
    get_unbonds_by_bonder(storage, bonder)
}
