# forge

> **Status: future work.** Planned replacement for nix's build
> infrastructure inside the sema-ecosystem. Not in any current
> production path. Skeleton-as-design until criome scaffolds.
>
> **Not the lojix deploy daemon.** GitHub redirects
> `LiGoldragon/lojix` → `LiGoldragon/forge` because this repo was
> renamed. Today's deploy stack: `LiGoldragon/lojix-cli` (current
> monolithic CLI). New deploy stack in development:
> `LiGoldragon/lojix` (daemon + thin client) +
> `LiGoldragon/signal-lojix` (wire contract). forge is a separate
> concern.

The forge daemon. Receives effect-bearing signal verbs over
UDS from the criome daemon; links `prism`
to emit Rust source from records; spawns nix subprocesses;
writes outputs into `arca`;
reports outcomes.

See `ARCHITECTURE.md`. Project-wide context:
criome/ARCHITECTURE.md.

## Status

**Skeleton-as-design.** All actor bodies are `todo!()`. Lands
when criome is ready to forward effect-bearing signal verbs.

## License

[License of Non-Authority](LICENSE.md).
