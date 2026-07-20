//! Shared plugin protocol, validation, and entrypoint helpers for xfetch.

pub mod entrypoints;
pub mod error;
pub mod io;
pub mod protocol;

pub use entrypoints::{
    InfoPluginInput, read_info_plugin_args_or_default, read_info_plugin_request,
    read_logo_animation_request, write_info_lines, write_logo_animation_frames,
};
pub use error::{PluginApiError, Result};
pub use io::{
    parse_json_slice, parse_json_str, read_json_from_stdin, to_json_vec, write_json_to_stdout,
};
pub use protocol::{
    AnimationFrame, EmptyArgs, InfoPluginRequest, InfoPluginResponse, LogoAnimationArgs,
    LogoAnimationRequest, LogoAnimationResponse, PluginKind, KIND_INFO_PROVIDER,
    KIND_LOGO_ANIMATION, PROTOCOL_VERSION,
};
