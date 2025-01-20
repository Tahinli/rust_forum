use chrono::{DateTime, Utc};

use crate::feature::post_interaction::PostInteraction;

use super::DATABASE_CONNECTIONS;

pub async fn create(
    post_creation_time: &DateTime<Utc>,
    user_id: &i64,
    interaction_id: &i64,
) -> Result<PostInteraction, sqlx::Error> {
    sqlx::query_as!(
        PostInteraction,
        r#"
            INSERT INTO "post_interaction"(post_creation_time, user_id, interaction_id)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        post_creation_time,
        user_id,
        interaction_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read(interaction_time: &DateTime<Utc>) -> Result<PostInteraction, sqlx::Error> {
    sqlx::query_as!(
        PostInteraction,
        r#"
            SELECT * FROM "post_interaction" WHERE "interaction_time" = $1
        "#,
        interaction_time
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn update(
    interaction_time: &DateTime<Utc>,
    interaction_id: &i64,
) -> Result<PostInteraction, sqlx::Error> {
    sqlx::query_as!(
        PostInteraction,
        r#"
        UPDATE "post_interaction" SET "interaction_id" = $2 WHERE "interaction_time" = $1
        RETURNING *
    "#,
        interaction_time,
        interaction_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn delete(interaction_time: &DateTime<Utc>) -> Result<PostInteraction, sqlx::Error> {
    sqlx::query_as!(
        PostInteraction,
        r#"
        DELETE FROM "post_interaction" WHERE "interaction_time" = $1
        RETURNING *
    "#,
        interaction_time
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all_for_post(
    post_creation_time: &DateTime<Utc>,
) -> Result<Vec<PostInteraction>, sqlx::Error> {
    sqlx::query_as!(
        PostInteraction,
        r#"
            SELECT * FROM "post_interaction" WHERE "post_creation_time" = $1
        "#,
        post_creation_time
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}
