//! Shared effect protocol, validation, and entrypoint helpers for xfetch.
//!
//! Effects are installable intro animations: the core renders the info lines,
//! sends them to an effect plugin, and the plugin returns a sequence of frames
//! (each a delay plus the transformed lines) that the core plays before
//! settling on the final content.

pub mod entrypoints;
pub mod error;
pub mod io;
pub mod protocol;
pub mod timeout;

pub use entrypoints::{read_effect_request, write_effect_frames};
pub use error::{EffectApiError, Result};
pub use io::{
    parse_json_slice, parse_json_str, read_json_from_stdin, to_json_vec, write_json_to_stdout,
};
pub use protocol::{
    EffectArgs, EffectFrame, EffectRequest, EffectResponse, KIND_EFFECT, PROTOCOL_VERSION,
};
pub use timeout::{TimedOut, with_timeout};
