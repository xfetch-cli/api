# xfetch-api-common

Shared primitives for the xfetch addon SDK crates:

- JSON wire protocol version and kind constants;
- the common `Frame { delay_ms, lines }` shape;
- stdin/stdout JSON helpers returning `IoError`;
- `with_timeout` for native addons, with the host-enforced wasm behavior.

This crate intentionally contains no per-kind request/response types.
Plugins, effects and extensions keep their own contracts in
`xfetch-plugin-api`, `xfetch-effect-api`, `xfetch-extension-api` and
`xfetch-command-api`.

Published first, before the per-kind crates.
