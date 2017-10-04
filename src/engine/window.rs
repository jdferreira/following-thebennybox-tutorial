use glium::{Display, Surface};
use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};

pub struct Window {
    pub display: Display,
}

impl Window {
    
    pub fn new(width: u32, height: u32, title: &str, events_loop: &EventsLoop) -> Self {
        let window_builder = WindowBuilder::new()
            .with_dimensions(width, height)
            // .with_min_dimensions(width, height)
            // .with_max_dimensions(width, height)
            // .with_decorations(false)
            .with_title(title);
        let context = ContextBuilder::new();
        let display = Display::new(window_builder, context, events_loop).unwrap();

        Window {
            display: display,
        }
    }

    pub fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 0.0);
        target.finish().unwrap();
    }

    pub fn set_title(&mut self, title: &str) {
        use std::ops::Deref;
        self.display.gl_window().deref().set_title(title);
    }

}