use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::database::post_interaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostInteraction {
    pub post_id: i64,
    pub user_id: i64,
    pub interaction_time: DateTime<Utc>,
    pub interaction_id: i64,
}

impl PostInteraction {
    pub async fn create(
        post_id: &i64,
        user_id: &i64,
        interaction_id: &i64,
    ) -> Result<PostInteraction, sqlx::Error> {
        post_interaction::create(post_id, user_id, interaction_id).await
    }

    pub async fn read(post_id: &i64, user_id: &i64) -> Result<PostInteraction, sqlx::Error> {
        post_interaction::read(post_id, user_id).await
    }

    pub async fn update(
        post_id: &i64,
        user_id: &i64,
        interaction_id: &i64,
    ) -> Result<PostInteraction, sqlx::Error> {
        post_interaction::update(post_id, user_id, interaction_id).await
    }

    pub async fn delete(post_id: &i64, user_id: &i64) -> Result<PostInteraction, sqlx::Error> {
        post_interaction::delete(post_id, user_id).await
    }

    pub async fn read_all_for_post(post_id: &i64) -> Result<Vec<PostInteraction>, sqlx::Error> {
        post_interaction::read_all_for_post(post_id).await
    }
}
