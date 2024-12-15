use sqlx::{Pool, Postgres};

use crate::feature::permission::Permission;

pub async fn create(
    name: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Permission, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
            INSERT INTO "permission"(name)
            VALUES ($1)
            RETURNING *
        "#,
        name,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(
    id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Permission, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
            SELECT * FROM "permission" WHERE "id" = $1
        "#,
        id,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn update(
    id: &i64,
    name: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Permission, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
        UPDATE "permission" SET "name" = $2 WHERE "id" = $1
        RETURNING *
    "#,
        id,
        name,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn delete(
    id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Permission, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
        DELETE FROM "permission" WHERE "id" = $1
        RETURNING *
    "#,
        id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read_all(
    database_connection: &Pool<Postgres>,
) -> Result<Vec<Permission>, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
            SELECT * FROM "permission"
        "#,
    )
    .fetch_all(database_connection)
    .await
}
