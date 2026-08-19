use std::fmt;

pub type Result<T> = std::result::Result<T, EffectApiError>;

#[derive(Debug)]
pub enum EffectApiError {
    Io(std::io::Error),
    Serialize(serde_json::Error),
    Deserialize(serde_json::Error),
    InvalidProtocolVersion { expected: u32, found: u32 },
    InvalidEffectKind { found: String },
    InvalidArgs(serde_json::Error),
    EmptyEffectFrames,
}

impl fmt::Display for EffectApiError {
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
            Self::InvalidEffectKind { found } => {
                write!(
                    f,
                    "Unexpected effect kind: expected 'effect', found '{}'",
                    found
                )
            }
            Self::InvalidArgs(err) => write!(f, "Failed to parse effect args: {}", err),
            Self::EmptyEffectFrames => write!(f, "Effects must return at least one frame"),
        }
    }
}

impl std::error::Error for EffectApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Serialize(err) => Some(err),
            Self::Deserialize(err) => Some(err),
            Self::InvalidArgs(err) => Some(err),
            Self::InvalidProtocolVersion { .. }
            | Self::InvalidEffectKind { .. }
            | Self::EmptyEffectFrames => None,
        }
    }
}

impl From<std::io::Error> for EffectApiError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
