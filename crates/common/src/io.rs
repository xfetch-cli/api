//! JSON stdin/stdout helpers shared by the addon SDKs.
//!
//! The per-kind crates keep their public `io` modules and error types; they
//! delegate the actual parsing and IO here and map [`IoError`] into their own
//! error enums.

use std::fmt;
use std::io::{self, Read, Write};

use serde::Serialize;
use serde::de::DeserializeOwned;

/// Failure while reading, parsing or writing a JSON protocol message.
#[derive(Debug)]
pub enum IoError {
    Io(io::Error),
    Serialize(serde_json::Error),
    Deserialize(serde_json::Error),
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {}", err),
            Self::Serialize(err) => write!(f, "Failed to serialize JSON: {}", err),
            Self::Deserialize(err) => write!(f, "Failed to parse JSON: {}", err),
        }
    }
}

impl std::error::Error for IoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Serialize(err) | Self::Deserialize(err) => Some(err),
        }
    }
}

impl From<io::Error> for IoError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for IoError {
    fn from(value: serde_json::Error) -> Self {
        Self::Deserialize(value)
    }
}

pub fn parse_json_str<T: DeserializeOwned>(input: &str) -> Result<T, IoError> {
    serde_json::from_str(input).map_err(IoError::Deserialize)
}

pub fn parse_json_slice<T: DeserializeOwned>(input: &[u8]) -> Result<T, IoError> {
    serde_json::from_slice(input).map_err(IoError::Deserialize)
}

pub fn to_json_vec<T: Serialize>(value: &T) -> Result<Vec<u8>, IoError> {
    serde_json::to_vec(value).map_err(IoError::Serialize)
}

pub fn read_json_from_stdin<T: DeserializeOwned>() -> Result<T, IoError> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    parse_json_str(&input)
}

pub fn write_json_to_stdout<T: Serialize>(value: &T) -> Result<(), IoError> {
    let body = to_json_vec(value)?;
    let mut stdout = io::stdout();
    stdout.write_all(&body)?;
    stdout.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Payload {
        value: u32,
    }

    #[test]
    fn parses_from_str_and_slice() {
        assert_eq!(
            parse_json_str::<Payload>(r#"{"value":7}"#).expect("str"),
            Payload { value: 7 }
        );
        assert_eq!(
            parse_json_slice::<Payload>(br#"{"value":7}"#).expect("slice"),
            Payload { value: 7 }
        );
    }

    #[test]
    fn serializes_to_vec() {
        let encoded = to_json_vec(&Payload { value: 7 }).expect("serialize");
        assert_eq!(encoded, br#"{"value":7}"#);
    }

    #[test]
    fn invalid_json_is_a_deserialize_error() {
        assert!(matches!(
            parse_json_str::<Payload>("{"),
            Err(IoError::Deserialize(_))
        ));
    }
}
