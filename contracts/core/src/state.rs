use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage, Env, StdResult};
use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

use crate::ContractError;

pub const OWNER_KEY: &str = "owner";
pub const OWNER: Item<Addr> = Item::new(OWNER_KEY);

pub const PAUSED_KEY: &str = "paused";
pub const PAUSED: Item<PauseInfo> = Item::new(PAUSED_KEY);


#[cw_serde]
#[derive(Default)]
pub struct PauseInfo {
    pub paused: bool,
    pub expires_at: Option<u64>,
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
            return Err(ContractError::PausedError {  });
        }

        Ok(self)
    }
}