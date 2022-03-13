use crate::core::{failable::Failable, io};
use gl::types::{GLchar, GLenum, GLint, GLuint};
use std::ffi::CString;

pub struct ShaderProgram {
    pub id: GLuint,
    pub vertex_shader_id: GLuint,
    pub fragment_shader_id: GLuint,
}

impl ShaderProgram {
    pub fn new(shader_name: &str) -> Failable<Self> {
        let vertex_shader_id = create_vertex_shader(shader_name)?;
        let fragment_shader_id = create_fragment_shader(shader_name)?;
        let program_id = create_shader_program(vertex_shader_id, fragment_shader_id)?;

        Ok(ShaderProgram {
            id: program_id,
            vertex_shader_id: vertex_shader_id,
            fragment_shader_id: fragment_shader_id,
        })
    }
}

fn create_vertex_shader(shader_name: &str) -> Failable<GLuint> {
    let shader_code = io::load_text_file(&format!("assets/shaders/opengl/{}.vert", shader_name))?;
    let prefix = if cfg!(target_os = "android") || cfg!(target_os = "ios") || cfg!(target_os = "emscripten") {
        "#version 100\n"
    } else {
        "#version 120\n"
    };

    Ok(compile_shader(gl::VERTEX_SHADER, &format!("{}{}", prefix, shader_code))?)
}

fn create_fragment_shader(shader_name: &str) -> Failable<GLuint> {
    let shader_code = io::load_text_file(&format!("assets/shaders/opengl/{}.frag", shader_name))?;
    let prefix = if cfg!(target_os = "android") || cfg!(target_os = "ios") || cfg!(target_os = "emscripten") {
        "#version 100\nprecision mediump float;\n"
    } else {
        "#version 120\n"
    };

    Ok(compile_shader(gl::FRAGMENT_SHADER, &format!("{}{}", prefix, shader_code))?)
}

fn create_shader_program(vertex_shader_id: GLuint, fragment_shader_id: GLuint) -> Failable<GLuint> {
    let shader_program_id = unsafe { gl::CreateProgram() };

    // Attach and link them.
    unsafe {
        gl::AttachShader(shader_program_id, vertex_shader_id);
        gl::AttachShader(shader_program_id, fragment_shader_id);
        gl::LinkProgram(shader_program_id);
    }

    // Check if there were any errors linking the shader program.
    let mut program_link_result: GLint = 1;
    unsafe {
        gl::GetProgramiv(shader_program_id, gl::LINK_STATUS, &mut program_link_result);
    }

    // 0 means program linking failed.
    if program_link_result == 0 {
        let mut error_message_length: GLint = 0;
        unsafe {
            gl::GetProgramiv(shader_program_id, gl::INFO_LOG_LENGTH, &mut error_message_length);
        }

        let error_message = vec![b' '; error_message_length as usize];

        unsafe {
            gl::GetProgramInfoLog(
                shader_program_id,
                error_message_length,
                std::ptr::null_mut(),
                error_message.as_ptr() as *mut GLchar,
            );
        }

        return Err(String::from_utf8_lossy(&error_message).into());
    }

    unsafe {
        gl::DetachShader(shader_program_id, vertex_shader_id);
        gl::DetachShader(shader_program_id, fragment_shader_id);
        gl::DeleteShader(vertex_shader_id);
        gl::DeleteShader(fragment_shader_id);
    }

    Ok(shader_program_id)
}

fn compile_shader(shader_type: GLenum, source: &str) -> Failable<GLuint> {
    let shader_id = unsafe { gl::CreateShader(shader_type) };
    let input = CString::new(source).unwrap();

    unsafe {
        gl::ShaderSource(shader_id, 1, &input.as_ptr(), std::ptr::null());
        gl::CompileShader(shader_id);
    }

    let mut shader_compilation_result: GLint = 1;
    unsafe {
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut shader_compilation_result);
    }

    // 0 means shader failed to compile.
    if shader_compilation_result == 0 {
        let mut error_message_length: GLint = 0;
        unsafe {
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut error_message_length);
        }

        let error_message = vec![b' '; error_message_length as usize];

        unsafe {
            gl::GetShaderInfoLog(shader_id, error_message_length, std::ptr::null_mut(), error_message.as_ptr() as *mut GLchar);
        }

        return Err(String::from_utf8_lossy(&error_message).into());
    }

    Ok(shader_id)
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
