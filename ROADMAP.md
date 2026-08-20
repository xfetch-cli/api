# API / SDK Roadmap

Status-driven roadmap for the protocol crates. The SDK is stable; changes land
only when the core or the ecosystem needs them.

## Done

- [x] `xfetch-plugin-api` — info providers and logo animations
- [x] `xfetch-extension-api` — config providers
- [x] `xfetch-effect-api` — intro effects
- [x] Versioned protocol constants (`PROTOCOL_VERSION`, kind constants)
- [x] Timeout contract: `with_timeout`/`TimedOut` as the single enforcement point
      (see `docs/timeouts.md`)
- [x] SDK documentation: `docs/getting-started.md`, `docs/protocol.md`,
      `docs/plugin-sdk.md`, `docs/extension-sdk.md`, `docs/effect-sdk.md`, `docs/examples.md`
- [x] Local CI (fmt, clippy, tests + Windows cross-target) via `scripts/`
