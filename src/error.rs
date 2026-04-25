//! Error type for the lojix daemon.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("nix invocation failed: {0}")]
    NixFailed(String),

    #[error("store-entry placement failed: {0}")]
    StoreWrite(String),

    #[error("invalid lojix-schema request: {0}")]
    BadRequest(String),
}

pub type Result<T> = std::result::Result<T, Error>;
