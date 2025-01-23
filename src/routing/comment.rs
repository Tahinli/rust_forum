use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::feature::comment::Comment;

#[derive(Debug, Serialize, Deserialize)]
struct CreateComment {
    post_creation_time: DateTime<Utc>,
    user_id: i64,
    comment: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateComment {
    creation_time: DateTime<Utc>,
    comment: String,
}

pub fn route() -> Router {
    Router::new()
        .route("/", post(create))
        .route("/{creation_time}", get(read))
        .route("/", patch(update))
        .route("/{creation_time}", delete(delete_))
        .route("/posts/{post_creation_time}", get(read_all_for_post))
}

async fn create(Json(create_comment): Json<CreateComment>) -> impl IntoResponse {
    match Comment::create(
        &create_comment.post_creation_time,
        &create_comment.user_id,
        &create_comment.comment,
    )
    .await
    {
        Ok(comment) => (StatusCode::CREATED, Json(serde_json::json!(comment))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read(Path(creation_time): Path<DateTime<Utc>>) -> impl IntoResponse {
    match Comment::read(&creation_time).await {
        Ok(comment) => (StatusCode::OK, Json(serde_json::json!(comment))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(Json(update_comment): Json<UpdateComment>) -> impl IntoResponse {
    match Comment::update(&update_comment.creation_time, &update_comment.comment).await {
        Ok(comment) => (StatusCode::ACCEPTED, Json(serde_json::json!(comment))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(Path(creation_time): Path<DateTime<Utc>>) -> impl IntoResponse {
    match Comment::delete(&creation_time).await {
        Ok(comment) => (StatusCode::NO_CONTENT, Json(serde_json::json!(comment))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_post(Path(post_creation_time): Path<DateTime<Utc>>) -> impl IntoResponse {
    match Comment::read_all_for_post(&post_creation_time).await {
        Ok(comments) => (StatusCode::OK, Json(serde_json::json!(comments))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
