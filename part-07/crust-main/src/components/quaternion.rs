use glm::{Mat4, Vec3};
use std::ops;

pub struct Quaternion {
    pub axis: Vec3,
    pub rotation: f32,
}

impl Quaternion {
    pub fn new(axis: &Vec3, angle: f32) -> Self {
        let radians = angle.to_radians();
        let half_radians = radians * 0.5;

        Quaternion {
            axis: *axis * half_radians.sin(),
            rotation: half_radians.cos(),
        }
    }

    pub fn dot(&self, other: &Quaternion) -> f32 {
        let temp = Quaternion {
            axis: self.axis * other.axis,
            rotation: self.rotation * other.rotation,
        };

        (temp.axis.x + temp.axis.y) + (temp.axis.z + temp.rotation)
    }

    pub fn normalize(&self) -> Self {
        let x = self.axis.x;
        let y = self.axis.y;
        let z = self.axis.z;
        let rotation = self.rotation;
        let magnitude = (rotation * rotation + x * x + y * y + z * z).sqrt();

        Quaternion {
            axis: glm::vec3(x / magnitude, y / magnitude, z / magnitude),
            rotation: rotation / magnitude,
        }
    }

    pub fn to_matrix(&self) -> Mat4 {
        let axis = self.axis;
        let rotation = self.rotation;
        let xx = axis.x * axis.x;
        let yy = axis.y * axis.y;
        let zz = axis.z * axis.z;
        let xz = axis.x * axis.z;
        let xy = axis.x * axis.y;
        let yz = axis.y * axis.z;
        let rx = rotation * axis.x;
        let ry = rotation * axis.y;
        let rz = rotation * axis.z;

        glm::mat4(
            1. - 2. * (yy + zz),
            2. * (xy + rz),
            2. * (xz - ry),
            0.,
            2. * (xy - rz),
            1. - 2. * (xx + zz),
            2. * (yz + rx),
            0.,
            2. * (xz + ry),
            2. * (yz - rx),
            1. - 2. * (xx + yy),
            0.,
            0.,
            0.,
            0.,
            1.,
        )
    }
}

// Addition
fn add(a: &Quaternion, b: &Quaternion) -> Quaternion {
    Quaternion {
        axis: a.axis + b.axis,
        rotation: a.rotation + b.rotation,
    }
}

// &Quaternion + &Quaternion
impl ops::Add<&Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn add(self, other: &Quaternion) -> Self::Output {
        add(&self, other)
    }
}

// &Quaternion + Quaternion
impl ops::Add<Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn add(self, other: Quaternion) -> Self::Output {
        add(&self, &other)
    }
}

// Quaternion + &Quaternion
impl ops::Add<&Quaternion> for Quaternion {
    type Output = Quaternion;

    fn add(self, other: &Quaternion) -> Self::Output {
        add(&self, &other)
    }
}

// Quaternion + Quaternion
impl ops::Add<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn add(self, other: Quaternion) -> Self::Output {
        add(&self, &other)
    }
}

// Subtraction
fn subtract(a: &Quaternion, b: &Quaternion) -> Quaternion {
    Quaternion {
        axis: a.axis - b.axis,
        rotation: a.rotation - b.rotation,
    }
}

// &Quaternion - &Quaternion
impl ops::Sub<&Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn sub(self, other: &Quaternion) -> Self::Output {
        subtract(&self, other)
    }
}

// &Quaternion - Quaternion
impl ops::Sub<Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn sub(self, other: Quaternion) -> Self::Output {
        subtract(&self, &other)
    }
}

// Quaternion - &Quaternion
impl ops::Sub<&Quaternion> for Quaternion {
    type Output = Quaternion;

    fn sub(self, other: &Quaternion) -> Self::Output {
        subtract(&self, &other)
    }
}

// Quaternion - Quaternion
impl ops::Sub<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn sub(self, other: Quaternion) -> Self::Output {
        subtract(&self, &other)
    }
}

// Multiplication
fn multiply(a: &Quaternion, b: &Quaternion) -> Quaternion {
    let axis_a = &a.axis;
    let rotation_a = a.rotation;
    let axis_b = &b.axis;
    let rotation_b = b.rotation;

    Quaternion {
        axis: glm::vec3(
            rotation_a * axis_b.x + axis_a.x * rotation_b + axis_a.y * axis_b.z - axis_a.z * axis_b.y,
            rotation_a * axis_b.y - axis_a.x * axis_b.z + axis_a.y * rotation_b + axis_a.z * axis_b.x,
            rotation_a * axis_b.z + axis_a.x * axis_b.y - axis_a.y * axis_b.x + axis_a.z * rotation_b,
        ),
        rotation: rotation_a * rotation_b - axis_a.x * axis_b.x - axis_a.y * axis_b.y - axis_a.z * axis_b.z,
    }
}

// &Quaternion * &Quaternion
impl ops::Mul<&Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, other: &Quaternion) -> Self::Output {
        multiply(&self, other)
    }
}

// &Quaternion * Quaternion
impl ops::Mul<Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Self::Output {
        multiply(&self, &other)
    }
}

// Quaternion * &Quaternion
impl ops::Mul<&Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: &Quaternion) -> Self::Output {
        multiply(&self, other)
    }
}

// Quaternion * Quaternion
impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Self::Output {
        multiply(&self, &other)
    }
}
