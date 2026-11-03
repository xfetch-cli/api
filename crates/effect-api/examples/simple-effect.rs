//! Minimal effect: return the original lines as a single frame.

use xfetch_effect_api::{EffectFrame, read_effect_request, write_effect_frames};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = read_effect_request()?;
    let frame = EffectFrame::new(0, request.lines);
    write_effect_frames(vec![frame])?;
    Ok(())
}
