# WebAssembly Guests

<p>
  xfetch can run plugins, effects and extensions compiled to WebAssembly. This
  guide covers the authoring side; the runtime reference (manifest schema and
  CLI tooling) lives in the core repository at
  <code>xfetch/docs/WASM.md</code>.
</p>

<h2>Two Guest Shapes</h2>

<table>
  <thead>
    <tr><th>Shape</th><th>Target</th><th>How to build</th></tr>
  </thead>
  <tbody>
    <tr>
      <td>Core module</td>
      <td><code>wasm32-wasip1</code></td>
      <td>Any language that produces a WASI command; JSON on stdin/stdout</td>
    </tr>
    <tr>
      <td>Component</td>
      <td>Component model</td>
      <td><code>componentize-py</code>, <code>componentize-js</code>, <code>wit-bindgen</code></td>
    </tr>
  </tbody>
</table>

<p>
  Core modules reuse the exact same protocol as native guests, so existing
  plugins only need a new target and a manifest. Components use the typed WIT
  contract in <code>wit/xfetch-runtime.wit</code>, printable with
  <code>xfetch wasm wit</code>.
</p>

<h2>Rust Core Modules</h2>

<p>
  Add the wasm target and build:
</p>

<pre><code class="language-bash">rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1</code></pre>

<p>
  The existing API crates work unchanged on <code>wasm32-wasip1</code>. Host
  operations come from <code>xfetch-guest-api</code>:
</p>

<pre><code class="language-toml">[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
xfetch-plugin-api = "0.2"
xfetch-guest-api = "0.2"</code></pre>

<pre><code class="language-rust">use serde::Deserialize;
use xfetch_guest_api::{http_request, log, protocol_version};
use xfetch_plugin_api::{read_info_plugin_args_or_default, write_info_lines};

#[derive(Debug, Default, Deserialize)]
struct Args {
    city: Option&lt;String&gt;,
}

fn main() {
    let args = read_info_plugin_args_or_default::&lt;Args&gt;().expect("request");
    let city = args.city.unwrap_or_else(|| "London".to_string());

    log("info", "fetching weather");
    let response = match http_request(
        "GET",
        &amp;format!("https://wttr.in/{}?format=3", city),
        &amp;[],
        None,
        Some(5_000),
    ) {
        Ok(response) =&gt; response,
        Err(err) =&gt; {
            write_info_lines(vec![format!("weather unavailable: {}", err)]).expect("response");
            return;
        }
    };

    let line = String::from_utf8_lossy(&amp;response.body).trim().to_string();
    let version = protocol_version().unwrap_or(1);
    write_info_lines(vec![line, format!("host protocol {}", version)]).expect("response");
}</code></pre>

<p>
  On non-wasm targets the same code compiles: host calls return
  <code>HostErrorKind::Unsupported</code>, which lets you unit test the guest
  natively.
</p>

<h2>Python Components</h2>

<p>
  <code>componentize-py</code> turns a Python class into a component. Generate
  bindings from the WIT contract to see the exact method signatures:
</p>

<pre><code class="language-bash">python3 -m venv .venv
. .venv/bin/activate
pip install componentize-py
componentize-py -d path/to/api/wit -w plugin bindings ./bindings</code></pre>

<p>
  Implement the generated protocol in your app module and bundle it:
</p>

<pre><code class="language-python">import json
from wit_world.imports import host

class WitWorld:
    def run(self, request: str) -&gt; str:
        payload = json.loads(request)
        host.log("info", "hello from python")
        return json.dumps({"lines": [f"kind: {payload.get('kind')}"]})</code></pre>

<pre><code class="language-bash">componentize-py -d path/to/api/wit -w plugin componentize app -p . -o dist/app.wasm</code></pre>

<p>
  Use <code>-w effect</code> or <code>-w extension</code> for the other
  contracts; the exported method is always <code>run</code>.
</p>

<h2>Go Core Modules</h2>

<p>
  Go 1.21+ targets WASI directly; no TinyGo is required:
</p>

<pre><code class="language-bash">GOOS=wasip1 GOARCH=wasm go build -o dist/app.wasm .</code></pre>

<p>
  The request/response types are plain JSON structs. Go guests that only
  compute values need no host calls; the runtime calls
  <code>proc_exit(0)</code> on return and the host treats it as a clean exit.
</p>

<h2>C Core Modules</h2>

<p>
  Freestanding C needs no WASI sysroot: declare the two or three WASI imports
  with clang attributes and build with <code>-nostdlib</code>.
</p>

<pre><code class="language-c">__attribute__((import_module("wasi_snapshot_preview1"), import_name("fd_write")))
extern unsigned int fd_write(unsigned int fd, const void *iovs, unsigned int iovs_len, unsigned int *nwritten);

__attribute__((export_name("_start")))
void _start(void) {
    /* read stdin and write a JSON response */
}</code></pre>

<pre><code class="language-bash">clang --target=wasm32-wasip1 -O2 -nostdlib -fno-stack-protector \
  -Wl,--no-entry -Wl,--export-memory -Wl,--allow-undefined \
  -o dist/app.wasm main.c</code></pre>

<h2>Manifest</h2>

<p>
  Ship an <code>xfetch-plugin.json</code>, <code>xfetch-effect.json</code> or
  <code>xfetch-extension.json</code> next to the source. The installer copies
  it beside the artifact. Minimal example:
</p>

<pre><code class="language-json">{
  "manifest_version": 1,
  "name": "my-guest",
  "kind": "info_provider",
  "runtime": "core",
  "build": "cargo build --release --target wasm32-wasip1",
  "artifact": "target/wasm32-wasip1/release/my-guest.wasm",
  "capabilities": {
    "http": { "allow": ["https://wttr.in/*"] }
  },
  "limits": { "timeout_ms": 15000, "memory_mb": 64 }
}</code></pre>

<p>
  Capabilities are deny-by-default; see the core reference for every field and
  the <code>exec</code>, filesystem and environment semantics.
</p>

<h2>Guest API Crate</h2>

<p>
  <code>xfetch-guest-api</code> implements the core-module host bridge:
</p>

<ul>
  <li><code>host_call(op, args)</code>: raw JSON dispatch.</li>
  <li><code>http_request(method, url, headers, body, timeout_ms)</code>: typed HTTP with base64 bodies.</li>
  <li><code>exec(program, args, stdin, env, timeout_ms)</code>: typed process execution.</li>
  <li><code>log(level, message)</code>: best-effort diagnostic output.</li>
  <li><code>protocol_version()</code>: host handshake.</li>
</ul>

<p>
  The crate also exports <code>xfetch_alloc</code> and <code>xfetch_free</code>
  automatically, which the host needs to place responses in guest memory.
</p>

<h2>Testing</h2>

<p>
  The same guest source builds natively, where host calls report
  <code>Unsupported</code> and stdin/stdout still work. For end-to-end checks:
</p>

<pre><code class="language-bash">xfetch wasm inspect ./dist/app.wasm
xfetch wasm run ./dist/app.wasm --request '{"version":1,"kind":"info_provider"}'
xfetch wasm run ./dist/effect.wasm --request_file request.json --kind effect</code></pre>
