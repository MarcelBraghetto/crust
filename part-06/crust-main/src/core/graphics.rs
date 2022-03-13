use crate::core::{display_size::DisplaySize, failable::Failable, failable_unit::FailableUnit};

pub trait Graphics {
    fn on_display_size_changed(&mut self) -> Failable<DisplaySize>;
    fn get_display_size(&self) -> Failable<DisplaySize>;
    fn render_begin(&mut self) -> FailableUnit;
    fn render_end(&mut self) -> FailableUnit;
}
