use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};

use crate::feature::role::Role;

pub fn route() -> Router {
    Router::new()
        .route("/{role_id}", get(read))
        .route("/", get(read_all))
}

async fn read(Path(role_id): Path<i64>) -> impl IntoResponse {
    match Role::read(&role_id).await {
        Ok(role) => (StatusCode::OK, Json(serde_json::json!(role))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all() -> impl IntoResponse {
    match Role::read_all().await {
        Ok(roles) => (StatusCode::OK, Json(serde_json::json!(roles))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
