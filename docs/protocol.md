# Protocol Reference

<p>
  The current wire protocol version is <code>1</code>.
</p>

<h2>Plugin Kinds</h2>

<ul>
  <li><code>logo_animation</code></li>
  <li><code>info_provider</code></li>
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

<h2>Compatibility</h2>

<p>
  Protocol changes should be introduced in this repository first and then
  adopted by the core and official plugin repositories together.
</p>
