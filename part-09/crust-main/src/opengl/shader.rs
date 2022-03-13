use crate::{
    components::{cache::Cache, model::Model},
    core::{failable::Failable, failable_unit::FailableUnit},
    opengl::{mesh::Mesh, shader_program::ShaderProgram, texture::Texture},
};
use gl::types::{GLint, GLsizei, GLuint, GLvoid};
use glm::Mat4;
use std::{ffi::CString, mem::size_of};

pub struct Shader {
    program: ShaderProgram,
    uniform_mvp: GLint,
    attr_vertex: GLuint,
    attr_texture: GLuint,
    stride: GLsizei,
    offset_vertex: GLsizei,
    offset_texture: GLsizei,
}

impl Shader {
    pub fn new(shader_name: &str) -> Failable<Self> {
        let program = ShaderProgram::new(shader_name)?;

        let uniform_mvp_key = CString::new("u_mvp")?;
        let uniform_mvp = unsafe { gl::GetUniformLocation(program.id, uniform_mvp_key.as_ptr()) };

        let attr_vertex_key = CString::new("a_vertexPosition")?;
        let attr_vertex = unsafe { gl::GetAttribLocation(program.id, attr_vertex_key.as_ptr()) };

        let attr_texture_key = CString::new("a_texCoord")?;
        let attr_texture = unsafe { gl::GetAttribLocation(program.id, attr_texture_key.as_ptr()) };

        let stride = (5 * size_of::<f32>()) as GLint;
        let offset_vertex = 0 as GLint;
        let offset_texture = (3 * size_of::<f32>()) as GLint;

        Ok(Shader {
            program: program,
            uniform_mvp: uniform_mvp,
            attr_vertex: attr_vertex as GLuint,
            attr_texture: attr_texture as GLuint,
            stride: stride,
            offset_vertex: offset_vertex,
            offset_texture: offset_texture,
        })
    }

    pub fn render_model(
        &self,
        model: &Model,
        projection_view: &Mat4,
        meshes: &mut Cache<Mesh>,
        textures: &mut Cache<Texture>,
    ) -> FailableUnit {
        unsafe {
            // Instruct OpenGL to starting using our shader program.
            gl::UseProgram(self.program.id);

            // Enable the 'a_vertexPosition' attribute.
            gl::EnableVertexAttribArray(self.attr_vertex);

            // Enable the 'a_texCoord' attribute.
            gl::EnableVertexAttribArray(self.attr_texture);

            // Populate the 'u_mvp' uniform in the shader program.
            gl::UniformMatrix4fv(self.uniform_mvp, 1, gl::FALSE, &model.transform(projection_view).c0.x);
        }

        // Apply the texture we want to paint the mesh with.
        textures.get(&model.texture_id)?.bind();

        // Bind the mesh data.
        let mesh = meshes.get(&model.mesh_id)?;

        unsafe {
            // Bind the vertex and index buffers.
            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.id_vertices);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.id_indices);

            // Configure the 'a_vertexPosition' attribute.
            gl::VertexAttribPointer(self.attr_vertex, 3, gl::FLOAT, gl::FALSE, self.stride, self.offset_vertex as *const GLvoid);

            // Configure the 'a_texCoord' attribute.
            gl::VertexAttribPointer(
                self.attr_texture,
                2,
                gl::FLOAT,
                gl::FALSE,
                self.stride,
                self.offset_texture as *const GLvoid,
            );

            // Execute the draw command - with how many indices to iterate.
            gl::DrawElements(gl::TRIANGLES, mesh.num_indices, gl::UNSIGNED_INT, std::ptr::null());

            // Tidy up.
            gl::DisableVertexAttribArray(self.attr_vertex);
            gl::DisableVertexAttribArray(self.attr_texture);
        }

        Ok(())
    }
}
