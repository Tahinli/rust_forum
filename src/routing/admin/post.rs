use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::delete, Json, Router,
};

use crate::feature::post::Post;

pub fn route() -> Router {
    Router::new().route("/{post_id}", delete(delete_))
}

async fn delete_(Path(post_id): Path<i64>) -> impl IntoResponse {
    match Post::read(&post_id).await {
        Ok(post) => match Post::delete(&post_id, &post.user_id).await {
            Ok(post) => (StatusCode::NO_CONTENT, Json(serde_json::json!(post))),
            Err(err_val) => (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!(err_val.to_string())),
            ),
        },
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
