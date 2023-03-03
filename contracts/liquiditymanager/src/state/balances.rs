use cosmwasm_std::{Addr, Coin, Env, MessageInfo, Order, StdError, StdResult, Storage, Uint128};
use cw_storage_plus::Map;

use crate::ContractError;

pub const BALANCES_KEY: &str = "balances";
pub const BALANCE: Map<(Addr, String), Uint128> = Map::new(BALANCES_KEY); // User account - Denomination

pub fn deposit_balance(
    storage: &mut dyn Storage,
    _env: Env,
    info: MessageInfo,
) -> Result<Vec<Coin>, ContractError> {
    // Save whole sended balances;
    if info.funds.is_empty() {
        return Err(ContractError::AssetNotFound {});
    }

    for item in info.funds.iter() {
        let key: (Addr, String) = (info.sender.clone(), item.denom.clone());
        match BALANCE.may_load(storage, key.clone())? {
            Some(amount) => {
                let new_amount = amount.checked_add(item.amount).unwrap();
                BALANCE.save(storage, key, &new_amount)?;
            }
            None => {
                BALANCE.save(storage, key, &item.amount)?;
            }
        }
    }

    Ok(info.funds)
}

pub fn inquiry_balance(
    storage: &mut dyn Storage,
    _env: Env,
    info: MessageInfo,
) -> StdResult<Vec<Coin>> {
    let deposit_balances: Vec<Result<(String, Uint128), StdError>> = BALANCE
        .prefix(info.sender)
        .range(storage, None, None, Order::Ascending)
        .collect::<Vec<_>>();

    let result: Vec<Coin> = deposit_balances
        .iter()
        .map(|x| {
            let assets = x.as_ref().unwrap();
            Coin {
                denom: assets.0.clone(),
                amount: assets.1,
            }
        })
        .collect::<Vec<_>>();

    Ok(result)
}

pub fn withdraw_balance(
    storage: &mut dyn Storage,
    _env: Env,
    info: MessageInfo,
    claim_asset: Coin,
) -> Result<Coin, ContractError> {
    let expected_key = (info.sender, claim_asset.denom.clone());
    match BALANCE.may_load(storage, expected_key.clone())? {
        Some(deposit_amount) => match deposit_amount.checked_sub(claim_asset.amount) {
            Ok(claimed_amount) => {
                BALANCE.save(storage, expected_key, &claimed_amount)?;
                Ok(claim_asset)
            }
            Err(_) => Err(ContractError::InsufficientWithdrawableAsset {}),
        },
        None => Err(ContractError::DepositAssetNotFound {
            val: claim_asset.denom,
        }),
    }
}
