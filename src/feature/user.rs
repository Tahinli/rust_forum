use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

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
    ) -> Result<User, sqlx::Error> {
        user::create(name, surname, gender, birth_date).await
    }

    pub async fn read(user_id: &i64) -> Result<User, sqlx::Error> {
        user::read(user_id).await
    }

    pub async fn update(
        user_id: &i64,
        name: &String,
        surname: &String,
        gender: &bool,
        birth_date: &NaiveDate,
        role_id: &i64,
    ) -> Result<User, sqlx::Error> {
        user::update(user_id, name, surname, gender, birth_date, role_id).await
    }

    pub async fn delete(user_id: &i64) -> Result<User, sqlx::Error> {
        user::delete(user_id).await
    }

    pub async fn read_all() -> Result<Vec<User>, sqlx::Error> {
        user::read_all().await
    }

    pub async fn read_all_for_name(name: &String) -> Result<Vec<User>, sqlx::Error> {
        user::read_all_for_name(name).await
    }

    pub async fn read_all_for_surname(surname: &String) -> Result<Vec<User>, sqlx::Error> {
        user::read_all_for_surname(surname).await
    }

    pub async fn read_all_for_birth_date(birth_date: &NaiveDate) -> Result<Vec<User>, sqlx::Error> {
        user::read_all_for_birth_date(birth_date).await
    }

    pub async fn read_all_for_role(role_id: &i64) -> Result<Vec<User>, sqlx::Error> {
        user::read_all_for_role(role_id).await
    }

    pub async fn read_all_for_gender(gender: &bool) -> Result<Vec<User>, sqlx::Error> {
        user::read_all_for_gender(gender).await
    }

    pub async fn read_all_id() -> Result<Vec<i64>, sqlx::Error> {
        user::read_all_id().await
    }

    pub async fn read_all_id_for_name(name: &String) -> Result<Vec<i64>, sqlx::Error> {
        user::read_all_id_for_name(name).await
    }

    pub async fn read_all_id_for_surname(surname: &String) -> Result<Vec<i64>, sqlx::Error> {
        user::read_all_id_for_surname(surname).await
    }

    pub async fn read_all_id_for_birth_date(
        birth_date: &NaiveDate,
    ) -> Result<Vec<i64>, sqlx::Error> {
        user::read_all_id_for_birth_date(birth_date).await
    }

    pub async fn read_all_id_for_role(role_id: &i64) -> Result<Vec<i64>, sqlx::Error> {
        user::read_all_id_for_role(role_id).await
    }

    pub async fn read_all_id_for_gender(gender: &bool) -> Result<Vec<i64>, sqlx::Error> {
        user::read_all_id_for_gender(gender).await
    }

    pub async fn count_all() -> Result<u64, sqlx::Error> {
        user::count_all().await
    }

    pub async fn count_all_for_name(name: &String) -> Result<u64, sqlx::Error> {
        user::count_all_for_name(name).await
    }

    pub async fn count_all_for_surname(surname: &String) -> Result<u64, sqlx::Error> {
        user::count_all_for_surname(surname).await
    }

    pub async fn count_all_for_birth_date(birth_date: &NaiveDate) -> Result<u64, sqlx::Error> {
        user::count_all_for_birth_date(birth_date).await
    }

    pub async fn count_all_for_role(role_id: &i64) -> Result<u64, sqlx::Error> {
        user::count_all_for_role(role_id).await
    }

    pub async fn count_all_for_gender(gender: &bool) -> Result<u64, sqlx::Error> {
        user::count_all_for_gender(gender).await
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
