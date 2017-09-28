use glium::glutin::{Event, EventsLoop, WindowEvent};
use time;

use super::window;
use super::game;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const TITLE: &'static str = "3D engine";
const FRAME_CAP: u32 = 5000;
const MAX_TICK_DURATION: f64 = 1.0 / FRAME_CAP as f64;

pub struct MainComponent {
    events_loop: EventsLoop,
    window: window::Window,
    game: game::Game,
    running: bool,
    last_game_tick: f64,
}

impl MainComponent {
    pub fn new() -> Self {
        let events_loop = EventsLoop::new();
        let window = window::Window::new(WIDTH, HEIGHT, TITLE, &events_loop);

        MainComponent {
            events_loop: events_loop,
            window: window,
            game: game::Game,
            running: false,
            last_game_tick: time::precise_time_s(),
        }
    }

    pub fn start(&mut self) {
        if self.running {
            return;
        }

        self.run();
    }

    fn stop(&mut self) {
        if !self.running {
            return;
        }

        self.running = false;
    }

    fn run(&mut self) {
        self.running = true;
        self.last_game_tick = time::precise_time_s();

        let mut unprocessed_time = 0.0;
        let mut frames = 0;
        let mut frame_time_counter = 0.0;

        while self.running {
            let elapsed = self.elapsed_since_last_game_tick();
            unprocessed_time += elapsed;
            frame_time_counter += elapsed;

            let needs_render = unprocessed_time > MAX_TICK_DURATION;

            while unprocessed_time > MAX_TICK_DURATION {
                unprocessed_time -= MAX_TICK_DURATION;
                self.handle_events();
                self.game.update();

                if frame_time_counter > 1.0 {
                    println!("FPS: {}", frames);
                    frames = 0;
                    frame_time_counter = 0.0;
                }
            }

            if needs_render {
                self.render();
                frames += 1;
            }
        }

        self.clean_up();
    }

    fn render(&mut self) {
        self.game.render();
        self.window.render();
    }

    fn clean_up(&mut self) {}

    fn elapsed_since_last_game_tick(&mut self) -> f64 {
        let now = time::precise_time_s();
        let elapsed = now - self.last_game_tick;
        self.last_game_tick = now;

        elapsed
    }

    pub fn handle_events(&mut self) {
        let mut close_requested = false;
        
        self.events_loop.poll_events(|ev| {
            if let Event::WindowEvent { event: WindowEvent::Closed, .. } = ev {
                close_requested = true;
            }
        });

        if close_requested {
            self.stop();
        }
    }

}
