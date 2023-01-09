use gui_lib::{self, GraphicsState, InnerSize};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    
    let size = window.inner_size();
    let size = gui_lib::InnerSize {
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
        _ => {}
    });
}

fn main() {
    pollster::block_on(run());
}
