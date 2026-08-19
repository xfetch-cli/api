//! Protocol types for xfetch extensions (config providers, lifecycle hooks, etc).
//!
//! Extensions are standalone binaries that communicate with xfetch via
//! a stdin/stdout JSON protocol, separate from the plugin system.

pub mod timeout;
pub mod types;

pub use timeout::{TimedOut, with_timeout};
pub use types::{
    ConfigProviderRequest, ConfigProviderResponse, ExtensionKind, KIND_CONFIG_PROVIDER,
};
