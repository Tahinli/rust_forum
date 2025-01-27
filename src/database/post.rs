use chrono::{DateTime, Utc};

use crate::feature::post::Post;

use super::DATABASE_CONNECTIONS;

pub async fn create(user_id: &i64, post: &String) -> Result<Post, sqlx::Error> {
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
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read(user_id: &i64, creation_time: &DateTime<Utc>) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
            SELECT * FROM "post" WHERE "user_id"= $1 AND "creation_time" = $2
        "#,
        user_id,
        creation_time
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn update(
    user_id: &i64,
    creation_time: &DateTime<Utc>,
    post: &String,
) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
        UPDATE "post" SET post = $3 WHERE "user_id" = $1 AND "creation_time" = $2
        RETURNING *
    "#,
        user_id,
        creation_time,
        post
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn delete(user_id: &i64, creation_time: &DateTime<Utc>) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
        DELETE FROM "post" WHERE "user_id" = $1 AND "creation_time" = $2
        RETURNING *
    "#,
        user_id,
        creation_time
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all() -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
            SELECT * FROM "post"
        "#,
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all_for_user(user_id: &i64) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
            SELECT * FROM "post" WHERE "user_id" = $1
        "#,
        user_id
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}
