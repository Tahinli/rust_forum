use sqlx::{Pool, Postgres};

use crate::feature::permission::Permission;

pub async fn create(
    role_id: &i64,
    permission_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Permission, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
            INSERT INTO "role_permission"(role_id, permission_id) 
            VALUES ($1, $2) 
            RETURNING *
        "#,
        role_id,
        permission_id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(
    role_id: &i64,
    permission_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Permission, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
            SELECT * FROM "role_permission" WHERE "role_id" = $1 AND "permission_id" = $2
        "#,
        role_id,
        permission_id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn update(
    role_id: &i64,
    permission_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Permission, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
        UPDATE "role_permission" SET "permission_id" = $2 WHERE "role_id" = $1
        RETURNING *
    "#,
        role_id,
        permission_id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn delete(
    role_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Permission, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
        DELETE FROM "role_permission" where "role_id" = $1
        RETURNING *
    "#,
        role_id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read_all(
    role_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<Permission>, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
            SELECT * FROM "role_permission" WHERE "role_id" = $1
        "#,
        role_id,
    )
    .fetch_all(database_connection)
    .await
}
