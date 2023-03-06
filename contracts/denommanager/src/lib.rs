pub mod contracts;
pub mod error;
pub mod execute;
pub mod msg;
pub mod query;
pub mod state;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
