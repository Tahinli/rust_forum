use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

use crate::feature::comment::Comment;

pub async fn create(
    post_creation_time: &DateTime<Utc>,
    commenter_id: &i64,
    comment: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Comment, sqlx::Error> {
    sqlx::query_as!(
        Comment,
        r#"
            INSERT INTO "comment"(post_creation_time, commenter_id, comment) 
            VALUES ($1, $2, $3) 
            RETURNING *
        "#,
        post_creation_time,
        commenter_id,
        comment,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(
    creation_time: &DateTime<Utc>,
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
    creation_time: &DateTime<Utc>,
    comment: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Comment, sqlx::Error> {
    sqlx::query_as!(
        Comment,
        r#"
        UPDATE "comment" SET "comment" = $2 WHERE "creation_time" = $1
        RETURNING *
    "#,
        creation_time,
        comment
    )
    .fetch_one(database_connection)
    .await
}

pub async fn delete(
    creation_time: &DateTime<Utc>,
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

pub async fn read_all_for_post(
    post_creation_time: &DateTime<Utc>,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<Comment>, sqlx::Error> {
    sqlx::query_as!(
        Comment,
        r#"
            SELECT * FROM "comment" WHERE "post_creation_time" = $1
        "#,
        post_creation_time
    )
    .fetch_all(database_connection)
    .await
}
