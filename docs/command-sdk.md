# Command SDK

<p>
  <code>xfetch-command-api</code> is the contract for CLI command addons. A
  command addon receives a JSON request with positional arguments and returns
  plain text; the core prints the text as-is unless the user asks for a layout.
</p>

<h2>Protocol</h2>

<ul>
  <li>Request: JSON on stdin for wasm core modules and component guests; native commands may also receive arguments as process arguments.</li>
  <li>Response: plain text on stdout, one field per line.</li>
  <li>Diagnostics: stderr only.</li>
  <li>Exit code: <code>0</code> on success; non-zero on failure.</li>
</ul>

<pre><code class="language-json">{
  "version": 1,
  "kind": "command",
  "command": "github",
  "args": ["owner/repo"],
  "cwd": "/home/user/project",
  "env": {}
}</code></pre>

<pre><code class="language-text">repo: owner/repo
author: owner
languages: C, JavaScript, PHP
stars: 1234
</code></pre>

<h2>Rust Core Module</h2>

<pre><code class="language-toml">[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
xfetch-command-api = "0.3"
xfetch-guest-api = "0.3"</code></pre>

<pre><code class="language-rust">use xfetch_command_api::{read_command_args_or_default, write_command_lines};

fn main() {
    let args = read_command_args_or_default().unwrap_or_default();
    let repo = args.first().map(String::as_str).unwrap_or("owner/repo");
    write_command_lines(&[format!("repo: {repo}")]).expect("write output");
}</code></pre>

<p>
  Wasm core modules use the same JSON request as native commands. Host calls
  come from <code>xfetch-guest-api</code> and remain governed by the manifest
  allowlist.
</p>

<h2>Component Guest</h2>

<p>
  Component guests implement the <code>command</code> world in
  <code>wit/xfetch-runtime.wit</code>:
</p>

<pre><code class="language-wit">world command {
    import host;
    export run: func(request: string) -&gt; result&lt;string, string&gt;;
}</code></pre>

<p>
  Parse the request string, use the typed <code>host.fetch</code> /
  <code>host.exec</code> imports as needed, and return the plain-text response.
  Returning an error string reports a command failure.
</p>

<h2>Native Command</h2>

<p>
  Native command addons can read positional arguments from
  <code>std::env::args().skip(1)</code> and write text to stdout. The core also
  sends the JSON request on stdin, so a native command may use either form.
</p>

<h2>Output Conventions</h2>

<ul>
  <li>Prefer <code>key: value</code> lines for readability and future layout support.</li>
  <li>Keep ANSI escapes out unless the command explicitly opts into terminal output.</li>
  <li>Do not write logs to stdout; use stderr.</li>
  <li>Return an empty response only when the command has nothing to show.</li>
</ul>

<h2>Security</h2>

<ul>
  <li>Native commands run unsandboxed with the current user's privileges.</li>
  <li>Wasm commands use the manifest capability policy: HTTP, exec, filesystem, environment and args are deny-by-default.</li>
  <li>Command output is printed by the core; commands should not assume they own the terminal.</li>
</ul>
