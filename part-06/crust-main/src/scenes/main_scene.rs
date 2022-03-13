use crate::{
    core::{display_size::DisplaySize, failable::Failable, failable_unit::FailableUnit, logs, renderer::Renderer, scene::Scene},
    log_tag,
};
use sdl2::keyboard::Scancode;

pub struct MainScene {
    display_size: DisplaySize,
}

impl MainScene {
    pub fn new(display_size: DisplaySize) -> Failable<Self> {
        Ok(MainScene {
            display_size: display_size,
        })
    }

    fn process_input(&mut self, event_pump: &sdl2::EventPump) -> FailableUnit {
        let key_state = event_pump.keyboard_state();

        if key_state.is_scancode_pressed(Scancode::Up) {
            logs::out(log_tag!(), "Key pressed: UP");
        }

        if key_state.is_scancode_pressed(Scancode::Down) {
            logs::out(log_tag!(), "Key pressed: DOWN");
        }

        if key_state.is_scancode_pressed(Scancode::A) {
            logs::out(log_tag!(), "Key pressed: A");
        }

        if key_state.is_scancode_pressed(Scancode::Z) {
            logs::out(log_tag!(), "Key pressed: Z");
        }

        if key_state.is_scancode_pressed(Scancode::Left) {
            logs::out(log_tag!(), "Key pressed: LEFT");
        }

        if key_state.is_scancode_pressed(Scancode::Right) {
            logs::out(log_tag!(), "Key pressed: RIGHT");
        }

        Ok(())
    }
}

impl Scene for MainScene {
    fn update(&mut self, _: f32, event_pump: &sdl2::EventPump) -> FailableUnit {
        self.process_input(event_pump)?;
        Ok(())
    }

    fn render(&mut self, _: &mut dyn Renderer) -> FailableUnit {
        Ok(())
    }

    fn on_display_size_changed(&mut self, display_size: DisplaySize) -> FailableUnit {
        self.display_size = display_size;
        Ok(())
    }
}
