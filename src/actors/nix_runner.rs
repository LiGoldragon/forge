//! NixRunner — spawns `nix build` and `nixos-rebuild`.

use crate::Result;

pub struct NixRunner;

pub struct RunNixSpec {
    pub flake_ref: String,
    pub attr: String,
}

pub struct RunNixOutcome {
    pub output_path: std::path::PathBuf,
    pub wall_ms: u64,
}

impl NixRunner {
    pub async fn run(_spec: RunNixSpec) -> Result<RunNixOutcome> {
        todo!("spawn nix build; capture stdout/stderr; parse output paths")
    }
}
