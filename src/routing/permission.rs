use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{feature::permission::Permission, AppState};

#[derive(Debug, Serialize, Deserialize)]
struct CreatePermission {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdatePermission {
    id: i64,
    name: String,
}

pub fn route(State(app_state): State<AppState>) -> Router<AppState> {
    Router::new()
        .route("/", post(create))
        .route("/:id", get(read))
        .route("/", patch(update))
        .route("/:id", delete(delete_))
        .route("/", get(read_all))
        .with_state(app_state)
}

async fn create(
    State(app_state): State<AppState>,
    Json(create_permission): Json<CreatePermission>,
) -> impl IntoResponse {
    match Permission::create(&create_permission.name, &app_state.database_connection).await {
        Ok(permission) => (StatusCode::CREATED, Json(serde_json::json!(permission))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read(State(app_state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match Permission::read(&id, &app_state.database_connection).await {
        Ok(permission) => (StatusCode::OK, Json(serde_json::json!(permission))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    State(app_state): State<AppState>,
    Json(update_permission): Json<UpdatePermission>,
) -> impl IntoResponse {
    match Permission::update(
        &update_permission.id,
        &update_permission.name,
        &app_state.database_connection,
    )
    .await
    {
        Ok(permission) => (StatusCode::ACCEPTED, Json(serde_json::json!(permission))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(State(app_state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match Permission::delete(&id, &app_state.database_connection).await {
        Ok(permission) => (StatusCode::NO_CONTENT, Json(serde_json::json!(permission))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all(State(app_state): State<AppState>) -> impl IntoResponse {
    match Permission::read_all(&app_state.database_connection).await {
        Ok(permissions) => (StatusCode::OK, Json(serde_json::json!(permissions))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
