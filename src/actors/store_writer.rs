//! StoreWriter — places nix-built artifacts into `lojix-store`.
//! StoreReaderPool — concurrent path lookups.

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
        todo!("copy closure with RPATH rewrite (patchelf); blake3; write tree under ~/.lojix/store/<hash>/")
    }
}
