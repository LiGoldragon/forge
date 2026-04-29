//! StoreWriter — bundles nix-built artifacts (RPATH rewrite,
//! deterministic timestamps) and writes the canonicalised tree
//! into arca's write-only `_staging/` directory. Hand-off to
//! arca-daemon happens in `arca_depositor.rs`.
//!
//! StoreReaderPool — concurrent path lookups against arca
//! stores (filesystem-direct; no daemon round-trip needed).

use crate::Result;

pub struct StoreWriter;
pub struct StoreReaderPool;

pub struct BundleSpec {
    pub nix_output: std::path::PathBuf,
}

pub struct BundleOutcome {
    pub store_entry_hash: [u8; 32],
}

impl StoreWriter {
    pub async fn bundle(_spec: BundleSpec) -> Result<BundleOutcome> {
        todo!("copy closure with RPATH rewrite (patchelf); deterministic timestamps; write canonicalised tree under ~/.arca/_staging/<deposit-id>/ — arca-daemon computes blake3 + moves into the target store")
    }
}
