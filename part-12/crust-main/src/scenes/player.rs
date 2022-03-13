use crate::components::orientation::Orientation;
use glm::Vec3;

pub struct Player {
    position: Vec3,
    move_speed: f32,
    turn_speed: f32,
    orientation: Orientation,
}

impl Player {
    pub fn new(position: Vec3, pitch: f32, yaw: f32, roll: f32) -> Self {
        Player {
            position: position,
            move_speed: 5.,
            turn_speed: 120.,
            orientation: Orientation::new(pitch, yaw, roll),
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn direction(&self) -> Vec3 {
        self.orientation.direction()
    }

    pub fn move_forward(&mut self, delta: f32) {
        self.position = self.position - (self.direction() * (self.move_speed * delta));
    }

    pub fn move_backward(&mut self, delta: f32) {
        self.position = self.position + (self.direction() * (self.move_speed * delta));
    }

    pub fn turn_left(&mut self, delta: f32) {
        self.orientation.add_yaw(self.turn_speed * delta);
    }

    pub fn turn_right(&mut self, delta: f32) {
        self.orientation.add_yaw(-self.turn_speed * delta);
    }

    pub fn move_up(&mut self, delta: f32) {
        self.position.y += self.move_speed * delta;
    }

    pub fn move_down(&mut self, delta: f32) {
        self.position.y -= self.move_speed * delta;
    }
}
