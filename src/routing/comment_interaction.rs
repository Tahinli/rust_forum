use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::feature::{comment_interaction::CommentInteraction, user::User};

use super::middleware::by_authorization_token_then_insert;

#[derive(Debug, Serialize, Deserialize)]
struct CreateCommentInteraction {
    pub interaction_id: i64,
}

pub fn route() -> Router {
    Router::new()
        .route(
            "/comments/{comment_id}",
            post(create).route_layer(axum::middleware::from_fn(
                by_authorization_token_then_insert,
            )),
        )
        .route(
            "/comments/{comment_id}",
            delete(delete_).route_layer(axum::middleware::from_fn(
                by_authorization_token_then_insert,
            )),
        )
        .route("/comments/{comment_id}", get(read_all_for_comment))
}

async fn create(
    Extension(user): Extension<Arc<User>>,
    Path(comment_id): Path<i64>,
    Json(create_comment_interaction): Json<CreateCommentInteraction>,
) -> impl IntoResponse {
    match CommentInteraction::create(
        &comment_id,
        &user.user_id,
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

async fn delete_(
    Extension(user): Extension<Arc<User>>,
    Path(comment_id): Path<i64>,
) -> impl IntoResponse {
    match CommentInteraction::delete(&comment_id, &user.user_id).await {
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

async fn read_all_for_comment(Path(comment_id): Path<i64>) -> impl IntoResponse {
    match CommentInteraction::read_all_for_comment(&comment_id).await {
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
