use tokio::net::TcpListener;

use crate::SERVER_CONFIG;

pub async fn start_server() {
    let router = crate::routing::route(&SERVER_CONFIG.concurrency_limit).await;
    let listener = TcpListener::bind(&SERVER_CONFIG.address).await.unwrap();

    println!("\n\thttp://{}", &SERVER_CONFIG.address);
    axum::serve(listener, router).await.unwrap()
}
