use jwt_simple::{
    claims::Claims,
    common::VerificationOptions,
    prelude::{HS256Key, MACLike},
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{database::login, SERVER_CONFIG};

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub user_id: i64,
    pub token: String,
}

impl Login {
    pub async fn create(
        user_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Login, sqlx::Error> {
        let key = HS256Key::generate();
        let claims = Claims::create(jwt_simple::prelude::Duration::from_mins(
            SERVER_CONFIG.login_token_time_limit as u64,
        ));
        let mut verification_options = VerificationOptions::default();
        verification_options.time_tolerance = Some(jwt_simple::prelude::Duration::from(0));

        let token = key.authenticate(claims).unwrap();

        login::create(user_id, &token, database_connection).await
    }

    pub async fn read(
        user_id: &i64,
        token: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Login, sqlx::Error> {
        login::read(user_id, token, database_connection).await
    }

    pub async fn update(
        user_id: &i64,
        token: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Login, sqlx::Error> {
        login::update(user_id, token, database_connection).await
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
