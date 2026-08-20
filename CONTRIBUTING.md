<h1>Contributing to the API</h1>

<p>
  Thanks for contributing to the <strong>xfetch</strong> SDK ecosystem.
  This repository hosts the protocol crates every plugin, extension and effect
  builds against:
</p>

<ul>
  <li><code>crates/plugin-api</code> — <code>xfetch-plugin-api</code>: info providers and logo animations</li>
  <li><code>crates/extension-api</code> — <code>xfetch-extension-api</code>: config providers</li>
  <li><code>crates/effect-api</code> — <code>xfetch-effect-api</code>: intro effects</li>
</ul>

<h2>Workflow</h2>

<ol>
  <li>Fork the repository and create a feature branch.</li>
  <li>Make your changes in the relevant crate under <code>crates/&lt;name&gt;/</code>.</li>
  <li>
    Run the full CI locally before opening the PR:
    <code>bash scripts/ci.sh</code> (Linux/macOS) or <code>./scripts/ci.ps1</code>
    (Windows), plus <code>bash scripts/ci-windows.sh</code> for the Windows
    cross-target check. PRs that fail CI are rejected.
  </li>
  <li>Document protocol changes in <a href="./docs/">docs/</a> (protocol.md, the matching <code>*sdk.md</code>, timeouts.md).</li>
  <li>Add an entry to <a href="./CHANGELOG.md">CHANGELOG.md</a>.</li>
  <li>Open a pull request.</li>
</ol>

<h2>API Rules</h2>

<ul>
  <li><strong>Semver.</strong> Breaking protocol changes bump the major version of the affected crate; additive changes (new fields, new constants) must remain backward compatible with the wire format.</li>
  <li><strong>Keep crates platform-neutral.</strong> The SDK types are plain serde structs — no OS-specific code, no runtime dependencies beyond <code>serde</code>/<code>serde_json</code> unless strictly necessary.</li>
  <li><strong>Every timeout helper stays in the API.</strong> <code>with_timeout</code>/<code>TimedOut</code> are the single place where process budgets are enforced; plugins, extensions and effects rely on them (see <a href="./docs/timeouts.md">timeouts.md</a>).</li>
  <li><strong>Document every public item.</strong> These crates are the reference documentation for the ecosystem; public types and functions need doc comments with protocol semantics.</li>
  <li><strong>Keep the wire protocol explicit.</strong> Use versioned constants (<code>PROTOCOL_VERSION</code>, kind constants) and never rely on runtime type names.</li>
  <li>Prefer stable, actively maintained dependencies and keep them minimal.</li>
</ul>

<h2>Code of Conduct</h2>

<p>
  Be respectful, constructive, and collaborative. Harassment, trolling, and
  personal attacks are not tolerated.
</p>
