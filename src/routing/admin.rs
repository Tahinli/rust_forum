pub mod comment;
pub mod contact;
pub mod interaction;
pub mod login;
pub mod post;
pub mod role;
pub mod user;
pub mod user_contact;

use axum::Router;

use super::middleware::builder_or_admin_by_authorization_token;

pub fn route() -> Router {
    Router::new()
        .nest("/logins", login::route())
        .nest("/users", user::route())
        .nest("/roles", role::route())
        .nest("/contacts", contact::route())
        .nest("/user_contacts", user_contact::route())
        .nest("/interactions", interaction::route())
        .nest("/posts", post::route())
        .nest("/comments", comment::route())
        .route_layer(axum::middleware::from_fn(
            builder_or_admin_by_authorization_token,
        ))
}
