use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::feature::{post::Post, user::User};

use super::middleware::{by_authorization_token_then_insert, by_uri_then_insert};

#[derive(Debug, Serialize, Deserialize)]
struct CreatePost {
    post: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdatePost {
    post: String,
}

pub fn route() -> Router {
    Router::new()
        .route(
            "/",
            post(create).route_layer(axum::middleware::from_fn(
                by_authorization_token_then_insert,
            )),
        )
        .route("/{post_id}", get(read))
        .route("/{post_id}", patch(update))
        .route_layer(axum::middleware::from_fn(
            by_authorization_token_then_insert,
        ))
        .route(
            "/{post_id}",
            delete(delete_).route_layer(axum::middleware::from_fn(
                by_authorization_token_then_insert,
            )),
        )
        .route("/", get(read_all))
        .route(
            "/users/{user_id}",
            get(read_all_for_user).route_layer(axum::middleware::from_fn(by_uri_then_insert)),
        )
}

async fn create(
    Extension(user): Extension<Arc<User>>,
    Json(create_post): Json<CreatePost>,
) -> impl IntoResponse {
    match Post::create(&user.user_id, &create_post.post).await {
        Ok(post) => (StatusCode::CREATED, Json(serde_json::json!(post))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read(Path(post_id): Path<i64>) -> impl IntoResponse {
    match Post::read(&post_id).await {
        Ok(post) => (StatusCode::OK, Json(serde_json::json!(post))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    Extension(user): Extension<Arc<User>>,
    Path(post_id): Path<i64>,
    Json(update_post): Json<UpdatePost>,
) -> impl IntoResponse {
    match Post::update(&post_id, &user.user_id, &update_post.post).await {
        Ok(post) => (StatusCode::ACCEPTED, Json(serde_json::json!(post))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(
    Extension(user): Extension<Arc<User>>,
    Path(post_id): Path<i64>,
) -> impl IntoResponse {
    match Post::delete(&post_id, &user.user_id).await {
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

async fn read_all_for_user(Extension(user): Extension<Arc<User>>) -> impl IntoResponse {
    match Post::read_all_for_user(&user.user_id).await {
        Ok(posts) => (StatusCode::OK, Json(serde_json::json!(posts))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
