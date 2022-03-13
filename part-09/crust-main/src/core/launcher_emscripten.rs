use crate::{
    core::{failable::Failable, failable_unit::FailableUnit, main_loop::MainLoop},
    opengl::opengl_engine::OpenGLEngine,
};
use std::{cell::RefCell, os::raw::c_int, rc::Rc};

pub fn launch() -> FailableUnit {
    LAUNCHER.with(|it| it.set_main_loop())
}

// We need to hold the data structures statically so we can bind the Emscripten C method callbacks. The static
// instances behave like lazy singletons - the `MAIN_LOOP` is never accessed *before* the `LAUNCHER` is, so there
// won't be any issues around order of initialization.
thread_local!(static LAUNCHER: EmscriptenLauncher = EmscriptenLauncher::new().unwrap());
thread_local!(static MAIN_LOOP: RefCell<MainLoop<OpenGLEngine>> = LAUNCHER.with(|it| RefCell::new(it.new_main_loop().unwrap())));

// This invokes our main loop within our LAUNCHER singleton and is declared as a function that can be called from
// C, allowing us to pass it to Emscripten as the 'main loop' callback. There is an alternate method available that
// takes a single argument cast as `void*` type but it starts to get pretty awful trying to coerce our Rust objects
// into and out of void pointers to be able to pass them through so we'll stick with the (admittedly less than ideal)
// singleton approach so the `MAIN_LOOP` doesn't need to be passed in as an argument.
extern "C" fn run_main_loop() {
    MAIN_LOOP.with(|it| it.borrow_mut().run().unwrap());
}

extern "C" {
    // This is the alias for the Emscripten C function which associates a callback to execute on every
    // iteration of the main loop: https://emscripten.org/docs/api_reference/emscripten.h.html#c.emscripten_set_main_loop
    fn emscripten_set_main_loop(func: unsafe extern "C" fn(), fps: c_int, simulate_infinite_loop: c_int);
}

struct EmscriptenLauncher {
    pub sdl: Rc<sdl2::Sdl>,
}

impl EmscriptenLauncher {
    pub fn new() -> Failable<Self> {
        let sdl = Rc::new(sdl2::init()?);
        sdl2::image::init(sdl2::image::InitFlag::PNG)?;

        Ok(EmscriptenLauncher { sdl: sdl })
    }

    pub fn new_main_loop(&self) -> Failable<MainLoop<OpenGLEngine>> {
        MainLoop::new(&self.sdl, OpenGLEngine::new(&self.sdl)?)
    }

    fn set_main_loop(&self) -> FailableUnit {
        unsafe {
            emscripten_set_main_loop(run_main_loop, 0, 1);
        }

        Ok(())
    }
}
