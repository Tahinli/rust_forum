use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::database::post;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub creation_time: DateTime<Utc>,
    pub user_id: i64,
    pub post: String,
}

impl Post {
    pub async fn create(user_id: &i64, post: &String) -> Result<Post, sqlx::Error> {
        post::create(user_id, post).await
    }

    pub async fn read(creation_time: &DateTime<Utc>) -> Result<Post, sqlx::Error> {
        post::read(creation_time).await
    }

    pub async fn update(
        creation_time: &DateTime<Utc>,
        user_id: &i64,
        post: &String,
    ) -> Result<Post, sqlx::Error> {
        post::update(creation_time, user_id, post).await
    }

    pub async fn delete(creation_time: &DateTime<Utc>) -> Result<Post, sqlx::Error> {
        post::delete(creation_time).await
    }

    pub async fn read_all() -> Result<Vec<Post>, sqlx::Error> {
        post::read_all().await
    }

    pub async fn read_all_for_user(user_id: &i64) -> Result<Vec<Post>, sqlx::Error> {
        post::read_all_for_user(user_id).await
    }
}
