//! Wire-level constants shared by every xfetch addon kind.

/// JSON protocol version used by native addons and WebAssembly core modules.
pub const JSON_PROTOCOL_VERSION: u32 = 1;

/// Plugin kind: animated logo frames.
pub const KIND_LOGO_ANIMATION: &str = "logo_animation";
/// Plugin kind: additional information lines.
pub const KIND_INFO_PROVIDER: &str = "info_provider";
/// Effect kind: intro animation over the rendered lines.
pub const KIND_EFFECT: &str = "effect";
/// Extension kind: config-provider hook.
pub const KIND_CONFIG_PROVIDER: &str = "config_provider";

/// Returns true when `version` is the current JSON protocol version.
pub const fn is_current_version(version: u32) -> bool {
    version == JSON_PROTOCOL_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_version_is_accepted() {
        assert!(is_current_version(JSON_PROTOCOL_VERSION));
    }

    #[test]
    fn other_versions_are_rejected() {
        assert!(!is_current_version(JSON_PROTOCOL_VERSION + 1));
        assert!(!is_current_version(0));
    }
}
