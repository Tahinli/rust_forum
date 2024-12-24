use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{feature::routing_permission::RoutingPermission, AppState};

#[derive(Debug, Serialize, Deserialize)]
struct CreateRoutingPermission {
    pub routing_id: i64,
    pub permission_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateRoutingPermission {
    pub routing_id: i64,
    pub permission_id: i64,
}

pub fn route(State(app_state): State<AppState>) -> Router<AppState> {
    Router::new()
        .route("/", post(create))
        .route(
            "/routings/:routing_id/permissions/:permission_id",
            get(read),
        )
        .route("/", patch(update))
        .route(
            "/routings/:routing_id/permissions/:permission_id",
            delete(delete_),
        )
        .route("/routings/:routing_id", get(read_all_for_routing))
        .route("/routings/:routing_id", delete(delete_all_for_routing))
        .with_state(app_state)
}

async fn create(
    State(app_state): State<AppState>,
    Json(create_role_permission): Json<CreateRoutingPermission>,
) -> impl IntoResponse {
    match RoutingPermission::create(
        &create_role_permission.routing_id,
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
    Path((routing_id, permission_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    match RoutingPermission::read(&routing_id, &permission_id, &app_state.database_connection).await
    {
        Ok(role_permission) => (StatusCode::OK, Json(serde_json::json!(role_permission))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    State(app_state): State<AppState>,
    Json(update_role): Json<UpdateRoutingPermission>,
) -> impl IntoResponse {
    match RoutingPermission::update(
        &update_role.routing_id,
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
    Path((routing_id, permission_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    match RoutingPermission::delete(&routing_id, &permission_id, &app_state.database_connection)
        .await
    {
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

async fn read_all_for_routing(
    State(app_state): State<AppState>,
    Path(routing_id): Path<i64>,
) -> impl IntoResponse {
    match RoutingPermission::read_all_for_routing(&routing_id, &app_state.database_connection).await
    {
        Ok(role_permissions) => (StatusCode::OK, Json(serde_json::json!(role_permissions))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_all_for_routing(
    State(app_state): State<AppState>,
    Path(routing_id): Path<i64>,
) -> impl IntoResponse {
    match RoutingPermission::delete_all_for_routing(&routing_id, &app_state.database_connection)
        .await
    {
        Ok(role_permissions) => (StatusCode::OK, Json(serde_json::json!(role_permissions))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
