use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::feature::comment_interaction::CommentInteraction;

#[derive(Debug, Serialize, Deserialize)]
struct CreateCommentInteraction {
    pub comment_creation_time: DateTime<Utc>,
    pub interaction_id: i64,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateCommentInteraction {
    pub interaction_time: DateTime<Utc>,
    pub comment_creation_time: DateTime<Utc>,
    pub interaction_id: i64,
    pub user_id: i64,
}

pub fn route() -> Router {
    Router::new()
        .route("/", post(create))
        .route("/{interaction_time}", get(read))
        .route("/", patch(update))
        .route("/{interaction_time}", delete(delete_))
        .route(
            "/comments/{comment_creation_time}",
            get(read_all_for_comment),
        )
}

async fn create(
    Json(create_comment_interaction): Json<CreateCommentInteraction>,
) -> impl IntoResponse {
    match CommentInteraction::create(
        &create_comment_interaction.comment_creation_time,
        &create_comment_interaction.user_id,
        &create_comment_interaction.interaction_id,
    )
    .await
    {
        Ok(comment_interaction) => (
            StatusCode::CREATED,
            Json(serde_json::json!(comment_interaction)),
        ),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read(Path(interaction_time): Path<DateTime<Utc>>) -> impl IntoResponse {
    match CommentInteraction::read(&interaction_time).await {
        Ok(comment_interaction) => (StatusCode::OK, Json(serde_json::json!(comment_interaction))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    Json(update_comment_interaction): Json<UpdateCommentInteraction>,
) -> impl IntoResponse {
    match CommentInteraction::update(
        &update_comment_interaction.interaction_time,
        &update_comment_interaction.interaction_id,
    )
    .await
    {
        Ok(comment_interaction) => (
            StatusCode::ACCEPTED,
            Json(serde_json::json!(comment_interaction)),
        ),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(Path(interaction_time): Path<DateTime<Utc>>) -> impl IntoResponse {
    match CommentInteraction::delete(&interaction_time).await {
        Ok(comment_interaction) => (
            StatusCode::NO_CONTENT,
            Json(serde_json::json!(comment_interaction)),
        ),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_comment(
    Path(comment_creation_time): Path<DateTime<Utc>>,
) -> impl IntoResponse {
    match CommentInteraction::read_all_for_comment(&comment_creation_time).await {
        Ok(comment_interactions) => (
            StatusCode::OK,
            Json(serde_json::json!(comment_interactions)),
        ),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
