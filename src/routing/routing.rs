use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::{feature::routing::Routing, AppState};

pub fn route(State(app_state): State<AppState>) -> Router<AppState> {
    Router::new()
        .route("/:id", get(read))
        .route("/", get(read_all))
        .with_state(app_state)
}

async fn read(State(app_state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match Routing::read(&id, &app_state.database_connection).await {
        Ok(routing) => (StatusCode::OK, Json(serde_json::json!(routing))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all(State(app_state): State<AppState>) -> impl IntoResponse {
    match Routing::read_all(&app_state.database_connection).await {
        Ok(routings) => (StatusCode::OK, Json(serde_json::json!(routings))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
