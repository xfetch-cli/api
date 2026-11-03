//! Timeout helpers for extensions.
//!
//! The implementation is shared with plugins and effects through
//! `xfetch-api-common`; this module preserves the existing public path.

pub use xfetch_api_common::timeout::{TimedOut, with_timeout};

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn reexported_helper_runs_tasks() {
        assert_eq!(with_timeout(Duration::from_secs(1), || 2 + 2), Ok(4));
    }
}
