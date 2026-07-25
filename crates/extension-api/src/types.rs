use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const EXTENSION_PROTOCOL_VERSION: u32 = 1;
pub const KIND_CONFIG_PROVIDER: &str = "config_provider";

/// Known extension kinds that xfetch can dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtensionKind {
    ConfigProvider,
}

impl ExtensionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConfigProvider => KIND_CONFIG_PROVIDER,
        }
    }

    pub fn from_wire(kind: &str) -> Option<Self> {
        match kind {
            KIND_CONFIG_PROVIDER => Some(Self::ConfigProvider),
            _ => None,
        }
    }
}

/// Request sent to a config-provider extension.
///
/// The extension receives the fully resolved config (after default → file → theme merge)
/// and can modify any field. The modified config is returned in the response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProviderRequest {
    pub version: u32,
    pub kind: String,
    pub config: Value,
    pub args: Option<Value>,
}

impl ConfigProviderRequest {
    pub fn new(config: Value, args: Option<Value>) -> Self {
        Self {
            version: EXTENSION_PROTOCOL_VERSION,
            kind: KIND_CONFIG_PROVIDER.to_string(),
            config,
            args,
        }
    }
}

/// Response from a config-provider extension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProviderResponse {
    pub config: Value,
}
