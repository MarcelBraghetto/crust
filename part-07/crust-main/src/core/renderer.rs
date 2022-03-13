use crate::core::failable_unit::FailableUnit;

pub trait Renderer {
    fn render_models(&mut self) -> FailableUnit;
}
