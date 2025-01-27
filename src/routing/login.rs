use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::feature::{auth::OneTimePassword, login::Login, user::User, user_contact::UserContact};

use super::middleware::{
    by_authorization_token_then_insert, user_and_token_then_insert, UserAndAuthorizationToken,
};

const CONTACT_EMAIL_DEFAULT_ID: i64 = 0;

#[derive(Debug, Serialize, Deserialize)]
struct CreateOneTimePassword {
    pub user_email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateLogin {
    user_email: String,
    one_time_password: String,
}

pub fn route() -> Router {
    Router::new()
        .route("/one_time_password", post(create_one_time_password))
        .route("/", post(create))
        .route(
            "/",
            get(read).route_layer(axum::middleware::from_fn(user_and_token_then_insert)),
        )
        .route(
            "/",
            patch(update).route_layer(axum::middleware::from_fn(user_and_token_then_insert)),
        )
        .route(
            "/",
            delete(delete_).route_layer(axum::middleware::from_fn(user_and_token_then_insert)),
        )
        .route(
            "/users",
            delete(delete_all_for_user).route_layer(axum::middleware::from_fn(
                by_authorization_token_then_insert,
            )),
        )
        .route("/count/users", get(count_all_for_user))
}
async fn create_one_time_password(
    Json(create_one_time_password): Json<CreateOneTimePassword>,
) -> impl IntoResponse {
    match UserContact::read_for_value(
        &CONTACT_EMAIL_DEFAULT_ID,
        &create_one_time_password.user_email,
    )
    .await
    {
        Ok(user_contact) => match User::read(&user_contact.user_id).await {
            Ok(user) => {
                match OneTimePassword::new(&user, &create_one_time_password.user_email).await {
                    Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({}))),
                    Err(err_val) => (
                        StatusCode::BAD_REQUEST,
                        Json(serde_json::json!(err_val.to_string())),
                    ),
                }
            }
            Err(err_val) => (
                // this must be impossible that's why I send 500
                StatusCode::INTERNAL_SERVER_ERROR,
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
    match UserContact::read_for_value(&CONTACT_EMAIL_DEFAULT_ID, &create_login.user_email).await {
        Ok(user_contact) => match User::read(&user_contact.user_id).await {
            Ok(user) => {
                let one_time_password =
                    OneTimePassword::from_string(&user, &create_login.one_time_password).await;

                match OneTimePassword::verify(&one_time_password).await {
                    true => match Login::create(&user.user_id).await {
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
            Err(err_val) => (
                // this must be impossible that's why I send 500
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!(err_val.to_string())),
            ),
        },
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read(
    Extension(user_and_authorization_token): Extension<Arc<UserAndAuthorizationToken>>,
) -> impl IntoResponse {
    match Login::read(
        &user_and_authorization_token.user.user_id,
        &user_and_authorization_token.authorization_token,
    )
    .await
    {
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

async fn delete_(
    Extension(user_and_authorization_token): Extension<Arc<UserAndAuthorizationToken>>,
) -> impl IntoResponse {
    match Login::delete(
        &user_and_authorization_token.user.user_id,
        &user_and_authorization_token.authorization_token,
    )
    .await
    {
        Ok(login) => (StatusCode::NO_CONTENT, Json(serde_json::json!(login))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn delete_all_for_user(Extension(user): Extension<Arc<User>>) -> impl IntoResponse {
    match Login::delete_all_for_user(&user.user_id).await {
        Ok(logins) => (StatusCode::OK, Json(serde_json::json!(logins))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all_for_user(Extension(user): Extension<Arc<User>>) -> impl IntoResponse {
    match Login::count_all_for_user(&user.user_id).await {
        Ok(login_count) => (StatusCode::OK, Json(serde_json::json!(login_count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
