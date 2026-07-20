use crate::error::{PluginApiError, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::{self, Read, Write};

pub fn parse_json_str<T: DeserializeOwned>(input: &str) -> Result<T> {
    serde_json::from_str(input).map_err(PluginApiError::Deserialize)
}

pub fn parse_json_slice<T: DeserializeOwned>(input: &[u8]) -> Result<T> {
    serde_json::from_slice(input).map_err(PluginApiError::Deserialize)
}

pub fn to_json_vec<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    serde_json::to_vec(value).map_err(PluginApiError::Serialize)
}

pub fn read_json_from_stdin<T: DeserializeOwned>() -> Result<T> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(PluginApiError::Io)?;

    parse_json_str(&input)
}

pub fn write_json_to_stdout<T: Serialize>(value: &T) -> Result<()> {
    let body = to_json_vec(value)?;
    let mut stdout = io::stdout();
    stdout.write_all(&body).map_err(PluginApiError::Io)?;
    stdout.flush().map_err(PluginApiError::Io)
}
