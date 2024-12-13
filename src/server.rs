use tokio::net::TcpListener;

use crate::{AppState, ServerConfig};

pub async fn start_server(app_state: AppState) {
    let server_config = ServerConfig::default();

    let router = crate::routing::route(axum::extract::State(app_state)).await;
    let listener = TcpListener::bind(&server_config.address).await.unwrap();
    println!("\n\thttp://{}", server_config.address);
    axum::serve(listener, router).await.unwrap()
}
