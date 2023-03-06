use cosmwasm_std::{StdResult, Storage};
use cw_storage_plus::Map;

use crate::error::ContractError;

pub const DENOMS_KEY: &str = "denoms";
pub const DENOMS: Map<String, String> = Map::new(DENOMS_KEY);

pub fn convert_denoms(storage: &dyn Storage, token: String) -> Result<String, ContractError> {
    match DENOMS.may_load(storage, token.clone())? {
        Some(denom) => Ok(denom),
        None => Err(ContractError::DenomNotFound { denom: token }),
    }
}

pub fn add_alias(
    storage: &mut dyn Storage,
    token: String,
    alias: String,
) -> StdResult<(String, String)> {
    DENOMS.save(storage, token.clone(), &alias)?;

    Ok((token, alias))
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{testing::MockStorage, Storage};

    use crate::state::denoms::DENOMS;

    use super::*;

    fn mock_aliases(storage: &mut dyn Storage) {
        DENOMS
            .save(storage, "0x0".to_string(), &"ETH".to_string())
            .unwrap();

        DENOMS
            .save(storage, "0x1".to_string(), &"OSMO".to_string())
            .unwrap();
    }

    #[test]
    fn test_add_alias() {
        let mut storage = MockStorage::new();

        let address: String = "0x0".to_string();
        let alias: String = "ETH".to_string();

        let result = add_alias(&mut storage, address.clone(), alias.clone()).unwrap();
        assert_eq!(result, (address.clone(), alias.clone()));

        let saved_res = DENOMS.load(&storage, address).unwrap();
        assert_eq!(saved_res, alias);
    }

    #[test]
    fn test_convert_denom_exists() {
        let mut storage = MockStorage::new();
        mock_aliases(&mut storage);

        let result = convert_denoms(&storage, "0x0".to_string()).unwrap();
        assert_eq!(result, "ETH".to_string());

        let result = convert_denoms(&storage, "0x1".to_string()).unwrap();
        assert_eq!(result, "OSMO".to_string());
    }

    #[test]
    fn test_convert_denom_not_exists() {
        let mut storage = MockStorage::new();
        mock_aliases(&mut storage);
        let not_exist_addr: String = "0x2".to_string();

        let err_result = convert_denoms(&storage, not_exist_addr).unwrap_err();
        assert!(matches!(err_result, ContractError::DenomNotFound { .. }));
    }
}
