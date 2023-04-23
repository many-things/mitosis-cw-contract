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
    match DELEGATE_AMOUNT.may_load(storage)? {
        Some(amount) => {
            let new_amount: Uint128 = amount.checked_add(balance.amount).unwrap();
            DELEGATE_AMOUNT.save(storage, &new_amount)?;

            Ok(new_amount)
        }
        None => {
            DELEGATE_AMOUNT.save(storage, &balance.amount)?;
            Ok(balance.amount)
        }
    }
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
