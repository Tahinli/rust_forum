use chrono::NaiveDate;
use sqlx::{Pool, Postgres};

use crate::feature::user::User;

pub async fn create(
    name: &String,
    surname: &String,
    gender: bool,
    birth_date: &NaiveDate,
    email: &String,
    database_connection: &Pool<Postgres>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            INSERT INTO "user"(name, surname, gender, birth_date, email, role_id) 
            VALUES ($1, $2, $3, $4, $5, $6) 
            RETURNING *
        "#,
        name,
        surname,
        gender,
        birth_date,
        email,
        2
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(
    email: &String,
    database_connection: &Pool<Postgres>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE "email" = $1
        "#,
        email
    )
    .fetch_one(database_connection)
    .await
}

pub async fn update(
    id: i64,
    name: &String,
    surname: &String,
    gender: &bool,
    birth_date: &NaiveDate,
    email: &String,
    role_id: i64,
    database_connection: &Pool<Postgres>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User,
    r#"
        UPDATE "user" SET "name" = $1, "surname" = $2, "gender" = $3, "birth_date" = $4, "email" = $5, "role_id" = $6 WHERE "id" = $7
        RETURNING *
    "#, name, surname, gender, birth_date, email, role_id, id).fetch_one(database_connection).await
}

pub async fn delete(id: i64, database_connection: &Pool<Postgres>) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        DELETE FROM "user" where "id" = $1
        RETURNING *
    "#,
        id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read_all(database_connection: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user"
        "#,
    )
    .fetch_all(database_connection)
    .await
}

pub async fn read_all_for_role(
    role_id: i64,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE "role_id" = $1
        "#,
        role_id
    )
    .fetch_all(database_connection)
    .await
}
