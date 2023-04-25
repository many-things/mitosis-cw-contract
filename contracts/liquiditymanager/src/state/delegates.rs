use cosmwasm_std::{Coin, Env, MessageInfo, Storage, Uint128};
use cw_storage_plus::Item;

use crate::ContractError;

pub const DELEGATE_AMOUNT_KEY: &str = "delegates";
pub const DELEGATE_AMOUNT: Item<Uint128> = Item::new(DELEGATE_AMOUNT_KEY);

pub fn delegate_balance(
    storage: &mut dyn Storage,
    _env: Env,
    _info: MessageInfo,
    balance: Coin,
) -> Result<Uint128, ContractError> {
}

pub fn undelegate_balance(
    storage: &mut dyn Storage,
    _env: Env,
    _info: MessageInfo,
    balance: Coin,
) -> Result<Uint128, ContractError> {
    match DELEGATE_AMOUNT.may_load(storage)? {
        Some(amount) => match amount.checked_sub(balance.amount) {
            Ok(claim_after_asset) => {
                DELEGATE_AMOUNT.save(storage, &claim_after_asset)?;
                Ok(claim_after_asset)
            }
            Err(_) => Err(ContractError::InsufficientDelegateAsset {}),
        },
        None => Err(ContractError::DelegateAssetNotFound {}),
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        coin, coins,
        testing::{mock_env, mock_info, MockStorage},
        Addr,
    };

    use super::*;
    const ADDR1_VALUE: &str = "addr1";

    #[test]
    fn test_delegate_balance_without_initial() {
        let addr1 = Addr::unchecked(ADDR1_VALUE);
        let env = mock_env();
        let denom = "uosmo".to_string();

        let mut storage: cosmwasm_std::MemoryStorage = MockStorage::new();
        let info = mock_info(addr1.as_ref(), &coins(100000, denom.clone()));

        let resp = delegate_balance(&mut storage, env, info, coin(100000, denom)).unwrap();
        assert_eq!(resp, Uint128::new(100000));

        // check saved result
        let saved_result = DELEGATE_AMOUNT.load(&storage).unwrap();
        assert_eq!(resp, saved_result)
    }

    #[test]
    fn test_delegate_balance_after_initial() {
        let addr1 = Addr::unchecked(ADDR1_VALUE);
        let env = mock_env();
        let denom = "uosmo".to_string();

        let mut storage: cosmwasm_std::MemoryStorage = MockStorage::new();
        let info = mock_info(addr1.as_ref(), &coins(100000, denom.clone()));

        DELEGATE_AMOUNT
            .save(&mut storage, &Uint128::new(50000))
            .unwrap();

        let resp = delegate_balance(&mut storage, env, info, coin(100000, denom)).unwrap();
        assert_eq!(resp, Uint128::new(150000));

        // check saved result
        let saved_result = DELEGATE_AMOUNT.load(&storage).unwrap();
        assert_eq!(resp, saved_result)
    }

    #[test]
    fn test_undelegate_balance_err() {
        let addr1 = Addr::unchecked(ADDR1_VALUE);
        let env = mock_env();
        let denom = "uosmo".to_string();

        // Not initial
        let mut storage: cosmwasm_std::MemoryStorage = MockStorage::new();
        let info = mock_info(addr1.as_ref(), &coins(100000, denom.clone()));

        let resp = undelegate_balance(
            &mut storage,
            env.clone(),
            info.clone(),
            coin(100000, denom.clone()),
        )
        .unwrap_err();
        assert!(matches!(resp, ContractError::DelegateAssetNotFound {}));

        // Initial but Insufficient
        DELEGATE_AMOUNT
            .save(&mut storage, &Uint128::new(50000))
            .unwrap();

        let insufficient_resp =
            undelegate_balance(&mut storage, env, info, coin(100000, denom)).unwrap_err();
        assert!(matches!(
            insufficient_resp,
            ContractError::InsufficientDelegateAsset {}
        ))
    }

    #[test]
    fn test_undelegate_sufficient_balance() {
        let addr1 = Addr::unchecked(ADDR1_VALUE);
        let env = mock_env();
        let denom = "uosmo".to_string();

        let mut storage: cosmwasm_std::MemoryStorage = MockStorage::new();
        let info = mock_info(addr1.as_ref(), &coins(100000, denom.clone()));

        DELEGATE_AMOUNT
            .save(&mut storage, &Uint128::new(150000))
            .unwrap();

        let resp = undelegate_balance(&mut storage, env, info, coin(100000, denom)).unwrap();
        assert_eq!(resp, Uint128::new(50000));

        // check saved result
        let saved_result = DELEGATE_AMOUNT.load(&storage).unwrap();
        assert_eq!(resp, saved_result)
    }
}
