use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    feature::{user::User, user_contact::UserContact},
    routing::middleware::by_uri_then_insert,
};

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserContact {
    pub contact_id: i64,
    pub contact_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateUserContact {
    pub contact_id: i64,
    pub contact_value: String,
}

pub fn route() -> Router {
    Router::new()
        .route(
            "/users/{user_id}",
            post(create).route_layer(axum::middleware::from_fn(by_uri_then_insert)),
        )
        .route(
            "/users/{user_id}/contacts/{contact_id}",
            get(read).route_layer(axum::middleware::from_fn(by_uri_then_insert)),
        )
        .route(
            "/users/{user_id}",
            patch(update).route_layer(axum::middleware::from_fn(by_uri_then_insert)),
        )
        .route(
            "/users/{user_id}/contacts/{contact_id}",
            delete(delete_).route_layer(axum::middleware::from_fn(by_uri_then_insert)),
        )
        .route(
            "/users/{user_id}",
            delete(delete_all_for_user).route_layer(axum::middleware::from_fn(by_uri_then_insert)),
        )
}

async fn create(
    Extension(user): Extension<Arc<User>>,
    Json(create_user_contact): Json<CreateUserContact>,
) -> impl IntoResponse {
    match UserContact::create(
        &user,
        &create_user_contact.contact_id,
        &create_user_contact.contact_value,
    )
    .await
    {
        Ok(user_contact) => (StatusCode::CREATED, Json(serde_json::json!(user_contact))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read(
    Extension(user): Extension<Arc<User>>,
    Path(contact_id): Path<i64>,
) -> impl IntoResponse {
    match UserContact::read(&user, &contact_id).await {
        Ok(user_contact) => (StatusCode::OK, Json(serde_json::json!(user_contact))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    Extension(user): Extension<Arc<User>>,
    Json(update_user_contact): Json<UpdateUserContact>,
) -> impl IntoResponse {
    match UserContact::update(
        &user,
        &update_user_contact.contact_id,
        &update_user_contact.contact_value,
    )
    .await
    {
        Ok(user_contact) => (StatusCode::ACCEPTED, Json(serde_json::json!(user_contact))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(
    Extension(user): Extension<Arc<User>>,
    Path(contact_id): Path<i64>,
) -> impl IntoResponse {
    match UserContact::delete(&user, &contact_id).await {
        Ok(user_contact) => (
            StatusCode::NO_CONTENT,
            Json(serde_json::json!(user_contact)),
        ),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_all_for_user(Extension(user): Extension<Arc<User>>) -> impl IntoResponse {
    match UserContact::delete_all_for_user(&user).await {
        Ok(user_contacts) => (StatusCode::OK, Json(serde_json::json!(user_contacts))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
