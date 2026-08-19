# Examples

<p>
  The crates ship with minimal examples that compile with the public API.
</p>

<h2>Plugin Examples</h2>

<ul>
  <li><code>crates/plugin-api/examples/info-plugin.rs</code></li>
  <li><code>crates/plugin-api/examples/logo-animation-plugin.rs</code></li>
</ul>

<h2>Effect Examples</h2>

<p>
  Effect implementations live in the
  <a href="https://github.com/xfetch-cli/effects">xfetch-cli/effects</a>
  repository (one crate per effect, binary <code>xfetch-effect-&lt;name&gt;</code>):
</p>

<ul>
  <li><code>effects/decrypt</code></li>
  <li><code>effects/glitch</code></li>
</ul>

<h2>Extension Examples</h2>

<p>
  Extensions are plain binaries using <code>xfetch-extension-api</code> types:
  read a <code>ConfigProviderRequest</code> from stdin and write a
  <code>ConfigProviderResponse</code> to stdout. Official extensions live in the
  <a href="https://github.com/xfetch-cli/extensions">xfetch-cli/extensions</a>
  repository.
</p>

<h2>Info Provider Example</h2>

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
}</code></pre>

<h2>Logo Animation Example</h2>

<pre><code class="language-rust">use xfetch_plugin_api::{AnimationFrame, read_logo_animation_request, write_logo_animation_frames};

fn main() {
    let request = match read_logo_animation_request() {
        Ok(value) =&gt; value,
        Err(err) =&gt; {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let frames = vec![AnimationFrame::new(80, request.lines)];

    if let Err(err) = write_logo_animation_frames(frames) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}</code></pre>

<h2>Effect Example (Decrypt)</h2>

<pre><code class="language-rust">use std::time::Duration;
use xfetch_effect_api::{EffectFrame, read_effect_request, with_timeout, write_effect_frames};
use xfetch_effects_lib::reveal;

const BUDGET: Duration = Duration::from_secs(10);

fn main() {
    let frames: Vec&lt;EffectFrame&gt; = with_timeout(BUDGET, || {
        let request = read_effect_request().expect("read request");
        let fps = request.args.fps.unwrap_or(30).max(1);
        let duration_ms = request.args.duration_ms.unwrap_or(1500).max(1);
        let frame_count = ((duration_ms * fps) / 1000).max(1);
        let frame_delay = (1000.0 / fps as f64) as u64;

        (0..=frame_count)
            .map(|i| {
                let progress = i as f64 / frame_count as f64;
                let delay = if i == frame_count { 1 } else { frame_delay };
                EffectFrame::new(
                    delay,
                    request.lines.iter().map(|l| reveal(l, progress, i)).collect(),
                )
            })
            .collect()
    })
    .expect("generate frames");

    write_effect_frames(frames).expect("write frames");
}</code></pre>

<h2>Extension Example</h2>

<pre><code class="language-rust">use std::io::Read;
use xfetch_extension_api::ConfigProviderRequest;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read request");

    let request: ConfigProviderRequest =
        serde_json::from_str(&input).expect("parse request");

    // Modify the config however you like, then write it back.
    let mut config = request.config;
    config["show_colors"] = serde_json::json!(false);

    let response = serde_json::json!({ "config": config });
    println!("{}", response);
}</code></pre>

