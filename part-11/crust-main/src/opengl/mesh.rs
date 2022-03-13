use crate::{
    components::mesh_data::MeshData,
    core::{failable::Failable, io},
};
use gl::types::{GLint, GLsizeiptr, GLuint, GLvoid};
use std::mem::size_of;

pub struct Mesh {
    pub id_vertices: GLuint,
    pub id_indices: GLuint,
    pub num_indices: GLint,
}

impl Mesh {
    pub fn new(path: &str) -> Failable<Self> {
        let mesh_data = io::load_obj_file(path)?;

        Ok(Mesh {
            id_vertices: create_vertex_buffer(&mesh_data),
            id_indices: create_index_buffer(&mesh_data),
            num_indices: mesh_data.indices.len() as GLint,
        })
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id_vertices);
            gl::DeleteBuffers(1, &self.id_indices);
        }
    }
}

fn create_vertex_buffer(data: &MeshData) -> GLuint {
    let mut buffer_data = vec![];

    for vertex in &data.vertices {
        // Position
        buffer_data.push(vertex.position.x);
        buffer_data.push(vertex.position.y);
        buffer_data.push(vertex.position.z);

        // Texture coordinate
        buffer_data.push(vertex.texture_coord.x);
        buffer_data.push(vertex.texture_coord.y);
    }

    let mut id: GLuint = 0;

    unsafe {
        gl::GenBuffers(1, &mut id);
        gl::BindBuffer(gl::ARRAY_BUFFER, id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (buffer_data.len() * size_of::<f32>()) as GLsizeiptr,
            buffer_data.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    id
}

fn create_index_buffer(data: &MeshData) -> GLuint {
    let mut id: GLuint = 0;
    let data_size = (data.indices.len() * size_of::<u32>()) as GLsizeiptr;

    unsafe {
        gl::GenBuffers(1, &mut id);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, data_size, data.indices.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }

    id
}
