use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{feature::user_contact::UserContact, AppState};

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserContact {
    pub user_id: i64,
    pub contact_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateUserContact {
    pub user_id: i64,
    pub contact_id: i64,
}

pub fn route(State(app_state): State<AppState>) -> Router<AppState> {
    Router::new()
        .route("/", post(create))
        .route("/roles/:user_id/contacts/:contact_id", get(read))
        .route("/", patch(update))
        .route("/roles/:user_id/contacts/:contact_id", delete(delete_))
        .route("/users/:user_id", get(read_all_for_user))
        .route("/users/:user_id", delete(delete_all_for_user))
        .with_state(app_state)
}

async fn create(
    State(app_state): State<AppState>,
    Json(create_user_contact): Json<CreateUserContact>,
) -> impl IntoResponse {
    match UserContact::create(
        &create_user_contact.user_id,
        &create_user_contact.contact_id,
        &app_state.database_connection,
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
    State(app_state): State<AppState>,
    Path((user_id, contact_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    match UserContact::read(&user_id, &contact_id, &app_state.database_connection).await {
        Ok(user_contact) => (StatusCode::OK, Json(serde_json::json!(user_contact))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    State(app_state): State<AppState>,
    Json(update_role): Json<UpdateUserContact>,
) -> impl IntoResponse {
    match UserContact::update(
        &update_role.user_id,
        &update_role.contact_id,
        &app_state.database_connection,
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
    State(app_state): State<AppState>,
    Path((user_id, contact_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    match UserContact::delete(&user_id, &contact_id, &app_state.database_connection).await {
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

async fn read_all_for_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<i64>,
) -> impl IntoResponse {
    match UserContact::read_all_for_user(&user_id, &app_state.database_connection).await {
        Ok(role_contacts) => (StatusCode::OK, Json(serde_json::json!(role_contacts))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_all_for_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<i64>,
) -> impl IntoResponse {
    match UserContact::delete_all_for_user(&user_id, &app_state.database_connection).await {
        Ok(role_contacts) => (StatusCode::OK, Json(serde_json::json!(role_contacts))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
