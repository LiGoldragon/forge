//! forge — the executor daemon.
//!
//! Receives effect-bearing signal-forge verbs over UDS from
//! the criome daemon, accompanied by a criome-signed capability
//! token authorising deposits into the target arca store. Links
//! `prism` to emit Rust source from records; spawns nix
//! subprocesses (crane + fenix); bundles the closure (RPATH
//! rewrite + deterministic timestamps) into arca's write-only
//! `~/.arca/_staging/`; asks arca-daemon — via signal-arca's
//! `Deposit` verb — to take ownership; reports the resulting
//! blake3 + narhash + wall_ms back to criome.
//!
//! forge does NOT compute the canonical blake3 itself —
//! arca-daemon does, after the atomic move out of staging.
//!
//! Internal actors (per
//! [criome/ARCHITECTURE.md §4](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md)):
//!
//! - **NixRunner** — spawns `nix build` / `nixos-rebuild`,
//!   streams stdout/stderr.
//! - **StoreWriter** — bundles nix output (RPATH rewrite +
//!   deterministic timestamps) into arca's write-only
//!   `~/.arca/_staging/<deposit-id>/`.
//! - **ArcaDepositor** — sends signal-arca `Deposit` to
//!   arca-daemon with the staging id + capability token;
//!   awaits the canonical blake3 reply.
//! - **FileMaterialiser** — projects arca entries into a
//!   working directory.
//!
//! All bodies are `todo!()` skeleton-as-design.

pub mod actors;
pub mod error;
pub mod uds;

pub use error::{Error, Result};
