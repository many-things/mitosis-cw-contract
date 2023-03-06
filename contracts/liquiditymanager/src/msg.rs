use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

use crate::query::{ConfigResponse, InquiryBalanceResponse, PauseInfoResponse};

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
}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(ConfigResponse)]
    ConfigQuery {},

    #[returns(PauseInfoResponse)]
    PauseInfoQuery {},

    #[returns(InquiryBalanceResponse)]
    InquiryBalanceQuery { depositor: Addr },
}