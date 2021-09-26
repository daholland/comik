use anyhow::Result;
use glium::glutin;

use crate::{ui, window};

#[derive(Debug)]
pub struct App {}

impl App {
    pub fn run(title: String) -> Result<()> {
        let event_loop = glutin::event_loop::EventLoop::with_user_event();
        let window = window::Window::new(title, &event_loop).unwrap();

        let mut egui = egui_glium::EguiGlium::new(&window.display);

        let mut ui = ui::Ui::new().unwrap();

        event_loop.run(move |event, _, control_flow| {
            let mut redraw = || {
                egui.begin_frame(&window.display);

                let quit = ui.tick(egui.ctx());

                let (needs_repaint, shapes) = egui.end_frame(&window.display);

                *control_flow = if quit {
                    glutin::event_loop::ControlFlow::Exit
                } else if needs_repaint {
                    window.display.gl_window().window().request_redraw();
                    glutin::event_loop::ControlFlow::Poll
                } else {
                    glutin::event_loop::ControlFlow::Wait
                };

                {
                    use glium::Surface as _;
                    let mut target = window.display.draw();

                    let clear_color = epi::egui::Rgba::from_gray(0.02);
                    target.clear_color(
                        clear_color[0],
                        clear_color[1],
                        clear_color[2],
                        clear_color[3],
                    );

                    // draw things behind egui here

                    egui.paint(&window.display, &mut target, shapes);

                    // draw things on top of egui here

                    target.finish().unwrap();
                }
            };

            match event {
                // Platform-dependent event handlers to workaround a winit bug
                // See: https://github.com/rust-windowing/winit/issues/987
                // See: https://github.com/rust-windowing/winit/issues/1619
                glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
                glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

                glutin::event::Event::WindowEvent { event, .. } => {
                    if egui.is_quit_event(&event) {
                        *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    }

                    egui.on_event(&event);

                    // TODO: ask egui if the events warrants a repaint instead
                    window.display.gl_window().window().request_redraw(); 
                }

                _ => (),
            }
        });

    }
}
