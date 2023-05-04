pub mod contracts;
pub mod errors;
pub mod execute;
pub mod query;
pub mod state;
pub mod verify;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
