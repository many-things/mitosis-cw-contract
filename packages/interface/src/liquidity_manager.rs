use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub denom: String,
    pub lp_denom: String,
    pub unbonding_period: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit {
        depositor: Option<Addr>,
    },
    Withdraw {
        withdrawer: Option<Addr>,
        amount: Coin,
    },
    Delegate {},
    Undelegate {},
    Bond {},
    StartUnbond {
        amount: Uint128,
    },
    Unbond {
        unbond_id: u64,
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
    ChangeConfig {
        unbonding_period: u64,
    },
}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(ConfigResponse)]
    GetConfig {},

    #[returns(PauseInfoResponse)]
    PauseInfo {},

    #[returns(GetBalanceResponse)]
    GetBalance { depositor: Addr },

    #[returns(GetTotalDelegatesResponse)]
    GetTotalDelegates {},

    #[returns(GetBondResponse)]
    GetBond { bonder: Addr },

    #[returns(GetUnbondResponse)]
    GetUnbond { unbond_id: u64 },

    #[returns(GetUnbondListResponse)]
    GetUnbondsByOwner { owner: Addr },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub unbonding_period: u64,
    pub denom: String,
    pub lp_denom: String,
}

#[cw_serde]
pub struct PauseInfoResponse {
    pub paused: bool,
    pub expires_at: Option<u64>,
}

#[cw_serde]
pub struct GetBalanceResponse {
    pub depositor: Addr,
    pub assets: Vec<Coin>,
}

#[cw_serde]
pub struct GetBondResponse {
    pub amount: Uint128,
    pub bond_time: u64,
}

#[cw_serde]
pub struct GetUnbondResponse {
    pub unbond_id: u64,
    pub owner: Addr,
    pub amount: Uint128,
    pub unbond_time: u64,
}

#[cw_serde]
pub struct GetUnbondListResponse {
    pub items: Vec<GetUnbondResponse>,
}

#[cw_serde]
pub struct GetTotalDelegatesResponse {
    pub amount: Coin,
}
