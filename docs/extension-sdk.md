# Extension SDK

<p>
  <code>xfetch-extension-api</code> provides the public types that extension
  authors are expected to use. Extensions are standalone binaries that
  communicate with xfetch over a stdin/stdout JSON protocol, separate from the
  plugin system.
</p>

<h2>Core Types</h2>

<ul>
  <li><code>ConfigProviderRequest</code></li>
  <li><code>ConfigProviderResponse</code></li>
  <li><code>ExtensionKind</code></li>
</ul>

<h2>Runtime Model</h2>

<p>
  xfetch writes one <code>ConfigProviderRequest</code> JSON to <code>stdin</code>
  (the fully resolved config plus optional <code>args</code>), the extension
  modifies it, and writes one <code>ConfigProviderResponse</code> JSON to
  <code>stdout</code> with the new config.
</p>

<pre><code class="language-json">{
  "version": 1,
  "kind": "config_provider",
  "config": { "show_colors": true, "modules": ["os", "cpu"] },
  "args": { "strategy": "random" }
}</code></pre>

<h2>Response</h2>

<pre><code class="language-json">{
  "config": { "show_colors": false, "modules": ["cpu", "memory"] }
}</code></pre>

<h2>Error Handling</h2>

<p>
  Extensions should write errors to <code>stderr</code> and exit with a
  non-zero code; the core skips the extension and keeps the previous config.
</p>

<h2>Design Notes</h2>

<ul>
  <li>The crate owns the shared wire protocol, not end-user config files.</li>
  <li>Extensions run at config load time and can modify or replace the effective config.</li>
  <li>Official extensions are expected to use the same public SDK as third-party extensions.</li>
</ul>
