# Timeouts

<p>
  Plugins and extensions have variable runtimes (network calls, daemon
  queries, heavy computation), so each one declares its own runtime budget in
  its code. This page explains how that works.
</p>

<h2>The problem</h2>

<p>
  The core runs plugins and extensions as subprocesses. Without any time
  control, a hung plugin hangs <code>xfetch</code> (and the config load, for
  extensions) forever.
</p>

<h2>The solution: <code>with_timeout</code></h2>

<p>
  Both crates expose the same helper:
</p>

<pre><code class="language-rust">pub fn with_timeout&lt;T: Send + 'static&gt;(
    budget: Duration,
    task: impl FnOnce() -> T + Send + 'static,
) -&gt; Result&lt;T, TimedOut&gt;</code></pre>

<ul>
  <li>The task runs on a worker thread.</li>
  <li>
    <code>Ok(task())</code> is returned when it finishes within
    <code>budget</code>.
  </li>
  <li>
    <code>Err(TimedOut)</code> is returned when the budget elapses. The
    worker thread keeps running until the process exits — which is
    immediate, because the plugin/extension responds (or errors) and
    terminates.
  </li>
</ul>

<h2>Guidelines</h2>

<ul>
  <li>
    Declare a <code>const BUDGET</code> that fits the work: ~2 s for local
    probes, 15–25 s for network calls.
  </li>
  <li>
    Wrap <strong>everything</strong> (including reading the request from
    stdin) in the closure.
  </li>
  <li>
    On <code>Err(TimedOut)</code>, respond with fallback lines (info
    plugins) or exit with an error (animation plugins, extensions).
  </li>
</ul>

<h2>Example</h2>

<pre><code class="language-rust">use std::time::Duration;
use xfetch_plugin_api::{read_info_plugin_args_or_default, with_timeout, write_info_lines};

const BUDGET: Duration = Duration::from_secs(10);

fn main() {
    let lines = with_timeout(BUDGET, || {
        let args = match read_info_plugin_args_or_default::&lt;MyArgs&gt;() {
            Ok(v) =&gt; v,
            Err(err) =&gt; {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };
        do_work(&args)
    })
    .unwrap_or_else(|_| vec!["MyPlugin: timed out".to_string()]);

    if let Err(err) = write_info_lines(lines) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}</code></pre>

<h2>Safety net</h2>

<p>
  <code>with_timeout</code> is the standard for official plugins and
  extensions (enforced in CI). As an extra safety net, the core can also kill
  the process after an optional per-plugin deadline
  (<code>timeout_secs</code> in the config), which protects against
  third-party or uncooperative plugins. See the main xfetch documentation.
</p>
