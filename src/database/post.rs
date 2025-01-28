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
        post,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read(post_id: &i64) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
            SELECT * FROM "post" WHERE "post_id"= $1
        "#,
        post_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn update(post_id: &i64, user_id: &i64, post: &String) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
        UPDATE "post" SET post = $3 WHERE "post_id" = $1 AND "user_id" = $2
        RETURNING *
    "#,
        post_id,
        user_id,
        post,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn delete(post_id: &i64, user_id: &i64) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
        DELETE FROM "post" WHERE "post_id" = $1 AND "user_id" = $2
        RETURNING *
    "#,
        post_id,
        user_id,
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
