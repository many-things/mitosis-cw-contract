use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, Coin, Deps, Env, QueryResponse};

use crate::{
    state::{balances::inquiry_balance, OWNER, PAUSED},
    ContractError,
};
