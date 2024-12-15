use sqlx::{Pool, Postgres};

use crate::feature::role_permission::RolePermission;

pub async fn create(
    role_id: &i64,
    permission_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<RolePermission, sqlx::Error> {
    sqlx::query_as!(
        RolePermission,
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
) -> Result<RolePermission, sqlx::Error> {
    sqlx::query_as!(
        RolePermission,
        r#"
            SELECT * FROM "role_permission" WHERE "role_id" = $1 AND "permission_id" = $2
        "#,
        role_id,
        permission_id,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn update(
    role_id: &i64,
    permission_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<RolePermission, sqlx::Error> {
    sqlx::query_as!(
        RolePermission,
        r#"
        UPDATE "role_permission" SET "permission_id" = $2 WHERE "role_id" = $1
        RETURNING *
    "#,
        role_id,
        permission_id,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn delete(
    role_id: &i64,
    permission_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<RolePermission, sqlx::Error> {
    sqlx::query_as!(
        RolePermission,
        r#"
        DELETE FROM "role_permission" WHERE "role_id" = $1 AND "permission_id" = $2
        RETURNING *
    "#,
        role_id,
        permission_id,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read_all_for_role(
    role_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<RolePermission>, sqlx::Error> {
    sqlx::query_as!(
        RolePermission,
        r#"
            SELECT * FROM "role_permission" WHERE "role_id" = $1
        "#,
        role_id
    )
    .fetch_all(database_connection)
    .await
}

pub async fn delete_all_for_role(
    role_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<RolePermission>, sqlx::Error> {
    sqlx::query_as!(
        RolePermission,
        r#"
        DELETE FROM "role_permission" WHERE "role_id" = $1
        RETURNING *
    "#,
        role_id,
    )
    .fetch_all(database_connection)
    .await
}
