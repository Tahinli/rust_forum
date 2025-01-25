use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};

use crate::feature::user::User;

use super::middleware;

pub fn route() -> Router {
    Router::new()
        .route("/{user_id}", get(read))
        // todo just for beta I think
        .route_layer(axum::middleware::from_fn(
            middleware::pass_by_authorization_token,
        ))
}

async fn read(Path(user_id): Path<i64>) -> impl IntoResponse {
    match User::read(&user_id).await {
        Ok(user) => (StatusCode::OK, Json(serde_json::json!(user))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
