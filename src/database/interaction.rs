use crate::feature::interaction::Interaction;

use super::DATABASE_CONNECTIONS;

pub async fn create(name: &String) -> Result<Interaction, sqlx::Error> {
    sqlx::query_as!(
        Interaction,
        r#"
            INSERT INTO "interaction"(name)
            VALUES ($1)
            RETURNING *
        "#,
        name,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read(id: &i64) -> Result<Interaction, sqlx::Error> {
    sqlx::query_as!(
        Interaction,
        r#"
            SELECT * FROM "interaction" WHERE "id" = $1
        "#,
        id
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn update(id: &i64, name: &String) -> Result<Interaction, sqlx::Error> {
    sqlx::query_as!(
        Interaction,
        r#"
        UPDATE "interaction" SET "name" = $2 WHERE "id" = $1
        RETURNING *
    "#,
        id,
        name
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn delete(id: &i64) -> Result<Interaction, sqlx::Error> {
    sqlx::query_as!(
        Interaction,
        r#"
        DELETE FROM "interaction" WHERE "id" = $1
        RETURNING *
    "#,
        id
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all() -> Result<Vec<Interaction>, sqlx::Error> {
    sqlx::query_as!(
        Interaction,
        r#"
            SELECT * FROM "interaction"
        "#,
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}
