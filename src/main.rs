use rust_forum::server::start_server;
use tracing::Level;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    start_server().await;
}
