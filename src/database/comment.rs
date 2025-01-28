use crate::feature::comment::Comment;

use super::DATABASE_CONNECTIONS;

pub async fn create(
    user_id: &i64,
    post_id: &i64,
    comment: &String,
) -> Result<Comment, sqlx::Error> {
    sqlx::query_as!(
        Comment,
        r#"
            INSERT INTO "comment"(user_id, post_id, comment)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        user_id,
        post_id,
        comment,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read(comment_id: &i64) -> Result<Comment, sqlx::Error> {
    sqlx::query_as!(
        Comment,
        r#"
            SELECT * FROM "comment" WHERE "comment_id" = $1
        "#,
        comment_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn update(
    comment_id: &i64,
    user_id: &i64,
    comment: &String,
) -> Result<Comment, sqlx::Error> {
    sqlx::query_as!(
        Comment,
        r#"
        UPDATE "comment" SET "comment" = $3 WHERE "comment_id" = $1 AND "user_id" = $2
        RETURNING *
    "#,
        comment_id,
        user_id,
        comment
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn delete(comment_id: &i64, user_id: &i64) -> Result<Comment, sqlx::Error> {
    sqlx::query_as!(
        Comment,
        r#"
        DELETE FROM "comment" WHERE "comment_id" = $1 AND "user_id" = $2
        RETURNING *
    "#,
        comment_id,
        user_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all_for_post(post_id: &i64) -> Result<Vec<Comment>, sqlx::Error> {
    sqlx::query_as!(
        Comment,
        r#"
            SELECT * FROM "comment" WHERE "post_id" = $1
        "#,
        post_id,
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}
