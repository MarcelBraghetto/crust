use crate::core::{display_size::DisplaySize, failable_unit::FailableUnit, renderer::Renderer};

pub trait Scene {
    fn update(&mut self, delta: f32, event_pump: &sdl2::EventPump) -> FailableUnit;
    fn render(&mut self, renderer: &mut dyn Renderer) -> FailableUnit;
    fn on_display_size_changed(&mut self, display_size: DisplaySize) -> FailableUnit;
}
