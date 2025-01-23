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

static TOKEN_META: LazyLock<TokenMeta> = LazyLock::new(TokenMeta::init);

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomClaim {
    pub user_id: i64,
}

pub struct TokenMeta {
    token_key: HS256Key,
    token_verification_options: Option<VerificationOptions>,
}

impl TokenMeta {
    fn init() -> Self {
        Self {
            token_key: HS256Key::generate(),
            token_verification_options: {
                let mut verification_options = VerificationOptions::default();
                verification_options.time_tolerance = Some(jwt_simple::prelude::Duration::from(0));
                Some(verification_options)
            },
        }
    }

    async fn create_token(user_id: &i64) -> Option<String> {
        let key = &TOKEN_META.token_key;
        let custom_claim = CustomClaim { user_id: *user_id };
        let claims = Claims::with_custom_claims(
            custom_claim,
            jwt_simple::prelude::Duration::from_mins(
                SERVER_CONFIG.login_token_expiration_time_limit as u64,
            ),
        );

        let token = key.authenticate(claims).unwrap();
        match TokenMeta::verify_token(&token).await {
            Ok(_) => Some(token),
            Err(_) => None,
        }
    }

    pub async fn verify_token(token: &String) -> Result<JWTClaims<CustomClaim>, jwt_simple::Error> {
        TOKEN_META
            .token_key
            .verify_token::<CustomClaim>(token, TOKEN_META.token_verification_options.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub user_id: i64,
    pub token: String,
    pub token_creation_time: DateTime<Utc>,
}

impl Login {
    pub async fn create(user_id: &i64) -> Result<Login, sqlx::Error> {
        User::read(user_id).await?;

        let token = TokenMeta::create_token(user_id)
            .await
            .expect("Should not panic if it isn't configured wrong");
        login::create(user_id, &token).await
    }

    pub async fn read(user_id: &i64, token: &String) -> Result<Login, sqlx::Error> {
        User::read(user_id).await?;

        login::read(user_id, token).await
    }

    pub async fn update(
        user_id: &i64,
        token: &String,
    ) -> Result<Login, Box<dyn std::error::Error>> {
        let login = Login::read(user_id, token).await?;
        match TokenMeta::verify_token(token).await {
            Ok(_) => Ok(login),
            Err(_) => {
                if DateTime::<Utc>::default()
                    .signed_duration_since(&login.token_creation_time)
                    .num_minutes()
                    <= SERVER_CONFIG.login_token_refresh_time_limit as i64
                {
                    Login::delete(user_id, token).await?;
                    let login = Login::create(user_id).await?;
                    Ok(login)
                } else {
                    Err(Box::new(ForumAuthError::TokenRefreshTimeOver))
                }
            }
        }
    }
    pub async fn delete(user_id: &i64, token: &String) -> Result<Login, sqlx::Error> {
        login::delete(user_id, token).await
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
