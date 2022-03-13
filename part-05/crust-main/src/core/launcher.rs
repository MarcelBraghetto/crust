use crate::{
    core::{failable_unit::FailableUnit, logs},
    log_tag,
};

pub fn launch() -> FailableUnit {
    logs::out(log_tag!(), "Init SDL2 ...");
    sdl2::init()?;

    logs::out(log_tag!(), "Init SDL2 Image ...");
    sdl2::image::init(sdl2::image::InitFlag::PNG)?;

    logs::out(log_tag!(), "SDL2 ready ...");

    Ok(())
}
