use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};

use crate::feature::contact::Contact;

pub fn route() -> Router {
    Router::new()
        .route("/{id}", get(read))
        .route("/", get(read_all))
}

async fn read(Path(id): Path<i64>) -> impl IntoResponse {
    match Contact::read(&id).await {
        Ok(contact) => (StatusCode::OK, Json(serde_json::json!(contact))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all() -> impl IntoResponse {
    match Contact::read_all().await {
        Ok(contacts) => (StatusCode::OK, Json(serde_json::json!(contacts))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
