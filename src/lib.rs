//! lojix — the lojix daemon.
//!
//! Receives effect-bearing signal verbs over UDS from the
//! criome daemon; spawns nix subprocesses; bundles outputs
//! into `lojix-store` with RPATH rewrite; reports outcomes
//! back.
//!
//! Internal actors (per
//! [criome/ARCHITECTURE.md §4](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md)):
//!
//! - **NixRunner** — spawns `nix build` / `nixos-rebuild`,
//!   streams stdout/stderr.
//! - **StoreWriter** + **StoreReaderPool** — places store
//!   entries in `~/.lojix/store/`, updates the redb index.
//! - **FileMaterialiser** — projects store entries into a
//!   working directory.
//!
//! All bodies are `todo!()` skeleton-as-design.

pub mod actors;
pub mod error;
pub mod uds;

pub use error::{Error, Result};
