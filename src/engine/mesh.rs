use glium::{Display, VertexBuffer, Frame, DrawParameters, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

pub struct Mesh {
    vb: VertexBuffer<Vertex>,
}

impl Mesh {
    pub fn new(display: &Display, vertices: &[Vertex]) -> Self {
        Mesh {
            vb: VertexBuffer::immutable(display, vertices).unwrap(),
        }
    }

    pub fn draw(&self, frame: &mut Frame, params: &DrawParameters) {
        frame.draw(self.vb, glium::index::NoIndices, self.program(), self.uniforms, params);
    }
}
