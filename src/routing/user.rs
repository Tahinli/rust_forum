use std::sync::Arc;

use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router,
};

use crate::feature::user::User;

use super::middleware::by_authorization_token_then_insert;

pub fn route() -> Router {
    Router::new()
        .route(
            "/",
            get(read).route_layer(axum::middleware::from_fn(
                by_authorization_token_then_insert,
            )),
        )
        .route("/{user_id}", get(read_anybody))
}

async fn read(Extension(user): Extension<Arc<User>>) -> impl IntoResponse {
    match User::read(&user.user_id).await {
        Ok(user) => (StatusCode::OK, Json(serde_json::json!(user))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_anybody(Path(user_id): Path<i64>) -> impl IntoResponse {
    match User::read(&user_id).await {
        Ok(user) => (StatusCode::OK, Json(serde_json::json!(user))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
