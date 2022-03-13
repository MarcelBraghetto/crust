use crate::components::quaternion::Quaternion;
use glm::{Mat4, Vec3, Vec4};

pub struct Orientation {
    pitch: f32,
    yaw: f32,
    roll: f32,
    x_axis: Vec3,
    y_axis: Vec3,
    z_axis: Vec3,
    direction_axis: Vec4,
}

impl Orientation {
    pub fn new(pitch: f32, yaw: f32, roll: f32) -> Self {
        Orientation {
            pitch: pitch,
            yaw: yaw,
            roll: roll,
            x_axis: glm::vec3(1., 0., 0.),
            y_axis: glm::vec3(0., 1., 0.),
            z_axis: glm::vec3(0., 0., 1.),
            direction_axis: glm::vec4(0., 0., 1., 0.),
        }
    }

    pub fn add_pitch(&mut self, pitch: f32) {
        self.pitch = wrap_angle(self.pitch + pitch);
    }

    pub fn add_yaw(&mut self, yaw: f32) {
        self.yaw = wrap_angle(self.yaw + yaw);
    }

    pub fn add_roll(&mut self, roll: f32) {
        self.roll = wrap_angle(self.roll + roll);
    }

    pub fn to_matrix(&self) -> Mat4 {
        let quat_pitch = Quaternion::new(&self.x_axis, self.pitch);
        let quat_yaw = Quaternion::new(&self.y_axis, self.yaw);
        let quat_roll = Quaternion::new(&self.z_axis, self.roll);
        let orientation = quat_roll * quat_yaw * quat_pitch;

        orientation.to_matrix()
    }

    pub fn direction(&self) -> Vec3 {
        let direction = self.to_matrix() * self.direction_axis;

        glm::normalize(glm::vec3(direction.x, direction.y, direction.z))
    }
}

fn wrap_angle(input: f32) -> f32 {
    (input % 360. + 360.) % 360.
}
