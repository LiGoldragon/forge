//! ArcaDepositor — sends `signal-arca::Deposit` to arca-daemon.
//!
//! After `StoreWriter` has bundled the nix closure (RPATH
//! rewrite + deterministic timestamps) into arca's write-only
//! `~/.arca/_staging/<deposit-id>/`, this actor presents the
//! criome-signed capability token to arca-daemon and asks it
//! to take ownership: arca-daemon verifies the token, computes
//! the blake3 of the staged tree, atomically moves it into the
//! target store at `~/.arca/<store>/<blake3>/`, updates the
//! per-store redb index, and replies with the hash.
//!
//! forge does NOT compute the blake3 itself — arca-daemon
//! does, after the move from staging.

use crate::Result;

pub struct ArcaDepositor;

pub struct DepositSpec {
    pub staging_id: String,
    pub capability_token: Vec<u8>,
}

pub struct DepositOutcome {
    pub arca_hash: [u8; 32],
}

impl ArcaDepositor {
    pub async fn deposit(_spec: DepositSpec) -> Result<DepositOutcome> {
        todo!(
            "send signal-arca Deposit {{ staging_id, capability_token }} \
             to arca-daemon over UDS; await reply with the canonical \
             blake3; arca-daemon performs the move from _staging/ into \
             the target store"
        )
    }
}
