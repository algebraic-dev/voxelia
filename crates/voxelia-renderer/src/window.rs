//! Definition of a [Window] and an abstraction over the winit event loop with events.

use winit::{
    dpi::{PhysicalPosition, Pixel},
    event::{DeviceEvent, Event, KeyboardInput, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window,
};

// Public re-exports
pub use winit::dpi::PhysicalSize;
pub use winit::event::{ElementState, MouseButton, VirtualKeyCode};

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
    MouseWheel {
        delta: MouseScrollDelta,
    },
    MouseInput {
        button: MouseButton,
        state: ElementState,
    },
    MouseMotion {
        delta: (f64, f64),
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

    pub fn center_window(&self) {
        if let Some(monitor) = self.window.current_monitor() {
            let screen_size = monitor.size();
            let window_size = self.window.outer_size();

            self.window.set_outer_position(PhysicalPosition {
                x: screen_size.width.saturating_sub(window_size.width) as f64 / 2.
                    + monitor.position().x as f64,
                y: screen_size.height.saturating_sub(window_size.height) as f64 / 2.
                    + monitor.position().y as f64,
            });

            self.window.set_inner_size(PhysicalSize::new(screen_size.width/2, screen_size.height/2));
        }
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn focus_cursor(&self) {
        self.window.set_cursor_grab(window::CursorGrabMode::Locked);
        self.window.set_cursor_visible(false);
    }
    
    /// Runs the window event loop.
    pub fn run(self, mut func: impl 'static + FnMut(&window::Window, WindowEvents)) -> ! {
        let id = self.window.id();

        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::MouseMotion { delta } => {
                        func(&self.window, WindowEvents::MouseMotion { delta })
                    }
                    DeviceEvent::MouseWheel { delta } => {
                        func(&self.window, WindowEvents::MouseWheel { delta })
                    }
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
