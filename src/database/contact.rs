use sqlx::{Pool, Postgres};

use crate::feature::contact::Contact;

pub async fn create(
    name: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Contact, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
            INSERT INTO "contact"(name) 
            VALUES ($1) 
            RETURNING *
        "#,
        name,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(id: &i64, database_connection: &Pool<Postgres>) -> Result<Contact, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
            SELECT * FROM "contact" WHERE "id" = $1
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
) -> Result<Contact, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
        UPDATE "contact" SET "name" = $2 WHERE "id" = $1
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
) -> Result<Contact, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
        DELETE FROM "contact" WHERE "id" = $1
        RETURNING *
    "#,
        id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read_all(database_connection: &Pool<Postgres>) -> Result<Vec<Contact>, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
            SELECT * FROM "contact"
        "#,
    )
    .fetch_all(database_connection)
    .await
}
