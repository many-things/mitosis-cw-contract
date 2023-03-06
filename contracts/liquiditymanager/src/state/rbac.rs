use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::{Item, Map};

use crate::error::ContractError;

pub const OWNER_KEY: &str = "owner";
pub const OWNER: Item<Addr> = Item::new(OWNER_KEY);

pub const ADDR_ROLE_KEY: &str = "roles";
pub const ADDR_ROLE: Map<(String, Addr), bool> = Map::new(ADDR_ROLE_KEY);

/** You might add ROLES here */
pub const GATEWAY_ROLE: &str = "gateway_role";
/** You might add ROLES here */

pub fn assert_owned(storage: &dyn Storage, sender: Addr) -> Result<(), ContractError> {
    let owner = OWNER.load(storage)?;

    if owner != sender {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

pub fn assert_role(storage: &dyn Storage, role: String, addr: Addr) -> Result<(), ContractError> {
    match ADDR_ROLE.may_load(storage, (role.clone(), addr.clone()))? {
        Some(result) => match result {
            true => Ok(()),
            false => Err(ContractError::RoleNotExist { addr, role }),
        },
        None => Err(ContractError::RoleNotExist { addr, role }),
    }
}

pub fn change_owner(storage: &mut dyn Storage, new_owner: Addr) -> Result<(), ContractError> {
    OWNER.save(storage, &new_owner)?;

    Ok(())
}

pub fn grant_role(
    storage: &mut dyn Storage,
    role: String,
    addr: Addr,
) -> Result<(String, Addr), ContractError> {
    ADDR_ROLE
        .save(storage, (role.clone(), addr.clone()), &true)
        .unwrap();

    Ok((role, addr))
}

pub fn revoke_role(
    storage: &mut dyn Storage,
    role: String,
    addr: Addr,
) -> Result<(String, Addr), ContractError> {
    assert_role(storage, role.clone(), addr.clone())?;

    ADDR_ROLE.remove(storage, (role.clone(), addr.clone()));
    Ok((role, addr))
}

#[cfg(test)]
mod test {
    use cosmwasm_std::testing::MockStorage;

    use super::*;

    const ADDR1_VALUE: &str = "addr1";
    const ADDR2_VALUE: &str = "addr2";

    fn mock_owner(storage: &mut dyn Storage, owner: Addr) {
        OWNER.save(storage, &owner).unwrap();
    }

    #[test]
    fn test_assert_owned() {
        let mut storage = MockStorage::new();

        let owner = Addr::unchecked(ADDR1_VALUE);
        let abuser = Addr::unchecked(ADDR2_VALUE);

        mock_owner(&mut storage, owner.clone());

        assert_owned(&storage, owner).unwrap();
        let failed = assert_owned(&storage, abuser).unwrap_err();
        assert!(matches!(failed, ContractError::Unauthorized {}));
    }

    #[test]
    fn test_change_owner() {
        let mut storage = MockStorage::new();
        let new_owner = Addr::unchecked(ADDR2_VALUE);

        change_owner(&mut storage, new_owner.clone()).unwrap();
        let saved = OWNER.load(&storage).unwrap();

        assert_eq!(saved, new_owner);
    }

    #[test]
    fn test_assert_role() {
        let mut storage = MockStorage::new();

        let role_owner = Addr::unchecked(ADDR1_VALUE);
        let role_abuser = Addr::unchecked(ADDR2_VALUE);
        let role = GATEWAY_ROLE.to_string();

        ADDR_ROLE
            .save(&mut storage, (role.clone(), role_owner.clone()), &true)
            .unwrap();

        assert_role(&storage, role.clone(), role_owner).unwrap(); // success
        let failure = assert_role(&storage, role, role_abuser).unwrap_err();

        assert!(matches!(failure, ContractError::RoleNotExist { .. }));
    }

    #[test]
    fn test_grant_role() {
        let mut storage = MockStorage::new();

        let approver = Addr::unchecked(ADDR1_VALUE);
        let role = GATEWAY_ROLE.to_string();

        let result = grant_role(&mut storage, role.clone(), approver.clone()).unwrap();
        assert_eq!(result, (role.clone(), approver.clone()));

        let saved = ADDR_ROLE.load(&storage, (role, approver)).unwrap();
        assert!(saved);
    }

    #[test]
    fn test_revoke_role() {
        use cosmwasm_std::StdError;
        let mut storage = MockStorage::new();

        let revoker = Addr::unchecked(ADDR1_VALUE);
        let role = GATEWAY_ROLE.to_string();

        let not_exist_err = revoke_role(&mut storage, role.clone(), revoker.clone()).unwrap_err();
        assert!(matches!(not_exist_err, ContractError::RoleNotExist { .. }));

        ADDR_ROLE
            .save(&mut storage, (role.clone(), revoker.clone()), &true)
            .unwrap();

        let result = revoke_role(&mut storage, role.clone(), revoker.clone()).unwrap();
        assert_eq!(result, (role.clone(), revoker.clone()));

        let storage_result = ADDR_ROLE.load(&storage, (role, revoker)).unwrap_err();
        assert!(matches!(storage_result, StdError::NotFound { .. }));
    }
}
