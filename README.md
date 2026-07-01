# forge

> **Status: future work.** Planned replacement for nix's build
> infrastructure inside the sema-ecosystem. Not in any current
> production path. Skeleton-as-design until criome scaffolds.
>
> **Not the Lojix deploy daemon.** The current deploy stack is
> `LiGoldragon/lojix` (daemon + thin clients) with
> `LiGoldragon/signal-lojix` and `LiGoldragon/meta-signal-lojix` wire
> contracts. forge is a separate concern.

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
