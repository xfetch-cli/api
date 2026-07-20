use xfetch_plugin_api::{AnimationFrame, read_logo_animation_request, write_logo_animation_frames};

fn main() {
    let request = match read_logo_animation_request() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let frames = vec![AnimationFrame::new(80, request.lines)];

    if let Err(err) = write_logo_animation_frames(frames) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
