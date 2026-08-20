# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability in the **xfetch API/SDK** (protocol crates consumed
by every plugin, extension and effect), please report it responsibly by contacting:

**Email:** `x@xscriptor.com`

### What to Include

When reporting a security issue, please provide:

1. **Description** — A clear explanation of the vulnerability
2. **Type** — What kind of security issue is it? (e.g., panic/DoS on malformed input, wire-protocol confusion, supply-chain)
3. **Steps to Reproduce** — Detailed steps to trigger the vulnerability
4. **Impact** — How severe is the issue? What could an attacker do?
5. **Affected Versions** — Which crate versions are affected?
6. **Proposed Fix** (optional) — If you have a suggestion for how to fix it

### Guidelines

- **Do not** open public GitHub issues for security vulnerabilities
- **Do not** disclose the vulnerability publicly until a fix is released
- **Do** give the maintainers reasonable time to address the issue before public disclosure
- Typically, we aim to respond within **7 days** and release a fix within **30 days** for critical issues

## Scope

The `api` repository publishes `xfetch-plugin-api`, `xfetch-extension-api` and
`xfetch-effect-api` — the single source of truth for the wire protocols. Because every
consumer depends on these crates, a flaw here is systemic:

- **Parsing of untrusted input**: deserialization panics, stack overflows or hangs on
  malformed requests/responses (serde structs must handle arbitrary input gracefully).
- **Protocol confusion**: version/kind constants that let one message type be interpreted as
  another, breaking the trust boundary between xfetch and its child processes.
- **Timeout contract**: `with_timeout`/`TimedOut` are the ecosystem's only enforcement point
  for process budgets — any path that can bypass or block them is a denial-of-service
  vulnerability.
- **Supply chain**: dependency changes that could be abused through the crates' popularity
  (suspicious new dependencies or build scripts).
