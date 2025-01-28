use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::database::post;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub post_id: i64,
    pub user_id: i64,
    pub creation_time: DateTime<Utc>,
    pub post: String,
}

impl Post {
    pub async fn create(user_id: &i64, post: &String) -> Result<Post, sqlx::Error> {
        post::create(user_id, post).await
    }

    pub async fn read(post_id: &i64) -> Result<Post, sqlx::Error> {
        post::read(post_id).await
    }

    pub async fn update(post_id: &i64, user_id: &i64, post: &String) -> Result<Post, sqlx::Error> {
        post::update(post_id, user_id, post).await
    }

    pub async fn delete(post_id: &i64, user_id: &i64) -> Result<Post, sqlx::Error> {
        post::delete(post_id, user_id).await
    }

    pub async fn read_all() -> Result<Vec<Post>, sqlx::Error> {
        post::read_all().await
    }

    pub async fn read_all_for_user(user_id: &i64) -> Result<Vec<Post>, sqlx::Error> {
        post::read_all_for_user(user_id).await
    }
}
