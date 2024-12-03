use chrono::NaiveDate;
use sqlx::{Pool, Postgres};

use crate::feature::user::{Role, User};

pub async fn create_user(
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
            INSERT INTO "user"(name, surname, gender, birth_date, email) 
            VALUES ($1, $2, $3, $4, $5) 
            RETURNING id, name, surname, gender, birth_date, email, role AS "role:Role"
        "#,
        name,
        surname,
        gender,
        birth_date,
        email
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read_user(
    email: &String,
    database_connection: &Pool<Postgres>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User,
        r#"
            SELECT id, name, surname, gender, birth_date, email, role AS "role:Role" FROM "user" WHERE "email" = $1
        "#, 
        email
    ).fetch_one(database_connection).await
}

pub async fn update_user(
    id: i64,
    name: &String,
    surname: &String,
    gender: &bool,
    birth_date: &NaiveDate,
    email: &String,
    database_connection: &Pool<Postgres>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User,
    r#"
        UPDATE "user" SET "name" = $1, "surname" = $2, "gender" = $3, "birth_date" = $4, "email" = $5 WHERE "id" = $6
        RETURNING id, name, surname, gender, birth_date, email, role AS "role:Role"
    "#, name, surname, gender, birth_date, email, id).fetch_one(database_connection).await
}

pub async fn delete_user(
    id: i64,
    database_connection: &Pool<Postgres>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        DELETE FROM "user" where id = $1
        RETURNING id, name, surname, gender, birth_date, email, role AS "role:Role"
    "#,
        id
    )
    .fetch_one(database_connection)
    .await
}
