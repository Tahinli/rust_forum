use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::feature::{post_interaction::PostInteraction, user::User};

use super::middleware::by_authorization_token_then_insert;

#[derive(Debug, Serialize, Deserialize)]
struct CreatePostInteraction {
    pub interaction_id: i64,
}

pub fn route() -> Router {
    Router::new()
        .route(
            "/posts/{post_id}",
            post(create).route_layer(axum::middleware::from_fn(
                by_authorization_token_then_insert,
            )),
        )
        .route(
            "/posts/{post_id}",
            delete(delete_).route_layer(axum::middleware::from_fn(
                by_authorization_token_then_insert,
            )),
        )
        .route("/posts/{post_id}", get(read_all_for_post))
}

async fn create(
    Extension(user): Extension<Arc<User>>,
    Path(post_id): Path<i64>,
    Json(create_post_interaction): Json<CreatePostInteraction>,
) -> impl IntoResponse {
    match PostInteraction::create(
        &post_id,
        &user.user_id,
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

async fn delete_(
    Extension(user): Extension<Arc<User>>,
    Path(post_id): Path<i64>,
) -> impl IntoResponse {
    match PostInteraction::delete(&post_id, &user.user_id).await {
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

async fn read_all_for_post(Path(post_id): Path<i64>) -> impl IntoResponse {
    match PostInteraction::read_all_for_post(&post_id).await {
        Ok(post_interactions) => (StatusCode::OK, Json(serde_json::json!(post_interactions))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
