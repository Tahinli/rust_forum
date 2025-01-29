pub mod admin;
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

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tower::limit::ConcurrencyLimitLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::database;

pub async fn route(concurrency_limit: &usize) -> Router {
    Router::new()
        .route("/", get(alive))
        .nest("/logins", login::route())
        .nest("/roles", role::route())
        .nest("/users", user::route())
        .nest("/posts", post::route())
        .nest("/comments", comment::route())
        .nest("/interactions", interaction::route())
        .nest("/post_interactions", post_interaction::route())
        .nest("/comment_interactions", comment_interaction::route())
        .nest("/contacts", contact::route())
        .nest("/user_contacts", user_contact::route())
        .nest("/admin", admin::route())
        .layer(CorsLayer::permissive())
        .layer(ConcurrencyLimitLayer::new(*concurrency_limit))
        .layer(TraceLayer::new_for_http())
}

pub async fn alive() -> impl IntoResponse {
    match database::is_alive().await {
        true => StatusCode::OK,
        false => StatusCode::SERVICE_UNAVAILABLE,
    }
}
