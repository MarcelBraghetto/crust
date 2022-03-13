use crate::{
    core::{
        display_size::DisplaySize, engine::Engine, failable::Failable, failable_unit::FailableUnit, graphics::Graphics, logs,
        renderer::Renderer, window,
    },
    log_tag,
};
use sdl2::video::{GLContext, GLProfile, Window};

pub struct OpenGLEngine {
    window: sdl2::video::Window,
    gl_context: GLContext,
}

impl OpenGLEngine {
    pub fn new(sdl: &sdl2::Sdl) -> Failable<Self> {
        let video = sdl.video()?;
        let attributes = video.gl_attr();

        if cfg!(target_os = "android") || cfg!(target_os = "ios") || cfg!(target_os = "emscripten") {
            attributes.set_context_profile(GLProfile::GLES);
        } else {
            attributes.set_context_profile(GLProfile::Compatibility);
        }

        attributes.set_context_version(2, 1);

        let window = OpenGLEngine::new_window(&sdl)?;
        let gl_context = OpenGLEngine::new_context(&sdl, &window)?;
        let engine = OpenGLEngine {
            window: window,
            gl_context: gl_context,
        };

        engine.update_viewport()?;

        Ok(engine)
    }

    fn new_window(sdl: &sdl2::Sdl) -> Failable<Window> {
        let video = sdl.video()?;
        let window_size = window::get_size(&video)?;
        let mut window_builder = video.window("crust", window_size.width.try_into()?, window_size.height.try_into()?);

        window_builder.position_centered();

        if cfg!(target_os = "android") || cfg!(target_os = "ios") {
            window_builder.fullscreen();
        }

        let flags = sdl2::sys::SDL_WindowFlags::SDL_WINDOW_ALLOW_HIGHDPI as u32
            | sdl2::sys::SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32
            | sdl2::sys::SDL_WindowFlags::SDL_WINDOW_OPENGL as u32;

        window_builder.set_window_flags(flags);

        Ok(window_builder.build().map_err(|_| String::from("Failed to create window"))?)
    }

    fn new_context(sdl: &sdl2::Sdl, window: &Window) -> Failable<GLContext> {
        logs::out(log_tag!(), "Creating context ...");
        let gl_context = window.gl_create_context()?;

        logs::out(log_tag!(), "Context created successfully ...");
        let video = sdl.video()?;

        gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

        unsafe {
            gl::ClearDepthf(1.);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
            gl::Enable(gl::CULL_FACE);
        }

        Ok(gl_context)
    }

    fn update_viewport(&self) -> FailableUnit {
        let display_size = self.get_display_size()?;

        unsafe {
            gl::Viewport(0, 0, display_size.width, display_size.height);
        }

        Ok(())
    }
}

impl Graphics for OpenGLEngine {
    fn get_display_size(&self) -> Failable<DisplaySize> {
        let size = self.window.drawable_size();

        Ok(DisplaySize {
            width: size.0.try_into()?,
            height: size.1.try_into()?,
        })
    }

    fn on_display_size_changed(&mut self) -> Failable<DisplaySize> {
        self.update_viewport()?;
        Ok(self.get_display_size()?)
    }

    fn render_begin(&mut self) -> FailableUnit {
        self.window.gl_make_current(&self.gl_context)?;

        unsafe {
            gl::ClearColor(0.5, 0.2, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        Ok(())
    }

    fn render_end(&mut self) -> FailableUnit {
        self.window.gl_swap_window();
        Ok(())
    }
}

impl Renderer for OpenGLEngine {
    fn render_models(&mut self) -> FailableUnit {
        Ok(())
    }
}

impl Engine for OpenGLEngine {}
