use chrono::{DateTime, Utc};

use crate::feature::comment_interaction::CommentInteraction;

use super::DATABASE_CONNECTIONS;

pub async fn create(
    comment_creation_time: &DateTime<Utc>,
    user_id: &i64,
    interaction_id: &i64,
) -> Result<CommentInteraction, sqlx::Error> {
    sqlx::query_as!(
        CommentInteraction,
        r#"
            INSERT INTO "comment_interaction"(comment_creation_time, user_id, interaction_id)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        comment_creation_time,
        user_id,
        interaction_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read(interaction_time: &DateTime<Utc>) -> Result<CommentInteraction, sqlx::Error> {
    sqlx::query_as!(
        CommentInteraction,
        r#"
            SELECT * FROM "comment_interaction" WHERE "interaction_time" = $1
        "#,
        interaction_time
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn update(
    interaction_time: &DateTime<Utc>,
    interaction_id: &i64,
) -> Result<CommentInteraction, sqlx::Error> {
    sqlx::query_as!(
        CommentInteraction,
        r#"
        UPDATE "comment_interaction" SET "interaction_id" = $2 WHERE "interaction_time" = $1
        RETURNING *
    "#,
        interaction_time,
        interaction_id
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn delete(interaction_time: &DateTime<Utc>) -> Result<CommentInteraction, sqlx::Error> {
    sqlx::query_as!(
        CommentInteraction,
        r#"
        DELETE FROM "comment_interaction" WHERE "interaction_time" = $1
        RETURNING *
    "#,
        interaction_time
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all_for_comment(
    comment_creation_time: &DateTime<Utc>,
) -> Result<Vec<CommentInteraction>, sqlx::Error> {
    sqlx::query_as!(
        CommentInteraction,
        r#"
            SELECT * FROM "comment_interaction" WHERE "comment_creation_time" = $1
        "#,
        comment_creation_time
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}
