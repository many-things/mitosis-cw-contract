use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub liquidity_manager: Addr,
    pub denom_manager: Addr,
}
