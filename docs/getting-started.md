# Getting Started

<p>
  The current public crate in this repository is <code>xfetch-plugin-api</code>.
</p>

<h2>Git Dependency</h2>

<pre><code class="language-toml">[dependencies]
serde = { version = "1", features = ["derive"] }
xfetch-plugin-api = { git = "https://github.com/xfetch-cli/api", package = "xfetch-plugin-api" }
</code></pre>

<h2>Local Development</h2>

<p>
  During multi-repo development, you can use a standard Cargo
  <code>path</code> dependency instead.
</p>

<pre><code class="language-toml">[dependencies]
serde = { version = "1", features = ["derive"] }
xfetch-plugin-api = { path = "../api/crates/plugin-api" }
</code></pre>

<h2>Runtime Model</h2>

<p>
  Plugins are standalone executables. The core writes one JSON request to
  <code>stdin</code>, the plugin reads it, and the plugin writes one JSON
  response to <code>stdout</code>.
</p>

<h2>Next Step</h2>

<ul>
  <li>See <a href="./plugin-sdk.md">Plugin SDK</a> for the public API.</li>
  <li>See <a href="./protocol.md">Protocol Reference</a> for the wire format.</li>
  <li>See <a href="./examples.md">Examples</a> for minimal plugin implementations.</li>
</ul>
