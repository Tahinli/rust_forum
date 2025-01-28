use crate::feature::post_interaction::PostInteraction;

use super::DATABASE_CONNECTIONS;

pub async fn create(
    post_id: &i64,
    user_id: &i64,
    interaction_id: &i64,
) -> Result<PostInteraction, sqlx::Error> {
    sqlx::query_as!(
        PostInteraction,
        r#"
            INSERT INTO "post_interaction"(post_id, user_id, interaction_id)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        post_id,
        user_id,
        interaction_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read(post_id: &i64, user_id: &i64) -> Result<PostInteraction, sqlx::Error> {
    sqlx::query_as!(
        PostInteraction,
        r#"
            SELECT * FROM "post_interaction" WHERE "post_id" = $1 AND "user_id" = $2
        "#,
        post_id,
        user_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn update(
    post_id: &i64,
    user_id: &i64,
    interaction_id: &i64,
) -> Result<PostInteraction, sqlx::Error> {
    sqlx::query_as!(
        PostInteraction,
        r#"
        UPDATE "post_interaction" SET "interaction_id" = $3 WHERE "post_id" = $1 AND "user_id" = $2
        RETURNING *
    "#,
        post_id,
        user_id,
        interaction_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn delete(post_id: &i64, user_id: &i64) -> Result<PostInteraction, sqlx::Error> {
    sqlx::query_as!(
        PostInteraction,
        r#"
        DELETE FROM "post_interaction" WHERE "post_id" = $1 AND "user_id" = $2
        RETURNING *
    "#,
        post_id,
        user_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all_for_post(post_id: &i64) -> Result<Vec<PostInteraction>, sqlx::Error> {
    sqlx::query_as!(
        PostInteraction,
        r#"
            SELECT * FROM "post_interaction" WHERE "post_id" = $1
        "#,
        post_id,
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}
