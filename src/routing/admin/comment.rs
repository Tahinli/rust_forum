use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::delete, Json, Router,
};

use crate::feature::comment::Comment;

pub fn route() -> Router {
    Router::new().route("/{comment_id}", delete(delete_))
}

async fn delete_(Path(comment_id): Path<i64>) -> impl IntoResponse {
    match Comment::read(&comment_id).await {
        Ok(comment) => match Comment::delete(&comment_id, &comment.user_id).await {
            Ok(comment) => (StatusCode::NO_CONTENT, Json(serde_json::json!(comment))),
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
