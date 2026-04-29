# ARCHITECTURE — forge

The forge daemon — the executor. Takes plan records from
criome (via signal verbs over UDS), links `prism` to emit
Rust source, runs nix, writes artifacts into `arca`, reports
outcomes back.

## Role in the sema-ecosystem

```
   sema records (Opus + OpusDeps + ...)
            │
            ▼
   criome (daemon) — plans the build
            │
            │  signal (rkyv: effect-bearing verbs)
            ▼
   forge (this daemon) — executes
            │
            │  links prism (records → .rs)
            │  spawns nix
            ▼
   nix build / nixos-rebuild
            │
            ▼
   /nix/store/<hash>-<name>/   (transient)
            │
            │  StoreWriter: copy closure + RPATH rewrite + blake3
            ▼
   ~/.arca/<blake3>/           (canonical, sema-referenced)
```

## Boundaries

Owns:

- The forge daemon binary.
- Internal actor system (NixRunner, StoreWriter, StoreReaderPool,
  FileMaterialiser).
- The prism library link (records → Rust source).
- Capability-token verification on incoming signal requests
  (tokens signed by criome).

Does not own:

- The signal contract (lives in
  [signal](https://github.com/LiGoldragon/signal)) and its
  forge-bound layer (lives in
  [signal-forge](https://github.com/LiGoldragon/signal-forge));
  this daemon consumes both — signal for Frame/handshake/auth
  + record types, signal-forge for the criome-to-forge verbs.
- The arca layout / reader-writer types (lives in
  [arca](https://github.com/LiGoldragon/arca); this daemon
  links it as a library and is the privileged writer at
  runtime).
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
    ├── store_writer.rs      — StoreWriter + StoreReaderPool
    └── file_materialiser.rs — projects arca entries → workdir
```

All bodies are `todo!()` skeleton-as-design.

## Cross-cutting context

- Project-wide architecture:
  [criome/ARCHITECTURE.md](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md)
- Sibling crates:
  [arca](https://github.com/LiGoldragon/arca),
  [prism](https://github.com/LiGoldragon/prism)

## Status

**Skeleton-as-design.** Lands when criome scaffolds and is
ready to forward effect-bearing signal verbs.
