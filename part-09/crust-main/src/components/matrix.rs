use glm::Mat4;

#[inline]
pub fn identity() -> Mat4 {
    glm::mat4(1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.)
}
