use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::database::post;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub creation_time: DateTime<Utc>,
    pub user_id: i64,
    pub post: String,
}

impl Post {
    pub async fn create(
        user_id: &i64,
        post: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Post, sqlx::Error> {
        post::create(user_id, post, database_connection).await
    }

    pub async fn read(
        creation_time: &DateTime<Utc>,
        database_connection: &Pool<Postgres>,
    ) -> Result<Post, sqlx::Error> {
        post::read(creation_time, database_connection).await
    }

    pub async fn update(
        creation_time: &DateTime<Utc>,
        user_id: &i64,
        post: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Post, sqlx::Error> {
        post::update(creation_time, user_id, post, database_connection).await
    }

    pub async fn delete(
        creation_time: &DateTime<Utc>,
        database_connection: &Pool<Postgres>,
    ) -> Result<Post, sqlx::Error> {
        post::delete(creation_time, database_connection).await
    }

    pub async fn read_all(database_connection: &Pool<Postgres>) -> Result<Vec<Post>, sqlx::Error> {
        post::read_all(database_connection).await
    }

    pub async fn read_all_for_user(
        user_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<Post>, sqlx::Error> {
        post::read_all_for_user(user_id, database_connection).await
    }
}
