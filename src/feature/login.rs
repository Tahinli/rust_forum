use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use jwt_simple::{
    claims::{Claims, JWTClaims},
    common::VerificationOptions,
    prelude::{HS256Key, MACLike},
};
use serde::{Deserialize, Serialize};

use crate::{database::login, error::ForumAuthError, SERVER_CONFIG};

use super::user::User;

static TOKEN_META: LazyLock<AuthorizationTokenMeta> = LazyLock::new(AuthorizationTokenMeta::init);

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomClaim {
    pub user_id: i64,
}

pub struct AuthorizationTokenMeta {
    authorization_token: HS256Key,
    authorization_token_verification_options: Option<VerificationOptions>,
}

impl AuthorizationTokenMeta {
    fn init() -> Self {
        Self {
            authorization_token: HS256Key::generate(),
            authorization_token_verification_options: {
                let mut verification_options = VerificationOptions::default();
                verification_options.time_tolerance = Some(jwt_simple::prelude::Duration::from(0));
                Some(verification_options)
            },
        }
    }

    async fn create_token(user_id: &i64) -> Option<String> {
        let key = &TOKEN_META.authorization_token;
        let custom_claim = CustomClaim { user_id: *user_id };
        let claims = Claims::with_custom_claims(
            custom_claim,
            jwt_simple::prelude::Duration::from_mins(
                SERVER_CONFIG.login_token_expiration_time_limit as u64,
            ),
        );

        let token = key.authenticate(claims).unwrap();
        match AuthorizationTokenMeta::verify_token(&token).await {
            Ok(_) => Some(token),
            Err(_) => None,
        }
    }

    pub async fn verify_token(token: &String) -> Result<JWTClaims<CustomClaim>, jwt_simple::Error> {
        TOKEN_META.authorization_token.verify_token::<CustomClaim>(
            token,
            TOKEN_META.authorization_token_verification_options.clone(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub user_id: i64,
    pub authorization_token: String,
    pub token_creation_time: DateTime<Utc>,
}

impl Login {
    pub async fn create(user_id: &i64) -> Result<Login, sqlx::Error> {
        User::read(user_id).await?;

        let token = AuthorizationTokenMeta::create_token(user_id)
            .await
            .expect("Should not panic if it isn't configured wrong");
        login::create(user_id, &token).await
    }

    pub async fn read(user_id: &i64, authorization_token: &String) -> Result<Login, sqlx::Error> {
        User::read(user_id).await?;

        login::read(user_id, authorization_token).await
    }

    pub async fn update(
        user_id: &i64,
        authorization_token: &String,
    ) -> Result<Login, Box<dyn std::error::Error>> {
        let login = Login::read(user_id, authorization_token).await?;
        match AuthorizationTokenMeta::verify_token(authorization_token).await {
            Ok(_) => Ok(login),
            Err(_) => {
                if DateTime::<Utc>::default()
                    .signed_duration_since(&login.token_creation_time)
                    .num_minutes()
                    <= SERVER_CONFIG.login_token_refresh_time_limit as i64
                {
                    Login::delete(user_id, authorization_token).await?;
                    let login = Login::create(user_id).await?;
                    Ok(login)
                } else {
                    Err(Box::new(ForumAuthError::TokenRefreshTimeOver))
                }
            }
        }
    }
    pub async fn delete(user_id: &i64, authorization_token: &String) -> Result<Login, sqlx::Error> {
        login::delete(user_id, authorization_token).await
    }

    pub async fn read_all_for_user(user_id: &i64) -> Result<Vec<Login>, sqlx::Error> {
        login::read_all_for_user(user_id).await
    }

    pub async fn delete_all_for_user(user_id: &i64) -> Result<Vec<Login>, sqlx::Error> {
        login::delete_all_for_user(user_id).await
    }

    pub async fn count_all_for_user(user_id: &i64) -> Result<u64, sqlx::Error> {
        login::count_all_for_user(user_id).await
    }
}
