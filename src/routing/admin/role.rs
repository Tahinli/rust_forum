use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::feature::role::Role;

#[derive(Debug, Serialize, Deserialize)]
struct UpdateRole {
    name: String,
}

pub fn route() -> Router {
    Router::new()
        .route("/{role_id}", patch(update))
        .route("/{role_id}", delete(delete_))
}

async fn update(
    Path(role_id): Path<i64>,
    Json(update_role): Json<UpdateRole>,
) -> impl IntoResponse {
    match Role::update(&role_id, &update_role.name).await {
        Ok(role) => (StatusCode::ACCEPTED, Json(serde_json::json!(role))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(Path(role_id): Path<i64>) -> impl IntoResponse {
    match Role::delete(&role_id).await {
        Ok(role) => (StatusCode::NO_CONTENT, Json(serde_json::json!(role))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
