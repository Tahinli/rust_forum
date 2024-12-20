use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

use crate::feature::post::Post;

pub async fn create(
    user_id: &i64,
    post: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
            INSERT INTO "post"(user_id, post) 
            VALUES ($1, $2) 
            RETURNING *
        "#,
        user_id,
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
    user_id: &i64,
    post: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
        UPDATE "post" SET user_id = $2, post = $3 WHERE "creation_time" = $1
        RETURNING *
    "#,
        creation_time,
        user_id,
        post
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
        DELETE FROM "post" WHERE "creation_time" = $1
        RETURNING *
    "#,
        creation_time
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read_all(database_connection: &Pool<Postgres>) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
            SELECT * FROM "post"
        "#,
    )
    .fetch_all(database_connection)
    .await
}

pub async fn read_all_for_user(
    user_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
            SELECT * FROM "post" WHERE "user_id" = $1
        "#,
        user_id
    )
    .fetch_all(database_connection)
    .await
}
