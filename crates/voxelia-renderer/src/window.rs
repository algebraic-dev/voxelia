//! Definition of a [Window] and an abstraction over the winit event loop with events.

use winit::{
    dpi::Pixel,
    event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window,
};

// Public re-exports
pub use winit::dpi::PhysicalSize;
pub use winit::event::VirtualKeyCode;

/// Instance of a window of a game with an event loop that will get all window events and push it
/// into somewhere else.
pub struct Window {
    pub window: window::Window,
    pub event_loop: EventLoop<()>,
}

/// Window events that help abstract over raw winit things.
pub enum WindowEvents {
    Resized(PhysicalSize<u32>),
    Keyboard {
        state: ElementState,
        virtual_keycode: Option<VirtualKeyCode>,
    },
    Draw,
}

impl Window {
    // Creates a new window using the title of the window and the default size.
    pub fn new<P: Pixel>(title: &str, size: PhysicalSize<P>) -> Self {
        let event_loop = EventLoop::new();

        let window = window::WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();

        window.set_inner_size(size);

        Self { event_loop, window }
    }

    /// Runs the window event loop.
    pub fn run(self, mut func: impl 'static + FnMut(&window::Window, WindowEvents)) -> ! {
        let id = self.window.id();

        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::MouseMotion { .. } => {}
                    DeviceEvent::MouseWheel { .. } => {}
                    _ => (),
                },
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == id => {
                    use WindowEvent::{CloseRequested, Resized, ScaleFactorChanged};

                    match event {
                        CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state,
                                    virtual_keycode,
                                    ..
                                },
                            ..
                        } => func(
                            &self.window,
                            WindowEvents::Keyboard {
                                state: state.to_owned(),
                                virtual_keycode: virtual_keycode.to_owned(),
                            },
                        ),
                        Resized(position) => {
                            func(&self.window, WindowEvents::Resized(position.to_owned()))
                        }
                        ScaleFactorChanged { new_inner_size, .. } => func(
                            &self.window,
                            WindowEvents::Resized(**new_inner_size.to_owned()),
                        ),
                        _ => {}
                    }
                }
                Event::RedrawRequested(window_id) if window_id == id => {
                    func(&self.window, WindowEvents::Draw)
                }
                Event::MainEventsCleared => func(&self.window, WindowEvents::Draw),
                _ => {}
            })
    }
}
