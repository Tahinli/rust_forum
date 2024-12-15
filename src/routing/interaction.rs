use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{feature::interaction::Interaction, AppState};

#[derive(Debug, Serialize, Deserialize)]
struct CreateInteraction {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateInteraction {
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
    Json(create_interaction): Json<CreateInteraction>,
) -> impl IntoResponse {
    match Interaction::create(&create_interaction.name, &app_state.database_connection).await {
        Ok(interaction) => (StatusCode::CREATED, Json(serde_json::json!(interaction))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read(State(app_state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match Interaction::read(&id, &app_state.database_connection).await {
        Ok(interaction) => (StatusCode::OK, Json(serde_json::json!(interaction))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    State(app_state): State<AppState>,
    Json(update_interaction): Json<UpdateInteraction>,
) -> impl IntoResponse {
    match Interaction::update(
        &update_interaction.id,
        &update_interaction.name,
        &app_state.database_connection,
    )
    .await
    {
        Ok(interaction) => (StatusCode::ACCEPTED, Json(serde_json::json!(interaction))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(State(app_state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match Interaction::delete(&id, &app_state.database_connection).await {
        Ok(interaction) => (StatusCode::NO_CONTENT, Json(serde_json::json!(interaction))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all(State(app_state): State<AppState>) -> impl IntoResponse {
    match Interaction::read_all(&app_state.database_connection).await {
        Ok(interactions) => (StatusCode::OK, Json(serde_json::json!(interactions))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
