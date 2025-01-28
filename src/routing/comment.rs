use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::feature::{comment::Comment, user::User};

use super::middleware::by_authorization_token_then_insert;

#[derive(Debug, Serialize, Deserialize)]
struct CreateComment {
    post_id: i64,
    comment: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateComment {
    comment: String,
}

pub fn route() -> Router {
    Router::new()
        .route(
            "/",
            post(create).route_layer(axum::middleware::from_fn(
                by_authorization_token_then_insert,
            )),
        )
        .route("/{comment_id}", get(read))
        .route(
            "/{comment_id}",
            patch(update).route_layer(axum::middleware::from_fn(
                by_authorization_token_then_insert,
            )),
        )
        .route(
            "/{comment_id}",
            delete(delete_).route_layer(axum::middleware::from_fn(
                by_authorization_token_then_insert,
            )),
        )
        .route("/posts/{post_id}", get(read_all_for_post))
}

async fn create(
    Extension(user): Extension<Arc<User>>,
    Json(create_comment): Json<CreateComment>,
) -> impl IntoResponse {
    match Comment::create(
        &user.user_id,
        &create_comment.post_id,
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

async fn read(Path(comment_id): Path<i64>) -> impl IntoResponse {
    match Comment::read(&comment_id).await {
        Ok(comment) => (StatusCode::OK, Json(serde_json::json!(comment))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    Extension(user): Extension<Arc<User>>,
    Path(comment_id): Path<i64>,
    Json(update_comment): Json<UpdateComment>,
) -> impl IntoResponse {
    match Comment::update(&comment_id, &user.user_id, &update_comment.comment).await {
        Ok(comment) => (StatusCode::ACCEPTED, Json(serde_json::json!(comment))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(
    Extension(user): Extension<Arc<User>>,
    Path(comment_id): Path<i64>,
) -> impl IntoResponse {
    match Comment::delete(&comment_id, &user.user_id).await {
        Ok(comment) => (StatusCode::NO_CONTENT, Json(serde_json::json!(comment))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_post(Path(comment_id): Path<i64>) -> impl IntoResponse {
    match Comment::read_all_for_post(&comment_id).await {
        Ok(comments) => (StatusCode::OK, Json(serde_json::json!(comments))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
