# lojix

The lojix daemon. Receives [`lojix-schema`](https://github.com/LiGoldragon/lojix-schema)
requests over UDS from the criome daemon; spawns nix
subprocesses; bundles outputs into [`lojix-store`](https://github.com/LiGoldragon/lojix-store);
reports outcomes.

The bare `lojix` name does double duty: this daemon AND the
family-namespace umbrella for `lojix-cli`, `lojix-store`,
`lojix-schema`. Same pattern as `nexus`.

See [`ARCHITECTURE.md`](ARCHITECTURE.md). Project-wide context:
[criome/ARCHITECTURE.md](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md).

## Status

**Skeleton-as-design.** All actor bodies are `todo!()`. Lands
when criome is ready to drive lojix-schema verbs.

## License

[License of Non-Authority](LICENSE.md).
