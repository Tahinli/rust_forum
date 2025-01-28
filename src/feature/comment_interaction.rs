use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::database::comment_interaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentInteraction {
    pub comment_id: i64,
    pub user_id: i64,
    pub interaction_time: DateTime<Utc>,
    pub interaction_id: i64,
}

impl CommentInteraction {
    pub async fn create(
        comment_id: &i64,
        user_id: &i64,
        interaction_id: &i64,
    ) -> Result<CommentInteraction, sqlx::Error> {
        comment_interaction::create(comment_id, user_id, interaction_id).await
    }

    pub async fn read(comment_id: &i64, user_id: &i64) -> Result<CommentInteraction, sqlx::Error> {
        comment_interaction::read(comment_id, user_id).await
    }

    pub async fn update(
        comment_id: &i64,
        user_id: &i64,
        interaction_id: &i64,
    ) -> Result<CommentInteraction, sqlx::Error> {
        comment_interaction::update(comment_id, user_id, interaction_id).await
    }

    pub async fn delete(
        comment_id: &i64,
        user_id: &i64,
    ) -> Result<CommentInteraction, sqlx::Error> {
        comment_interaction::delete(comment_id, user_id).await
    }

    pub async fn read_all_for_comment(
        comment_id: &i64,
    ) -> Result<Vec<CommentInteraction>, sqlx::Error> {
        comment_interaction::read_all_for_comment(comment_id).await
    }
}
