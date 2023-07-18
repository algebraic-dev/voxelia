mod instance;

#[tokio::main]
async fn main() {
    voxelia_renderer::run().await
}
