# INTENT — forge

*What the psyche has explicitly intended for this project.
Synthesised from psyche statements and applicable workspace
constraints; not embellished. `ARCHITECTURE.md` says what forge
IS; this file says what the psyche wants it to BE.*

## Purpose

`forge` is the criome-stack executor daemon — the planned
replacement for nix's build infrastructure inside the
sema-ecosystem. It takes plan records from `criome` over signal
verbs, links `prism` to emit Rust source from records, runs nix,
writes build outputs into `arca`'s staging directory, and reports
outcomes back. criome plans and signs capability tokens; forge
executes.

**Not the deploy orchestrator, and not the lojix deploy work.**
GitHub redirects `LiGoldragon/lojix` → `LiGoldragon/forge` because
this repo was previously named `lojix` and got renamed. The
operator-facing deploy stack is `lojix-cli` (today's monolithic CLI)
and `lojix` (the new daemon + thin client); forge is a separate
concern — the criome-stack executor.

## Constraints

- **forge executes; it does not plan.** Plan creation lives in
  criome; forge consumes plan records and runs the build/store/deposit
  pipeline. Sema state is not owned here.
- **forge deposits into arca's write-only staging and asks
  arca-daemon to take ownership.** forge does NOT compute the
  blake3 hash or write to arca's canonical store — arca-daemon is
  the privileged writer that verifies the token, computes the
  hash, and performs the atomic move.
- **Every incoming request is a capability-token-verified
  `signal-forge` verb.** forge verifies criome-signed capability
  tokens on inbound requests and presents a criome-signed token to
  arca-daemon when depositing. The signal contract lives in
  `signal` + `signal-forge`; this daemon consumes both and does not
  redefine framing.
- **Internal work is an actor system.** NixRunner, StoreWriter,
  StoreReaderPool, FileMaterialiser, ArcaDepositor are Kameo actors
  that are data-bearing nouns; no zero-state holders. Per
  `primary/skills/actor-systems.md`.

## Stack discipline

- Closed enums; typed `Error`; full English words with no
  crate-name prefix on types. Per `primary/skills/naming.md` and
  `primary/skills/rust-discipline.md`.
- The schema-engine cutover is the natural first implementation:
  the hand-written `signal_channel!` body, Layer-2 Command/Effect
  types, and storage descriptors converge into a single
  `forge/forge.schema` file emitted by the schema macro library —
  no hand-written body to migrate, because the skeleton state means
  the schema-shaped form is the day-one substrate.

## Scope — today, not eventually

forge eventually executes effects Sema cannot perform directly —
nix builds, store writes, deploy actions. In the
eventually-self-hosting stack, build/deploy is one operation inside
ideal Criome's Sema-native substrate; forge is a realization step,
built rightly for the scope it serves when criome lands. Today it
is **skeleton-as-design** — types pinned, bodies `todo!()`, not in
any current production path — and lands when criome scaffolds and
is ready to forward effect-bearing signal verbs. Per
`primary/ESSENCE.md` §"Today and eventually".

*Source statements live in Spirit intent records and the project's
`ARCHITECTURE.md`. Workspace-shape intent stays in
`primary/INTENT.md` and the named skills above.*
