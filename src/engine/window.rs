use glium::{Display, Surface};
use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};
use glium::glutin::{Event, WindowEvent};

pub struct Window {
    pub display: Display,
    pub closed: bool,
    events_loop: EventsLoop,
}

impl Window {
    
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let window = WindowBuilder::new()
            .with_dimensions(width, height)
            .with_min_dimensions(width, height)
            .with_max_dimensions(width, height)
            // .with_decorations(false)
            .with_title(title);
        let context = ContextBuilder::new();
        let events_loop = EventsLoop::new();

        let display = Display::new(window, context, &events_loop).unwrap();

        Window {
            display: display,
            closed: false,
            events_loop: events_loop,
        }
    }

    pub fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 0.0);
        target.finish().unwrap();
    }

    pub fn events(&mut self) {
        let mut closed = self.closed;

        self.events_loop.poll_events(|ev| {
            if let Event::WindowEvent { event: WindowEvent::Closed, .. } = ev {
                closed = true;
            }
        });

        self.closed = closed;
    }
}