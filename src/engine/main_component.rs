use glium::glutin::EventsLoop;

use time;

use super::window;
use super::game;
use super::input;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const TITLE: &'static str = "3D engine";
const FRAME_CAP: u32 = 5000;
const MAX_TICK_DURATION: f64 = 1.0 / FRAME_CAP as f64;

pub struct MainComponent<'a> {
    events_loop: EventsLoop,
    input_state: input::InputState,
    window: window::Window<'a>,
    game: game::Game,
    running: bool,
    last_game_tick: f64,
}

impl<'a> MainComponent<'a> {
    pub fn new() -> Self {
        let events_loop = EventsLoop::new();
        let window = window::Window::new(WIDTH, HEIGHT, TITLE, &events_loop);
        let game = game::Game::new(&window.display); // TODO: Can I make window.display private?
        
        MainComponent {
            events_loop: events_loop,
            input_state: Default::default(),
            window: window,
            game: game,
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
        let mut one_second_counter = 0.0;
        let mut frames = 0;
        let mut updates = 0;

        while self.running {
            let elapsed = self.elapsed_since_last_game_tick();
            unprocessed_time += elapsed;
            one_second_counter += elapsed;

            let needs_render = unprocessed_time > MAX_TICK_DURATION;

            while unprocessed_time > MAX_TICK_DURATION {
                self.game.update();
                
                unprocessed_time -= MAX_TICK_DURATION;
                updates += 1;

                if one_second_counter > 1.0 {
                    let title = format!("FPS: {} // Updates: {}", frames, updates);
                    self.window.set_title(&title);
                    one_second_counter = 0.0;
                    frames = 0;
                    updates = 0;
                }
            }

            if needs_render {
                self.handle_events();
                self.game.input(&self.input_state);
                self.render();
                frames += 1;
            }
        }

        self.clean_up();
    }

    fn render(&mut self) {
        let game = &self.game;

        self.window.draw(|mut frame| {
            frame.clear_screen();
            frame.clear_color(0.0, 0.5, 1.0, 1.0);
            game.render(&mut frame);
        });
    }

    fn clean_up(&mut self) {}

    fn elapsed_since_last_game_tick(&mut self) -> f64 {
        let now = time::precise_time_s();
        let elapsed = now - self.last_game_tick;
        self.last_game_tick = now;

        elapsed
    }

    fn handle_events(&mut self) {
        self.input_state.handle_events(&mut self.events_loop);
        if self.input_state.is_close_requested() {
            self.stop();
        }
    }
}
