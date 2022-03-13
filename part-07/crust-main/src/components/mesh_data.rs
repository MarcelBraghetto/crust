use crate::components::vertex::Vertex;
use std::vec::Vec;

pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}
