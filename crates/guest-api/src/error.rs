//! Errors returned by host calls.
//!
//! The host answers every call with either `{"ok":true,"value":...}` or
//! `{"ok":false,"error":{"kind","message"}}`. This module turns the failure
//! form into a typed error and keeps the raw wire values stable.

use std::fmt;

use serde_json::Value;

/// Machine-readable host failure categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostErrorKind {
    /// The manifest does not allow the requested capability.
    Denied,
    /// The operation failed at runtime.
    Failed,
    /// The operation exceeded its deadline.
    Timeout,
    /// The payload exceeded a configured size limit.
    TooLarge,
    /// The host does not implement the operation, or the build has no host
    /// bridge (non-wasm targets).
    Unsupported,
}

impl HostErrorKind {
    /// Stable wire label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Denied => "denied",
            Self::Failed => "failed",
            Self::Timeout => "timeout",
            Self::TooLarge => "too_large",
            Self::Unsupported => "unsupported",
        }
    }

    /// Parses a wire label; unknown labels map to [`HostErrorKind::Failed`].
    pub fn from_wire(kind: &str) -> Self {
        match kind {
            "denied" => Self::Denied,
            "timeout" => Self::Timeout,
            "too_large" => Self::TooLarge,
            "unsupported" => Self::Unsupported,
            _ => Self::Failed,
        }
    }
}

impl fmt::Display for HostErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A host call that could not complete.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostCallError {
    pub kind: HostErrorKind,
    pub message: String,
}

impl HostCallError {
    pub fn new(kind: HostErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    /// Builds an error from the host wire form.
    pub fn from_wire(value: &Value) -> Self {
        let error = value.get("error").unwrap_or(value);
        let kind = error
            .get("kind")
            .and_then(Value::as_str)
            .map(HostErrorKind::from_wire)
            .unwrap_or(HostErrorKind::Failed);
        let message = error
            .get("message")
            .and_then(Value::as_str)
            .unwrap_or("host call failed")
            .to_string();
        Self { kind, message }
    }

    /// Failure for targets without the wasm host bridge.
    pub fn not_wasm() -> Self {
        Self::new(
            HostErrorKind::Unsupported,
            "host calls are only available on wasm32 guests",
        )
    }

    /// Failure for a malformed host response.
    pub fn malformed(message: impl Into<String>) -> Self {
        Self::new(HostErrorKind::Failed, message)
    }
}

impl fmt::Display for HostCallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "host call {}: {}", self.kind, self.message)
    }
}

impl std::error::Error for HostCallError {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_wire_error() {
        let error = HostCallError::from_wire(&json!({
            "ok": false,
            "error": { "kind": "denied", "message": "nope" }
        }));
        assert_eq!(error.kind, HostErrorKind::Denied);
        assert_eq!(error.message, "nope");
    }

    #[test]
    fn unknown_kind_defaults_to_failed() {
        let error = HostCallError::from_wire(&json!({
            "error": { "kind": "strange", "message": "?" }
        }));
        assert_eq!(error.kind, HostErrorKind::Failed);
    }
}
