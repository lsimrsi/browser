use std::fs;

use wb::{self, html_parser::Parser, GraphicsState, InnerSize};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub async fn run() {
    env_logger::init();
    let html = fs::read_to_string("./window/src/basic.html")
        .expect("Should have been able to read the file");
    let nodes = Parser::parse(html);
    println!("nodes:\n{}", nodes);
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let size = window.inner_size();
    let size = wb::InnerSize {
        width: size.width,
        height: size.height,
    };

    let mut state = GraphicsState::new(&window, size).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                let new_size = InnerSize {
                    width: physical_size.width,
                    height: physical_size.height,
                };
                state.resize(new_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                let new_size = InnerSize {
                    width: (**new_inner_size).width,
                    height: (**new_inner_size).height,
                };
                state.resize(new_size);
            }

            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                // Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // // The system is out of memory, we should probably quit
                // Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => {
                    eprintln!("{:?}", e);
                    *control_flow = ControlFlow::Exit
                }
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        _ => {}
    });
}

fn main() {
    pollster::block_on(run());
}
