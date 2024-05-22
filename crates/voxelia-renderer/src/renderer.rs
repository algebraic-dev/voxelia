//! Definition of a [Renderer], the renderer contains all the information needed to render something
//! into a window.

use crate::Window;

/// This is the state of the renderer. It stores all the information that is needed to render meshs
/// to the screen
pub struct Renderer {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl Renderer {
    /// Creates a new [Renderer] with a bunch of structures inside of it in order to access the GPU.
    pub async fn new(window: &Window) -> Self {
        let window = &window.window;
        let size = window.inner_size();

        // A context for all the wgpu objects. It's used to create an [Adapter] and a [Surface]
        // And does not need to be keep alive.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: Default::default(),
        });

        // Creates a surface that needs to live at least the same time as the window.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // The adapter is a handle to a physical device on the system. It's used to create a
        // [Device] and a [Queue].
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        // Gets the capabilities of the surface like the formats and the present modes.
        let surface_caps = surface.get_capabilities(&adapter);

        // Gets the format out of srgb if it's available.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        // Configures the surface with the format and the size of the window.
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        // Configures the surface
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
        }
    }

     /// Resizes the surface to a new size.
     pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;

            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.surface.configure(&self.device, &self.config);
        }
    }
}