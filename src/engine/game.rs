use std::rc::Rc;

use glium::{Display, Program};
use glium::glutin::{VirtualKeyCode, MouseButton};

use super::input::InputState;
use super::mesh::{Mesh, Vertex};
use super::window::Frame;

pub struct Game {
    mesh: Mesh,
}

impl Game {
    pub fn new(display: &Display) -> Self {
        let vertices = vec![
            Vertex::new(-1.0, -1.0, 0.0),
            Vertex::new( 1.0, -1.0, 0.0),
            Vertex::new( 0.0,  1.0, 0.0),
        ];

        let indices = vec![0, 1, 2];

        let shader = Rc::new(
            Program::from_source(
                display,
                include_str!("../../assets/shader.glslv"),
                include_str!("../../assets/shader.glslf"),
                None,
            ).unwrap()
        );

        Game {
            mesh: Mesh::new(display, &vertices, &indices, shader),
        }
    }

    pub fn update(&mut self) {} 

    pub fn render(&self, frame: &mut Frame) {
        frame.draw(&self.mesh);
    }

    pub fn input(&mut self, input_state: &InputState) {
        if input_state.key_is_pressed(VirtualKeyCode::Space) {
            println!("Pressed space");
        }
        
        if input_state.key_is_released(VirtualKeyCode::Space) {
            println!("Released space");
        }

        if input_state.mouse_is_pressed(MouseButton::Right) {
            println!("Pressed right mouse button at {:?}", input_state.mouse_position());
        }

        if input_state.mouse_is_released(MouseButton::Right) {
            println!("Released right mouse button at {:?}", input_state.mouse_position());
        }

    }
}
