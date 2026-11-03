//! Standard entrypoints for config-provider extensions.

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::error::Result;
use crate::io::{read_json_from_stdin, write_json_to_stdout};
use crate::types::{ConfigProviderRequest, ConfigProviderResponse};

/// Reads and validates a [`ConfigProviderRequest`] from stdin.
pub fn read_config_provider_request() -> Result<ConfigProviderRequest> {
    let request: ConfigProviderRequest = read_json_from_stdin()?;
    request.validate()?;
    Ok(request)
}

/// Reads a request and returns its typed `args`, or `T::default()` when the
/// core sent no arguments.
pub fn read_config_provider_args_or_default<T>() -> Result<T>
where
    T: DeserializeOwned + Default,
{
    read_config_provider_request()?.parse_args_or_default()
}

/// Writes a [`ConfigProviderResponse`] to stdout.
pub fn write_config_provider_response(response: &ConfigProviderResponse) -> Result<()> {
    write_json_to_stdout(response)
}

/// Convenience wrapper for the common case: return an already-built config.
pub fn write_config(config: Value) -> Result<()> {
    write_config_provider_response(&ConfigProviderResponse::new(config))
}
