# Changelog

## 2026-11-06 — v0.3.1

### Added

- New `command` world in `wit/xfetch-runtime.wit` for CLI command addons. The
  request is JSON and the response is plain text lines.
- New `xfetch-command-api` crate with the command request type, stdin/stdout
  helpers and line formatting utilities.
- `docs/command-sdk.md` documents the command protocol, Rust core-module
  example, component world and native command conventions.

### Changed

- All API crates bumped to `0.3.1`.
- `xfetch-guest-api` no longer declares an unused direct `serde` dependency.
- Removed the unused `parse_json` helper from `xfetch-command-api`.
- Updated the protocol reference, WebAssembly guest guide, README, ROADMAP and
  getting-started guide for the command kind and 0.3.1.

## 2026-11-03 — v0.3.0

### Added

- `xfetch-api-common` 0.3.0: shared JSON protocol constants, frame shape,
  timeout helper and stdin/stdout JSON helpers. It carries no per-kind
  request/response types; plugins, effects and extensions keep their own
  contracts.
- `xfetch-extension-api`: `io` module, standard entrypoints
  (`read_config_provider_request`, `read_config_provider_args_or_default`,
  `write_config_provider_response`, `write_config`), `ExtensionApiError`,
  `ConfigProviderRequest::validate()`, `kind_enum()`, `parse_args()` and
  `parse_args_or_default()`.
- Effect and extension examples: `crates/effect-api/examples/simple-effect.rs`
  and `crates/extension-api/examples/simple-extension.rs`.
- `rust-tests.yml`: fmt, clippy `-D warnings`, workspace tests and example
  builds on push and pull request.

### Changed

- All SDK crates bumped to `0.3.0`.
- `with_timeout` and JSON IO now delegate to `xfetch-api-common`; the existing
  public paths and error variants are preserved.
- `LogoAnimationRequest` and `EffectRequest` tolerate a missing optional
  `args` field through `serde(default)`.
- `PluginApiError` and `EffectApiError` implement `From<serde_json::Error>`
  and `From<xfetch-api-common::IoError>`.
- `PROTOCOL_VERSION` and the kind constants now reference the shared
  protocol module; values are unchanged (`1`, `logo_animation`,
  `info_provider`, `effect`, `config_provider`).
- `xfetch-guest-api` uses `base64` 0.23.
- `security-audit.yml` uses `actions/checkout` 7.0.1 and the current stable
  `dtolnay/rust-toolchain` pin.
- `AnimationFrame` and `EffectFrame` gained `From`/`Into` conversions to the
  shared `Frame`; the existing types and field layout are unchanged.

### Compatibility

- JSON protocol remains version `1`.
- WIT remains `xfetch:runtime@0.1.0`.
- Existing public items, crate names and request/response shapes are
  preserved; the release is additive.

## 2026-10-24 — Dependency security automation

- Added `.github/workflows/security-audit.yml`: runs `cargo audit` 0.22.2 with `--deny warnings` on push, pull request and a weekly schedule. The workflow generates a temporary lockfile before auditing because the workspace does not version one.
- Added weekly Dependabot updates for Cargo dependencies and GitHub Actions (`open-pull-requests-limit: 5`, minor/patch grouped).
- Actions are pinned by commit SHA and checkouts do not persist credentials. No crate source or public API changed; a generated lockfile audits clean (16 crates, 0 vulnerabilities, 0 warnings).

## 2026-09-12
- Added `xfetch-guest-api`: host-call bridge for WebAssembly core-module guests (`host_call`, `http_request`, `exec`, `log`, `protocol_version`) plus the `xfetch_alloc`/`xfetch_free` exports the host needs.
- Added `wit/xfetch-runtime.wit`: the component-model contract with `plugin`, `effect` and `extension` worlds and the typed `fetch`/`exec`/`log`/`protocol-version` host interface.
- `with_timeout` is now wasm-compatible: on `wasm32` the task runs inline (no worker threads) and the host enforces the deadline through wasmtime epochs.


## 2026-08-19
- Added `with_timeout(budget, task)` to `xfetch-plugin-api` and `xfetch-extension-api`. Plugins and extensions declare their own runtime budget in code; the task runs on a worker thread and `Err(TimedOut)` is returned when the budget elapses, so a hung plugin/extension can never hang xfetch.
