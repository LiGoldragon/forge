//! Internal actors. Each handles one capability area.
//!
//! `NixRunner` spawns nix processes; `StoreWriter` bundles the
//! nix closure (RPATH rewrite + deterministic timestamps) into
//! arca's write-only `~/.arca/_staging/<deposit-id>/`;
//! `ArcaDepositor` sends `signal-arca::Deposit` to arca-daemon
//! and awaits the canonical blake3; `FileMaterialiser` projects
//! arca entries into working directories.
//!
//! All bodies are `todo!()` until the criome daemon is ready
//! to drive them.

pub mod nix_runner;
pub mod store_writer;
pub mod arca_depositor;
pub mod file_materialiser;
