use crate::core::{graphics::Graphics, renderer::Renderer};

pub trait Engine: Graphics + Renderer {}
