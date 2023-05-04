use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, CosmosMsg, HexBinary};

#[cw_serde]
pub struct InstantiateMsg {
    pub liquidity_manager: Addr,
    pub denom_manager: Addr,
    pub public_key: HexBinary,
}

#[cw_serde]
pub enum ExecuteMsg {
    ChangeOwner {
        new_owner: Addr,
        new_public_key: HexBinary,
    },
    ChangeLiquidityManager {
        new_liquidity_manager: Addr,
    },
    ChangeDenomManager {
        new_denom_manager: Addr,
    },
    Pause {
        expires_at: u64,
    },
    Send {
        to: String,
        op_id: u64,
        op_args: Vec<Binary>,
    },
    Execute {
        msgs: Vec<CosmosMsg>,
        signature: HexBinary,
    },
    Release {},
}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    GetConfig {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub liquidity_manager: Addr,
    pub denom_manager: Addr,
    pub public_key: HexBinary,
}
