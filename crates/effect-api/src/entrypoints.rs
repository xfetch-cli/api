use crate::error::Result;
use crate::io::{read_json_from_stdin, write_json_to_stdout};
use crate::protocol::{EffectFrame, EffectRequest, EffectResponse};

/// Reads and validates an `EffectRequest` from stdin. The standard entrypoint
/// for effect binaries.
pub fn read_effect_request() -> Result<EffectRequest> {
    let request: EffectRequest = read_json_from_stdin()?;
    request.validate()?;
    Ok(request)
}

/// Validates and writes the effect frames to stdout. The standard exit for
/// effect binaries.
pub fn write_effect_frames(frames: Vec<EffectFrame>) -> Result<()> {
    let response = EffectResponse::new(frames);
    response.validate()?;
    write_json_to_stdout(&response)
}
