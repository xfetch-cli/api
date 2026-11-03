//! Minimal config provider: mark the config and return it unchanged otherwise.

use xfetch_extension_api::{read_config_provider_request, write_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = read_config_provider_request()?;
    let mut config = request.config;

    if let Some(object) = config.as_object_mut() {
        object.insert("example_extension".to_string(), serde_json::json!(true));
    }

    write_config(config)?;
    Ok(())
}
