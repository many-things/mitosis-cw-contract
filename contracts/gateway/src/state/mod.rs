pub mod context;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Env, StdResult, Storage};
use cw_storage_plus::Item;

use crate::errors::ContractError;

pub const OWNER_KEY: &str = "owner";
pub const OWNER: Item<Addr> = Item::new(OWNER_KEY);

pub const PAUSED_KEY: &str = "paused";
pub const PAUSED: Item<PauseInfo> = Item::new(PAUSED_KEY);

pub const LIQUIDITY_MANAGER_KEY: &str = "liquidity_manager";
pub const LIQUIDITY_MANAGER: Item<Addr> = Item::new(LIQUIDITY_MANAGER_KEY);

pub const DENOM_MANAGER_KEY: &str = "denom_manager";
pub const DENOM_MANAGER: Item<Addr> = Item::new(DENOM_MANAGER_KEY);

#[cw_serde]
#[derive(Default)]
pub struct PauseInfo {
    pub paused: bool,
    pub expires_at: Option<u64>,
}

pub fn assert_owned(storage: &dyn Storage, sender: Addr) -> Result<(), ContractError> {
    let owner = OWNER.load(storage)?;

    if owner != sender {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

impl PauseInfo {
    pub fn refresh(self, storage: &mut dyn Storage, env: &Env) -> StdResult<Self> {
        if self.paused {
            if let Some(expiry) = self.expires_at {
                if expiry <= env.block.time.seconds() {
                    PAUSED.save(storage, &Default::default())?;
                    return Ok(Default::default());
                }
            }
        }

        Ok(self)
    }

    pub fn assert_paused(self) -> Result<Self, ContractError> {
        if !self.paused {
            return Err(ContractError::NotPausedError {});
        }

        Ok(self)
    }

    pub fn assert_not_paused(self) -> Result<Self, ContractError> {
        if self.paused {
            return Err(ContractError::PausedError {});
        }

        Ok(self)
    }
}
