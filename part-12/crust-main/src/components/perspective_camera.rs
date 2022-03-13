use crate::core::display_size::DisplaySize;
use glm::{Mat4, Vec3};

pub struct PerspectiveCamera {
    projection: Mat4,
    up: Vec3,
    position: Vec3,
    target: Vec3,
}

impl PerspectiveCamera {
    pub fn new(display_size: &DisplaySize) -> Self {
        PerspectiveCamera {
            projection: glm::ext::perspective(
                66.0f32.to_radians(),
                (display_size.width as f32) / (display_size.height as f32),
                0.01,
                100.0,
            ),
            up: glm::vec3(0., 1., 0.),
            position: glm::vec3(0., 0., 0.),
            target: glm::vec3(0., 0., 0.),
        }
    }

    pub fn configure(&mut self, position: Vec3, direction: Vec3) {
        self.position = position;
        self.target = position - direction;
    }

    pub fn projection_view(&self) -> Mat4 {
        self.projection * glm::ext::look_at(self.position, self.target, self.up)
    }
}
