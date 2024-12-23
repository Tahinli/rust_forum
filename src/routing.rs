pub mod comment;
pub mod comment_interaction;
pub mod contact;
pub mod interaction;
pub mod permission;
pub mod post;
pub mod post_interaction;
pub mod role;
pub mod role_permission;
pub mod routing;
pub mod user;
pub mod user_contact;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use tower_http::cors::CorsLayer;

use crate::{database, AppState};

pub async fn route(State(app_state): State<AppState>) -> Router {
    Router::new()
        .route("/", get(alive))
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
            "/role_permissions",
            role_permission::route(axum::extract::State(app_state.clone())),
        )
        .nest(
            "/contacts",
            contact::route(axum::extract::State(app_state.clone())),
        )
        .nest(
            "/user_contacts",
            user_contact::route(axum::extract::State(app_state.clone())),
        )
        .nest(
            "/routings",
            routing::route(axum::extract::State(app_state.clone())),
        )
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}

async fn alive(State(app_state): State<AppState>) -> impl IntoResponse {
    match database::is_alive(&app_state.database_connection).await {
        true => StatusCode::OK,
        false => StatusCode::SERVICE_UNAVAILABLE,
    }
}
