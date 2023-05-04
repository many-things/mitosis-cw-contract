use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, CosmosMsg, HexBinary};

#[cw_serde]
pub struct InstantiateMsg {
    pub liquidity_manager: Addr,
    pub denom_manager: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    ChangeOwner {
        new_owner: Addr,
    },
    ChangeLiquidityManager {
        new_liquidity_manager: Addr,
    },
    ChangeDenomManager {
        new_denom_manager: Addr,
    },
    ChangePublicKey {
        public_key: HexBinary,
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

    #[returns(PublicKeyResponse)]
    GetPublicKey {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub liquidity_manager: Addr,
    pub denom_manager: Addr,
}

#[cw_serde]
pub struct PublicKeyResponse {
    pub public_key: HexBinary,
}
