# Protocol Reference

<p>
  The current wire protocol version is <code>1</code>.
</p>

<h2>Plugin Kinds</h2>

<ul>
  <li><code>logo_animation</code></li>
  <li><code>info_provider</code></li>
  <li><code>effect</code></li>
  <li><code>config_provider</code> (extension protocol)</li>
</ul>

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

<h2>Effect Request</h2>

<p>
  The core renders the info lines and sends them to an effect binary
  (<code>xfetch-effect-&lt;name&gt;</code>); the effect returns per-frame states.
</p>

<pre><code class="language-json">{
  "version": 1,
  "kind": "effect",
  "lines": ["\u001b[32m os: Arch \u001b[0m", "\u001b[32m cpu: 3.2 GHz \u001b[0m"],
  "args": {
    "style": "glitch",
    "duration_ms": 800,
    "fps": 30
  }
}</code></pre>

<h2>Effect Response</h2>

<pre><code class="language-json">{
  "frames": [
    { "delay_ms": 33, "lines": ["@peo@f8i*xo&23#", "1qnx4w9@&k6c"] },
    { "delay_ms": 33, "lines": ["os: Arch", "cpu: 3.2 GHz"] }
  ]
}</code></pre>

<p>
  The last frame should reach the final content. Effects must keep ANSI escape
  sequences intact while transforming the lines.
</p>

<h2>Config Provider Request (Extension)</h2>

<pre><code class="language-json">{
  "version": 1,
  "kind": "config_provider",
  "config": { "show_colors": true, "modules": ["os", "cpu"] },
  "args": { "strategy": "random" }
}</code></pre>

<h2>Config Provider Response (Extension)</h2>

<pre><code class="language-json">{
  "config": { "show_colors": false, "modules": ["cpu", "memory"] }
}</code></pre>

<h2>Compatibility</h2>

<p>
  Protocol changes should be introduced in this repository first and then
  adopted by the core and official plugin repositories together.
</p>
