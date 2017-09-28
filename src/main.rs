extern crate glium;

mod engine;

use engine::main_component;

fn main() {
    let mut component = main_component::MainComponent::new();
    component.start();
}
