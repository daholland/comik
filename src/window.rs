use anyhow::Result;
use glium::glutin;

#[derive(Debug)]
pub struct Window {
    pub display: glium::Display,
}

impl Window {
    pub fn new(title: String, event_loop: &glutin::event_loop::EventLoop<()>) -> Result<Self> {
        let display = Self::create_display(title, event_loop).unwrap();

        Ok(Window {
            display,
        })
    }

    pub fn set_title(&self, title: String) -> Result<()> {
        self.display.gl_window().window().set_title(&title);

        Ok(())
    }

    fn create_display(
        title: String,
        event_loop: &glutin::event_loop::EventLoop<()>,
    ) -> Result<glium::Display> {
        let window_builder = glutin::window::WindowBuilder::new()
            .with_resizable(true)
            .with_inner_size(glutin::dpi::LogicalSize {
                width: 1280.0,
                height: 720.0,
            })
            .with_title(title);

        let context_builder = glutin::ContextBuilder::new()
            .with_depth_buffer(0)
            .with_srgb(true)
            .with_stencil_buffer(0)
            .with_vsync(true);

        Ok(glium::Display::new(window_builder, context_builder, event_loop).unwrap())
    }
}
