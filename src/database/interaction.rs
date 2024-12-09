use sqlx::{Pool, Postgres};

use crate::feature::interaction::Interaction;

pub async fn create(
    name: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Interaction, sqlx::Error> {
    sqlx::query_as!(
        Interaction,
        r#"
            INSERT INTO "interaction"(name) 
            VALUES ($1) 
            RETURNING *
        "#,
        name,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(
    name: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Interaction, sqlx::Error> {
    sqlx::query_as!(
        Interaction,
        r#"
            SELECT * FROM "interaction" WHERE "name" = $1
        "#,
        name
    )
    .fetch_one(database_connection)
    .await
}

pub async fn update(
    id: &i64,
    name: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Interaction, sqlx::Error> {
    sqlx::query_as!(
        Interaction,
        r#"
        UPDATE "interaction" SET "name" = $1 WHERE "id" = $2
        RETURNING *
    "#,
        name,
        id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn delete(
    id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Interaction, sqlx::Error> {
    sqlx::query_as!(
        Interaction,
        r#"
        DELETE FROM "interaction" where "id" = $1
        RETURNING *
    "#,
        id
    )
    .fetch_one(database_connection)
    .await
}
