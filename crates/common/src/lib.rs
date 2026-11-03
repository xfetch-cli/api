//! Shared primitives for the xfetch addon SDK crates.
//!
//! This crate deliberately contains no per-kind protocol types. Plugins,
//! effects and extensions keep their own request/response contracts; only the
//! wire constants, JSON IO, frame shape and timeout helper are shared.

pub mod frame;
pub mod io;
pub mod protocol;
pub mod timeout;

pub use frame::Frame;
pub use io::IoError;
pub use protocol::{JSON_PROTOCOL_VERSION, is_current_version};
pub use timeout::{TimedOut, with_timeout};
