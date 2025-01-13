use sqlx::{Pool, Postgres};

use crate::feature::login::Login;

pub async fn create(
    user_id: &i64,
    token: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Login, sqlx::Error> {
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
    .fetch_one(database_connection)
    .await
}

pub async fn read(
    user_id: &i64,
    token: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Login, sqlx::Error> {
    sqlx::query_as!(
        Login,
        r#"
            SELECT * FROM "login" WHERE "user_id" = $1 AND "token" = $2
        "#,
        user_id,
        token
    )
    .fetch_one(database_connection)
    .await
}

pub async fn delete(
    user_id: &i64,
    token: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Login, sqlx::Error> {
    sqlx::query_as!(
        Login,
        r#"
        DELETE FROM "login" WHERE "user_id" = $1 AND "token" = $2
        RETURNING *
    "#,
        user_id,
        token,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read_all_for_user(
    user_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<Login>, sqlx::Error> {
    sqlx::query_as!(
        Login,
        r#"
            SELECT * FROM "login" WHERE "user_id" = $1
        "#,
        user_id,
    )
    .fetch_all(database_connection)
    .await
}

pub async fn delete_all_for_user(
    user_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<Login>, sqlx::Error> {
    sqlx::query_as!(
        Login,
        r#"
        DELETE FROM "login" WHERE "user_id" = $1
        RETURNING *
    "#,
        user_id,
    )
    .fetch_all(database_connection)
    .await
}

pub async fn count_all_for_user(
    user_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<u64, sqlx::Error> {
    sqlx::query!(
        r#"
            SELECT COUNT(user_id) FROM "login" WHERE "user_id" = $1
        "#,
        user_id,
    )
    .fetch_one(database_connection)
    .await?
    .count
    .map_or(0, |count| count)
    .try_into()
    .or(Ok(0))
}
