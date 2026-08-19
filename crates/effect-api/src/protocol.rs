use crate::error::{EffectApiError, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const PROTOCOL_VERSION: u32 = 1;
pub const KIND_EFFECT: &str = "effect";

/// Parameters an effect can tune. `args` carries effect-specific free-form
/// parameters (parsed by the effect via `EffectArgs::parse_args`).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct EffectArgs {
    pub style: Option<String>,
    pub duration_ms: Option<u64>,
    pub fps: Option<u64>,
    pub args: Option<Value>,
}

impl EffectArgs {
    pub fn parse_args<T: DeserializeOwned>(&self) -> Result<Option<T>> {
        self.args
            .clone()
            .map(|value| serde_json::from_value(value).map_err(EffectApiError::InvalidArgs))
            .transpose()
    }

    pub fn parse_args_or_default<T>(&self) -> Result<T>
    where
        T: DeserializeOwned + Default,
    {
        Ok(self.parse_args()?.unwrap_or_default())
    }
}

/// One frame of the effect: how long to hold it and the transformed lines.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EffectFrame {
    pub delay_ms: u64,
    pub lines: Vec<String>,
}

impl EffectFrame {
    pub fn new(delay_ms: u64, lines: Vec<String>) -> Self {
        Self { delay_ms, lines }
    }
}

/// The content lines handed to the effect, as rendered by the core (icon +
/// value per line). The effect transforms them into per-frame states; the last
/// frame should reach the original (final) content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EffectRequest {
    pub version: u32,
    pub kind: String,
    pub lines: Vec<String>,
    pub args: EffectArgs,
}

impl EffectRequest {
    pub fn new(lines: Vec<String>, args: EffectArgs) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            kind: KIND_EFFECT.to_string(),
            lines,
            args,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.version != PROTOCOL_VERSION {
            return Err(EffectApiError::InvalidProtocolVersion {
                expected: PROTOCOL_VERSION,
                found: self.version,
            });
        }

        if self.kind != KIND_EFFECT {
            return Err(EffectApiError::InvalidEffectKind {
                found: self.kind.clone(),
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EffectResponse {
    pub frames: Vec<EffectFrame>,
}

impl EffectResponse {
    pub fn new(frames: Vec<EffectFrame>) -> Self {
        Self { frames }
    }

    pub fn validate(&self) -> Result<()> {
        if self.frames.is_empty() {
            return Err(EffectApiError::EmptyEffectFrames);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_defaults_to_current_protocol_version() {
        let request = EffectRequest::new(vec!["line".to_string()], EffectArgs::default());

        assert_eq!(request.version, PROTOCOL_VERSION);
        assert_eq!(request.kind, KIND_EFFECT);
    }

    #[test]
    fn request_validation_rejects_wrong_kind() {
        let request = EffectRequest {
            version: PROTOCOL_VERSION,
            kind: "logo_animation".to_string(),
            lines: Vec::new(),
            args: EffectArgs::default(),
        };

        assert!(matches!(
            request.validate(),
            Err(EffectApiError::InvalidEffectKind { .. })
        ));
    }

    #[test]
    fn response_validation_rejects_empty_frames() {
        let response = EffectResponse::new(Vec::new());

        assert!(matches!(
            response.validate(),
            Err(EffectApiError::EmptyEffectFrames)
        ));
    }

    #[test]
    fn args_parse_typed() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct Args {
            glyphs: String,
        }

        let args = EffectArgs {
            args: Some(serde_json::json!({ "glyphs": "abc" })),
            ..EffectArgs::default()
        };

        assert_eq!(
            args.parse_args::<Args>().expect("parse args"),
            Some(Args {
                glyphs: "abc".to_string()
            })
        );
    }
}
