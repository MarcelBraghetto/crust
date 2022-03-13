use crate::core::{display_size::DisplaySize, failable::Failable};

#[cfg(not(target_os = "emscripten"))]
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

#[cfg(target_os = "emscripten")]
pub fn get_size(_: &sdl2::VideoSubsystem) -> Failable<DisplaySize> {
    use std::os::raw::{c_char, c_int};

    extern "C" {
        fn emscripten_run_script_int(code: *const c_char) -> c_int;
    }

    unsafe {
        let width: i32 = emscripten_run_script_int(b"document.getElementById('canvas').width;\0" as *const _ as *const c_char)
            .try_into()
            .map_err(|_| "Failed to get HTML canvas width!")?;

        let height: i32 = emscripten_run_script_int(b"document.getElementById('canvas').height;\0" as *const _ as *const c_char)
            .try_into()
            .map_err(|_| "Failed to get HTML canvas height!")?;

        Ok(DisplaySize {
            width: width,
            height: height,
        })
    }
}
