# Examples

<p>
  The crate ships with minimal examples that compile with the public API.
</p>

<h2>Included Examples</h2>

<ul>
  <li><code>crates/plugin-api/examples/info-plugin.rs</code></li>
  <li><code>crates/plugin-api/examples/logo-animation-plugin.rs</code></li>
</ul>

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
