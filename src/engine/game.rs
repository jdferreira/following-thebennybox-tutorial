use glium::glutin::{VirtualKeyCode, MouseButton};
use super::input::InputState;

pub struct Game;

impl Game {
    pub fn update(&mut self) {} 

    pub fn render(&mut self) {}

    pub fn input(&mut self, input_state: &InputState) {
        if input_state.key_is_pressed(VirtualKeyCode::Space) {
            println!("Pressed space");
        }
        
        if input_state.key_is_released(VirtualKeyCode::Space) {
            println!("Released space");
        }

        if input_state.mouse_is_pressed(MouseButton::Right) {
            println!("Pressed right mouse button at {?:}", input_state.mouse_position());
        }

        if input_state.mouse_is_released(MouseButton::Right) {
            println!("Released right mouse button at {:?}", input_state.mouse_position());
        }

    }
}
