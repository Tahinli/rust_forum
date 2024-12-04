use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

use crate::feature::comment_interaction::CommentInteraction;

pub async fn create(
    comment_creation_time: &DateTime<Utc>,
    interaction_id: i64,
    interactor_id: i64,
    database_connection: &Pool<Postgres>,
) -> Result<CommentInteraction, sqlx::Error> {
    sqlx::query_as!(
        CommentInteraction,
        r#"
            INSERT INTO "comment_interaction"(comment_creation_time, interaction_id, interactor_id) 
            VALUES ($1, $2, $3) 
            RETURNING *
        "#,
        comment_creation_time,
        interaction_id,
        interactor_id,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(
    interaction_time: &DateTime<Utc>,
    database_connection: &Pool<Postgres>,
) -> Result<CommentInteraction, sqlx::Error> {
    sqlx::query_as!(
        CommentInteraction,
        r#"
            SELECT * FROM "comment_interaction" WHERE "interaction_time" = $1
        "#,
        interaction_time
    )
    .fetch_one(database_connection)
    .await
}

pub async fn update(
    interaction_time: &DateTime<Utc>,
    interaction_id: i64,
    database_connection: &Pool<Postgres>,
) -> Result<CommentInteraction, sqlx::Error> {
    sqlx::query_as!(
        CommentInteraction,
        r#"
        UPDATE "comment_interaction" SET "interaction_id" = $1 WHERE "interaction_time" = $2
        RETURNING *
    "#,
        interaction_id,
        interaction_time
    )
    .fetch_one(database_connection)
    .await
}

pub async fn delete(
    interaction_time: &DateTime<Utc>,
    database_connection: &Pool<Postgres>,
) -> Result<CommentInteraction, sqlx::Error> {
    sqlx::query_as!(
        CommentInteraction,
        r#"
        DELETE FROM "comment_interaction" where "interaction_time" = $1
        RETURNING *
    "#,
        interaction_time
    )
    .fetch_one(database_connection)
    .await
}
