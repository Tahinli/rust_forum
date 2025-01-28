use crate::feature::role::Role;

use super::DATABASE_CONNECTIONS;

pub async fn create(name: &String) -> Result<Role, sqlx::Error> {
    sqlx::query_as!(
        Role,
        r#"
            INSERT INTO "role"(name)
            VALUES ($1)
            RETURNING *
        "#,
        name,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read(role_id: &i64) -> Result<Role, sqlx::Error> {
    sqlx::query_as!(
        Role,
        r#"
            SELECT * FROM "role" WHERE "role_id" = $1
        "#,
        role_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn update(role_id: &i64, name: &String) -> Result<Role, sqlx::Error> {
    sqlx::query_as!(
        Role,
        r#"
        UPDATE "role" SET "name" = $2 WHERE "role_id" = $1
        RETURNING *
    "#,
        role_id,
        name,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn delete(role_id: &i64) -> Result<Role, sqlx::Error> {
    sqlx::query_as!(
        Role,
        r#"
        DELETE FROM "role" WHERE "role_id" = $1
        RETURNING *
    "#,
        role_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all() -> Result<Vec<Role>, sqlx::Error> {
    sqlx::query_as!(
        Role,
        r#"
            SELECT * FROM "role"
        "#,
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}
