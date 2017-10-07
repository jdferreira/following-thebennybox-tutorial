use glium::{Display, Surface, DrawParameters, Depth, DepthTest, BackfaceCullingMode, Frame as GliumFrame};
use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};

use super::mesh::Mesh;

pub struct Window<'a> {
    pub display: Display,
    params: DrawParameters<'a>,
}

impl<'a> Window<'a> {
    
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
            params: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                backface_culling: BackfaceCullingMode::CullClockwise,
                ..Default::default()
            }
        }
    }

    pub fn draw<F>(&mut self, callback: F)
    where for <'b> F: FnOnce(Frame<'b>) {
        let mut target = self.display.draw();
        callback(Frame::new(&mut target, &self.params));
        target.finish().unwrap();
    }

    pub fn set_title(&mut self, title: &str) {
        use std::ops::Deref;
        self.display.gl_window().deref().set_title(title);
    }

}

pub struct Frame<'a> {
    frame: &'a mut GliumFrame,
    params: &'a DrawParameters<'a>
}

impl<'a> Frame<'a> {

    fn new(frame: &'a mut GliumFrame, params: &'a DrawParameters<'a>) -> Self {
        Frame { frame, params }
    }

    pub fn clear_screen(&mut self) {
        // Clear the screen to black and clear the depth and stencil buffers
        self.frame.clear_depth(1.0);
        self.frame.clear_stencil(0); // TODO: Is 0 the correct number?
    }

    pub fn clear_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.frame.clear_color_srgb(red, green, blue, alpha);
    }

    pub fn draw(&mut self, mesh: &Mesh) {
        mesh.draw(self.frame, self.params);
    }

}
