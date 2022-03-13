use crate::{
    components::texture_data::TextureData,
    core::{failable::Failable, io},
};
use gl::types::{GLint, GLuint, GLvoid};

pub struct Texture {
    id: GLuint,
}

impl Texture {
    pub fn new(path: &str) -> Failable<Self> {
        let mut texture_data = io::load_png(path)?;

        Ok(Texture {
            id: create_texture(&mut texture_data),
        })
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

fn create_texture(data: &mut TextureData) -> GLuint {
    let mut id: GLuint = 0;

    unsafe {
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            data.width as GLint,
            data.height as GLint,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.surface().without_lock().unwrap().as_ptr() as *const GLvoid,
        );
    }

    id
}
