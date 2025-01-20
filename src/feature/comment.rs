use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::database::comment;

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub creation_time: DateTime<Utc>,
    pub post_creation_time: DateTime<Utc>,
    pub user_id: i64,
    pub comment: String,
}

impl Comment {
    pub async fn create(
        post_creation_time: &DateTime<Utc>,
        user_id: &i64,
        comment: &String,
    ) -> Result<Comment, sqlx::Error> {
        comment::create(post_creation_time, user_id, comment).await
    }

    pub async fn read(creation_time: &DateTime<Utc>) -> Result<Comment, sqlx::Error> {
        comment::read(creation_time).await
    }

    pub async fn update(
        creation_time: &DateTime<Utc>,
        comment: &String,
    ) -> Result<Comment, sqlx::Error> {
        comment::update(creation_time, comment).await
    }

    pub async fn delete(creation_time: &DateTime<Utc>) -> Result<Comment, sqlx::Error> {
        comment::delete(creation_time).await
    }

    pub async fn read_all_for_post(
        post_creation_time: &DateTime<Utc>,
    ) -> Result<Vec<Comment>, sqlx::Error> {
        comment::read_all_for_post(post_creation_time).await
    }
}
