use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddAlias { token: String, denom: String },
    ChangeOwner { new_owner: Addr },
    GrantRole { role: String, addr: Addr },
    RevokeRole { role: String, addr: Addr },
    Pause { expires_at: u64 },
    Release {},
}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    GetConfig {},

    #[returns(ConvertResponse)]
    Convert { token: String },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
}

#[cw_serde]
pub struct ConvertResponse {
    pub token: String,
    pub alias: String,
}
