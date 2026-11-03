//! JSON stdin/stdout helpers for effects.
//!
//! The actual implementation lives in `xfetch-api-common`; the public
//! signatures and [`EffectApiError`] mapping are preserved here.

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::{EffectApiError, Result};

pub fn parse_json_str<T: DeserializeOwned>(input: &str) -> Result<T> {
    xfetch_api_common::io::parse_json_str(input).map_err(EffectApiError::from)
}

pub fn parse_json_slice<T: DeserializeOwned>(input: &[u8]) -> Result<T> {
    xfetch_api_common::io::parse_json_slice(input).map_err(EffectApiError::from)
}

pub fn to_json_vec<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    xfetch_api_common::io::to_json_vec(value).map_err(EffectApiError::from)
}

pub fn read_json_from_stdin<T: DeserializeOwned>() -> Result<T> {
    xfetch_api_common::io::read_json_from_stdin().map_err(EffectApiError::from)
}

pub fn write_json_to_stdout<T: Serialize>(value: &T) -> Result<()> {
    xfetch_api_common::io::write_json_to_stdout(value).map_err(EffectApiError::from)
}
