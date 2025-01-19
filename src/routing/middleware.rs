use std::sync::Arc;

use axum::{
    body::{to_bytes, Body},
    extract::{Request, State},
    http::{self, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use sqlx::{Pool, Postgres};

use crate::{
    feature::{login::TokenMeta, user::User},
    AppState,
};

#[derive(Debug)]
struct UserAndRequest {
    user: User,
    request: Request,
}

#[derive(Debug)]
struct UserAndTargetUserAndRequest {
    user: User,
    target_user: User,
    request: Request,
}

async fn user_extraction(
    request: Request,
    database_connection: &Pool<Postgres>,
) -> Option<UserAndRequest> {
    if let Some(authorization_header) = request.headers().get(http::header::AUTHORIZATION) {
        if let Ok(authorization_header) = authorization_header.to_str() {
            if let Some((bearer, authorization_header)) = authorization_header.split_once(' ') {
                if bearer == "bearer" {
                    if let Ok(claims) =
                        TokenMeta::verify_token(&authorization_header.to_string()).await
                    {
                        return Some(UserAndRequest {
                            user: User::read(&claims.custom, database_connection).await.ok()?,
                            request,
                        });
                    }
                }
            }
        }
    }
    None
}

async fn target_user_extraction_from_json(
    json: &serde_json::Value,
    database_connection: &Pool<Postgres>,
) -> Option<User> {
    if let Some(target_user_id) = json.get("user_id") {
        if target_user_id.is_i64() {
            if let Some(target_user_id) = target_user_id.as_i64() {
                return User::read(&target_user_id, database_connection).await.ok();
            }
        }
    }

    None
}

async fn user_and_target_user_extraction(
    request: Request,
    database_connection: &Pool<Postgres>,
) -> Option<UserAndTargetUserAndRequest> {
    let user_and_request = user_extraction(request, database_connection).await?;
    let user = user_and_request.user;
    let request = user_and_request.request;

    let (parts, body) = request.into_parts();
    let bytes = to_bytes(body, usize::MAX).await.ok()?;
    let json: serde_json::Value = serde_json::from_slice(&bytes).ok()?;

    let body = Body::from(json.to_string());
    let request = Request::from_parts(parts, body);

    Some(UserAndTargetUserAndRequest {
        user,
        target_user: target_user_extraction_from_json(&json, database_connection).await?,
        request,
    })
}

pub async fn pass(
    State(app_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    match user_extraction(request, &app_state.database_connection).await {
        Some(user_and_request) => {
            let user = Arc::new(user_and_request.user);
            let mut request = user_and_request.request;

            request.extensions_mut().insert(user);

            Ok(next.run(request).await)
        }
        None => Err(StatusCode::FORBIDDEN),
    }
}

pub async fn pass_builder(
    State(app_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(user_and_request) = user_extraction(request, &app_state.database_connection).await {
        let user = user_and_request.user;
        let mut request = user_and_request.request;

        if User::is_builder(&user).await {
            let user = Arc::new(user);
            request.extensions_mut().insert(user);

            return Ok(next.run(request).await);
        }
    }
    Err(StatusCode::FORBIDDEN)
}

pub async fn pass_admin(
    State(app_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(user_and_request) = user_extraction(request, &app_state.database_connection).await {
        let user = user_and_request.user;
        let mut request = user_and_request.request;

        if User::is_admin(&user).await {
            let user = Arc::new(user);
            request.extensions_mut().insert(user);

            return Ok(next.run(request).await);
        }
    }
    Err(StatusCode::FORBIDDEN)
}

pub async fn pass_builder_or_admin(
    State(app_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(user_and_request) = user_extraction(request, &app_state.database_connection).await {
        let user = user_and_request.user;
        let mut request = user_and_request.request;

        if User::is_builder_or_admin(&user).await {
            let user = Arc::new(user);
            request.extensions_mut().insert(user);

            return Ok(next.run(request).await);
        }
    }
    Err(StatusCode::FORBIDDEN)
}

pub async fn pass_self(
    State(app_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(user_and_target_user_and_request) =
        user_and_target_user_extraction(request, &app_state.database_connection).await
    {
        let user = user_and_target_user_and_request.user;
        let target_user = user_and_target_user_and_request.target_user;
        let mut request = user_and_target_user_and_request.request;

        if User::is_self(&user, &target_user).await {
            let user = Arc::new(user);
            request.extensions_mut().insert(user);

            return Ok(next.run(request).await);
        }
    }
    Err(StatusCode::FORBIDDEN)
}

pub async fn pass_higher(
    State(app_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(user_and_target_user_and_request) =
        user_and_target_user_extraction(request, &app_state.database_connection).await
    {
        let user = user_and_target_user_and_request.user;
        let target_user = user_and_target_user_and_request.target_user;
        let mut request = user_and_target_user_and_request.request;

        if User::is_higher(&user, &target_user).await {
            let user = Arc::new(user);
            request.extensions_mut().insert(user);

            return Ok(next.run(request).await);
        }
    }
    Err(StatusCode::FORBIDDEN)
}

pub async fn pass_higher_or_self(
    State(app_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(user_and_target_user_and_request) =
        user_and_target_user_extraction(request, &app_state.database_connection).await
    {
        let user = user_and_target_user_and_request.user;
        let target_user = user_and_target_user_and_request.target_user;
        let mut request = user_and_target_user_and_request.request;

        if User::is_higher_or_self(&user, &target_user).await {
            let user = Arc::new(user);
            request.extensions_mut().insert(user);

            return Ok(next.run(request).await);
        }
    }
    Err(StatusCode::FORBIDDEN)
}
