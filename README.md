<h1 align="center">
<img src="https://raw.githubusercontent.com/xfetch-cli/assets/main/logo/banner/xfetch.svg" width="30%" alt="XFetch banner" />API</h1>

<p>
  <em>
    Shared crates for the <strong><a href="https://github.com/xfetch-cli/xfetch">xfetch</a></strong> ecosystem.
  </em>
</p>

<p>
  The current release line is <strong>0.3.1</strong>. It keeps the JSON
  protocol at version <code>1</code> and the WIT at
  <code>xfetch:runtime@0.1.0</code>; the changes are additive.
</p>

<h2>Current Scope</h2>

<ul>
  <li><code>crates/common</code> (<code>xfetch-api-common</code>): shared protocol constants, frame shape, timeout and JSON IO helpers.</li>
  <li><code>crates/plugin-api</code>: shared protocol crate for xfetch plugins.</li>
  <li><code>crates/extension-api</code>: shared protocol crate for xfetch extensions (config providers).</li>
  <li><code>crates/effect-api</code>: shared protocol crate for xfetch effects (intro animations over the info lines).</li>
  <li><code>crates/guest-api</code>: host-call helpers for WebAssembly guests (<code>http_request</code>, <code>exec</code>, <code>log</code>, <code>protocol_version</code>).</li>
  <li><code>crates/command-api</code>: shared protocol crate for CLI command addons.</li>
  <li><code>wit/xfetch-runtime.wit</code>: component-model contract (<code>plugin</code>, <code>effect</code>, <code>extension</code> and <code>command</code> worlds).</li>
  <li><code>docs/</code>: focused documentation for setup, protocol, SDK usage, wasm guests, and examples.</li>
  <li><code>crates/*/examples/</code>: minimal plugin, effect and extension binaries that compile against the public crates.</li>
</ul>

<h2>Main Crate</h2>

<p>
  The first public crate in this repository is <code>xfetch-plugin-api</code>.
</p>

<ul>
  <li>Defines the wire protocol used between <code>xfetch</code> and plugins.</li>
  <li>Provides typed helpers for reading requests and writing responses.</li>
  <li>Exposes a stable base for official and third-party plugin development.</li>
</ul>

<h2>Effect API</h2>

<p>
  <code>crates/effect-api</code> (<code>xfetch-effect-api</code>) is the contract for
  <strong>effects</strong>: installable intro animations. The core renders the info
  lines and sends them to an effect binary (<code>xfetch-effect-&lt;name&gt;</code>);
  the effect returns a sequence of frames (delay + transformed lines) that the core
  plays before settling on the final content. Effect implementations are collected in
  the <a href="https://github.com/xfetch-cli/effects">xfetch-cli/effects</a> repository
  (the reference <code>decrypt</code> effect lives there).
</p>

<ul>
  <li>Request carries the rendered lines plus <code>style</code>, <code>duration_ms</code>, <code>fps</code> and free-form <code>args</code>.</li>
  <li>Response is a non-empty list of <code>EffectFrame { delay_ms, lines }</code>.</li>
  <li>Effects are opt-in: a missing binary or invalid response simply skips the intro (no behavior change).</li>
</ul>

<h2>Command API</h2>

<p>
  <code>crates/command-api</code> (<code>xfetch-command-api</code>) is the
  contract for CLI command addons. Commands receive arguments and return plain
  text lines, so a command owns its own output format. Components use the
  <code>command</code> world in the WIT package; core modules use the JSON
  request on stdin and plain text on stdout.
</p>

<h2>Documentation</h2>

<ul>
  <li><a href="./docs/README.md">Documentation Index</a></li>
  <li><a href="./docs/getting-started.md">Getting Started</a></li>
  <li><a href="./docs/plugin-sdk.md">Plugin SDK</a></li>
  <li><a href="./docs/protocol.md">Protocol Reference</a></li>
  <li><a href="./docs/extension-sdk.md">Extension SDK</a></li>
  <li><a href="./docs/effect-sdk.md">Effect SDK</a></li>
  <li><a href="./docs/command-sdk.md">Command SDK</a></li>
  <li><a href="./docs/examples.md">Examples</a></li>
</ul>

<h2>Notes</h2>

<ul>
  <li>This repository owns shared contracts, not end-user config files.</li>
  <li>Official plugins are expected to use the same public SDK as external plugins.</li>
  <li>Protocol changes should land here first and then be adopted by the core and plugin repos.</li>
  <li>Publishing order: <code>xfetch-api-common</code> first, then <code>xfetch-plugin-api</code>, <code>xfetch-extension-api</code>, <code>xfetch-effect-api</code> and <code>xfetch-command-api</code>; <code>xfetch-guest-api</code> is independent.</li>
</ul>
