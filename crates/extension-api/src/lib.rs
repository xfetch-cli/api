//! Protocol types and helpers for xfetch extensions (config providers).
//!
//! Extensions are standalone binaries that communicate with xfetch through a
//! JSON stdin/stdout protocol, separate from the plugin and effect systems.

pub mod entrypoints;
pub mod error;
pub mod io;
pub mod timeout;
pub mod types;

pub use entrypoints::{
    read_config_provider_args_or_default, read_config_provider_request, write_config,
    write_config_provider_response,
};
pub use error::{ExtensionApiError, Result};
pub use io::{
    parse_json_slice, parse_json_str, read_json_from_stdin, to_json_vec, write_json_to_stdout,
};
pub use timeout::{TimedOut, with_timeout};
pub use types::{
    ConfigProviderRequest, ConfigProviderResponse, EXTENSION_PROTOCOL_VERSION, ExtensionKind,
    KIND_CONFIG_PROVIDER, PROTOCOL_VERSION,
};
