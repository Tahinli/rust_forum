use crate::feature::user_contact::UserContact;

use super::DATABASE_CONNECTIONS;

pub async fn create(user_id: &i64, contact_id: &i64) -> Result<UserContact, sqlx::Error> {
    sqlx::query_as!(
        UserContact,
        r#"
            INSERT INTO "user_contact"(user_id, contact_id)
            VALUES ($1, $2)
            RETURNING *
        "#,
        user_id,
        contact_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read(user_id: &i64, contact_id: &i64) -> Result<UserContact, sqlx::Error> {
    sqlx::query_as!(
        UserContact,
        r#"
            SELECT * FROM "user_contact" WHERE "user_id" = $1 AND "contact_id" = $2
        "#,
        user_id,
        contact_id
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn update(user_id: &i64, contact_id: &i64) -> Result<UserContact, sqlx::Error> {
    sqlx::query_as!(
        UserContact,
        r#"
        UPDATE "user_contact" SET "contact_id" = $2 WHERE "user_id" = $1
        RETURNING *
    "#,
        user_id,
        contact_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn delete(user_id: &i64, contact_id: &i64) -> Result<UserContact, sqlx::Error> {
    sqlx::query_as!(
        UserContact,
        r#"
        DELETE FROM "user_contact" WHERE "user_id" = $1 AND "contact_id" = $2
        RETURNING *
    "#,
        user_id,
        contact_id,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all_for_user(user_id: &i64) -> Result<Vec<UserContact>, sqlx::Error> {
    sqlx::query_as!(
        UserContact,
        r#"
            SELECT * FROM "user_contact" WHERE "user_id" = $1
        "#,
        user_id,
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn delete_all_for_user(user_id: &i64) -> Result<Vec<UserContact>, sqlx::Error> {
    sqlx::query_as!(
        UserContact,
        r#"
        DELETE FROM "user_contact" WHERE "user_id" = $1
        RETURNING *
    "#,
        user_id,
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}
