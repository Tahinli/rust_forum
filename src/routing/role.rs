use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::feature::role::Role;

use super::middleware;

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
        .route_layer(axum::middleware::from_fn(middleware::pass_builder_or_admin))
        .route("/:id", get(read))
        .route_layer(axum::middleware::from_fn(middleware::pass))
        .route("/", patch(update))
        .route_layer(axum::middleware::from_fn(middleware::pass_builder_or_admin))
        .route("/:id", delete(delete_))
        .route_layer(axum::middleware::from_fn(middleware::pass_builder_or_admin))
        .route("/", get(read_all))
        .route_layer(axum::middleware::from_fn(middleware::pass))
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

async fn read(Path(id): Path<i64>) -> impl IntoResponse {
    match Role::read(&id).await {
        Ok(role) => (StatusCode::OK, Json(serde_json::json!(role))),
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

async fn read_all() -> impl IntoResponse {
    match Role::read_all().await {
        Ok(roles) => (StatusCode::OK, Json(serde_json::json!(roles))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
