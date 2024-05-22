use voxelia_renderer::{pass::{phong::PhongPass, Pass}, renderer::Renderer, PhysicalSize, Window, WindowEvents};

#[tokio::main]
async fn main() {
    env_logger::init();

    let window = Window::new("Voxelia", PhysicalSize::new(500, 500));

    let mut renderer = Renderer::new(&window).await;
    let mut phong = PhongPass::new(&renderer);

    window.run(move |_window, event| {
        match event {
            WindowEvents::Keyboard { .. } => {}
            WindowEvents::Resized(size) => {
                renderer.resize(size);
                phong.resize(&renderer);
            }
            WindowEvents::Draw => {
                phong.draw(&renderer).unwrap();
            }
        }
    })
}