use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
}
