use chrono::NaiveDate;
use sqlx::{Pool, Postgres};

use crate::feature::user::User;

pub async fn create(
    name: &String,
    surname: &String,
    gender: &bool,
    birth_date: &NaiveDate,
    database_connection: &Pool<Postgres>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            INSERT INTO "user"(name, surname, gender, birth_date, role_id) 
            VALUES ($1, $2, $3, $4, $5) 
            RETURNING *
        "#,
        name,
        surname,
        gender,
        birth_date,
        2
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(id: &i64, database_connection: &Pool<Postgres>) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE "id" = $1
        "#,
        id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn update(
    id: &i64,
    name: &String,
    surname: &String,
    gender: &bool,
    birth_date: &NaiveDate,
    role_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User,
    r#"
        UPDATE "user" SET "name" = $2, "surname" = $3, "gender" = $4, "birth_date" = $5, "role_id" = $6 WHERE "id" = $1
        RETURNING *
    "#, id, name, surname, gender, birth_date, role_id).fetch_one(database_connection).await
}

pub async fn delete(id: &i64, database_connection: &Pool<Postgres>) -> Result<User, sqlx::Error> {
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

pub async fn read_all_for_name(
    name: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE "name" = $1
        "#,
        name
    )
    .fetch_all(database_connection)
    .await
}

pub async fn read_all_for_surname(
    surname: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE "surname" = $1
        "#,
        surname
    )
    .fetch_all(database_connection)
    .await
}

pub async fn read_all_for_birth_date(
    birth_date: &NaiveDate,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE "birth_date" = $1
        "#,
        birth_date
    )
    .fetch_all(database_connection)
    .await
}

pub async fn read_all_for_role(
    role_id: &i64,
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

pub async fn read_all_for_gender(
    gender: &bool,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE "gender" = $1
        "#,
        gender
    )
    .fetch_all(database_connection)
    .await
}

pub async fn read_all_id(database_connection: &Pool<Postgres>) -> Result<Vec<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
            SELECT "id" FROM "user"
        "#,
    )
    .fetch_all(database_connection)
    .await?
    .iter()
    .map(|record| record.id)
    .collect::<Vec<i64>>())
}

pub async fn read_all_id_for_role(
    role_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
            SELECT "id" FROM "user" WHERE "role_id" = $1
        "#,
        role_id
    )
    .fetch_all(database_connection)
    .await?
    .iter()
    .map(|record| record.id)
    .collect::<Vec<i64>>())
}

pub async fn read_all_id_for_gender(
    gender: &bool,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
            SELECT "id" FROM "user" WHERE "gender" = $1
        "#,
        gender
    )
    .fetch_all(database_connection)
    .await?
    .iter()
    .map(|record| record.id)
    .collect::<Vec<i64>>())
}

pub async fn read_all_id_for_name(
    name: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
            SELECT "id" FROM "user" WHERE "name" = $1
        "#,
        name
    )
    .fetch_all(database_connection)
    .await?
    .iter()
    .map(|record| record.id)
    .collect::<Vec<i64>>())
}

pub async fn read_all_id_for_surname(
    surname: &String,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
            SELECT "id" FROM "user" WHERE "surname" = $1
        "#,
        surname
    )
    .fetch_all(database_connection)
    .await?
    .iter()
    .map(|record| record.id)
    .collect::<Vec<i64>>())
}

pub async fn read_all_id_for_birth_date(
    birth_date: &NaiveDate,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
            SELECT "id" FROM "user" WHERE "birth_date" = $1
        "#,
        birth_date
    )
    .fetch_all(database_connection)
    .await?
    .iter()
    .map(|record| record.id)
    .collect::<Vec<i64>>())
}

pub async fn count_all(database_connection: &Pool<Postgres>) -> Result<u64, sqlx::Error> {
    sqlx::query!(
        r#"
            SELECT COUNT(id) FROM "user"
        "#,
    )
    .fetch_one(database_connection)
    .await?
    .count
    .map_or(0, |count| count)
    .try_into()
    .or(Ok(0))
}

pub async fn count_all_for_gender(
    gender: &bool,
    database_connection: &Pool<Postgres>,
) -> Result<u64, sqlx::Error> {
    sqlx::query!(
        r#"
            SELECT COUNT(id) FROM "user" WHERE "gender" = $1
        "#,
        gender
    )
    .fetch_one(database_connection)
    .await?
    .count
    .map_or(0, |count| count)
    .try_into()
    .or(Ok(0))
}

pub async fn count_all_for_role(
    role_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<u64, sqlx::Error> {
    sqlx::query!(
        r#"
            SELECT COUNT(id) FROM "user" WHERE "role_id" = $1
        "#,
        role_id
    )
    .fetch_one(database_connection)
    .await?
    .count
    .map_or(0, |count| count)
    .try_into()
    .or(Ok(0))
}

pub async fn count_all_for_name(
    name: &String,
    database_connection: &Pool<Postgres>,
) -> Result<u64, sqlx::Error> {
    sqlx::query!(
        r#"
            SELECT COUNT(id) FROM "user" WHERE "name" = $1
        "#,
        name
    )
    .fetch_one(database_connection)
    .await?
    .count
    .map_or(0, |count| count)
    .try_into()
    .or(Ok(0))
}

pub async fn count_all_for_surname(
    surname: &String,
    database_connection: &Pool<Postgres>,
) -> Result<u64, sqlx::Error> {
    sqlx::query!(
        r#"
            SELECT COUNT(id) FROM "user" WHERE "surname" = $1
        "#,
        surname
    )
    .fetch_one(database_connection)
    .await?
    .count
    .map_or(0, |count| count)
    .try_into()
    .or(Ok(0))
}

pub async fn count_all_for_birth_date(
    birth_date: &NaiveDate,
    database_connection: &Pool<Postgres>,
) -> Result<u64, sqlx::Error> {
    sqlx::query!(
        r#"
            SELECT COUNT(id) FROM "user" WHERE "birth_date" = $1
        "#,
        birth_date
    )
    .fetch_one(database_connection)
    .await?
    .count
    .map_or(0, |count| count)
    .try_into()
    .or(Ok(0))
}
