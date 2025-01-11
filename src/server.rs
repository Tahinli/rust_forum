use tokio::net::TcpListener;

use crate::{AppState, SERVER_CONFIG};

pub async fn start_server(app_state: AppState) {
    let server_config = &SERVER_CONFIG;

    let router = crate::routing::route(
        &server_config.concurrency_limit,
        axum::extract::State(app_state),
    )
    .await;
    let listener = TcpListener::bind(&server_config.address).await.unwrap();
    println!("\n\thttp://{}", server_config.address);
    axum::serve(listener, router).await.unwrap()
}
