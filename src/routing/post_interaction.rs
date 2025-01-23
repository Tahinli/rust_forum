use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::feature::post_interaction::PostInteraction;

#[derive(Debug, Serialize, Deserialize)]
struct CreatePostInteraction {
    pub post_creation_time: DateTime<Utc>,
    pub interaction_id: i64,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdatePostInteraction {
    pub interaction_time: DateTime<Utc>,
    pub post_creation_time: DateTime<Utc>,
    pub interaction_id: i64,
    pub user_id: i64,
}

pub fn route() -> Router {
    Router::new()
        .route("/", post(create))
        .route("/{interaction_time}", get(read))
        .route("/", patch(update))
        .route("/{interaction_time}", delete(delete_))
        .route("/posts/{post_creation_time}", get(read_all_for_post))
}

async fn create(Json(create_post_interaction): Json<CreatePostInteraction>) -> impl IntoResponse {
    match PostInteraction::create(
        &create_post_interaction.post_creation_time,
        &create_post_interaction.user_id,
        &create_post_interaction.interaction_id,
    )
    .await
    {
        Ok(post_interaction) => (
            StatusCode::CREATED,
            Json(serde_json::json!(post_interaction)),
        ),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read(Path(interaction_time): Path<DateTime<Utc>>) -> impl IntoResponse {
    match PostInteraction::read(&interaction_time).await {
        Ok(post_interaction) => (StatusCode::OK, Json(serde_json::json!(post_interaction))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(Json(update_post_interaction): Json<UpdatePostInteraction>) -> impl IntoResponse {
    match PostInteraction::update(
        &update_post_interaction.interaction_time,
        &update_post_interaction.interaction_id,
    )
    .await
    {
        Ok(post_interaction) => (
            StatusCode::ACCEPTED,
            Json(serde_json::json!(post_interaction)),
        ),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(Path(interaction_time): Path<DateTime<Utc>>) -> impl IntoResponse {
    match PostInteraction::delete(&interaction_time).await {
        Ok(post_interaction) => (
            StatusCode::NO_CONTENT,
            Json(serde_json::json!(post_interaction)),
        ),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_post(Path(post_creation_time): Path<DateTime<Utc>>) -> impl IntoResponse {
    match PostInteraction::read_all_for_post(&post_creation_time).await {
        Ok(post_interactions) => (StatusCode::OK, Json(serde_json::json!(post_interactions))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
