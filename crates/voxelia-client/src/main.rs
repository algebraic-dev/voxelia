use specs::{Builder, WorldExt};
use voxelia_client::chunk::{Chunk, Created};
use voxelia_client::graphics::Graphics;
use voxelia_client::RendererPlugin;

use voxelia_renderer::{PhysicalSize, Window, WindowEvents};

#[tokio::main]
async fn main() {
    env_logger::init();

    let window = Window::new("Voxelia", PhysicalSize::new(500, 500));
    let info = Graphics::new(&window).await;

    let mut engine = voxelia_engine::Builder::new()
        .with(RendererPlugin { info })
        .build();

    engine
        .world
        .create_entity()
        .with(Chunk {
            data: [
                [[1, 0, 0, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]],
                [[1, 0, 0, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]],
                [[1, 0, 0, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]],
                [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]],
            ],
        })
        .with(Created)
        .build();

    window.run(move |_window, event| match event {
        WindowEvents::Keyboard { .. } => {}
        WindowEvents::Resized(size) => engine.world.write_resource::<Graphics>().resize(size),
        WindowEvents::Draw => engine.run(),
    })
}
