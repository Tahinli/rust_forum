use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get},
    Extension, Json, Router,
};

use crate::{
    feature::{login::Login, user::User},
    routing::middleware::by_uri_then_insert,
};

pub fn route() -> Router {
    Router::new()
        .route(
            "/users/{user_id}",
            delete(delete_all_for_user).route_layer(axum::middleware::from_fn(by_uri_then_insert)),
        )
        .route(
            "/count/users/{user_id}",
            get(count_all_for_user).route_layer(axum::middleware::from_fn(by_uri_then_insert)),
        )
}

async fn delete_all_for_user(Extension(user): Extension<Arc<User>>) -> impl IntoResponse {
    match Login::delete_all_for_user(&user.user_id).await {
        Ok(logins) => (StatusCode::OK, Json(serde_json::json!(logins))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all_for_user(Extension(user): Extension<Arc<User>>) -> impl IntoResponse {
    match Login::count_all_for_user(&user.user_id).await {
        Ok(login_count) => (StatusCode::OK, Json(serde_json::json!(login_count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
