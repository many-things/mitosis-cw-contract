use cosmwasm_std::{Addr, Coin, Env, MessageInfo, Order, StdError, StdResult, Storage, Uint128};
use cw_storage_plus::Map;

use crate::ContractError;

pub const BALANCES_KEY: &str = "balances";
pub const BALANCE: Map<(Addr, String), Uint128> = Map::new(BALANCES_KEY); // User account - Denomination

pub fn deposit_balance(
    storage: &mut dyn Storage,
    _env: Env,
    info: MessageInfo,
    depositor: Addr,
) -> Result<Vec<Coin>, ContractError> {
    // Save whole sended balances;
    for item in info.funds.iter() {
        let key: (Addr, String) = (depositor.clone(), item.denom.clone());
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

pub fn inquiry_balance(storage: &dyn Storage, _env: Env, depositor: Addr) -> StdResult<Vec<Coin>> {
    let deposit_balances: Vec<Result<(String, Uint128), StdError>> = BALANCE
        .prefix(depositor)
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
    _info: MessageInfo,
    withdrawer: Addr,
    claim_asset: Coin,
) -> Result<Coin, ContractError> {
    let expected_key = (withdrawer, claim_asset.denom.clone());
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

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        coin, coins,
        testing::mock_env,
        testing::{mock_info, MockStorage},
    };

    use super::*;

    const ADDR1_VALUE: &str = "addr1";
    const ADDR2_VALUE: &str = "addr2";

    #[test]
    fn test_deposit_single_balance() {
        let addr1 = Addr::unchecked(ADDR1_VALUE);
        let env = mock_env();
        let denom = "uosmo".to_string();

        let mut storage = MockStorage::new();
        let info = mock_info(addr1.as_ref(), &coins(100000, denom.clone()));

        let response =
            deposit_balance(&mut storage, env.clone(), info.clone(), addr1.clone()).unwrap();
        assert_eq!(response, coins(100000, denom.clone()));

        let deposit_variable = BALANCE
            .may_load(&storage, (addr1.clone(), denom.clone()))
            .unwrap()
            .unwrap();

        assert_eq!(deposit_variable, Uint128::new(100000));

        // Add exists assets
        let response = deposit_balance(&mut storage, env, info, addr1.clone()).unwrap();
        assert_eq!(response, coins(100000, denom.clone()));

        let deposit_variable = BALANCE.may_load(&storage, (addr1, denom)).unwrap().unwrap();

        assert_eq!(deposit_variable, Uint128::new(200000));
    }

    #[test]
    fn test_deposit_single_balance_to_other() {
        let sender = Addr::unchecked(ADDR1_VALUE);
        let depositor: Addr = Addr::unchecked(ADDR2_VALUE);
        let env = mock_env();
        let denom = "uosmo".to_string();

        let mut storage = MockStorage::new();
        let info = mock_info(sender.as_ref(), &coins(100000, denom.clone()));

        let response = deposit_balance(&mut storage, env, info, depositor.clone()).unwrap();
        assert_eq!(response, coins(100000, denom.clone()));

        assert!(matches!(
            BALANCE.load(&storage, (sender, denom.clone())).unwrap_err(),
            StdError::NotFound { .. }
        ));

        let deposit_variable = BALANCE.load(&storage, (depositor, denom)).unwrap();

        assert_eq!(deposit_variable, Uint128::new(100000));
    }

    #[test]
    fn test_deposit_multiple_balances() {
        let sender = Addr::unchecked(ADDR1_VALUE);
        let env = mock_env();
        let osmo = "uosmo".to_string();
        let usdc = "uusdc".to_string();

        let mut storage = MockStorage::new();
        let info = mock_info(
            sender.as_ref(),
            &[coin(100000, osmo.clone()), coin(200000, usdc.clone())],
        );

        let response = deposit_balance(&mut storage, env, info, sender.clone()).unwrap();
        assert_eq!(
            response,
            vec![coin(100000, osmo.clone()), coin(200000, usdc.clone())]
        );

        let deposit_uosmo_variable = BALANCE.load(&storage, (sender.clone(), osmo)).unwrap();

        let deposit_uusdc_variable = BALANCE.load(&storage, (sender, usdc)).unwrap();

        assert_eq!(deposit_uosmo_variable, Uint128::new(100000));
        assert_eq!(deposit_uusdc_variable, Uint128::new(200000));
    }

    #[test]
    fn test_inquiry_balances() {
        let depositor = Addr::unchecked(ADDR1_VALUE);
        let mut storage = MockStorage::new();
        let env = mock_env();

        let osmo = coin(100000, "uosmo");
        let usdc = coin(200000, "uusdc");

        let _ = BALANCE.save(
            &mut storage,
            (depositor.clone(), osmo.denom.clone()),
            &osmo.amount,
        );
        let _ = BALANCE.save(
            &mut storage,
            (depositor.clone(), usdc.denom.clone()),
            &usdc.amount,
        );

        let result = inquiry_balance(&storage, env, depositor).unwrap();
        assert_eq!(result, vec![osmo, usdc]);
    }

    #[test]
    fn test_withdraw_insufficient_balance() {
        let withdrawer = Addr::unchecked(ADDR1_VALUE);
        let mut storage = MockStorage::new();
        let env = mock_env();
        let info = mock_info(withdrawer.as_ref(), &[]);
        let claim = coin(200000, "uosmo");

        // Not exist
        let not_exist = withdraw_balance(
            &mut storage,
            env.clone(),
            info.clone(),
            withdrawer.clone(),
            claim.clone(),
        )
        .unwrap_err();

        assert!(matches!(
            not_exist,
            ContractError::DepositAssetNotFound { .. }
        ));

        let _ = BALANCE.save(
            &mut storage,
            (withdrawer.clone(), claim.denom.clone()),
            &Uint128::new(100000),
        );

        // Insufficient
        let insufficient =
            withdraw_balance(&mut storage, env, info, withdrawer, claim).unwrap_err();

        assert!(matches!(
            insufficient,
            ContractError::InsufficientWithdrawableAsset { .. }
        ));
    }

    #[test]
    fn test_withdraw_sufficient_balance() {
        let withdrawer = Addr::unchecked(ADDR1_VALUE);
        let mut storage = MockStorage::new();
        let env = mock_env();
        let info = mock_info(withdrawer.as_ref(), &[]);
        let claim = coin(50000, "uosmo");

        let _ = BALANCE.save(
            &mut storage,
            (withdrawer.clone(), claim.denom.clone()),
            &Uint128::new(200000),
        );

        let withdrawed =
            withdraw_balance(&mut storage, env, info, withdrawer.clone(), claim.clone()).unwrap();
        assert_eq!(withdrawed, claim);

        let deposit_variable = BALANCE.load(&storage, (withdrawer, claim.denom)).unwrap();
        assert_eq!(deposit_variable, Uint128::new(150000));
    }
}
