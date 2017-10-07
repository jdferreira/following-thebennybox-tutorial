use std::rc::Rc;

use glium::{Display, DrawParameters, Frame, Program, Surface, VertexBuffer};
use glium::index::{PrimitiveType, IndexBuffer};
use glium::uniforms::EmptyUniforms;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex {
            position: [x, y, z],
        }
    }
}

implement_vertex!(Vertex, position);

pub struct Mesh {
    vb: VertexBuffer<Vertex>,
    indices: IndexBuffer<u32>,
    shader: Rc<Program>
}

impl Mesh {
    pub fn new(display: &Display, vertices: &[Vertex], indices: &[u32], shader: Rc<Program>) -> Self {
        Mesh {
            vb: VertexBuffer::new(display, vertices).unwrap(),
            indices: IndexBuffer::new(display, PrimitiveType::TrianglesList, indices).unwrap(),
            shader: shader
        }
    }

    pub fn draw(&self, frame: &mut Frame, params: &DrawParameters) {
        frame.draw(
            &self.vb,
            &self.indices,
            &self.shader,
            &EmptyUniforms,
            params,
        ).unwrap();
    }

}
