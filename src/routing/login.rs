use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::feature::{auth::OneTimePassword, login::Login, user::User, user_contact::UserContact};

use super::middleware::{user_and_token_then_insert, UserAndAuthorizationToken};

const CONTACT_EMAIL_DEFAULT_ID: i64 = 0;

#[derive(Debug, Serialize, Deserialize)]
struct CreateOneTimePassword {
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateLogin {
    user_id: i64,
    one_time_password: String,
}

pub fn route() -> Router {
    Router::new()
        .route("/one_time_password", post(create_one_time_password))
        .route("/", post(create))
        .route("/users/{user_id}/tokens/{token}", get(read))
        .route(
            "/",
            patch(update).route_layer(axum::middleware::from_fn(user_and_token_then_insert)),
        )
        .route("/users/{user_id}/tokens/{token}", delete(delete_))
        .route("/users/{user_id}", get(read_all_for_user))
        .route("/users/{user_id}", delete(delete_all_for_user))
        .route("/count/users/{user_id}", get(count_all_for_user))
}
async fn create_one_time_password(
    Json(create_one_time_password): Json<CreateOneTimePassword>,
) -> impl IntoResponse {
    //todo get user from middleware or something
    let user = User::read(&create_one_time_password.user_id).await.unwrap();
    match UserContact::read(&user, &CONTACT_EMAIL_DEFAULT_ID).await {
        Ok(user_email) => match OneTimePassword::new(&user, &user_email.contact_value).await {
            Ok(_) => (StatusCode::CREATED, Json(serde_json::json!(""))),
            Err(err_val) => (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!(err_val.to_string())),
            ),
        },
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
async fn create(Json(create_login): Json<CreateLogin>) -> impl IntoResponse {
    let one_time_password = OneTimePassword {
        user_id: create_login.user_id,
        one_time_password: create_login.one_time_password,
    };

    match OneTimePassword::verify(&one_time_password).await {
        true => match Login::create(&one_time_password.user_id).await {
            Ok(login) => (StatusCode::CREATED, Json(serde_json::json!(login))),
            Err(err_val) => (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!(err_val.to_string())),
            ),
        },
        false => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(
                "One Time Password Authentication Failed".to_string()
            )),
        ),
    }
}

async fn read(Path((user_id, token)): Path<(i64, String)>) -> impl IntoResponse {
    match Login::read(&user_id, &token).await {
        Ok(login) => (StatusCode::OK, Json(serde_json::json!(login))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    Extension(user_and_authorization_token): Extension<Arc<UserAndAuthorizationToken>>,
) -> impl IntoResponse {
    match Login::update(
        &user_and_authorization_token.user.user_id,
        &user_and_authorization_token.authorization_token,
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

async fn delete_(Path((user_id, token)): Path<(i64, String)>) -> impl IntoResponse {
    match Login::delete(&user_id, &token).await {
        Ok(login) => (StatusCode::NO_CONTENT, Json(serde_json::json!(login))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_user(Path(user_id): Path<i64>) -> impl IntoResponse {
    match Login::read_all_for_user(&user_id).await {
        Ok(logins) => (StatusCode::OK, Json(serde_json::json!(logins))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_all_for_user(Path(user_id): Path<i64>) -> impl IntoResponse {
    match Login::delete_all_for_user(&user_id).await {
        Ok(logins) => (StatusCode::OK, Json(serde_json::json!(logins))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all_for_user(Path(user_id): Path<i64>) -> impl IntoResponse {
    match Login::count_all_for_user(&user_id).await {
        Ok(login_count) => (StatusCode::OK, Json(serde_json::json!(login_count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
