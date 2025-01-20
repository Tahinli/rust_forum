use rust_forum::server::start_server;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    start_server().await;
}
