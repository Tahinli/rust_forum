use crate::feature::comment_interaction::CommentInteraction;

use super::DATABASE_CONNECTIONS;

pub async fn create(
    comment_id: &i64,
    user_id: &i64,
    interaction_id: &i64,
) -> Result<CommentInteraction, sqlx::Error> {
    sqlx::query_as!(
        CommentInteraction,
        r#"
            INSERT INTO "comment_interaction"(comment_id, user_id, interaction_id)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        comment_id,
        user_id,
        interaction_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read(comment_id: &i64, user_id: &i64) -> Result<CommentInteraction, sqlx::Error> {
    sqlx::query_as!(
        CommentInteraction,
        r#"
            SELECT * FROM "comment_interaction" WHERE "comment_id" = $1 AND "user_id" = $2
        "#,
        comment_id,
        user_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn update(
    comment_id: &i64,
    user_id: &i64,
    interaction_id: &i64,
) -> Result<CommentInteraction, sqlx::Error> {
    sqlx::query_as!(
        CommentInteraction,
        r#"
        UPDATE "comment_interaction" SET "interaction_id" = $3 WHERE "comment_id" = $1 AND "user_id" = $2
        RETURNING *
    "#,
        comment_id,
        user_id,
        interaction_id
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn delete(comment_id: &i64, user_id: &i64) -> Result<CommentInteraction, sqlx::Error> {
    sqlx::query_as!(
        CommentInteraction,
        r#"
        DELETE FROM "comment_interaction" WHERE "comment_id" = $1 AND "user_id" = $2
        RETURNING *
    "#,
        comment_id,
        user_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all_for_comment(
    comment_id: &i64,
) -> Result<Vec<CommentInteraction>, sqlx::Error> {
    sqlx::query_as!(
        CommentInteraction,
        r#"
            SELECT * FROM "comment_interaction" WHERE "comment_id" = $1
        "#,
        comment_id
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}
