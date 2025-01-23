use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
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
    id: i64,
    name: String,
}

pub fn route() -> Router {
    Router::new()
        .route("/", post(create))
        .route("/{id}", get(read))
        .route("/", patch(update))
        .route("/{id}", delete(delete_))
        .route("/", get(read_all))
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

async fn read(Path(id): Path<i64>) -> impl IntoResponse {
    match Contact::read(&id).await {
        Ok(contact) => (StatusCode::OK, Json(serde_json::json!(contact))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(Json(update_contact): Json<UpdateContact>) -> impl IntoResponse {
    match Contact::update(&update_contact.id, &update_contact.name).await {
        Ok(contact) => (StatusCode::ACCEPTED, Json(serde_json::json!(contact))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(Path(id): Path<i64>) -> impl IntoResponse {
    match Contact::delete(&id).await {
        Ok(contact) => (StatusCode::NO_CONTENT, Json(serde_json::json!(contact))),
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
