use crate::error::{PluginApiError, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const PROTOCOL_VERSION: u32 = 1;
pub const KIND_LOGO_ANIMATION: &str = "logo_animation";
pub const KIND_INFO_PROVIDER: &str = "info_provider";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluginKind {
    LogoAnimation,
    InfoProvider,
}

impl PluginKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogoAnimation => KIND_LOGO_ANIMATION,
            Self::InfoProvider => KIND_INFO_PROVIDER,
        }
    }

    pub fn from_wire(kind: &str) -> Option<Self> {
        match kind {
            KIND_LOGO_ANIMATION => Some(Self::LogoAnimation),
            KIND_INFO_PROVIDER => Some(Self::InfoProvider),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmptyArgs {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnimationFrame {
    pub delay_ms: u64,
    pub lines: Vec<String>,
}

impl AnimationFrame {
    pub fn new(delay_ms: u64, lines: Vec<String>) -> Self {
        Self { delay_ms, lines }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct LogoAnimationArgs {
    pub fps: Option<u64>,
    pub duration_ms: Option<u64>,
    #[serde(rename = "loop")]
    pub loop_enabled: Option<bool>,
    pub style: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogoAnimationRequest {
    pub version: u32,
    pub kind: String,
    pub lines: Vec<String>,
    pub frames: Option<Vec<Vec<String>>>,
    pub args: LogoAnimationArgs,
}

impl LogoAnimationRequest {
    pub fn new(
        lines: Vec<String>,
        frames: Option<Vec<Vec<String>>>,
        args: LogoAnimationArgs,
    ) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            kind: KIND_LOGO_ANIMATION.to_string(),
            lines,
            frames,
            args,
        }
    }

    pub fn validate(&self) -> Result<()> {
        validate_request(self.version, &self.kind, PluginKind::LogoAnimation)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogoAnimationResponse {
    pub frames: Vec<AnimationFrame>,
}

impl LogoAnimationResponse {
    pub fn new(frames: Vec<AnimationFrame>) -> Self {
        Self { frames }
    }

    pub fn validate(&self) -> Result<()> {
        if self.frames.is_empty() {
            return Err(PluginApiError::EmptyAnimationFrames);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InfoPluginRequest {
    pub version: u32,
    pub kind: String,
    pub args: Option<Value>,
}

impl InfoPluginRequest {
    pub fn new(args: Option<Value>) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            kind: KIND_INFO_PROVIDER.to_string(),
            args,
        }
    }

    pub fn validate(&self) -> Result<()> {
        validate_request(self.version, &self.kind, PluginKind::InfoProvider)
    }

    pub fn parse_args<T: DeserializeOwned>(&self) -> Result<Option<T>> {
        self.args
            .clone()
            .map(|value| serde_json::from_value(value).map_err(PluginApiError::InvalidArgs))
            .transpose()
    }

    pub fn parse_args_or_default<T>(&self) -> Result<T>
    where
        T: DeserializeOwned + Default,
    {
        Ok(self.parse_args()?.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InfoPluginResponse {
    pub lines: Vec<String>,
}

impl InfoPluginResponse {
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }
}

fn validate_request(version: u32, kind: &str, expected_kind: PluginKind) -> Result<()> {
    if version != PROTOCOL_VERSION {
        return Err(PluginApiError::InvalidProtocolVersion {
            expected: PROTOCOL_VERSION,
            found: version,
        });
    }

    if kind != expected_kind.as_str() {
        return Err(PluginApiError::InvalidPluginKind {
            expected: expected_kind,
            found: kind.to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logo_request_defaults_to_current_protocol_version() {
        let request = LogoAnimationRequest::new(
            vec!["line".to_string()],
            None,
            LogoAnimationArgs::default(),
        );

        assert_eq!(request.version, PROTOCOL_VERSION);
        assert_eq!(request.kind, KIND_LOGO_ANIMATION);
    }

    #[test]
    fn info_request_defaults_to_current_protocol_version() {
        let request = InfoPluginRequest::new(None);

        assert_eq!(request.version, PROTOCOL_VERSION);
        assert_eq!(request.kind, KIND_INFO_PROVIDER);
    }

    #[test]
    fn loop_field_keeps_wire_name() {
        let args = LogoAnimationArgs {
            loop_enabled: Some(true),
            ..LogoAnimationArgs::default()
        };

        let json = serde_json::to_string(&args).expect("serialize args");

        assert!(json.contains("\"loop\":true"));
        assert!(!json.contains("loop_enabled"));
    }

    #[test]
    fn info_request_parses_typed_args() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct Args {
            username: String,
        }

        let request = InfoPluginRequest::new(Some(serde_json::json!({
            "username": "xfetch"
        })));

        let args = request.parse_args::<Args>().expect("parse args");

        assert_eq!(
            args,
            Some(Args {
                username: "xfetch".to_string()
            })
        );
    }

    #[test]
    fn logo_request_validation_rejects_wrong_kind() {
        let request = LogoAnimationRequest {
            version: PROTOCOL_VERSION,
            kind: KIND_INFO_PROVIDER.to_string(),
            lines: Vec::new(),
            frames: None,
            args: LogoAnimationArgs::default(),
        };

        assert!(matches!(
            request.validate(),
            Err(PluginApiError::InvalidPluginKind { .. })
        ));
    }

    #[test]
    fn logo_response_validation_rejects_empty_frames() {
        let response = LogoAnimationResponse::new(Vec::new());

        assert!(matches!(
            response.validate(),
            Err(PluginApiError::EmptyAnimationFrames)
        ));
    }
}
