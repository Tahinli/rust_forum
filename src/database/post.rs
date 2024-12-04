use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

use crate::feature::post::Post;

pub async fn create(
    creation_time: &DateTime<Utc>,
    poster_id: i64,
    post: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
            INSERT INTO "post"(creation_time, poster_id, post) 
            VALUES ($1, $2, $3) 
            RETURNING *
        "#,
        creation_time,
        poster_id,
        post
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(
    creation_time: &DateTime<Utc>,
    database_connection: &Pool<Postgres>,
) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
            SELECT * FROM "post" WHERE "creation_time" = $1
        "#,
        creation_time
    )
    .fetch_one(database_connection)
    .await
}

pub async fn update(
    creation_time: &DateTime<Utc>,
    poster_id: i64,
    post: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
        UPDATE "post" SET poster_id = $1, post = $2 WHERE "creation_time" = $3
        RETURNING *
    "#,
        poster_id,
        post,
        creation_time
    )
    .fetch_one(database_connection)
    .await
}

pub async fn delete(
    creation_time: &DateTime<Utc>,
    database_connection: &Pool<Postgres>,
) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
        DELETE FROM "post" where creation_time = $1
        RETURNING *
    "#,
        creation_time
    )
    .fetch_one(database_connection)
    .await
}
