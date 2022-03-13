use crate::components::{matrix, orientation::Orientation};
use glm::{Mat4, Vec3};

pub struct Model {
    pub mesh_id: String,
    pub texture_id: String,
    pub shader_id: String,
    pub position: Vec3,
    pub scale: Vec3,
    orientation: Orientation,
    identity: Mat4,
}

impl Model {
    pub fn new(mesh_id: &str, texture_id: &str, shader_id: &str, position: Vec3, scale: Vec3) -> Self {
        Model {
            mesh_id: mesh_id.to_owned(),
            texture_id: texture_id.to_owned(),
            shader_id: shader_id.to_owned(),
            position: position,
            scale: scale,
            orientation: Orientation::new(0., 0., 0.),
            identity: matrix::identity(),
        }
    }

    pub fn orientation(&mut self) -> &mut Orientation {
        &mut self.orientation
    }

    pub fn transform(&self, projection: &Mat4) -> Mat4 {
        *projection
            * glm::ext::translate(&self.identity, self.position)
            * self.orientation.to_matrix()
            * glm::ext::scale(&self.identity, self.scale)
    }
}
