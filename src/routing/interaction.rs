use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use crate::feature::interaction::Interaction;

#[derive(Debug, Serialize, Deserialize)]
struct CreateInteraction {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateInteraction {
    id: i64,
    name: String,
}

pub fn route() -> Router {
    Router::new()
        .route("/{id}", get(read))
        .route("/", get(read_all))
}

async fn read(Path(id): Path<i64>) -> impl IntoResponse {
    match Interaction::read(&id).await {
        Ok(interaction) => (StatusCode::OK, Json(serde_json::json!(interaction))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all() -> impl IntoResponse {
    match Interaction::read_all().await {
        Ok(interactions) => (StatusCode::OK, Json(serde_json::json!(interactions))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
