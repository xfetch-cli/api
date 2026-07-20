# xfetch api

<p>
  Shared contracts for the <strong>xfetch</strong> ecosystem.
</p>

<p>
  This repository is intentionally general. It can host multiple shared crates over time,
  while the first extracted contract is the plugin SDK used by the core, official plugins,
  and future third-party plugins.
</p>

<h2>Repository Layout</h2>

<ul>
  <li><code>crates/plugin-api</code>: public plugin SDK with protocol types, validation, typed entrypoints, and examples.</li>
  <li><code>docs/README.md</code>: protocol overview and repository guidelines.</li>
</ul>

<h2>Current Scope</h2>

<p>
  The first public crate already includes real developer-facing capabilities:
</p>

<ul>
  <li>Protocol version constants.</li>
  <li>Plugin kind helpers and validation.</li>
  <li>Shared request and response types.</li>
  <li>Typed helpers for reading requests, defaulting optional args, and writing responses.</li>
  <li>Examples that compile as part of the crate.</li>
  <li>Structured error types for plugin authors.</li>
</ul>

<p>
  User-facing configuration models remain in the <strong>xfetch</strong> core for now.
</p>

<h2>Public Goal</h2>

<p>
  The plugin crate is not only an internal bridge. It is intended to be the supported way for
  developers to create plugins that communicate with <strong>xfetch</strong>.
</p>

<h2>Professional Direction</h2>

<ul>
  <li>Keep the wire protocol stable and versioned.</li>
  <li>Move repeated protocol logic out of plugin binaries.</li>
  <li>Give third-party authors a small but production-ready SDK.</li>
  <li>Keep official plugins on the same public entrypoints used by external authors.</li>
  <li>Expand this repository with more shared crates only when the project genuinely needs them.</li>
</ul>

<h2>Goals</h2>

<ul>
  <li>Reduce protocol duplication between the core and official plugins.</li>
  <li>Make compatibility changes explicit and versioned.</li>
  <li>Prepare the ecosystem for third-party plugin authors.</li>
  <li>Keep the repository broad enough for future shared contracts.</li>
</ul>

<h2>Next Steps</h2>

<ol>
  <li>Publish this repository so other repos can depend on it safely by Git or releases.</li>
  <li>Add compatibility tests across repositories.</li>
  <li>Consider templates or starter repositories for third-party plugin authors.</li>
</ol>
