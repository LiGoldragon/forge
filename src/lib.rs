//! forge — the executor daemon.
//!
//! Receives effect-bearing signal verbs over UDS from the
//! criome daemon; links `prism` to emit Rust source from
//! records; spawns nix subprocesses; writes outputs into
//! `arca` with RPATH rewrite; reports outcomes back.
//!
//! Internal actors (per
//! [criome/ARCHITECTURE.md §4](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md)):
//!
//! - **NixRunner** — spawns `nix build` / `nixos-rebuild`,
//!   streams stdout/stderr.
//! - **StoreWriter** + **StoreReaderPool** — places store
//!   entries in `~/.arca/`, updates the redb index.
//! - **FileMaterialiser** — projects arca entries into a
//!   working directory.
//!
//! All bodies are `todo!()` skeleton-as-design.

pub mod actors;
pub mod error;
pub mod uds;

pub use error::{Error, Result};
