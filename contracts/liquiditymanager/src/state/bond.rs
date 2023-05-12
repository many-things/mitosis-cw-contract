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
    pub unbond_time: u64, // expected unbond time
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
        owner: MultiIndex::new(|_, u| u.owner.clone(), UNBONDS_KEY, "UNBOND_OWNER"),
    };

    IndexedMap::new(UNBONDS_KEY, indexes)
}

pub fn init_unbonds_id(storage: &mut dyn Storage) -> StdResult<()> {
    UNBONDS_ID.save(storage, &0u64)?;
    Ok(())
}

pub fn get_unbonds_by_owner(storage: &dyn Storage, bonder: Addr) -> StdResult<Vec<UnbondInfo>> {
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
    let current_unbonds = get_unbonds_by_owner(storage, bonder.clone())?;
    let config = CONFIG.load(storage)?;

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
        unbond_time: env.block.time.seconds() + config.unbonding_period, // saving expected unbond time.
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
    let mut bond: BondInfo = BONDS.load(storage, bonder.clone())?;

    if unbond.owner != bonder {
        return Err(ContractError::Unauthorized {});
    } else if unbond.unbond_time > env.block.time.seconds() {
        return Err(ContractError::UnbondingNotFinished {});
    }

    unbonds().remove(storage, unbond_id)?;
    bond.amount = bond.amount.checked_sub(unbond.amount).unwrap();
    BONDS.save(storage, bonder, &bond)?;

    Ok(unbond)
}

pub fn query_bond(storage: &dyn Storage, bonder: Addr) -> StdResult<BondInfo> {
    match BONDS.may_load(storage, bonder)? {
        Some(bond_info) => Ok(bond_info),
        None => Ok(BondInfo {
            amount: Uint128::zero(),
            bond_time: 0u64,
        }),
    }
}

pub fn query_unbond(storage: &dyn Storage, unbond_id: u64) -> StdResult<UnbondInfo> {
    unbonds().load(storage, unbond_id)
}

