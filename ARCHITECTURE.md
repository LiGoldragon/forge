# ARCHITECTURE — lojix

The lojix daemon — the executor of the lojix family. Takes
plan records from criome (via `lojix-schema` requests over
UDS), runs nix, places artifacts into `lojix-store`, reports
outcomes back.

## Role in the sema-ecosystem

```
   sema records (Opus + OpusDeps + ...)
            │
            ▼
   criome (daemon) — plans the build
            │
            │  lojix-schema (rkyv: RunNix, BundleIntoLojixStore, ...)
            ▼
   lojix (this daemon) — executes
            │
            │  spawns
            ▼
   nix build / nixos-rebuild
            │
            ▼
   /nix/store/<hash>-<name>/   (transient)
            │
            │  StoreWriter: copy closure + RPATH rewrite + blake3
            ▼
   ~/.lojix/store/<blake3>/    (canonical, sema-referenced)
```

## Boundaries

Owns:

- The lojix daemon binary.
- Internal actor system (NixRunner, StoreWriter, StoreReaderPool,
  FileMaterialiser).
- Capability-token verification on incoming lojix-schema requests
  (tokens signed by criome).

Does not own:

- The lojix-schema contract (lives in
  [lojix-schema](https://github.com/LiGoldragon/lojix-schema)).
- The store layout / reader-writer types (lives in
  [lojix-store](https://github.com/LiGoldragon/lojix-store);
  this daemon links it as a library and is the privileged
  writer at runtime).
- Plan creation — criome plans; this daemon executes.
- Sema state.

## Code map

```
src/
├── lib.rs           — module entry + re-exports
├── main.rs          — binary entry, tokio runtime, daemon loop
├── error.rs         — Error enum
├── uds.rs           — UDS listener (lojix-schema requests in)
└── actors/
    ├── mod.rs
    ├── nix_runner.rs        — spawns nix build / nixos-rebuild
    ├── store_writer.rs      — StoreWriter + StoreReaderPool
    └── file_materialiser.rs — projects store entries → workdir
```

All bodies are `todo!()` skeleton-as-design.

## Cross-cutting context

- Project-wide architecture:
  [criome/ARCHITECTURE.md](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md)
- Family peers:
  [lojix-store](https://github.com/LiGoldragon/lojix-store),
  [lojix-cli](https://github.com/LiGoldragon/lojix-cli),
  [lojix-schema](https://github.com/LiGoldragon/lojix-schema)

## Status

**Skeleton-as-design.** Lands when criome scaffolds and is
ready to drive `lojix-schema` verbs.
