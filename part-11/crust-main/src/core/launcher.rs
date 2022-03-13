use crate::{
    core::{failable_unit::FailableUnit, logs, main_loop::MainLoop},
    log_tag,
    opengl::opengl_engine::OpenGLEngine,
};

pub fn launch() -> FailableUnit {
    if cfg!(target_os = "android") || cfg!(target_os = "ios") {
        sdl2::hint::set("SDL_IOS_ORIENTATIONS", "LandscapeLeft LandscapeRight");
    }

    logs::out(log_tag!(), "Init SDL2 ...");
    let sdl = sdl2::init()?;

    logs::out(log_tag!(), "Init SDL2 Image ...");
    sdl2::image::init(sdl2::image::InitFlag::PNG)?;

    logs::out(log_tag!(), "SDL2 ready ...");

    logs::out(log_tag!(), "Init OpenGL ...");
    let engine = OpenGLEngine::new(&sdl)?;

    logs::out(log_tag!(), "Init main loop ...");
    let mut main_loop = MainLoop::new(&sdl, engine)?;

    while !main_loop.run()? {
        // Keep looping until the main loop returns 'true'
    }

    logs::out(log_tag!(), "Finished ...");

    Ok(())
}
