use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::feature::role::Role;

#[derive(Debug, Serialize, Deserialize)]
struct CreateRole {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateRole {
    id: i64,
    name: String,
}

pub fn route() -> Router {
    Router::new()
        .route("/", post(create))
        .route("/", patch(update))
        .route("/{id}", delete(delete_))
}

async fn create(Json(create_role): Json<CreateRole>) -> impl IntoResponse {
    match Role::create(&create_role.name).await {
        Ok(role) => (StatusCode::CREATED, Json(serde_json::json!(role))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(Json(update_role): Json<UpdateRole>) -> impl IntoResponse {
    match Role::update(&update_role.id, &update_role.name).await {
        Ok(role) => (StatusCode::ACCEPTED, Json(serde_json::json!(role))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(Path(id): Path<i64>) -> impl IntoResponse {
    match Role::delete(&id).await {
        Ok(role) => (StatusCode::NO_CONTENT, Json(serde_json::json!(role))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
