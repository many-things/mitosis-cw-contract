use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::{Item, Map};

use crate::error::ContractError;

pub const OWNER_KEY: &str = "owner";
pub const OWNER: Item<Addr> = Item::new(OWNER_KEY);

pub const ADDR_ROLE_KEY: &str = "roles";
pub const ADDR_ROLE: Map<(String, Addr), bool> = Map::new(ADDR_ROLE_KEY);

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

pub fn grant_role(storage: &mut dyn Storage, role: String, addr: Addr) {
    ADDR_ROLE.save(storage, (role, addr), &true).unwrap();
}

pub fn revoke_role(storage: &mut dyn Storage, role: String, addr: Addr) {
    assert_role(storage, role.clone(), addr.clone()).unwrap();

    ADDR_ROLE.remove(storage, (role, addr));
}
