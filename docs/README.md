# API Docs

<p>
  Focused documentation for the shared crates in this repository.
</p>

<h2>Documents</h2>

<ul>
  <li><a href="./getting-started.md">Getting Started</a>: dependency setup and first integration steps.</li>
  <li><a href="./plugin-sdk.md">Plugin SDK</a>: public types, entrypoints, and error handling.</li>
  <li><a href="./extension-sdk.md">Extension SDK</a>: public types and the config-provider protocol.</li>
  <li><a href="./effect-sdk.md">Effect SDK</a>: public types, entrypoints, and error handling.</li>
  <li><a href="./protocol.md">Protocol Reference</a>: wire formats, kinds, versioning, and JSON examples.</li>
  <li><a href="./timeouts.md">Timeouts</a>: the <code>with_timeout</code> helper and runtime budgets.</li>
  <li><a href="./wasm-guests.md">WebAssembly Guests</a>: building core modules and components in Rust, Python, Go and C.</li>
  <li><a href="./examples.md">Examples</a>: runnable patterns based on the crate examples.</li>
</ul>

<h2>Current Focus</h2>

<p>
  The public surface is centered on four crates:
  <code>xfetch-plugin-api</code>, <code>xfetch-extension-api</code>,
  <code>xfetch-effect-api</code>, and <code>xfetch-guest-api</code> (host calls
  for WebAssembly guests).
</p>

<p>
  Component guests target the WIT package in
  <a href="../wit/xfetch-runtime.wit"><code>wit/xfetch-runtime.wit</code></a>.
</p>
