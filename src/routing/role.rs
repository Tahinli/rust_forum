use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};

use crate::{feature::role::Role, AppState};

pub fn route(State(app_state): State<AppState>) -> Router<AppState> {
    Router::new()
        .route("/:name", post(create))
        .route("/:id", get(read))
        .route("/:id/:name", patch(update))
        .route("/:id", delete(_delete))
        .route("/", get(read_all))
        .with_state(app_state)
}

async fn create(Path(name): Path<String>, State(app_state): State<AppState>) -> impl IntoResponse {
    match Role::create(&name, &app_state.database_connection).await {
        Ok(role) => (StatusCode::CREATED, Json(serde_json::json!(role))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read(Path(id): Path<i64>, State(app_state): State<AppState>) -> impl IntoResponse {
    match Role::read(&id, &app_state.database_connection).await {
        Ok(role) => (StatusCode::OK, Json(serde_json::json!(role))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    Path((id, name)): Path<(i64, String)>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    match Role::update(&id, &name, &app_state.database_connection).await {
        Ok(role) => (StatusCode::ACCEPTED, Json(serde_json::json!(role))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn _delete(Path(id): Path<i64>, State(app_state): State<AppState>) -> impl IntoResponse {
    match Role::delete(&id, &app_state.database_connection).await {
        Ok(role) => (StatusCode::NO_CONTENT, Json(serde_json::json!(role))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all(State(app_state): State<AppState>) -> impl IntoResponse {
    match Role::read_all(&app_state.database_connection).await {
        Ok(roles) => (StatusCode::OK, Json(serde_json::json!(roles))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
