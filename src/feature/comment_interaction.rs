use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::database::comment_interaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentInteraction {
    pub interaction_time: DateTime<Utc>,
    pub comment_creation_time: DateTime<Utc>,
    pub interaction_id: i64,
    pub user_id: i64,
}

impl CommentInteraction {
    pub async fn create(
        comment_creation_time: &DateTime<Utc>,
        user_id: &i64,
        interaction_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<CommentInteraction, sqlx::Error> {
        comment_interaction::create(
            comment_creation_time,
            user_id,
            interaction_id,
            database_connection,
        )
        .await
    }

    pub async fn read(
        interaction_time: &DateTime<Utc>,
        database_connection: &Pool<Postgres>,
    ) -> Result<CommentInteraction, sqlx::Error> {
        comment_interaction::read(interaction_time, database_connection).await
    }

    pub async fn update(
        interaction_time: &DateTime<Utc>,
        interaction_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<CommentInteraction, sqlx::Error> {
        comment_interaction::update(interaction_time, interaction_id, database_connection).await
    }

    pub async fn delete(
        interaction_time: &DateTime<Utc>,
        database_connection: &Pool<Postgres>,
    ) -> Result<CommentInteraction, sqlx::Error> {
        comment_interaction::delete(interaction_time, database_connection).await
    }

    pub async fn read_all_for_comment(
        comment_creation_time: &DateTime<Utc>,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<CommentInteraction>, sqlx::Error> {
        comment_interaction::read_all_for_comment(comment_creation_time, database_connection).await
    }
}
