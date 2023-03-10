pub mod contracts;
mod error;
pub mod execute;
pub mod query;
pub mod state;

pub use crate::error::ContractError;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
