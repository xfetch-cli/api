<h1 align="center">
<img src="https://raw.githubusercontent.com/xfetch-cli/assets/main/logo/banner/xfetch.svg" width="30%" alt="XFetch banner" />API</h1>

<p>
  <em>
    Shared crates for the <strong><a href="https://github.com/xfetch-cli/xfetch">xfetch</a></strong> ecosystem.
  </em>
</p>

<p>
  Today this repository contains the public plugin SDK used by the core,
  official plugins, and third-party plugin authors.
</p>

<h2>Current Scope</h2>

<ul>
  <li><code>crates/plugin-api</code>: shared protocol crate for xfetch plugins.</li>
  <li><code>crates/extension-api</code>: shared protocol crate for xfetch extensions (config providers).</li>
  <li><code>crates/effect-api</code>: shared protocol crate for xfetch effects (intro animations over the info lines).</li>
  <li><code>docs/</code>: focused documentation for setup, protocol, SDK usage, and examples.</li>
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

<h2>Documentation</h2>

<ul>
  <li><a href="./docs/README.md">Documentation Index</a></li>
  <li><a href="./docs/getting-started.md">Getting Started</a></li>
  <li><a href="./docs/plugin-sdk.md">Plugin SDK</a></li>
  <li><a href="./docs/protocol.md">Protocol Reference</a></li>
  <li><a href="./docs/examples.md">Examples</a></li>
</ul>

<h2>Notes</h2>

<ul>
  <li>This repository owns shared contracts, not end-user config files.</li>
  <li>Official plugins are expected to use the same public SDK as external plugins.</li>
  <li>Protocol changes should land here first and then be adopted by the core and plugin repos.</li>
</ul>
