use std::sync::Arc;

use axum::{
    extract::{OriginalUri, Request},
    http::{self, HeaderMap, StatusCode, Uri},
    middleware::Next,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::{
    error::ForumAuthError,
    feature::{login::AuthorizationTokenMeta, user::User},
};

#[derive(Debug, Serialize, Deserialize)]
struct UserAndTargetUser {
    user: User,
    target_user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAndAuthorizationToken {
    pub user: User,
    pub authorization_token: String,
}

async fn authorization_token_extraction(
    request_headers: &HeaderMap,
) -> Result<String, ForumAuthError> {
    if let Some(authorization_header) = request_headers.get(http::header::AUTHORIZATION) {
        if let Ok(authorization_header) = authorization_header.to_str() {
            if let Some((bearer, authorization_token)) = authorization_header.split_once(' ') {
                if bearer.to_lowercase() == "bearer" {
                    return Ok(authorization_token.to_owned());
                }
            }
        }
    }
    Err(ForumAuthError::AuthenticationFailed("".to_owned()))
}

async fn user_extraction_from_authorization_token(
    authorization_token: &String,
) -> Result<User, ForumAuthError> {
    match AuthorizationTokenMeta::verify_token(&authorization_token.to_string()).await {
        Ok(claims) => User::read(&claims.custom.user_id)
            .await
            .map_err(|err_val| ForumAuthError::AuthenticationFailed(err_val.to_string())),
        Err(err_val) => Err(ForumAuthError::AuthenticationFailed(err_val.to_string())),
    }
}

async fn user_extraction_from_header(request_headers: &HeaderMap) -> Result<User, ForumAuthError> {
    match authorization_token_extraction(request_headers).await {
        Ok(authorization_token) => {
            user_extraction_from_authorization_token(&authorization_token).await
        }
        Err(err_val) => Err(err_val),
    }
}

async fn user_extraction_from_uri(request_uri: &Uri) -> Result<User, ForumAuthError> {
    let request_uri_parts = request_uri.path().split('/').collect::<Vec<&str>>();
    for (index, uri_part) in request_uri_parts.iter().enumerate() {
        if *uri_part == "users" {
            if let Some(user_id) = request_uri_parts.get(index + 1) {
                if let Ok(user_id) = user_id.parse::<i64>() {
                    return User::read(&user_id).await.map_err(|err_val| {
                        ForumAuthError::AuthenticationFailed(err_val.to_string())
                    });
                }
            }
        }
    }
    Err(ForumAuthError::AuthenticationFailed("".to_owned()))
}

async fn user_from_header_and_target_user_from_uri_extraction(
    request_headers: &HeaderMap,
    request_uri: &Uri,
) -> Result<UserAndTargetUser, ForumAuthError> {
    let user = user_extraction_from_header(request_headers).await?;
    let target_user = user_extraction_from_uri(request_uri).await?;

    Ok(UserAndTargetUser { user, target_user })
}

pub async fn user_and_token_then_insert(mut request: Request, next: Next) -> impl IntoResponse {
    if let Ok(authorization_token) = authorization_token_extraction(&request.headers()).await {
        if let Ok(user) = user_extraction_from_authorization_token(&authorization_token).await {
            let user_and_token = Arc::new(UserAndAuthorizationToken {
                user,
                authorization_token,
            });

            request.extensions_mut().insert(user_and_token);
            return next.run(request).await;
        }
    }
    StatusCode::FORBIDDEN.into_response()
}

pub async fn by_authorization_token(request: Request, next: Next) -> impl IntoResponse {
    match user_extraction_from_header(request.headers()).await {
        Ok(_) => next.run(request).await,
        Err(_) => StatusCode::FORBIDDEN.into_response(),
    }
}

pub async fn by_authorization_token_then_insert(
    mut request: Request,
    next: Next,
) -> impl IntoResponse {
    match user_extraction_from_header(request.headers()).await {
        Ok(user) => {
            let user = Arc::new(user);
            request.extensions_mut().insert(user);

            next.run(request).await
        }
        Err(_) => StatusCode::FORBIDDEN.into_response(),
    }
}

pub async fn by_uri_then_insert(mut request: Request, next: Next) -> impl IntoResponse {
    if let Ok(target_user) = user_extraction_from_uri(
        request
            .extensions()
            .get::<OriginalUri>()
            .expect("Shouldn't panic, how we couldn't have uri"),
    )
    .await
    {
        let target_user = Arc::new(target_user);
        request.extensions_mut().insert(target_user);

        return next.run(request).await;
    }
    StatusCode::BAD_REQUEST.into_response()
}

pub async fn builder_by_authorization_token(request: Request, next: Next) -> impl IntoResponse {
    if let Ok(user) = user_extraction_from_header(request.headers()).await {
        if User::is_builder(&user).await {
            return next.run(request).await;
        }
    }
    StatusCode::FORBIDDEN.into_response()
}

pub async fn builder_by_authorization_token_then_insert(
    mut request: Request,
    next: Next,
) -> impl IntoResponse {
    if let Ok(user) = user_extraction_from_header(request.headers()).await {
        if User::is_builder(&user).await {
            let user = Arc::new(user);
            request.extensions_mut().insert(user);

            return next.run(request).await;
        }
    }
    StatusCode::FORBIDDEN.into_response()
}

pub async fn builder_or_admin_by_authorization_token(
    request: Request,
    next: Next,
) -> impl IntoResponse {
    if let Ok(user) = user_extraction_from_header(request.headers()).await {
        if User::is_builder_or_admin(&user).await {
            return next.run(request).await;
        }
    }

    StatusCode::FORBIDDEN.into_response()
}

pub async fn builder_or_admin_by_authorization_token_then_insert(
    mut request: Request,
    next: Next,
) -> impl IntoResponse {
    if let Ok(user) = user_extraction_from_header(request.headers()).await {
        if User::is_builder_or_admin(&user).await {
            let user = Arc::new(user);
            request.extensions_mut().insert(user);

            return next.run(request).await;
        }
    }
    StatusCode::FORBIDDEN.into_response()
}

pub async fn builder_by_authorization_token_and_target_user_by_uri_then_insert_target(
    mut request: Request,
    next: Next,
) -> impl IntoResponse {
    if let Ok(user_and_target_user) = user_from_header_and_target_user_from_uri_extraction(
        request.headers(),
        request
            .extensions()
            .get::<OriginalUri>()
            .expect("Shouldn't panic, how we couldn't have uri"),
    )
    .await
    {
        let user = user_and_target_user.user;
        let target_user = user_and_target_user.target_user;

        if user.is_builder().await && user.is_default(&target_user).await {
            let target_user = Arc::new(target_user);
            request.extensions_mut().insert(target_user);

            return next.run(request).await;
        }
    }
    StatusCode::FORBIDDEN.into_response()
}

pub async fn builder_or_admin_by_authorization_token_and_target_user_by_uri_then_insert_target(
    mut request: Request,
    next: Next,
) -> impl IntoResponse {
    if let Ok(user_and_target_user) = user_from_header_and_target_user_from_uri_extraction(
        request.headers(),
        request
            .extensions()
            .get::<OriginalUri>()
            .expect("Shouldn't panic, how we couldn't have uri"),
    )
    .await
    {
        let user = user_and_target_user.user;
        let target_user = user_and_target_user.target_user;

        if user.is_default(&target_user).await {
            let target_user = Arc::new(target_user);
            request.extensions_mut().insert(target_user);

            return next.run(request).await;
        }
    }
    StatusCode::FORBIDDEN.into_response()
}
