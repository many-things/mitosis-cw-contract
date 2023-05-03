use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Not paused")]
    NotPausedError {},

    #[error("Paused")]
    PausedError {},

    #[error("Public key not registered")]
    PublicKeyNotRegistered {},

    #[error("Reply Not Found: {id:?}")]
    ReplyIdNotFound { id: u64 },

    #[error("Denom Not Found: {denom:?}")]
    DenomNotFound { denom: String },

    #[error("Role Error: Addr {addr:?} has not role {role:?}")]
    RoleNotExist { addr: Addr, role: String },

    #[error("{msg:?}")]
    InvalidArgument { msg: String },

    #[error("Pay Error: You must send one asset")]
    MustPayOne {},

    #[error("Withdraw not flushed")]
    WithdrawNotFlushed {},

    #[error("Unbond not flushed")]
    UnbondNotFlushed {},

    #[error("lengths wrong")]
    WrongLength {},

    #[error("invalid pub key")]
    InvalidPubKey {},
}
