# Getting Started

<p>
  This repository hosts three public crates: <code>xfetch-plugin-api</code>
  (plugins), <code>xfetch-extension-api</code> (extensions), and
  <code>xfetch-effect-api</code> (effects).
</p>

<h2>Git Dependency</h2>

<pre><code class="language-toml">[dependencies]
serde = { version = "1", features = ["derive"] }
xfetch-plugin-api = { git = "https://github.com/xfetch-cli/api", package = "xfetch-plugin-api" }
xfetch-extension-api = { git = "https://github.com/xfetch-cli/api", package = "xfetch-extension-api" }
xfetch-effect-api = { git = "https://github.com/xfetch-cli/api", package = "xfetch-effect-api" }
</code></pre>

<h2>Local Development</h2>

<p>
  During multi-repo development, you can use a standard Cargo
  <code>path</code> dependency instead.
</p>

<pre><code class="language-toml">[dependencies]
serde = { version = "1", features = ["derive"] }
xfetch-plugin-api = { path = "../api/crates/plugin-api" }
xfetch-extension-api = { path = "../api/crates/extension-api" }
xfetch-effect-api = { path = "../api/crates/effect-api" }
</code></pre>

<h2>Runtime Model</h2>

<p>
  Plugins, extensions and effects are standalone executables. The core writes
  one JSON request to <code>stdin</code>, the program reads it, and writes one
  JSON response to <code>stdout</code>.
</p>

<h2>Next Step</h2>

<ul>
  <li>See <a href="./plugin-sdk.md">Plugin SDK</a> for the plugin API.</li>
  <li>See <a href="./extension-sdk.md">Extension SDK</a> for the extension API.</li>
  <li>See <a href="./effect-sdk.md">Effect SDK</a> for the effect API.</li>
  <li>See <a href="./protocol.md">Protocol Reference</a> for the wire formats.</li>
  <li>See <a href="./examples.md">Examples</a> for minimal implementations.</li>
</ul>
