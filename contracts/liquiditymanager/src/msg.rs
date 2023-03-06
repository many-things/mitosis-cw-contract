use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

use crate::query::{ConfigResponse, GetBalanceResponse, PauseInfoResponse};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit {
        depositor: Option<Addr>,
    },
    Withdraw {
        withdrawer: Option<Addr>,
        amount: Coin,
    },
    ChangeOwner {
        new_owner: Addr,
    },
    GrantRole {
        role: String,
        addr: Addr,
    },
    RevokeRole {
        role: String,
        addr: Addr,
    },
    Pause {
        expires_at: u64,
    },
    Release {},
}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(ConfigResponse)]
    Config {},

    #[returns(PauseInfoResponse)]
    PauseInfo {},

    #[returns(GetBalanceResponse)]
    GetBalance { depositor: Addr },
}
