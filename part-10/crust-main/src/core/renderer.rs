use crate::{components::model::Model, core::failable_unit::FailableUnit};
use glm::Mat4;
use std::vec::Vec;

pub trait Renderer {
    fn render_models(&mut self, models: &Vec<Model>, projection_view: &Mat4) -> FailableUnit;
}
