use crate::core::{display_size::DisplaySize, failable::Failable};

pub fn get_size(video: &sdl2::VideoSubsystem) -> Failable<DisplaySize> {
    let mut width = 640i32;
    let mut height = 480i32;

    if cfg!(target_os = "android") || cfg!(target_os = "ios") {
        let display_mode = video.desktop_display_mode(0)?;
        width = display_mode.w;
        height = display_mode.h;
    }

    Ok(DisplaySize {
        width: width,
        height: height,
    })
}
