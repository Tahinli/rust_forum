pub mod role;
pub mod user;

use axum::Router;

use super::middleware::pass_builder_or_admin_by_authorization_token;

pub fn route() -> Router {
    Router::new()
        .nest("/users", user::route())
        .nest("/roles", role::route())
        .route_layer(axum::middleware::from_fn(
            pass_builder_or_admin_by_authorization_token,
        ))
}
