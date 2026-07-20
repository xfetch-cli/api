use crate::protocol::PluginKind;
use std::fmt;

pub type Result<T> = std::result::Result<T, PluginApiError>;

#[derive(Debug)]
pub enum PluginApiError {
    Io(std::io::Error),
    Serialize(serde_json::Error),
    Deserialize(serde_json::Error),
    InvalidProtocolVersion { expected: u32, found: u32 },
    InvalidPluginKind { expected: PluginKind, found: String },
    InvalidArgs(serde_json::Error),
    EmptyAnimationFrames,
}

impl fmt::Display for PluginApiError {
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
            Self::InvalidPluginKind { expected, found } => {
                write!(
                    f,
                    "Unexpected plugin kind: expected '{}', found '{}'",
                    expected.as_str(),
                    found
                )
            }
            Self::InvalidArgs(err) => write!(f, "Failed to parse plugin args: {}", err),
            Self::EmptyAnimationFrames => write!(f, "Animation plugins must return at least one frame"),
        }
    }
}

impl std::error::Error for PluginApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Serialize(err) => Some(err),
            Self::Deserialize(err) => Some(err),
            Self::InvalidArgs(err) => Some(err),
            Self::InvalidProtocolVersion { .. }
            | Self::InvalidPluginKind { .. }
            | Self::EmptyAnimationFrames => None,
        }
    }
}

impl From<std::io::Error> for PluginApiError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
