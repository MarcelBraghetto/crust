pub mod components;
pub mod core;
pub mod log_tag;
pub mod opengl;
pub mod scenes;

use crate::core::{failable_unit::FailableUnit, logs};

pub fn main() {
    std::process::exit(match launch() {
        Ok(_) => 0,
        Err(err) => {
            logs::out(log_tag!(), &format!("Fatal error: {:?}", err));
            1
        }
    });
}

#[cfg(not(target_os = "emscripten"))]
fn launch() -> FailableUnit {
    core::launcher::launch()
}

#[cfg(target_os = "emscripten")]
fn launch() -> FailableUnit {
    core::launcher_emscripten::launch()
}