pub fn query_unbonds_by_owner(storage: &dyn Storage, bonder: Addr) -> StdResult<Vec<UnbondInfo>> {
    get_unbonds_by_owner(storage, bonder)
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{testing::mock_env, testing::MockStorage};

    use crate::state::ConfigInfo;

    use super::*;

    const ADDR1_VALUE: &str = "addr1";
    const ADDR2_VALUE: &str = "addr2";

    fn initialize_bond(storage: &mut dyn Storage, bonder: Addr, amount: Uint128, bond_time: u64) {
        BONDS
            .save(storage, bonder, &BondInfo { amount, bond_time })
            .unwrap();
    }

    fn initialize_unbond(
        storage: &mut dyn Storage,
        owner: Addr,
        amount: Uint128,
        unbond_time: u64,
    ) -> u64 {
        let unbond_id = UNBONDS_ID.load(storage).unwrap();
        let new_unbond = UnbondInfo {
            unbond_id,
            amount,
            owner,
            unbond_time,
        };

        unbonds().save(storage, unbond_id, &new_unbond).unwrap();
        UNBONDS_ID.save(storage, &(unbond_id + 1)).unwrap();

        unbond_id
    }

    #[test]
    fn test_initialize_bond() {
        let bonder = Addr::unchecked(ADDR1_VALUE);
        let mut storage: cosmwasm_std::MemoryStorage = MockStorage::new();
        let env = mock_env();

        let result = bond(&mut storage, env, bonder.clone(), Uint128::new(100000)).unwrap();
        let saved_info = BONDS.load(&storage, bonder).unwrap();

        assert_eq!(result, saved_info);
    }

    #[test]
    fn test_exist_bond() {
        let bonder = Addr::unchecked(ADDR1_VALUE);
        let mut storage = MockStorage::new();
        let env = mock_env();

        initialize_bond(&mut storage, bonder.clone(), Uint128::new(100000), 0);

        let result = bond(&mut storage, env, bonder.clone(), Uint128::new(100000)).unwrap();
        let saved_info = BONDS.load(&storage, bonder).unwrap();

        assert_eq!(result, saved_info);
        assert_eq!(result.amount, Uint128::new(200000));
    }

    #[test]
    fn test_start_unbond_successfully() {
        let bonder = Addr::unchecked(ADDR1_VALUE);
        let mut storage = MockStorage::new();
        let env = mock_env();

        CONFIG
            .save(
                &mut storage,
                &ConfigInfo {
                    unbonding_period: 20u64,
                },
            )
            .unwrap();

        initialize_bond(&mut storage, bonder.clone(), Uint128::new(100000), 0);
        init_unbonds_id(&mut storage).unwrap();

        let first_unbonding = start_unbond(
            &mut storage,
            env.clone(),
            bonder.clone(),
            Uint128::new(50000),
        )
        .unwrap();
        assert_eq!(first_unbonding.amount, Uint128::new(50000));
        assert_eq!(first_unbonding.unbond_id, 0u64); // first initialize

        let second_unbonding =
            start_unbond(&mut storage, env, bonder, Uint128::new(40000)).unwrap();

        assert_eq!(second_unbonding.amount, Uint128::new(40000));
        assert_eq!(second_unbonding.unbond_id, 1u64); // second initialize
    }

    #[test]
    fn test_start_unbond_failure() {
        let bonder = Addr::unchecked(ADDR1_VALUE);
        let mut storage = MockStorage::new();
        let env = mock_env();

        CONFIG
            .save(
                &mut storage,
                &ConfigInfo {
                    unbonding_period: 20u64,
                },
            )
            .unwrap();

        initialize_bond(&mut storage, bonder.clone(), Uint128::new(100000), 0);
        init_unbonds_id(&mut storage).unwrap();
        initialize_unbond(&mut storage, bonder.clone(), Uint128::new(40000), 0u64);
        initialize_unbond(&mut storage, bonder.clone(), Uint128::new(60000), 0u64);

        let insufficient_err =
            start_unbond(&mut storage, env, bonder, Uint128::new(50000)).unwrap_err();
        assert!(matches!(
            insufficient_err,
            ContractError::InsufficientBondAmount {}
        ))
    }

    #[test]
    fn test_finish_unbond_failure() {
        let bonder = Addr::unchecked(ADDR1_VALUE);
        let not_bonder: Addr = Addr::unchecked(ADDR2_VALUE);

        let mut storage = MockStorage::new();
        let env = mock_env();

        CONFIG
            .save(
                &mut storage,
                &ConfigInfo {
                    unbonding_period: 20u64,
                },
            )
            .unwrap();

        initialize_bond(&mut storage, bonder.clone(), Uint128::new(100000), 0);
        initialize_bond(&mut storage, not_bonder.clone(), Uint128::new(100000), 0);

        init_unbonds_id(&mut storage).unwrap();
        let unbond_id = initialize_unbond(
            &mut storage,
            bonder.clone(),
            Uint128::new(40000),
            env.block.time.seconds() + 40u64,
        );

        let not_period_unbonding =
            finish_unbond(&mut storage, env.clone(), bonder, unbond_id).unwrap_err();

        assert!(matches!(
            not_period_unbonding,
            ContractError::UnbondingNotFinished {}
        ));

        let not_owned_unbonding =
            finish_unbond(&mut storage, env, not_bonder, unbond_id).unwrap_err();

        println!("{}", not_owned_unbonding);
        assert!(matches!(
            not_owned_unbonding,
            ContractError::Unauthorized {}
        ));
    }

    #[test]
    fn test_finish_unbond_success() {
        let bonder = Addr::unchecked(ADDR1_VALUE);

        let mut storage: cosmwasm_std::MemoryStorage = MockStorage::new();
        let env = mock_env();

        CONFIG
            .save(
                &mut storage,
                &ConfigInfo {
                    unbonding_period: 20u64,
                },
            )
            .unwrap();

        initialize_bond(&mut storage, bonder.clone(), Uint128::new(100000), 0);

        init_unbonds_id(&mut storage).unwrap();
        let unbond_id = initialize_unbond(
            &mut storage,
            bonder.clone(),
            Uint128::new(40000),
            env.block.time.seconds() - 30u64,
        );

        let success_unbonding =
            finish_unbond(&mut storage, env, bonder.clone(), unbond_id).unwrap();

        assert_eq!(success_unbonding.amount, Uint128::new(40000));

        // removed amount successfully
        let bond = BONDS.load(&storage, bonder).unwrap();
        assert_eq!(bond.amount, Uint128::new(60000));

        // successfully removed
        assert!(!unbonds().has(&storage, unbond_id))
    }

    #[test]
    fn test_query_bond() {
        let mut storage: cosmwasm_std::MemoryStorage = MockStorage::new();

        let bonder = Addr::unchecked(ADDR1_VALUE);
        let empty_bond = query_bond(&storage, bonder.clone()).unwrap();

        assert_eq!(
            empty_bond,
            BondInfo {
                amount: Uint128::zero(),
                bond_time: 0u64
            }
        );

        initialize_bond(&mut storage, bonder.clone(), Uint128::new(100000), 12);
        let initialized_bond = query_bond(&storage, bonder).unwrap();
        assert_eq!(
            initialized_bond,
            BondInfo {
                amount: Uint128::new(100000),
                bond_time: 12u64
            }
        )
    }
}
