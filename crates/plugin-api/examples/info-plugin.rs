use serde::Deserialize;
use xfetch_plugin_api::{read_info_plugin_args_or_default, write_info_lines};

#[derive(Debug, Default, Deserialize)]
struct ExampleArgs {
    label: Option<String>,
}

fn main() {
    let args = match read_info_plugin_args_or_default::<ExampleArgs>() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let label = args
        .label
        .unwrap_or_else(|| "example".to_string());

    if let Err(err) = write_info_lines(vec![format!("plugin says: {}", label)]) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
