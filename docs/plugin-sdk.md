# Plugin SDK

<p>
  <code>xfetch-plugin-api</code> provides the public types and helpers that
  plugin authors are expected to use.
</p>

<h2>Core Types</h2>

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

<h2>Entrypoints</h2>

<ul>
  <li><code>read_logo_animation_request()</code></li>
  <li><code>read_info_plugin_request&lt;T&gt;()</code></li>
  <li><code>read_info_plugin_args_or_default&lt;T&gt;()</code></li>
  <li><code>write_logo_animation_frames()</code></li>
  <li><code>write_info_lines()</code></li>
</ul>

<h2>Error Handling</h2>

<p>
  The crate exposes <code>PluginApiError</code> for invalid protocol versions,
  unexpected plugin kinds, invalid typed arguments, and malformed responses.
</p>

<p>
  Plugins should write these errors to <code>stderr</code> and exit with a
  non-zero code.
</p>

<h2>Design Notes</h2>

<ul>
  <li>The crate owns the shared wire protocol, not end-user config files.</li>
  <li>Higher-level helpers validate requests before plugin logic runs.</li>
  <li>Official plugins are expected to use the same public SDK as third-party plugins.</li>
</ul>
