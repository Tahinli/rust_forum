use sqlx::{Pool, Postgres};

use crate::feature::role::Role;

pub async fn create(
    name: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Role, sqlx::Error> {
    sqlx::query_as!(
        Role,
        r#"
            INSERT INTO "role"(name) 
            VALUES ($1) 
            RETURNING *
        "#,
        name,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(id: &i64, database_connection: &Pool<Postgres>) -> Result<Role, sqlx::Error> {
    sqlx::query_as!(
        Role,
        r#"
            SELECT * FROM "role" WHERE "id" = $1
        "#,
        id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn update(
    id: &i64,
    name: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Role, sqlx::Error> {
    sqlx::query_as!(
        Role,
        r#"
        UPDATE "role" SET "name" = $1 WHERE "id" = $2
        RETURNING *
    "#,
        name,
        id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn delete(id: &i64, database_connection: &Pool<Postgres>) -> Result<Role, sqlx::Error> {
    sqlx::query_as!(
        Role,
        r#"
        DELETE FROM "role" where "id" = $1
        RETURNING *
    "#,
        id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read_all(database_connection: &Pool<Postgres>) -> Result<Vec<Role>, sqlx::Error> {
    sqlx::query_as!(
        Role,
        r#"
            SELECT * FROM "role"
        "#,
    )
    .fetch_all(database_connection)
    .await
}
