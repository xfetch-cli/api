# Effect SDK

<p>
  <code>xfetch-effect-api</code> provides the public types and helpers that
  effect authors are expected to use.
</p>

<h2>Core Types</h2>

<ul>
  <li><code>EffectArgs</code></li>
  <li><code>EffectFrame</code></li>
  <li><code>EffectRequest</code></li>
  <li><code>EffectResponse</code></li>
</ul>

<h2>Entrypoints</h2>

<ul>
  <li><code>read_effect_request()</code></li>
  <li><code>write_effect_frames()</code></li>
</ul>

<h2>Error Handling</h2>

<p>
  The crate exposes <code>EffectApiError</code> for invalid protocol versions,
  unexpected effect kinds, invalid typed arguments, and empty frame
  responses.
</p>

<p>
  Effects should write these errors to <code>stderr</code> and exit with a
  non-zero code.
</p>

<h2>Design Notes</h2>

<ul>
  <li>The crate owns the shared wire protocol, not end-user config files.</li>
  <li>Higher-level helpers validate requests before effect logic runs.</li>
  <li>Effects must keep ANSI escape sequences intact when transforming lines (see <code>xfetch-effects-lib</code> in the effects repository).</li>
</ul>

<h2>Reference Implementations</h2>

<p>
  Official effects live in <a href="https://github.com/xfetch-cli/effects">xfetch-cli/effects</a>:
  <code>decrypt</code> and <code>glitch</code>, both built on
  <code>xfetch-effect-api</code>.
</p>
