use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::feature::post::Post;

#[derive(Debug, Serialize, Deserialize)]
struct CreatePost {
    user_id: i64,
    post: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdatePost {
    creation_time: DateTime<Utc>,
    user_id: i64,
    post: String,
}

pub fn route() -> Router {
    Router::new()
        .route("/", post(create))
        .route("/{creation_time}", get(read))
        .route("/", patch(update))
        .route("/{creation_time}", delete(delete_))
        .route("/", get(read_all))
}

async fn create(Json(create_post): Json<CreatePost>) -> impl IntoResponse {
    match Post::create(&create_post.user_id, &create_post.post).await {
        Ok(post) => (StatusCode::CREATED, Json(serde_json::json!(post))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read(Path(creation_time): Path<DateTime<Utc>>) -> impl IntoResponse {
    match Post::read(&creation_time).await {
        Ok(post) => (StatusCode::OK, Json(serde_json::json!(post))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(Json(update_role): Json<UpdatePost>) -> impl IntoResponse {
    match Post::update(
        &update_role.creation_time,
        &update_role.user_id,
        &update_role.post,
    )
    .await
    {
        Ok(post) => (StatusCode::ACCEPTED, Json(serde_json::json!(post))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(Path(creation_time): Path<DateTime<Utc>>) -> impl IntoResponse {
    match Post::delete(&creation_time).await {
        Ok(post) => (StatusCode::NO_CONTENT, Json(serde_json::json!(post))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all() -> impl IntoResponse {
    match Post::read_all().await {
        Ok(posts) => (StatusCode::OK, Json(serde_json::json!(posts))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
