//! Internal actors. Each handles one capability area.
//!
//! `NixRunner` spawns nix processes; `StoreWriter` +
//! `StoreReaderPool` manage `lojix-store`; `FileMaterialiser`
//! projects store entries into working directories.
//!
//! All bodies are `todo!()` until the criome daemon is ready
//! to drive them.

pub mod nix_runner;
pub mod store_writer;
pub mod file_materialiser;
