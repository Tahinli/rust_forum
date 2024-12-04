use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

use crate::feature::post_interaction::PostInteraction;

pub async fn create(
    post_creation_time: &DateTime<Utc>,
    interaction_id: i64,
    interactor_id: i64,
    interaction_time: &DateTime<Utc>,
    database_connection: &Pool<Postgres>,
) -> Result<PostInteraction, sqlx::Error> {
    sqlx::query_as!(
        PostInteraction,
        r#"
            INSERT INTO "post_interaction"(post_creation_time, interaction_id, interactor_id, interaction_time) 
            VALUES ($1, $2, $3, $4) 
            RETURNING *
        "#,
        post_creation_time, interaction_id, interactor_id, interaction_time,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(
    interaction_time: &DateTime<Utc>,
    database_connection: &Pool<Postgres>,
) -> Result<PostInteraction, sqlx::Error> {
    sqlx::query_as!(
        PostInteraction,
        r#"
            SELECT * FROM "post_interaction" WHERE "interaction_time" = $1
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
) -> Result<PostInteraction, sqlx::Error> {
    sqlx::query_as!(
        PostInteraction,
        r#"
        UPDATE "post_interaction" SET "interaction_id" = $1 WHERE "interaction_time" = $2
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
) -> Result<PostInteraction, sqlx::Error> {
    sqlx::query_as!(
        PostInteraction,
        r#"
        DELETE FROM "post_interaction" where "interaction_time" = $1
        RETURNING *
    "#,
        interaction_time
    )
    .fetch_one(database_connection)
    .await
}
