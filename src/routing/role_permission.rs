use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{feature::role_permission::RolePermission, AppState};

#[derive(Debug, Serialize, Deserialize)]
struct CreateRolePermission {
    pub role_id: i64,
    pub permission_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateRolePermission {
    pub role_id: i64,
    pub permission_id: i64,
}

pub fn route(State(app_state): State<AppState>) -> Router<AppState> {
    Router::new()
        .route("/", post(create))
        .route("/roles/:role_id/permissions/:permission_id", get(read))
        .route("/", patch(update))
        .route(
            "/roles/:role_id/permissions/:permission_id",
            delete(delete_),
        )
        .route("/roles/:role_id", get(read_all_for_role))
        .route("/roles/:role_id", delete(delete_all_for_role))
        .with_state(app_state)
}

async fn create(
    State(app_state): State<AppState>,
    Json(create_role_permission): Json<CreateRolePermission>,
) -> impl IntoResponse {
    match RolePermission::create(
        &create_role_permission.role_id,
        &create_role_permission.permission_id,
        &app_state.database_connection,
    )
    .await
    {
        Ok(role_permission) => (
            StatusCode::CREATED,
            Json(serde_json::json!(role_permission)),
        ),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read(
    State(app_state): State<AppState>,
    Path((role_id, permission_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    match RolePermission::read(&role_id, &permission_id, &app_state.database_connection).await {
        Ok(role_permission) => (StatusCode::OK, Json(serde_json::json!(role_permission))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    State(app_state): State<AppState>,
    Json(update_role): Json<UpdateRolePermission>,
) -> impl IntoResponse {
    match RolePermission::update(
        &update_role.role_id,
        &update_role.permission_id,
        &app_state.database_connection,
    )
    .await
    {
        Ok(role_permission) => (
            StatusCode::ACCEPTED,
            Json(serde_json::json!(role_permission)),
        ),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(
    State(app_state): State<AppState>,
    Path((role_id, permission_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    match RolePermission::delete(&role_id, &permission_id, &app_state.database_connection).await {
        Ok(role_permission) => (
            StatusCode::NO_CONTENT,
            Json(serde_json::json!(role_permission)),
        ),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_role(
    State(app_state): State<AppState>,
    Path(role_id): Path<i64>,
) -> impl IntoResponse {
    match RolePermission::read_all_for_role(&role_id, &app_state.database_connection).await {
        Ok(role_permissions) => (StatusCode::OK, Json(serde_json::json!(role_permissions))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_all_for_role(
    State(app_state): State<AppState>,
    Path(role_id): Path<i64>,
) -> impl IntoResponse {
    match RolePermission::delete_all_for_role(&role_id, &app_state.database_connection).await {
        Ok(role_permissions) => (StatusCode::OK, Json(serde_json::json!(role_permissions))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
