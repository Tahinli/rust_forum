use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use jwt_simple::{
    claims::{Claims, JWTClaims},
    common::VerificationOptions,
    prelude::{HS256Key, MACLike},
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{database::login, error::ForumAuthError, SERVER_CONFIG};

use super::user::User;

static TOKEN_META: LazyLock<TokenMeta> = LazyLock::new(TokenMeta::init);

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

        let claims = Claims::with_custom_claims(
            *user_id,
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

    pub async fn verify_token(token: &String) -> Result<JWTClaims<i64>, jwt_simple::Error> {
        let token_meta = &TOKEN_META;
        token_meta
            .token_key
            .verify_token::<i64>(token, token_meta.token_verification_options.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub user_id: i64,
    pub token: String,
    pub token_creation_time: DateTime<Utc>,
}

impl Login {
    pub async fn create(
        user_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Login, sqlx::Error> {
        User::read(user_id, database_connection).await?;

        let token = TokenMeta::create_token(user_id)
            .await
            .expect("Should not panic if it isn't configured wrong");
        login::create(user_id, &token, database_connection).await
    }

    pub async fn read(
        user_id: &i64,
        token: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Login, sqlx::Error> {
        User::read(user_id, database_connection).await?;

        login::read(user_id, token, database_connection).await
    }

    pub async fn update(
        user_id: &i64,
        token: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Login, Box<dyn std::error::Error>> {
        let login = Login::read(user_id, token, database_connection).await?;
        match TokenMeta::verify_token(token).await {
            Ok(_) => Ok(login),
            Err(_) => {
                if DateTime::<Utc>::default()
                    .signed_duration_since(&login.token_creation_time)
                    .num_minutes()
                    <= SERVER_CONFIG.login_token_refresh_time_limit as i64
                {
                    Login::delete(user_id, token, database_connection).await?;
                    let login = Login::create(user_id, database_connection).await?;
                    Ok(login)
                } else {
                    Err(Box::new(ForumAuthError::TokenRefreshTimeOver))
                }
            }
        }
    }
    pub async fn delete(
        user_id: &i64,
        token: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Login, sqlx::Error> {
        login::delete(user_id, token, database_connection).await
    }

    pub async fn read_all_for_user(
        user_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<Login>, sqlx::Error> {
        login::read_all_for_user(user_id, database_connection).await
    }

    pub async fn delete_all_for_user(
        user_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<Login>, sqlx::Error> {
        login::delete_all_for_user(user_id, database_connection).await
    }

    pub async fn count_all_for_user(
        user_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<u64, sqlx::Error> {
        login::count_all_for_user(user_id, database_connection).await
    }
}
