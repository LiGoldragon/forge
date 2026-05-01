# forge

The forge daemon. Receives effect-bearing signal verbs over
UDS from the criome daemon; links `prism`
to emit Rust source from records; spawns nix subprocesses;
writes outputs into `arca`;
reports outcomes.

See [`ARCHITECTURE.md`](ARCHITECTURE.md). Project-wide context:
criome/ARCHITECTURE.md.

## Status

**Skeleton-as-design.** All actor bodies are `todo!()`. Lands
when criome is ready to forward effect-bearing signal verbs.

## License

[License of Non-Authority](LICENSE.md).
