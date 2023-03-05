use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Not paused")]
    NotPausedError {},

    #[error("Paused")]
    PausedError {},

    #[error("Reply Not Found: {id:?}")]
    ReplyIdNotFound { id: u64 },

    #[error("Denom Not Found: {denom:?}")]
    DenomNotFound { denom: String },

    #[error("Role Error: Addr {addr:?} has not role {role:?}")]
    RoleNotExist { addr: Addr, role: String },
}
