use std::sync::Arc;

use axum::{
    body::{to_bytes, Body},
    extract::Request,
    http::{self, Method, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use crate::feature::{login::TokenMeta, user::User};

#[derive(Debug)]
struct UserAndRequest {
    user: User,
    request: Request,
}

#[derive(Debug)]
struct TargetUserAndRequest {
    target_user: User,
    request: Request,
}

#[derive(Debug)]
struct UserAndTargetUserAndRequest {
    user: User,
    target_user: User,
    request: Request,
}

async fn user_extraction(request: Request) -> Option<UserAndRequest> {
    if let Some(authorization_header) = request.headers().get(http::header::AUTHORIZATION) {
        if let Ok(authorization_header) = authorization_header.to_str() {
            if let Some((bearer, authorization_header)) = authorization_header.split_once(' ') {
                if bearer == "bearer" {
                    if let Ok(claims) =
                        TokenMeta::verify_token(&authorization_header.to_string()).await
                    {
                        return Some(UserAndRequest {
                            user: User::read(&claims.custom).await.ok()?,
                            request,
                        });
                    }
                }
            }
        }
    }
    None
}

async fn target_user_extraction_from_uri(request: Request) -> Option<TargetUserAndRequest> {
    let uri_parts = request.uri().path().split('/').collect::<Vec<&str>>();
    for (index, uri_part) in uri_parts.iter().enumerate() {
        if *uri_part == "users" {
            if let Some(target_user_id) = uri_parts.get(index) {
                if let Ok(target_user_id) = (*target_user_id).parse::<i64>() {
                    if let Ok(target_user) = User::read(&target_user_id).await {
                        return Some(TargetUserAndRequest {
                            target_user,
                            request,
                        });
                    }
                }
            }
        }
    }
    None
}

async fn target_user_extraction_from_json(request: Request) -> Option<TargetUserAndRequest> {
    let (parts, body) = request.into_parts();
    let bytes = to_bytes(body, usize::MAX).await.ok()?;
    let json: serde_json::Value = serde_json::from_slice(&bytes).ok()?;

    let body = Body::from(json.to_string());
    let request = Request::from_parts(parts, body);

    if let Some(target_user_id) = json.get("user_id") {
        if target_user_id.is_i64() {
            if let Some(target_user_id) = target_user_id.as_i64() {
                if let Ok(target_user) = User::read(&target_user_id).await {
                    return Some(TargetUserAndRequest {
                        target_user,
                        request,
                    });
                }
            }
        }
    }

    None
}

async fn user_and_target_user_extraction(request: Request) -> Option<UserAndTargetUserAndRequest> {
    let user_and_request = user_extraction(request).await?;
    let user = user_and_request.user;
    let request = user_and_request.request;

    let target_user_and_request = if request.method() == Method::GET {
        target_user_extraction_from_uri(request).await
    } else {
        target_user_extraction_from_json(request).await
    }?;

    let target_user = target_user_and_request.target_user;
    let request = target_user_and_request.request;

    Some(UserAndTargetUserAndRequest {
        user,
        target_user,
        request,
    })
}

pub async fn pass(request: Request, next: Next) -> Result<impl IntoResponse, StatusCode> {
    match user_extraction(request).await {
        Some(user_and_request) => {
            let user = Arc::new(user_and_request.user);
            let mut request = user_and_request.request;

            request.extensions_mut().insert(user);

            Ok(next.run(request).await)
        }
        None => Err(StatusCode::FORBIDDEN),
    }
}

pub async fn pass_builder(request: Request, next: Next) -> Result<impl IntoResponse, StatusCode> {
    if let Some(user_and_request) = user_extraction(request).await {
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

pub async fn pass_builder_or_admin(
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(user_and_request) = user_extraction(request).await {
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

pub async fn pass_self(request: Request, next: Next) -> Result<impl IntoResponse, StatusCode> {
    if let Some(user_and_target_user_and_request) = user_and_target_user_extraction(request).await {
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

pub async fn pass_higher(request: Request, next: Next) -> Result<impl IntoResponse, StatusCode> {
    if let Some(user_and_target_user_and_request) = user_and_target_user_extraction(request).await {
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
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(user_and_target_user_and_request) = user_and_target_user_extraction(request).await {
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
