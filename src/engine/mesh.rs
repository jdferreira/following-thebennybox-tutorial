use std::rc::Rc;
use std::cell::Cell;

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
    shader: Rc<Program>,
    counter: Cell<u32>,
}

impl Mesh {
    pub fn new(display: &Display, vertices: &[Vertex], indices: &[u32], shader: Rc<Program>) -> Self {
        Mesh {
            vb: VertexBuffer::new(display, vertices).unwrap(),
            indices: IndexBuffer::new(display, PrimitiveType::TrianglesList, indices).unwrap(),
            shader: shader,
            counter: Cell::new(0),
        }
    }

    pub fn draw(&self, frame: &mut Frame, params: &DrawParameters) {
        {
            let c = self.counter.get();
            if c % 500 == 0 {
                println!("{:5} vb: {:?}", c, self.vb.read().unwrap());
                println!("      indices: {:?}", self.indices.read().unwrap());
            }
            self.counter.set(c + 1);
        }
        
        frame.draw(
            &self.vb,
            &self.indices,
            &self.shader,
            &EmptyUniforms,
            params,
        ).unwrap();
    }

}
