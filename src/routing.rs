pub mod role;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use tower_http::cors::CorsLayer;

use crate::{database, AppState};

pub async fn route(State(app_state): State<AppState>) -> Router {
    Router::new()
        .route("/", get(alive))
        .nest(
            "/role",
            role::route(axum::extract::State(app_state.clone())),
        )
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}

async fn alive(State(app_state): State<AppState>) -> impl IntoResponse {
    match database::is_alive(&app_state.database_connection).await {
        true => StatusCode::OK,
        false => StatusCode::SERVICE_UNAVAILABLE,
    }
}
