//! Voxelia renderer library. It uses WGPU in order to render meshes to the screen.

use state::State;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod camera;
pub mod mesh;
pub mod pass_data;
pub mod pipeline;
pub mod renderable;
pub mod renderer;
pub mod state;
pub mod texture;

pub async fn run() {
    env_logger::init();

    // Create a new event loop and window to show the result.
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Creates a new rendering state
    let mut state = State::new(window).await;
    let id = state.window().id();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == id && !state.input(event) => {
            use WindowEvent::*;

            match event {
                CloseRequested => *control_flow = ControlFlow::Exit,
                Resized(physical_size) => state.resize(*physical_size),
                ScaleFactorChanged { new_inner_size, .. } => state.resize(**new_inner_size),
                _ => {}
            }
        }
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();

            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.renderer.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => state.window().request_redraw(),
        _ => {}
    });
}
