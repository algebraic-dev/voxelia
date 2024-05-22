use std::time::{SystemTime, UNIX_EPOCH};

use specs::{Builder, Join, WorldExt};
use specs::{
    Component, Entities, NullStorage, ReadStorage, System, VecStorage, WriteExpect, WriteStorage,
};
use voxelia_engine::Plugin;
use voxelia_renderer::model::{chunk, Mesh};
use voxelia_renderer::{
    camera::{Camera, Projection},
    globals::Globals,
    model::{Material, MaterialId},
    pass::{phong::PhongPass, Pass},
    renderer::Renderer,
    texture::Texture,
    PhysicalSize, Window, WindowEvents,
};

pub struct RenderInfo {
    renderer: Renderer,
    materials: Vec<Material>,
    globals: Globals,
    pass: PhongPass,
    projection: Projection,
    camera: Camera,
}

impl RenderInfo {
    pub fn update_camera(&mut self) {
        self.globals
            .update_camera(&self.renderer, &self.camera, &self.projection)
    }

    pub fn resize(&mut self) {
        self.pass.resize(&self.renderer)
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct ChunkRenderer {
    data: Mesh,
}

/// Chunk component that stores the information about a chunk.
#[derive(Component)]
#[storage(VecStorage)]
pub struct Chunk {
    data: [[[u8; 4]; 4]; 4],
}

/// Component for things that were created right now
#[derive(Component)]
#[storage(NullStorage)]
pub struct Created;

pub struct RendererSystem;

impl<'a> System<'a> for RendererSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, RenderInfo>,
        ReadStorage<'a, Chunk>,
        WriteStorage<'a, Created>,
        WriteStorage<'a, ChunkRenderer>,
    );

    fn run(&mut self, (entities, info, chunk, mut created, mut renders): Self::SystemData) {
        let entities_to_remove: Vec<_> = (&entities, &chunk, &created)
            .join()
            .map(|(entity, chunk, _)| (entity, chunk))
            .collect();

        for (entity, chunk) in entities_to_remove {
            created.remove(entity);
            let data = chunk::to_mesh(chunk.data, MaterialId(0), &info.renderer);
            renders.insert(entity, ChunkRenderer { data }).unwrap();
        }

        let meshes = renders.join().map(|x| &x.data).collect::<Vec<_>>();

        info.pass
            .draw(&info.renderer, &info.materials, &meshes, &info.globals)
            .unwrap();
    }
}

pub struct RendererPlugin {
    info: RenderInfo,
}

impl Plugin for RendererPlugin {
    fn setup(self, world: &mut voxelia_engine::WorldBuilder) {
        world.with_component::<Chunk>();
        world.with_component::<Created>();
        world.with_component::<ChunkRenderer>();

        world.with_resource(self.info);
        world.with_system(RendererSystem, "renderer system", &[])
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let window = Window::new("Voxelia", PhysicalSize::new(500, 500));

    let renderer = Renderer::new(&window).await;
    let globals = Globals::new(&renderer);
    let phong = PhongPass::new(&renderer, &globals);

    let texture = Texture::from_bytes(&renderer, include_bytes!("b.jpeg"), "Bulacha").unwrap();
    let material = Material::from_texture(&renderer, texture, &phong.texture_bind_group_layout);


    let projection = Projection::new(renderer.size);
    let camera = Camera::new(
        (20.0, 20.0 as f32, 25.0 as f32),
        cgmath::Deg(-90.0 - 30.0),
        cgmath::Deg(-30.0),
    );

    let mut info = RenderInfo {
        renderer,
        materials: vec![material],
        globals,
        pass: phong,
        projection,
        camera,
    };

    info.update_camera();

    let mut engine = voxelia_engine::Builder::new().with(RendererPlugin { info }).build();

    engine.world.create_entity().with(Chunk {
        data: [
            [[1, 0, 0 ,1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]],
            [[1, 0, 0 ,1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]],
            [[1, 0, 0 ,1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]],
            [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]],
        ],
    }).with(Created).build();

    window.run(move |_window, event| match event {
        WindowEvents::Keyboard { .. } => {}
        WindowEvents::Resized(size) => {
            let mut info = engine.world.write_resource::<RenderInfo>();
            info.projection.aspect =
                info.renderer.config.width as f32 / info.renderer.config.height as f32;
            info.renderer.resize(size);
            info.resize();
            info.update_camera();
        }
        WindowEvents::Draw => engine.run(),
    })
}
