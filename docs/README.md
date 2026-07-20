# Plugin Protocol

<p>
  This document describes the first shared contract extracted into the
  <strong>api</strong> repository.
</p>

<p>
  The goal is to provide a small but serious SDK for plugin authors, not only an internal
  bridge between existing repositories.
</p>

<h2>Overview</h2>

<p>
  xfetch plugins are standalone executables. The core sends a JSON request through
  <code>stdin</code>, the plugin processes it, and the plugin returns a JSON response through
  <code>stdout</code>.
</p>

<p>
  The shared crate for this contract is:
</p>

<pre><code>crates/plugin-api</code></pre>

<h2>Protocol Version</h2>

<p>
  The current wire protocol version is <code>1</code>.
</p>

<h2>Plugin Kinds</h2>

<ul>
  <li><code>logo_animation</code></li>
  <li><code>info_provider</code></li>
</ul>

<h2>Shared Types</h2>

<ul>
  <li><code>AnimationFrame</code></li>
  <li><code>PluginKind</code></li>
  <li><code>EmptyArgs</code></li>
  <li><code>LogoAnimationArgs</code></li>
  <li><code>LogoAnimationRequest</code></li>
  <li><code>LogoAnimationResponse</code></li>
  <li><code>InfoPluginRequest</code></li>
  <li><code>InfoPluginResponse</code></li>
</ul>

<h2>Entrypoint Helpers</h2>

<ul>
  <li><code>read_logo_animation_request()</code></li>
  <li><code>read_info_plugin_request&lt;T&gt;()</code></li>
  <li><code>read_info_plugin_args_or_default&lt;T&gt;()</code></li>
  <li><code>write_logo_animation_frames()</code></li>
  <li><code>write_info_lines()</code></li>
</ul>

<h2>Quick Start</h2>

<p>
  Once this repository is published, third-party plugins can depend on the SDK directly:
</p>

<pre><code class="language-toml">[dependencies]
serde = { version = "1", features = ["derive"] }
xfetch-plugin-api = { git = "https://github.com/xfetch-cli/api", package = "xfetch-plugin-api" }
</code></pre>

<p>
  During local workspace development, a standard Cargo <code>path</code> dependency can be used
  instead.
</p>

<h2>Error Handling</h2>

<p>
  The crate exposes <code>PluginApiError</code> so plugin authors can report clear failures
  for invalid protocol versions, unexpected plugin kinds, invalid typed arguments, and
  malformed responses.
</p>

<p>
  Plugins are expected to print these errors to <code>stderr</code> and exit with a non-zero code.
</p>

<h2>Logo Animation Request</h2>

<pre><code class="language-json">{
  "version": 1,
  "kind": "logo_animation",
  "lines": ["line 1", "line 2"],
  "frames": [["frame 1"], ["frame 2"]],
  "args": {
    "fps": 12,
    "duration_ms": 1200,
    "loop": true,
    "style": "wave"
  }
}</code></pre>

<h2>Logo Animation Response</h2>

<pre><code class="language-json">{
  "frames": [
    {
      "delay_ms": 80,
      "lines": ["rendered line 1", "rendered line 2"]
    }
  ]
}</code></pre>

<h2>Info Provider Request</h2>

<pre><code class="language-json">{
  "version": 1,
  "kind": "info_provider",
  "args": {
    "username": "example",
    "max_lines": 5
  }
}</code></pre>

<h2>Info Provider Response</h2>

<pre><code class="language-json">{
  "lines": [
    "line one",
    "line two"
  ]
}</code></pre>

<h2>Design Notes</h2>

<ul>
  <li>The shared crate only owns the wire protocol, not end-user configuration files.</li>
  <li>Plugin-specific <code>args</code> stay flexible through <code>serde_json::Value</code>.</li>
  <li>Higher-level entrypoint helpers validate the request before plugin logic runs.</li>
  <li>Official plugins are expected to use the same public helpers available to third parties.</li>
</ul>

<h2>Compatibility</h2>

<p>
  Any protocol change should be introduced in this repository first and then adopted by the
  core and official plugins together.
</p>

<h2>Examples</h2>

<ul>
  <li><code>crates/plugin-api/examples/info-plugin.rs</code></li>
  <li><code>crates/plugin-api/examples/logo-animation-plugin.rs</code></li>
</ul>

<p>
  These examples compile with the crate and are intended to stay in sync with the public API.
</p>

<pre><code class="language-rust">use serde::Deserialize;
use xfetch_plugin_api::{read_info_plugin_args_or_default, write_info_lines};

#[derive(Debug, Default, Deserialize)]
struct ExampleArgs {
    label: Option&lt;String&gt;,
}

fn main() {
    let args = match read_info_plugin_args_or_default::&lt;ExampleArgs&gt;() {
        Ok(value) =&gt; value,
        Err(err) =&gt; {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let label = args.label.unwrap_or_else(|| "example".to_string());

    if let Err(err) = write_info_lines(vec![format!("plugin says: {}", label)]) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
</code></pre>
