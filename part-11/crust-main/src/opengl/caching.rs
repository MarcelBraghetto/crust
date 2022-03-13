use crate::{
    components::cache::Cache,
    opengl::{mesh::Mesh, shader::Shader, texture::Texture},
};

pub fn new_mesh_cache() -> Cache<Mesh> {
    Cache::new("meshes", |key| Mesh::new(key))
}

pub fn new_shader_cache() -> Cache<Shader> {
    Cache::new("shaders", |key| Shader::new(key))
}

pub fn new_texture_cache() -> Cache<Texture> {
    Cache::new("textures", |key| Texture::new(key))
}
