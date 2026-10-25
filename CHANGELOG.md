# Changelog

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
