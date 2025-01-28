use crate::feature::contact::Contact;

use super::DATABASE_CONNECTIONS;

pub async fn create(name: &String) -> Result<Contact, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
            INSERT INTO "contact"(name)
            VALUES ($1)
            RETURNING *
        "#,
        name,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read(contact_id: &i64) -> Result<Contact, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
            SELECT * FROM "contact" WHERE "contact_id" = $1
        "#,
        contact_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn update(contact_id: &i64, name: &String) -> Result<Contact, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
        UPDATE "contact" SET "name" = $2 WHERE "contact_id" = $1
        RETURNING *
    "#,
        contact_id,
        name,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn delete(contact_id: &i64) -> Result<Contact, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
        DELETE FROM "contact" WHERE "contact_id" = $1
        RETURNING *
    "#,
        contact_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all() -> Result<Vec<Contact>, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
            SELECT * FROM "contact"
        "#,
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}
