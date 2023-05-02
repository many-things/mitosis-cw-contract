use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Storage};
use cw_storage_plus::Item;

use crate::ContractError;

pub const WITHDRAW_KEY: &str = "withdraws";
pub const WITHDRAW: Item<WithdrawInfo> = Item::new(WITHDRAW_KEY);

pub const UNBOND_KEY: &str = "unbonds";
pub const UNBOND: Item<WithdrawInfo> = Item::new(UNBOND_KEY);

#[cw_serde]
pub struct WithdrawInfo {
    pub to_address: Addr,
    pub amount: Coin,
}

pub fn set_withdraw_info(
    storage: &mut dyn Storage,
    to_address: Addr,
    amount: Coin,
) -> Result<(), ContractError> {
    match WITHDRAW.may_load(storage)? {
        Some(_) => Err(ContractError::WithdrawNotFlushed {}),
        None => {
            WITHDRAW.save(storage, &WithdrawInfo { to_address, amount })?;

            Ok(())
        }
    }
}

// get_withdraw_info returns only
pub fn get_withdraw_info(storage: &mut dyn Storage) -> Result<WithdrawInfo, ContractError> {
    let result = WITHDRAW.load(storage)?;

    // Flush withdraw info for refresh.
    WITHDRAW.remove(storage);

    Ok(result)
}

pub fn set_unbond_info(
    storage: &mut dyn Storage,
    to_address: Addr,
    amount: Coin,
) -> Result<(), ContractError> {
    match UNBOND.may_load(storage)? {
        Some(_) => Err(ContractError::UnbondNotFlushed {}),
        None => {
            UNBOND.save(storage, &WithdrawInfo { to_address, amount })?;

            Ok(())
        }
    }
}

// get_unbond_info returns only
pub fn get_unbond_info(storage: &mut dyn Storage) -> Result<WithdrawInfo, ContractError> {
    let result = UNBOND.load(storage)?;

    // Flush withdraw info for refresh
    UNBOND.remove(storage);

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::{coin, testing::MockStorage};

    const ADDR1_VALUE: &str = "addr1";

    #[test]
    fn test_set_withdraw_success() {
        let mut storage: cosmwasm_std::MemoryStorage = MockStorage::new();
        let addr = Addr::unchecked(ADDR1_VALUE);

        set_withdraw_info(&mut storage, addr.clone(), coin(100000, "uosmo")).unwrap();
        let saved_result = WITHDRAW.load(&storage).unwrap();

        assert_eq!(saved_result.to_address, addr);
        assert_eq!(saved_result.amount, coin(100000, "uosmo"));
    }

    #[test]
    fn test_set_withdraw_failure() {
        let mut storage = MockStorage::new();
        let addr = Addr::unchecked(ADDR1_VALUE);

        WITHDRAW
            .save(
                &mut storage,
                &WithdrawInfo {
                    to_address: addr.clone(),
                    amount: coin(100000, "uosmo"),
                },
            )
            .unwrap();

        let result = set_withdraw_info(&mut storage, addr, coin(1000, "uosmo")).unwrap_err();
        assert!(matches!(result, ContractError::WithdrawNotFlushed {}))
    }

    #[test]
    fn test_get_withdraw_success() {
        let mut storage = MockStorage::new();
        let addr = Addr::unchecked(ADDR1_VALUE);

        WITHDRAW
            .save(
                &mut storage,
                &WithdrawInfo {
                    to_address: addr.clone(),
                    amount: coin(100000, "uosmo"),
                },
            )
            .unwrap();

        let result = get_withdraw_info(&mut storage).unwrap();
        assert_eq!(
            result,
            WithdrawInfo {
                to_address: addr,
                amount: coin(100000, "uosmo"),
            }
        );

        // remove after withdraw successfully
        let load_err = WITHDRAW.load(&storage).unwrap_err();
        assert!(matches!(load_err, cosmwasm_std::StdError::NotFound { .. }));
    }

    #[test]
    fn test_set_unbond_success() {
        let mut storage: cosmwasm_std::MemoryStorage = MockStorage::new();
        let addr = Addr::unchecked(ADDR1_VALUE);

        set_unbond_info(&mut storage, addr.clone(), coin(100000, "uosmo")).unwrap();
        let saved_result = UNBOND.load(&storage).unwrap();

        assert_eq!(saved_result.to_address, addr);
        assert_eq!(saved_result.amount, coin(100000, "uosmo"));
    }

    #[test]
    fn test_set_unbond_failure() {
        let mut storage = MockStorage::new();
        let addr = Addr::unchecked(ADDR1_VALUE);

        UNBOND
            .save(
                &mut storage,
                &WithdrawInfo {
                    to_address: addr.clone(),
                    amount: coin(100000, "uosmo"),
                },
            )
            .unwrap();

        let result = set_unbond_info(&mut storage, addr, coin(1000, "uosmo")).unwrap_err();
        assert!(matches!(result, ContractError::UnbondNotFlushed {}))
    }

    #[test]
    fn test_get_unbond_success() {
        let mut storage = MockStorage::new();
        let addr = Addr::unchecked(ADDR1_VALUE);

        UNBOND
            .save(
                &mut storage,
                &WithdrawInfo {
                    to_address: addr.clone(),
                    amount: coin(100000, "uosmo"),
                },
            )
            .unwrap();

        let result = get_unbond_info(&mut storage).unwrap();
        assert_eq!(
            result,
            WithdrawInfo {
                to_address: addr,
                amount: coin(100000, "uosmo"),
            }
        );

        // remove after withdraw successfully
        let load_err = UNBOND.load(&storage).unwrap_err();
        assert!(matches!(load_err, cosmwasm_std::StdError::NotFound { .. }));
    }
}
