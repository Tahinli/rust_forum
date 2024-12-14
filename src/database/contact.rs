use sqlx::{Pool, Postgres};

use crate::feature::contact::Contact;

pub async fn create(
    email: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Contact, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
            INSERT INTO "contact"(email) 
            VALUES ($1) 
            RETURNING *
        "#,
        email,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(
    user_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Contact, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
            SELECT * FROM "contact" WHERE "user_id" = $1
        "#,
        user_id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn update(
    user_id: &i64,
    email: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Contact, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
        UPDATE "contact" SET "email" = $2 WHERE "user_id" = $1
        RETURNING *
    "#,
        user_id,
        email,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn delete(
    user_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Contact, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
        DELETE FROM "contact" where "user_id" = $1
        RETURNING *
    "#,
        user_id
    )
    .fetch_one(database_connection)
    .await
}
