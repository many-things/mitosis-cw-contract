use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage, Env, StdResult};
use cosmwasm_schema::cw_serde;
use cw_storage_plus::Map;

use crate::ContractError;

pub const BALANCES_KEY: &str = "balances";
pub const BALANCE: Map<(&str, &str), Uint128> = Map::new(balances);