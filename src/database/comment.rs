use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

use crate::feature::comment::Comment;

pub async fn create(
    post_creation_time: DateTime<Utc>,
    creation_time: DateTime<Utc>,
    commenter_id: i64,
    comment: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Comment, sqlx::Error> {
    sqlx::query_as!(
        Comment,
        r#"
            INSERT INTO "comment"(post_creation_time, creation_time, commenter_id, comment) 
            VALUES ($1, $2, $3, $4) 
            RETURNING *
        "#,
        post_creation_time,
        creation_time,
        commenter_id,
        comment,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(
    creation_time: DateTime<Utc>,
    database_connection: &Pool<Postgres>,
) -> Result<Comment, sqlx::Error> {
    sqlx::query_as!(
        Comment,
        r#"
            SELECT * FROM "comment" WHERE "creation_time" = $1
        "#,
        creation_time
    )
    .fetch_one(database_connection)
    .await
}

pub async fn update(
    creation_time: DateTime<Utc>,
    comment: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Comment, sqlx::Error> {
    sqlx::query_as!(
        Comment,
        r#"
        UPDATE "comment" SET "comment" = $1 WHERE "creation_time" = $2
        RETURNING *
    "#,
        comment,
        creation_time
    )
    .fetch_one(database_connection)
    .await
}

pub async fn delete(
    creation_time: DateTime<Utc>,
    database_connection: &Pool<Postgres>,
) -> Result<Comment, sqlx::Error> {
    sqlx::query_as!(
        Comment,
        r#"
        DELETE FROM "comment" where "creation_time" = $1
        RETURNING *
    "#,
        creation_time
    )
    .fetch_one(database_connection)
    .await
}
