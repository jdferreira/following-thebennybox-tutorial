use super::window;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const TITLE: &'static str = "3D engine";

pub struct MainComponent {
    window: window::Window,
}

impl MainComponent {
    pub fn new() -> Self {
        MainComponent {
            window: window::Window::new(WIDTH, HEIGHT, TITLE),
        }
    }

    pub fn start(&mut self) {
        self.run();
    }

    fn stop(&mut self) {}

    fn run(&mut self) {
        while !self.window.closed {
            self.render();
            self.window.events();
        }
    }

    fn render(&mut self) {
        self.window.render();
    }

    fn clean_up(&mut self) {}
}
