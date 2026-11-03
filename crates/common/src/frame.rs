//! Shared frame shape for logo animations and effects.
//!
//! The per-kind SDKs keep their public `AnimationFrame`/`EffectFrame` names;
//! this type is the common representation used by shared tooling and
//! conversions.

use serde::{Deserialize, Serialize};

/// One rendered frame: how long to hold it and the lines to print.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Frame {
    pub delay_ms: u64,
    pub lines: Vec<String>,
}

impl Frame {
    pub fn new(delay_ms: u64, lines: Vec<String>) -> Self {
        Self { delay_ms, lines }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_roundtrips_as_json() {
        let frame = Frame::new(16, vec!["one".to_string(), "two".to_string()]);
        let encoded = serde_json::to_string(&frame).expect("serialize");
        let decoded: Frame = serde_json::from_str(&encoded).expect("deserialize");

        assert_eq!(decoded, frame);
    }
}
