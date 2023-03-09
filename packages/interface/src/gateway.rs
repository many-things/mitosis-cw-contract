use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub liquidity_manager: Addr,
    pub denom_manager: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    ChangeOwner { new_owner: Addr },
    ChangeLiquidityManager { new_liquidity_manager: Addr },
    ChangeDenomManager { new_denom_manager: Addr },
    Pause { expires_at: u64 },
    Releaes {},
}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub liquidity_manager: Addr,
    pub denom_manager: Addr,
}
