pub mod comment;
pub mod comment_interaction;
pub mod contact;
pub mod interaction;
pub mod login;
pub mod middleware;
pub mod post;
pub mod post_interaction;
pub mod role;
pub mod user;
pub mod user_contact;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use tower::limit::ConcurrencyLimitLayer;
use tower_http::cors::CorsLayer;

use crate::{database, AppState};

pub async fn route(concurrency_limit: &usize, State(app_state): State<AppState>) -> Router {
    Router::new()
        .route("/", get(alive))
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            middleware::pass,
        ))
        .nest(
            "/roles",
            role::route(axum::extract::State(app_state.clone())),
        )
        .nest(
            "/users",
            user::route(axum::extract::State(app_state.clone())),
        )
        .nest(
            "/posts",
            post::route(axum::extract::State(app_state.clone())),
        )
        .nest(
            "/comments",
            comment::route(axum::extract::State(app_state.clone())),
        )
        .nest(
            "/interactions",
            interaction::route(axum::extract::State(app_state.clone())),
        )
        .nest(
            "/post_interactions",
            post_interaction::route(axum::extract::State(app_state.clone())),
        )
        .nest(
            "/comment_interactions",
            comment_interaction::route(axum::extract::State(app_state.clone())),
        )
        .nest(
            "/contacts",
            contact::route(axum::extract::State(app_state.clone())),
        )
        .nest(
            "/user_contacts",
            user_contact::route(axum::extract::State(app_state.clone())),
        )
        .layer(CorsLayer::permissive())
        .layer(ConcurrencyLimitLayer::new(*concurrency_limit))
        .with_state(app_state)
}

pub async fn alive(State(app_state): State<AppState>) -> impl IntoResponse {
    match database::is_alive(&app_state.database_connection).await {
        true => StatusCode::OK,
        false => StatusCode::SERVICE_UNAVAILABLE,
    }
}
