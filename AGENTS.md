# Agent instructions — forge

## Repo role

The **forge daemon** — executor for effects sema can't perform directly (nix builds, store writes, deploy actions). Skeleton-as-design today; bodies are `todo!()`.

The phrasing *"effects sema can't perform directly"* is itself a today-vs-eventually marker: today's `sema-db` is a storage kernel; the eventual `Sema` represents all meaning and all computation, at which point build/deploy folds into the Sema substrate and forge's separate-executor role goes away. See `ESSENCE.md` §"Today and eventually".
