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

    #[error("Assets Not Found")]
    AssetNotFound {},

    #[error("Deposit Asset {val:?} Not Found")]
    DepositAssetNotFound { val: String },

    #[error("Insufficient Withdrawable Asset")]
    InsufficientWithdrawableAsset {},

    #[error("Reply Id not found: {id:?}")]
    ReplyIdNotFound { id: u64 },

    #[error("Role Error: Addr {addr:?} has not role {role:?}")]
    RoleNotExist { addr: Addr, role: String },
}