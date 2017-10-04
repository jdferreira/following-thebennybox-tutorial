use std::collections::HashSet;
use glium::glutin::{Event, EventsLoop, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState, MouseButton};
// use glium::glutin::{DeviceEvent};

pub struct InputState {
    close_requested: bool,
    keys_down: HashSet<VirtualKeyCode>,
    keys_pressed_before: HashSet<VirtualKeyCode>,
    keys_up: HashSet<VirtualKeyCode>,
    mouse_down: HashSet<MouseButton>,
    mouse_pressed_before: HashSet<MouseButton>,
    mouse_up: HashSet<MouseButton>,
    mouse_position: (f64, f64),
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            close_requested: false,
            keys_down: HashSet::new(),
            keys_pressed_before: HashSet::new(),
            keys_up: HashSet::new(),
            mouse_down: HashSet::new(),
            mouse_pressed_before: HashSet::new(),
            mouse_up: HashSet::new(),
            mouse_position: (0.0, 0.0),
        }
    }
}

impl InputState {

    pub fn handle_events(&mut self, events_loop: &mut EventsLoop) {
        self.prepare();

        events_loop.poll_events(|ev| match ev {
            Event::WindowEvent { event, .. } => self.window_event(event),
            _ => (),
            // Event::Awakened => new_state.awakened(),
            // Event::DeviceEvent { event, .. } => new_state.device_event(event),
        });
    }

    fn window_event(&mut self, ev: WindowEvent) {
        match ev {
            WindowEvent::Closed => self.close_requested = true,
            WindowEvent::KeyboardInput { input, .. } => self.keyboard_input(input),
            WindowEvent::MouseInput { button, state, .. } => self.mouse_input(button, state),
            WindowEvent::MouseMoved { position, .. } => self.mouse_moved(position),
            _ => (),
        }
    }

    // fn device_event(&mut self, _ev: DeviceEvent) { }
    // fn awakened(&mut self) { }

    fn keyboard_input(&mut self, input: KeyboardInput) {
        let keycode = match input.virtual_keycode {
            Some(keycode) => keycode,
            None => return,
        };

        if input.state == ElementState::Pressed {
            self.keys_down.insert(keycode);
        }
        else if input.state == ElementState::Released {
            self.keys_down.remove(&keycode);
            self.keys_up.insert(keycode);
        }
    }

    fn mouse_input(&mut self, button: MouseButton, state: ElementState) {
        if state == ElementState::Pressed {
            self.mouse_down.insert(button);
        }
        else if state == ElementState::Released {
            self.mouse_down.remove(&button);
            self.mouse_up.insert(button);
        }
    }

    fn mouse_moved(&mut self, position: (f64, f64)) {
        self.mouse_position = position;
    }

    fn prepare(&mut self) {
        self.close_requested = false;
        
        self.keys_pressed_before.extend(self.keys_down.iter());
        for keycode in self.keys_up.iter() {
            self.keys_pressed_before.remove(keycode);
        }
        
        self.keys_down.clear();
        self.keys_up.clear();

        self.mouse_pressed_before.extend(self.mouse_down.iter());
        for button in self.mouse_up.iter() {
            self.mouse_pressed_before.remove(button);
        }
        
        self.mouse_down.clear();
        self.mouse_up.clear();
    }

    pub fn is_close_requested(&self) -> bool {
        self.close_requested
    }

    pub fn key_is_down(&self, keycode: VirtualKeyCode) -> bool {
        self.keys_down.contains(&keycode)
    }

    pub fn key_is_pressed(&self, keycode: VirtualKeyCode) -> bool {
        self.keys_down.contains(&keycode) && !self.keys_pressed_before.contains(&keycode)
    }

    pub fn key_is_released(&self, keycode: VirtualKeyCode) -> bool {
        self.keys_up.contains(&keycode)
    }

    pub  fn mouse_is_down(&self, button: MouseButton) -> bool {
        self.mouse_down.contains(&button)
    }

    pub fn mouse_is_pressed(&self, button: MouseButton) -> bool {
        self.mouse_down.contains(&button) && !self.mouse_pressed_before.contains(&button)
    }

    pub fn mouse_is_released(&self, button: MouseButton) -> bool {
        self.mouse_up.contains(&button)
    }

    pub fn mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }
}