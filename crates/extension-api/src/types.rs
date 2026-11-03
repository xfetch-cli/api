use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{ExtensionApiError, Result};

pub const PROTOCOL_VERSION: u32 = xfetch_api_common::protocol::JSON_PROTOCOL_VERSION;
pub const EXTENSION_PROTOCOL_VERSION: u32 = PROTOCOL_VERSION;
pub const KIND_CONFIG_PROVIDER: &str = xfetch_api_common::protocol::KIND_CONFIG_PROVIDER;

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
/// The extension receives the fully resolved config (after default -> file ->
/// theme merge) and can modify any field. The modified config is returned in
/// the response.
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
            version: PROTOCOL_VERSION,
            kind: KIND_CONFIG_PROVIDER.to_string(),
            config,
            args,
        }
    }

    /// Validates the wire version and kind.
    pub fn validate(&self) -> Result<()> {
        if self.version != PROTOCOL_VERSION {
            return Err(ExtensionApiError::InvalidProtocolVersion {
                expected: PROTOCOL_VERSION,
                found: self.version,
            });
        }

        if self.kind != KIND_CONFIG_PROVIDER {
            return Err(ExtensionApiError::InvalidExtensionKind {
                expected: ExtensionKind::ConfigProvider,
                found: self.kind.clone(),
            });
        }

        Ok(())
    }

    pub fn kind_enum(&self) -> Option<ExtensionKind> {
        ExtensionKind::from_wire(&self.kind)
    }

    pub fn parse_args<T: DeserializeOwned>(&self) -> Result<Option<T>> {
        self.args
            .clone()
            .map(|value| serde_json::from_value(value).map_err(ExtensionApiError::InvalidArgs))
            .transpose()
    }

    pub fn parse_args_or_default<T>(&self) -> Result<T>
    where
        T: DeserializeOwned + Default,
    {
        Ok(self.parse_args()?.unwrap_or_default())
    }
}

/// Response from a config-provider extension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProviderResponse {
    pub config: Value,
}

impl ConfigProviderResponse {
    pub fn new(config: Value) -> Self {
        Self { config }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_defaults_to_current_protocol_version() {
        let request = ConfigProviderRequest::new(Value::Null, None);

        assert_eq!(request.version, PROTOCOL_VERSION);
        assert_eq!(request.kind, KIND_CONFIG_PROVIDER);
        assert_eq!(request.kind_enum(), Some(ExtensionKind::ConfigProvider));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn validation_rejects_wrong_version_and_kind() {
        let mut request = ConfigProviderRequest::new(Value::Null, None);
        request.version = PROTOCOL_VERSION + 1;
        assert!(matches!(
            request.validate(),
            Err(ExtensionApiError::InvalidProtocolVersion { .. })
        ));

        let mut request = ConfigProviderRequest::new(Value::Null, None);
        request.kind = "info_provider".to_string();
        assert!(matches!(
            request.validate(),
            Err(ExtensionApiError::InvalidExtensionKind { .. })
        ));
    }

    #[test]
    fn parses_typed_args() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct Args {
            name: String,
        }

        let request =
            ConfigProviderRequest::new(Value::Null, Some(serde_json::json!({ "name": "x" })));

        assert_eq!(
            request.parse_args::<Args>().expect("parse args"),
            Some(Args {
                name: "x".to_string()
            })
        );
    }
}
