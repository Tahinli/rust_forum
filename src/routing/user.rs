use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};

use crate::feature::user::User;

pub fn route() -> Router {
    Router::new().route("/{user_id}", get(read))
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
