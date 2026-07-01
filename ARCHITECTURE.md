# ARCHITECTURE — forge

> **Status: future work.** forge is the planned replacement for nix's
> build infrastructure inside the sema-ecosystem — once criome scaffolds
> and is ready to forward effect-bearing signal verbs, forge runs the
> builds, links `prism` to emit Rust source from records, writes
> outputs into `arca`. Today's stack still builds with nix directly;
> forge is **not** in any current production path. The repo is
> skeleton-as-design while criome is being built.
>
> **Build-system family, phased Nix replacement.** forge is the
> emerging build-system family rather than a single binary: `forge-core`
> is the shared standardization contract that sub-forges adopt as they
> mature, and `forge-nix-builder` is the first sub-forge — it wraps Nix
> and extracts as a library under this forge daemon (still the
> Criome-stack executor) rather than replacing Nix outright. The plan
> keeps what is eternal in Nix (content-addressing, derivation graphs,
> hermetic builds) while moving authorization to Criome and binary
> signing to the workspace content-addressed store; generated Rust may
> later become content-addressed crates directly, removing
> generated-code repos. Nix phases out as forge matures.
>
> **Not related to the lojix deploy work.** GitHub redirects
> `LiGoldragon/lojix` → `LiGoldragon/forge` because this repo was
> previously named `lojix` and got renamed. The current deploy stack
> lives in `LiGoldragon/lojix-cli` (today's monolithic CLI) and
> `LiGoldragon/lojix` (the new daemon + thin client repo, in
> development). The contract crate is `LiGoldragon/signal-lojix`.
> forge is a separate concern — the criome-stack executor, not the
> operator-facing deploy orchestrator.

The forge daemon — the executor. Takes plan records from
criome (via signal verbs over UDS), links `prism` to emit
Rust source, runs nix, writes artifacts into `arca`, reports
outcomes back.

> **Scope (today vs eventually).** forge eventually executes effects
> sema can't perform directly — nix builds, store writes, deploy
> actions. In the eventually-self-hosting stack, build/deploy is one
> operation inside ideal Criome's Sema-native substrate; forge is a
> realization step, built rightly for the scope it serves when criome
> lands. See `~/primary/ARCHITECTURE.md` §"Workspace vision and intent".

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

## Pending schema-engine upgrade

**Status:** scheduled for migration to schema-language-based contract per `reports/designer/326-v13-spirit-complete-schema-vision.md` + `reports/designer/324-migration-mvp-spirit-handover-re-specification.md`.

**Target:** this component's hand-written `signal_channel!` invocation + Layer 2 Command/Effect + storage types convert to a single `forge/forge.schema` file. The brilliant macro library (`primary-ezqx.1`) reads the schema + emits all the wire types + ShortHeader projection + dispatcher + VersionProjection + storage descriptors.

**Sequence:** per `reports/designer/316` forge family direction. Spirit is the MVP pilot landing first via `primary-ezqx.1`; this component's schema cutover follows after pilot succeeds and the forge family direction settles (forge currently sits as skeleton-as-design, so cutover bundles with first-real-implementation rather than retrofitting code that does not yet exist).

**Per-component concerns:** Per `/316` forge family direction; schema cutover follows persona triad. The skeleton state means the schema-engine-shaped form is the natural first implementation — no hand-written `signal_channel!` body to migrate; the `.schema` file becomes the substrate from day one once the macro library lands.

**References:**
- `reports/designer/326-v13-spirit-complete-schema-vision.md` — uniform header form + schema-language design
- `reports/designer/324-migration-mvp-spirit-handover-re-specification.md` — migration MVP + handover state
- `reports/designer/322-spirit-mvp-positional-schema-worked-example.md` — Spirit MVP worked example
- `reports/operator/174-schema-import-header-design-critique-2026-05-24.md` — header/body/feature separation + lowering rules
- `reports/designer/316-…` — forge family direction
