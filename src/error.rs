//! Error type for the forge daemon.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("nix invocation failed: {0}")]
    NixFailed(String),

    #[error("store-entry placement failed: {0}")]
    StoreWrite(String),

    #[error("invalid signal request: {0}")]
    BadRequest(String),
}

pub type Result<T> = std::result::Result<T, Error>;
