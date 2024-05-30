use specs::{Builder, WorldExt};

use voxelia_client::structures::graphics::Graphics;
use voxelia_client::RendererPlugin;

use voxelia_engine::{
    chunk::{Chunk, ChunkPlugin},
    events::{Created, EventsPlugin},
    BasicPlugin, Engine, Position,
};

use voxelia_renderer::{
    model::Material, texture::Texture, window::ElementState, window::MouseButton, PhysicalSize,
    Window, WindowEvents,
};

/// Loads all the resources that are needed to run the game
async fn load(graphics: &mut Graphics) {
    let texture =
        Texture::from_bytes(&graphics.renderer, include_bytes!("../../../assets/b.jpeg"), "Bulacha").unwrap();

    let material = Material::from_texture(
        &graphics.renderer,
        texture,
        &graphics.pass.texture_bind_group_layout,
    );

    graphics.add_material(material);
}

/// Starts all the things in the engine
async fn start_engine<'a, 'b>(engine: &mut Engine<'a, 'b>) {
    engine
        .world
        .create_entity()
        .with(Position::new(0.0, 0.0, 0.0))
        .with(Chunk {
            data: [
                1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            ],
        })
        .with(Created)
        .build();
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let window = Window::new("Voxelia", PhysicalSize::new(500, 500));
    let mut graphics = Graphics::new(&window).await;

    load(&mut graphics).await;

    window.center_window();
    window.focus_cursor();

    graphics.resize(window.size());

    let mut engine = voxelia_engine::Builder::new()
        .with(BasicPlugin)
        .with(EventsPlugin)
        .with(ChunkPlugin)
        .with(RendererPlugin { graphics })
        .build();

    start_engine(&mut engine).await;

    window.run(move |_window, event| match event {
        WindowEvents::Keyboard {
            state,
            virtual_keycode: Some(key),
        } => {
            let mut graphics = engine.world.write_resource::<Graphics>();
            graphics.camera_controller.process_keyboard(key, state);
        }
        WindowEvents::MouseWheel { delta } => {
            let mut graphics = engine.world.write_resource::<Graphics>();
            graphics.camera_controller.process_scroll(&delta);
        }
        WindowEvents::MouseInput {
            button: MouseButton::Left,
            state,
            ..
        } => {
            let mut graphics = engine.world.write_resource::<Graphics>();
            graphics.camera_controller.mouse_pressed = state == ElementState::Pressed;
        }
        WindowEvents::MouseMotion { delta } => {
            let mut graphics = engine.world.write_resource::<Graphics>();
            graphics.camera_controller.process_mouse(delta.0, delta.1)
        }
        WindowEvents::Resized(size) => engine.world.write_resource::<Graphics>().resize(size),
        WindowEvents::Draw => engine.run(),
        _ => (),
    })
}
