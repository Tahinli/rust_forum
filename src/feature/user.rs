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
    pub user_id: i64,
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

    pub async fn read(
        user_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<User, sqlx::Error> {
        user::read(user_id, database_connection).await
    }

    pub async fn update(
        user_id: &i64,
        name: &String,
        surname: &String,
        gender: &bool,
        birth_date: &NaiveDate,
        role_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<User, sqlx::Error> {
        user::update(
            user_id,
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
        user_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<User, sqlx::Error> {
        user::delete(user_id, database_connection).await
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

    pub async fn is_builder(user: &User) -> bool {
        if user.role_id == 0 {
            true
        } else {
            false
        }
    }

    pub async fn is_admin(user: &User) -> bool {
        if user.role_id == 1 {
            true
        } else {
            false
        }
    }

    pub async fn is_banned(user: &User) -> bool {
        if user.role_id == -1 {
            true
        } else {
            false
        }
    }

    pub async fn is_builder_or_admin(user: &User) -> bool {
        if user.role_id == 0 || user.role_id == 1 {
            true
        } else {
            false
        }
    }

    pub async fn is_self(user: &User, target_user: &User) -> bool {
        if user.user_id == target_user.user_id {
            true
        } else {
            false
        }
    }

    pub async fn is_higher(user: &User, target_user: &User) -> bool {
        if user.user_id >= 0 {
            if user.user_id < target_user.user_id {
                return true;
            }
        }

        false
    }

    pub async fn is_higher_or_self(user: &User, target_user: &User) -> bool {
        if User::is_self(user, target_user).await {
            true
        } else {
            User::is_higher(user, target_user).await
        }
    }
}
