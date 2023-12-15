use crate::drawing::{context_setup, draw_to_context};
use crate::easy_file_io::write_draw_error;
use glium::glutin::surface::WindowSurface;
use glium::{Display, DrawError};
use std::error::Error;
use std::process;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;

pub struct Core {
    pub window: Window,
    pub display: Display<WindowSurface>,
    pub event_loop: EventLoop<()>,
}

pub fn init_display() -> Result<Core, Box<dyn Error>> {
    let event_loop = winit::event_loop::EventLoop::new();

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_inner_size(1024, 780)
        .with_title("Trident Engine")
        .build(&event_loop);

    let core = Core {
        window,
        display,
        event_loop,
    };

    Ok(core)
}

pub fn run_event_loop(core: Core) {
    let draw_data = context_setup(&core.display);

    core.event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
            }
            Event::MainEventsCleared => match draw_to_context(&core.display, &draw_data) {
                Ok(_) => {}
                Err(draw_err) => {
                    write_draw_error("map_editor/engine_draw_error_output.txt", draw_err);
                    process::exit(1);
                }
            },

            _ => (),
        }
    });
}
