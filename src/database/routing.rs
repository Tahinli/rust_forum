use sqlx::{Pool, Postgres};

use crate::feature::routing::Routing;

pub async fn create(
    endpoint: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Routing, sqlx::Error> {
    sqlx::query_as!(
        Routing,
        r#"
            INSERT INTO "routing"(endpoint) 
            VALUES ($1) 
            RETURNING *
        "#,
        endpoint,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(id: &i64, database_connection: &Pool<Postgres>) -> Result<Routing, sqlx::Error> {
    sqlx::query_as!(
        Routing,
        r#"
            SELECT * FROM "routing" WHERE "id" = $1
        "#,
        id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn update(
    id: &i64,
    endpoint: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Routing, sqlx::Error> {
    sqlx::query_as!(
        Routing,
        r#"
        UPDATE "routing" SET "endpoint" = $2 WHERE "id" = $1
        RETURNING *
    "#,
        id,
        endpoint,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn delete(
    id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Routing, sqlx::Error> {
    sqlx::query_as!(
        Routing,
        r#"
        DELETE FROM "routing" WHERE "id" = $1
        RETURNING *
    "#,
        id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read_all(database_connection: &Pool<Postgres>) -> Result<Vec<Routing>, sqlx::Error> {
    sqlx::query_as!(
        Routing,
        r#"
            SELECT * FROM "routing"
        "#,
    )
    .fetch_all(database_connection)
    .await
}

pub async fn delete_all(database_connection: &Pool<Postgres>) -> Result<Vec<Routing>, sqlx::Error> {
    sqlx::query_as!(
        Routing,
        r#"
        DELETE FROM "routing"
        RETURNING *
    "#,
    )
    .fetch_all(database_connection)
    .await
}
