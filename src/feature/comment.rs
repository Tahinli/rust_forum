use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::database::comment;

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub comment_id: i64,
    pub user_id: i64,
    pub post_id: i64,
    pub creation_time: DateTime<Utc>,
    pub comment: String,
}

impl Comment {
    pub async fn create(
        user_id: &i64,
        post_id: &i64,
        comment: &String,
    ) -> Result<Comment, sqlx::Error> {
        comment::create(user_id, post_id, comment).await
    }

    pub async fn read(comment_id: &i64) -> Result<Comment, sqlx::Error> {
        comment::read(comment_id).await
    }

    pub async fn update(
        comment_id: &i64,
        user_id: &i64,
        comment: &String,
    ) -> Result<Comment, sqlx::Error> {
        comment::update(comment_id, user_id, comment).await
    }

    pub async fn delete(comment_id: &i64, user_id: &i64) -> Result<Comment, sqlx::Error> {
        comment::delete(comment_id, user_id).await
    }

    pub async fn read_all_for_post(post_id: &i64) -> Result<Vec<Comment>, sqlx::Error> {
        comment::read_all_for_post(post_id).await
    }
}
