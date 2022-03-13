use crate::{
    components::{mesh_data::MeshData, texture_data::TextureData, vertex::Vertex},
    core::failable::Failable,
};
use sdl2::{pixels::PixelFormatEnum, rwops::RWops, surface::Surface};
use std::{
    collections::HashMap,
    io::{BufReader, Read},
    path::Path,
    vec::Vec,
};

pub fn load_text_file(path: &str) -> Failable<String> {
    let mut stream = RWops::from_file(Path::new(path), "r")?;
    let mut content = String::new();

    stream.read_to_string(&mut content)?;

    Ok(content)
}

pub fn load_obj_file(path: &str) -> Failable<MeshData> {
    let data = load_text_file(path)?;
    let mut input = BufReader::new(data.as_bytes());
    let (models, _) = tobj::load_obj_buf(&mut input, true, |_| Ok((Vec::new(), HashMap::new())))?;
    let mut vertices: Vec<Vertex> = vec![];
    let mut indices: Vec<u32> = vec![];

    for model in &models {
        let mesh = &model.mesh;

        for index in &mesh.indices {
            vertices.push(Vertex {
                position: glm::vec3(
                    mesh.positions[(3 * index + 0) as usize],
                    mesh.positions[(3 * index + 1) as usize],
                    mesh.positions[(3 * index + 2) as usize],
                ),
                texture_coord: glm::vec2(
                    mesh.texcoords[(2 * index + 0) as usize],
                    -1. - mesh.texcoords[(2 * index + 1) as usize],
                ),
            });

            indices.push((vertices.len() - 1) as u32);
        }
    }

    Ok(MeshData {
        vertices: vertices,
        indices: indices,
    })
}

pub fn load_png(path: &str) -> Failable<TextureData> {
    let surface: Surface = sdl2::image::LoadSurface::from_file(Path::new(path))?;
    Ok(TextureData::new(surface.convert_format(PixelFormatEnum::RGBA32)?))
}
