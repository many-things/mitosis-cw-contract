use cosmwasm_std::{Coin, Env, MessageInfo, Storage, Uint128};
use cw_storage_plus::Item;

use crate::ContractError;

pub const DELEGATE_AMOUNT_KEY: &str = "delegates";
pub const DELEGATE_AMOUNT: Item<Uint128> = Item::new(DELEGATE_AMOUNT_KEY);

pub fn delegate_balance(
    _storage: &mut dyn Storage,
    _env: Env,
    _info: MessageInfo,
    _balance: Coin,
) -> Result<Uint128, ContractError> {
    unimplemented!()
}

pub fn undelegate_balance(
    _storage: &mut dyn Storage,
    _env: Env,
    _info: MessageInfo,
    _balance: Coin,
) -> Result<Uint128, ContractError> {
    unimplemented!()
}
