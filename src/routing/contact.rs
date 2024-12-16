use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{feature::contact::Contact, AppState};

#[derive(Debug, Serialize, Deserialize)]
struct CreateContact {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateContact {
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
    Json(create_contact): Json<CreateContact>,
) -> impl IntoResponse {
    match Contact::create(&create_contact.name, &app_state.database_connection).await {
        Ok(contact) => (StatusCode::CREATED, Json(serde_json::json!(contact))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read(State(app_state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match Contact::read(&id, &app_state.database_connection).await {
        Ok(contact) => (StatusCode::OK, Json(serde_json::json!(contact))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    State(app_state): State<AppState>,
    Json(update_contact): Json<UpdateContact>,
) -> impl IntoResponse {
    match Contact::update(
        &update_contact.id,
        &update_contact.name,
        &app_state.database_connection,
    )
    .await
    {
        Ok(contact) => (StatusCode::ACCEPTED, Json(serde_json::json!(contact))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(State(app_state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match Contact::delete(&id, &app_state.database_connection).await {
        Ok(contact) => (StatusCode::NO_CONTENT, Json(serde_json::json!(contact))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all(State(app_state): State<AppState>) -> impl IntoResponse {
    match Contact::read_all(&app_state.database_connection).await {
        Ok(contacts) => (StatusCode::OK, Json(serde_json::json!(contacts))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
