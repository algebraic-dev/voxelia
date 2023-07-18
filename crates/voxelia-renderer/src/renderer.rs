//! The renderer module that constains the [Renderer] struct that is responsible for rendering
//! meshes to the screen.

use crate::{pass_data::Globals, renderable::Renderable, texture};

/// This is the state of the renderer. It stores all the information that is needed to render meshs
/// to the screen
pub struct Renderer {
    /// The window that we are rendering to.
    pub window: winit::window::Window,

    /// The plataform-specific part of the project that interfaces with the windowing system
    pub surface: wgpu::Surface,

    /// A GPU Device connection
    pub device: wgpu::Device,

    /// The buffered command queue.
    pub queue: wgpu::Queue,

    /// The configuration of the surface like it's size.
    pub config: wgpu::SurfaceConfiguration,

    /// The size of the window
    pub size: winit::dpi::PhysicalSize<u32>,

    /// Depth texture
    pub depth_texture: texture::Texture,
}

impl Renderer {
    pub async fn new(window: winit::window::Window) -> Self {
        let size = window.inner_size();

        // A context for all the wgpu objects. It's used to create an [Adapter] and a [Surface]
        // And does not need to be keep alive.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
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

        // Creates a depth texture.
        let depth_texture = texture::Texture::create_depth_texture(&device, &config, "Depth");

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            depth_texture,
        }
    }

    /// Resizes the window to a new size and reconfigures the surface.
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;

            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.surface.configure(&self.device, &self.config);

            //self.camera.aspect = self.config.width as f32 / self.config.height as f32;

            self.depth_texture =
                texture::Texture::create_depth_texture(&self.device, &self.config, "Depth");
        }
    }

    pub fn render<Obj: Renderable>(
        &mut self,
        renderable: &Obj,
        globals: &Globals,
    ) -> Result<(), wgpu::SurfaceError> {
        // Gives a surface to create a new frame of.
        let output = self.surface.get_current_texture()?;

        // Describes a new texture view so it can handle textures from the surface.
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create a new encoder so we can just send commands to the GPU in a queue.
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            renderable.render(&mut render_pass, globals);
        }

        // Submits the commands to the GPU.
        self.queue.submit(std::iter::once(encoder.finish()));

        // Presents the frame to the screen.
        output.present();

        Ok(())
    }
}
