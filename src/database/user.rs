use chrono::NaiveDate;

use crate::feature::user::User;

use super::DATABASE_CONNECTIONS;

pub async fn create(
    name: &String,
    surname: &String,
    gender: &bool,
    birth_date: &NaiveDate,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            INSERT INTO "user"(name, surname, gender, birth_date)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        "#,
        name,
        surname,
        gender,
        birth_date,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read(id: &i64) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE "user_id" = $1
        "#,
        id
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn update(
    id: &i64,
    name: &String,
    surname: &String,
    gender: &bool,
    birth_date: &NaiveDate,
    role_id: &i64,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User,
    r#"
        UPDATE "user" SET "name" = $2, "surname" = $3, "gender" = $4, "birth_date" = $5, "role_id" = $6 WHERE "user_id" = $1
        RETURNING *
    "#, id, name, surname, gender, birth_date, role_id).fetch_one(&*DATABASE_CONNECTIONS).await
}

pub async fn delete(id: &i64) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        DELETE FROM "user" WHERE "user_id" = $1
        RETURNING *
    "#,
        id
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all() -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user"
        "#,
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all_for_name(name: &String) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE "name" = $1
        "#,
        name
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all_for_surname(surname: &String) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE "surname" = $1
        "#,
        surname
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all_for_birth_date(birth_date: &NaiveDate) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE "birth_date" = $1
        "#,
        birth_date
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all_for_role(role_id: &i64) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE "role_id" = $1
        "#,
        role_id
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all_for_gender(gender: &bool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM "user" WHERE "gender" = $1
        "#,
        gender
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await
}

pub async fn read_all_id() -> Result<Vec<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
            SELECT "user_id" FROM "user"
        "#,
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await?
    .iter()
    .map(|record| record.user_id)
    .collect::<Vec<i64>>())
}

pub async fn read_all_id_for_name(name: &String) -> Result<Vec<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
            SELECT "user_id" FROM "user" WHERE "name" = $1
        "#,
        name
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await?
    .iter()
    .map(|record| record.user_id)
    .collect::<Vec<i64>>())
}

pub async fn read_all_id_for_surname(surname: &String) -> Result<Vec<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
            SELECT "user_id" FROM "user" WHERE "surname" = $1
        "#,
        surname
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await?
    .iter()
    .map(|record| record.user_id)
    .collect::<Vec<i64>>())
}

pub async fn read_all_id_for_birth_date(birth_date: &NaiveDate) -> Result<Vec<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
            SELECT "user_id" FROM "user" WHERE "birth_date" = $1
        "#,
        birth_date
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await?
    .iter()
    .map(|record| record.user_id)
    .collect::<Vec<i64>>())
}

pub async fn read_all_id_for_role(role_id: &i64) -> Result<Vec<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
            SELECT "user_id" FROM "user" WHERE "role_id" = $1
        "#,
        role_id
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await?
    .iter()
    .map(|record| record.user_id)
    .collect::<Vec<i64>>())
}

pub async fn read_all_id_for_gender(gender: &bool) -> Result<Vec<i64>, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
            SELECT "user_id" FROM "user" WHERE "gender" = $1
        "#,
        gender
    )
    .fetch_all(&*DATABASE_CONNECTIONS)
    .await?
    .iter()
    .map(|record| record.user_id)
    .collect::<Vec<i64>>())
}

pub async fn count_all() -> Result<u64, sqlx::Error> {
    sqlx::query!(
        r#"
            SELECT COUNT(user_id) FROM "user"
        "#,
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await?
    .count
    .map_or(0, |count| count)
    .try_into()
    .or(Ok(0))
}

pub async fn count_all_for_name(name: &String) -> Result<u64, sqlx::Error> {
    sqlx::query!(
        r#"
            SELECT COUNT(user_id) FROM "user" WHERE "name" = $1
        "#,
        name
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await?
    .count
    .map_or(0, |count| count)
    .try_into()
    .or(Ok(0))
}

pub async fn count_all_for_surname(surname: &String) -> Result<u64, sqlx::Error> {
    sqlx::query!(
        r#"
            SELECT COUNT(user_id) FROM "user" WHERE "surname" = $1
        "#,
        surname
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await?
    .count
    .map_or(0, |count| count)
    .try_into()
    .or(Ok(0))
}

pub async fn count_all_for_birth_date(birth_date: &NaiveDate) -> Result<u64, sqlx::Error> {
    sqlx::query!(
        r#"
            SELECT COUNT(user_id) FROM "user" WHERE "birth_date" = $1
        "#,
        birth_date
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await?
    .count
    .map_or(0, |count| count)
    .try_into()
    .or(Ok(0))
}

pub async fn count_all_for_role(role_id: &i64) -> Result<u64, sqlx::Error> {
    sqlx::query!(
        r#"
            SELECT COUNT(user_id) FROM "user" WHERE "role_id" = $1
        "#,
        role_id
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await?
    .count
    .map_or(0, |count| count)
    .try_into()
    .or(Ok(0))
}

pub async fn count_all_for_gender(gender: &bool) -> Result<u64, sqlx::Error> {
    sqlx::query!(
        r#"
            SELECT COUNT(user_id) FROM "user" WHERE "gender" = $1
        "#,
        gender
    )
    .fetch_one(&*DATABASE_CONNECTIONS)
    .await?
    .count
    .map_or(0, |count| count)
    .try_into()
    .or(Ok(0))
}
