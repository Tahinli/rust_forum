use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    feature::{auth::OneTimePassword, login::Login},
    AppState,
};

#[derive(Debug, Serialize, Deserialize)]
struct CreateLogin {
    pub one_time_password: OneTimePassword,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateLogin {
    pub user_id: i64,
    pub token: String,
}

pub fn route(State(app_state): State<AppState>) -> Router<AppState> {
    Router::new()
        .route("/", post(create))
        .route("/users/:user_id/token/:token", get(read))
        .route("/", patch(update))
        .route("/users/:user_id/token/:token", delete(delete_))
        .route("/users/:user_id", get(read_all_for_user))
        .route("/users/:user_id", delete(delete_all_for_user))
        .route("/count/users/:user_id", get(count_all_for_user))
        .with_state(app_state)
}

async fn create(
    State(app_state): State<AppState>,
    Json(create_login): Json<CreateLogin>,
) -> impl IntoResponse {
    match OneTimePassword::verify(&create_login.one_time_password).await {
        true => {
            match Login::create(
                &create_login.one_time_password.user_id,
                &app_state.database_connection,
            )
            .await
            {
                Ok(login) => (StatusCode::CREATED, Json(serde_json::json!(login))),
                Err(err_val) => (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!(err_val.to_string())),
                ),
            }
        }
        false => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(
                "One Time Password Authentication Failed".to_string()
            )),
        ),
    }
}

async fn read(
    State(app_state): State<AppState>,
    Path((user_id, token)): Path<(i64, String)>,
) -> impl IntoResponse {
    match Login::read(&user_id, &token, &app_state.database_connection).await {
        Ok(login) => (StatusCode::OK, Json(serde_json::json!(login))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    State(app_state): State<AppState>,
    Json(update_role): Json<UpdateLogin>,
) -> impl IntoResponse {
    match Login::update(
        &update_role.user_id,
        &update_role.token,
        &app_state.database_connection,
    )
    .await
    {
        Ok(login) => (StatusCode::ACCEPTED, Json(serde_json::json!(login))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_(
    State(app_state): State<AppState>,
    Path((user_id, token)): Path<(i64, String)>,
) -> impl IntoResponse {
    match Login::delete(&user_id, &token, &app_state.database_connection).await {
        Ok(login) => (StatusCode::NO_CONTENT, Json(serde_json::json!(login))),
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
    match Login::read_all_for_user(&user_id, &app_state.database_connection).await {
        Ok(logins) => (StatusCode::OK, Json(serde_json::json!(logins))),
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
    match Login::delete_all_for_user(&user_id, &app_state.database_connection).await {
        Ok(logins) => (StatusCode::OK, Json(serde_json::json!(logins))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all_for_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<i64>,
) -> impl IntoResponse {
    match Login::count_all_for_user(&user_id, &app_state.database_connection).await {
        Ok(login_count) => (StatusCode::OK, Json(serde_json::json!(login_count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
