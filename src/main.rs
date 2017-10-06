extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate time;

mod engine;

use engine::main_component;

fn main() {
    let mut component = main_component::MainComponent::new();
    component.start();
}
