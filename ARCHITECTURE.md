# ARCHITECTURE — forge

The forge daemon — the executor. Takes plan records from
criome (via signal verbs over UDS), links `prism` to emit
Rust source, runs nix, writes artifacts into `arca`, reports
outcomes back.

## Role in the sema-ecosystem

```
   sema records (target Graph + DependsOn graphs + Contains nodes + edges)
            │
            ▼
   criome (daemon) — plans the build, signs capability tokens
            │
            │  signal-forge (rkyv: effect-bearing verbs)
            │  + criome-signed token authorising arca writes
            ▼
   forge (this daemon) — executes
            │
            │  links prism (records → .rs)
            │  spawns nix
            ▼
   nix build / nixos-rebuild
            │
            ▼
   /nix/store/<hash>-<name>/         (transient)
            │
            │  StoreWriter actor: copy closure + RPATH rewrite +
            │                     deterministic timestamps
            ▼
   ~/.arca/_staging/<deposit-id>/     (write-only staging)
            │
            │  signal-arca: Deposit { staging_id, capability_token }
            ▼
   arca-daemon — verifies token, computes blake3, atomic move
            │
            ▼
   ~/.arca/<store>/<blake3>/         (canonical, sema-referenced)
```

forge **writes to arca's staging directory** and asks
arca-daemon to take ownership. forge does NOT compute the
blake3 or write to arca's canonical store — arca-daemon does.

## Boundaries

Owns:

- The forge daemon binary.
- Internal actor system (NixRunner, StoreWriter, StoreReaderPool,
  FileMaterialiser, ArcaDepositor).
- The prism library link (records → Rust source).
- Capability-token verification on incoming signal-forge
  requests from criome (tokens signed by criome).
- Holds the criome-signed capability token forge presents to
  arca-daemon when depositing build outputs.

Does not own:

- The signal contract (lives in
  signal) and its
  forge-bound layer (lives in
  signal-forge);
  this daemon consumes both — signal for Frame/handshake/auth
  + record types, signal-forge for the criome-to-forge verbs.
- The arca store layout (lives in
  arca; arca-daemon is
  the privileged writer; this daemon deposits into arca's
  write-only staging and asks arca-daemon to take ownership).
- Plan creation — criome plans; this daemon executes.
- Sema state.

## Code map

```
src/
├── lib.rs           — module entry + re-exports
├── main.rs          — binary entry, tokio runtime, daemon loop
├── error.rs         — Error enum
├── uds.rs           — UDS listener (signal requests in)
└── actors/
    ├── mod.rs
    ├── nix_runner.rs        — spawns nix build / nixos-rebuild
    ├── store_writer.rs      — bundles nix output (RPATH rewrite,
    │                          deterministic timestamps) and writes
    │                          into arca's _staging directory
    ├── arca_depositor.rs    — sends signal-arca Deposit verb to
    │                          arca-daemon with the staging id +
    │                          capability token; awaits hash reply
    └── file_materialiser.rs — projects arca entries → workdir
```

All bodies are `todo!()` skeleton-as-design.

## Cross-cutting context

- Project-wide architecture:
  criome/ARCHITECTURE.md
- Sibling crates:
  arca,
  prism

## Status

**Skeleton-as-design.** Lands when criome scaffolds and is
ready to forward effect-bearing signal verbs.
