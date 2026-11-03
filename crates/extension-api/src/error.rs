//! Errors returned by extension helpers.

use std::fmt;

use crate::types::ExtensionKind;

pub type Result<T> = std::result::Result<T, ExtensionApiError>;

#[derive(Debug)]
pub enum ExtensionApiError {
    Io(std::io::Error),
    Serialize(serde_json::Error),
    Deserialize(serde_json::Error),
    InvalidProtocolVersion {
        expected: u32,
        found: u32,
    },
    InvalidExtensionKind {
        expected: ExtensionKind,
        found: String,
    },
    InvalidArgs(serde_json::Error),
}

impl fmt::Display for ExtensionApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {}", err),
            Self::Serialize(err) => write!(f, "Failed to serialize JSON: {}", err),
            Self::Deserialize(err) => write!(f, "Failed to parse JSON: {}", err),
            Self::InvalidProtocolVersion { expected, found } => {
                write!(
                    f,
                    "Unsupported protocol version: expected {}, found {}",
                    expected, found
                )
            }
            Self::InvalidExtensionKind { expected, found } => {
                write!(
                    f,
                    "Unexpected extension kind: expected '{}', found '{}'",
                    expected.as_str(),
                    found
                )
            }
            Self::InvalidArgs(err) => write!(f, "Failed to parse extension args: {}", err),
        }
    }
}

impl std::error::Error for ExtensionApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Serialize(err) | Self::Deserialize(err) | Self::InvalidArgs(err) => Some(err),
            Self::InvalidProtocolVersion { .. } | Self::InvalidExtensionKind { .. } => None,
        }
    }
}

impl From<std::io::Error> for ExtensionApiError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for ExtensionApiError {
    fn from(value: serde_json::Error) -> Self {
        Self::Deserialize(value)
    }
}

impl From<xfetch_api_common::IoError> for ExtensionApiError {
    fn from(value: xfetch_api_common::IoError) -> Self {
        match value {
            xfetch_api_common::IoError::Io(err) => Self::Io(err),
            xfetch_api_common::IoError::Serialize(err) => Self::Serialize(err),
            xfetch_api_common::IoError::Deserialize(err) => Self::Deserialize(err),
        }
    }
}
