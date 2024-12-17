use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::database::user;

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub email: String,
    pub phone: String,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub surname: String,
    pub gender: bool,
    pub birth_date: NaiveDate,
    pub role_id: i64,
    pub creation_time: DateTime<Utc>,
}

impl User {
    pub async fn create(
        name: &String,
        surname: &String,
        gender: &bool,
        birth_date: &NaiveDate,
        database_connection: &Pool<Postgres>,
    ) -> Result<User, sqlx::Error> {
        user::create(name, surname, gender, birth_date, database_connection).await
    }

    pub async fn read(id: &i64, database_connection: &Pool<Postgres>) -> Result<User, sqlx::Error> {
        user::read(id, database_connection).await
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
        user::update(
            id,
            name,
            surname,
            gender,
            birth_date,
            role_id,
            database_connection,
        )
        .await
    }

    pub async fn delete(
        id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<User, sqlx::Error> {
        user::delete(id, database_connection).await
    }

    pub async fn read_all(database_connection: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
        user::read_all(database_connection).await
    }

    pub async fn read_all_for_name(
        name: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<User>, sqlx::Error> {
        user::read_all_for_name(name, database_connection).await
    }

    pub async fn read_all_for_surname(
        surname: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<User>, sqlx::Error> {
        user::read_all_for_surname(surname, database_connection).await
    }

    pub async fn read_all_for_birth_date(
        birth_date: &NaiveDate,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<User>, sqlx::Error> {
        user::read_all_for_birth_date(birth_date, database_connection).await
    }

    pub async fn read_all_for_role(
        role_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<User>, sqlx::Error> {
        user::read_all_for_role(role_id, database_connection).await
    }

    pub async fn read_all_for_gender(
        gender: &bool,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<User>, sqlx::Error> {
        user::read_all_for_gender(gender, database_connection).await
    }

    pub async fn read_all_id(
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<i64>, sqlx::Error> {
        user::read_all_id(database_connection).await
    }

    pub async fn read_all_id_for_name(
        name: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<i64>, sqlx::Error> {
        user::read_all_id_for_name(name, database_connection).await
    }

    pub async fn read_all_id_for_surname(
        surname: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<i64>, sqlx::Error> {
        user::read_all_id_for_surname(surname, database_connection).await
    }

    pub async fn read_all_id_for_birth_date(
        birth_date: &NaiveDate,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<i64>, sqlx::Error> {
        user::read_all_id_for_birth_date(birth_date, database_connection).await
    }

    pub async fn read_all_id_for_role(
        role_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<i64>, sqlx::Error> {
        user::read_all_id_for_role(role_id, database_connection).await
    }

    pub async fn read_all_id_for_gender(
        gender: &bool,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<i64>, sqlx::Error> {
        user::read_all_id_for_gender(gender, database_connection).await
    }

    pub async fn count_all(database_connection: &Pool<Postgres>) -> Result<u64, sqlx::Error> {
        user::count_all(database_connection).await
    }

    pub async fn count_all_for_name(
        name: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<u64, sqlx::Error> {
        user::count_all_for_name(name, database_connection).await
    }

    pub async fn count_all_for_surname(
        surname: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<u64, sqlx::Error> {
        user::count_all_for_surname(surname, database_connection).await
    }

    pub async fn count_all_for_birth_date(
        birth_date: &NaiveDate,
        database_connection: &Pool<Postgres>,
    ) -> Result<u64, sqlx::Error> {
        user::count_all_for_birth_date(birth_date, database_connection).await
    }

    pub async fn count_all_for_role(
        role_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<u64, sqlx::Error> {
        user::count_all_for_role(role_id, database_connection).await
    }

    pub async fn count_all_for_gender(
        gender: &bool,
        database_connection: &Pool<Postgres>,
    ) -> Result<u64, sqlx::Error> {
        user::count_all_for_gender(gender, database_connection).await
    }
}
