use std::sync::Arc;

use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::delete, Extension, Json,
    Router,
};
use chrono::{DateTime, Utc};

use crate::{
    feature::{post::Post, user::User},
    routing::middleware::by_uri_then_insert,
};

pub fn route() -> Router {
    Router::new().route(
        "/users/{user_id}/creation_times/{creation_time}",
        delete(delete_).route_layer(axum::middleware::from_fn(by_uri_then_insert)),
    )
}

async fn delete_(
    Extension(user): Extension<Arc<User>>,
    Path(creation_time): Path<DateTime<Utc>>,
) -> impl IntoResponse {
    match Post::delete(&user.user_id, &creation_time).await {
        Ok(post) => (StatusCode::NO_CONTENT, Json(serde_json::json!(post))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
