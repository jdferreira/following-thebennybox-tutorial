use glium::{Display, Surface};
use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};

pub struct Window {
    pub display: Display,
}

impl Window {
    
    pub fn new(width: u32, height: u32, title: &str, events_loop: &EventsLoop) -> Window {
        let window = WindowBuilder::new()
            .with_dimensions(width, height)
            // .with_min_dimensions(width, height)
            // .with_max_dimensions(width, height)
            // .with_decorations(false)
            .with_title(title);
        let context = ContextBuilder::new();
        let display = Display::new(window, context, events_loop).unwrap();

        Window {
            display: display,
        }
    }

    pub fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 0.0);
        target.finish().unwrap();
    }

}