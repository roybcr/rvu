use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("RVU")
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |ev, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match ev {
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::Resized(_) => (),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    device_id: _,
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    is_synthetic: _,
                } => *control_flow = ControlFlow::Exit,

                WindowEvent::ScaleFactorChanged {
                    scale_factor: _,
                    new_inner_size: _,
                } => (),
                _ => (),
            },

            Event::RedrawRequested(_) => (),
            _ => (),
        }
    });
}
