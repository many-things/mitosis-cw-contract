use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::query::{ConfigResponse, ConvertResponse};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddAlias { addr: String, denom: String },
}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    ConfigQuery {},

    #[returns(ConvertResponse)]
    ConvertQuery { token: String },
}
