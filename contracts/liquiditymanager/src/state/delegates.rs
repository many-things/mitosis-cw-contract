use cosmwasm_std::{Storage, Uint128};
use cw_storage_plus::Item;

use crate::ContractError;

pub const DELEGATE_BALANCES_KEY: &str = "delegate_balances";
pub const DELEGATE_BALANCE: Item<Uint128> = Item::new(DELEGATE_BALANCES_KEY);

pub fn delegate_balance(
    storage: &mut dyn Storage,
    amount: Uint128,
) -> Result<Uint128, ContractError> {
    // Save whole sended balances;
    let delegates = DELEGATE_BALANCE.load(storage)?;
    let new_amount = delegates.checked_add(amount).unwrap(); // cannot be errored

    DELEGATE_BALANCE.save(storage, &new_amount).unwrap();

    Ok(new_amount)
}

pub fn undelegate_balance(
    storage: &mut dyn Storage,
    amount: Uint128,
) -> Result<Uint128, ContractError> {
    let delegates = DELEGATE_BALANCE.load(storage)?;
    let new_amount = delegates
        .checked_sub(amount)
        .map_err(|_| ContractError::InsufficientUndelegateAsset {})?;

    DELEGATE_BALANCE.save(storage, &new_amount).unwrap();

    Ok(new_amount)
}

#[cfg(test)]
mod test {
    use cosmwasm_std::testing::MockStorage;

    use super::*;

    #[test]
    fn test_delegates() {
        let mut storage = MockStorage::new();
        let expected = Uint128::new(3000);

        DELEGATE_BALANCE
            .save(&mut storage, &Uint128::new(1000))
            .unwrap();

        let result = delegate_balance(&mut storage, Uint128::new(2000)).unwrap();
        assert_eq!(result, expected);

        let saved = DELEGATE_BALANCE.load(&storage).unwrap();
        assert_eq!(saved, expected)
    }

    #[test]
    fn test_undelegate_success() {
        let mut storage = MockStorage::new();
        let expected = Uint128::new(1000);

        DELEGATE_BALANCE
            .save(&mut storage, &Uint128::new(3000))
            .unwrap();

        let result = undelegate_balance(&mut storage, Uint128::new(2000)).unwrap();
        assert_eq!(result, expected);

        let saved = DELEGATE_BALANCE.load(&storage).unwrap();
        assert_eq!(saved, expected);
    }

    #[test]
    fn test_undelegate_failure() {
        let mut storage = MockStorage::new();
        DELEGATE_BALANCE
            .save(&mut storage, &Uint128::new(3000))
            .unwrap();

        let result = undelegate_balance(&mut storage, Uint128::new(4000)).unwrap_err();
        assert!(matches!(
            result,
            ContractError::InsufficientUndelegateAsset {}
        ))
    }
}
