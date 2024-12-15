use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::database::post_interaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostInteraction {
    pub interaction_time: DateTime<Utc>,
    pub post_creation_time: DateTime<Utc>,
    pub interaction_id: i64,
    pub user_id: i64,
}

impl PostInteraction {
    pub async fn create(
        post_creation_time: &DateTime<Utc>,
        user_id: &i64,
        interaction_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<PostInteraction, sqlx::Error> {
        post_interaction::create(
            post_creation_time,
            user_id,
            interaction_id,
            database_connection,
        )
        .await
    }

    pub async fn read(
        interaction_time: &DateTime<Utc>,
        database_connection: &Pool<Postgres>,
    ) -> Result<PostInteraction, sqlx::Error> {
        post_interaction::read(interaction_time, database_connection).await
    }

    pub async fn update(
        interaction_time: &DateTime<Utc>,
        interaction_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<PostInteraction, sqlx::Error> {
        post_interaction::update(interaction_time, interaction_id, database_connection).await
    }

    pub async fn delete(
        interaction_time: &DateTime<Utc>,
        database_connection: &Pool<Postgres>,
    ) -> Result<PostInteraction, sqlx::Error> {
        post_interaction::delete(interaction_time, database_connection).await
    }

    pub async fn read_all_for_post(
        post_creation_time: &DateTime<Utc>,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<PostInteraction>, sqlx::Error> {
        post_interaction::read_all_for_post(post_creation_time, database_connection).await
    }
}
