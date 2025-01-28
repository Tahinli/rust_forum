use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::feature::contact::Contact;

#[derive(Debug, Serialize, Deserialize)]
struct CreateContact {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateContact {
    name: String,
}

pub fn route() -> Router {
    Router::new()
        .route("/", post(create))
        .route("/{contact_id}", patch(update))
        .route("/{contact_id}", delete(delete_))
}

async fn create(Json(create_contact): Json<CreateContact>) -> impl IntoResponse {
    match Contact::create(&create_contact.name).await {
        Ok(contact) => (StatusCode::CREATED, Json(serde_json::json!(contact))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    Path(contact_id): Path<i64>,
    Json(update_contact): Json<UpdateContact>,
) -> impl IntoResponse {
    match Contact::update(&contact_id, &update_contact.name).await {
        Ok(contact) => (StatusCode::ACCEPTED, Json(serde_json::json!(contact))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(Path(contact_id): Path<i64>) -> impl IntoResponse {
    match Contact::delete(&contact_id).await {
        Ok(contact) => (StatusCode::NO_CONTENT, Json(serde_json::json!(contact))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
