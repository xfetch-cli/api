# xfetch-command-api

Shared types and IO helpers for xfetch command addons.

A command addon receives a JSON request on stdin and writes plain text to
stdout. The response is intentionally line-oriented rather than JSON so a
command can render its own key/value output.

```rust
use xfetch_command_api::{read_command_args_or_default, write_command_lines};

fn main() {
    let args = read_command_args_or_default().unwrap_or_default();
    let repo = args.first().map(String::as_str).unwrap_or("owner/repo");
    write_command_lines(&[format!("repo: {repo}")]).unwrap();
}
```

Component guests use the `command` world in `wit/xfetch-runtime.wit` and return
the same plain text response from `run`.
