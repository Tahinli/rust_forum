use crate::feature::login::Login;

use super::DATABASE_CONNECTIONS;

pub async fn create(user_id: &i64, token: &String) -> Result<Login, sqlx::Error> {
    sqlx::query_as!(
        Login,
        r#"
            INSERT INTO "login"(user_id, token)
            VALUES ($1, $2)
            RETURNING *
        "#,
        user_id,
        token,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read(user_id: &i64, token: &String) -> Result<Login, sqlx::Error> {
    sqlx::query_as!(
        Login,
        r#"
            SELECT * FROM "login" WHERE "user_id" = $1 AND "token" = $2
        "#,
        user_id,
        token
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn delete(user_id: &i64, token: &String) -> Result<Login, sqlx::Error> {
    sqlx::query_as!(
        Login,
        r#"
        DELETE FROM "login" WHERE "user_id" = $1 AND "token" = $2
        RETURNING *
    "#,
        user_id,
        token,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all_for_user(user_id: &i64) -> Result<Vec<Login>, sqlx::Error> {
    sqlx::query_as!(
        Login,
        r#"
            SELECT * FROM "login" WHERE "user_id" = $1
        "#,
        user_id,
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn delete_all_for_user(user_id: &i64) -> Result<Vec<Login>, sqlx::Error> {
    sqlx::query_as!(
        Login,
        r#"
        DELETE FROM "login" WHERE "user_id" = $1
        RETURNING *
    "#,
        user_id,
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn count_all_for_user(user_id: &i64) -> Result<u64, sqlx::Error> {
    sqlx::query!(
        r#"
            SELECT COUNT(user_id) FROM "login" WHERE "user_id" = $1
        "#,
        user_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await?
    .count
    .map_or(0, |count| count)
    .try_into()
    .or(Ok(0))
}
