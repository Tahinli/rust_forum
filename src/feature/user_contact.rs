use serde::{Deserialize, Serialize};

use crate::database::user_contact;

use super::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserContact {
    pub user_id: i64,
    pub contact_id: i64,
    pub contact_value: String,
}

impl UserContact {
    pub async fn create(
        user: &User,
        contact_id: &i64,
        contact_value: &String,
    ) -> Result<UserContact, sqlx::Error> {
        user_contact::create(&user.user_id, contact_id, contact_value).await
    }

    pub async fn read(user: &User, contact_id: &i64) -> Result<UserContact, sqlx::Error> {
        user_contact::read(&user.user_id, contact_id).await
    }

    pub async fn read_for_value(
        contact_id: &i64,
        contact_value: &String,
    ) -> Result<UserContact, sqlx::Error> {
        user_contact::read_for_value(contact_id, contact_value).await
    }

    pub async fn update(
        user: &User,
        contact_id: &i64,
        contact_value: &String,
    ) -> Result<UserContact, sqlx::Error> {
        user_contact::update(&user.user_id, contact_id, contact_value).await
    }

    pub async fn delete(user: &User, contact_id: &i64) -> Result<UserContact, sqlx::Error> {
        user_contact::delete(&user.user_id, contact_id).await
    }

    pub async fn read_all_for_user(user: &User) -> Result<Vec<UserContact>, sqlx::Error> {
        user_contact::read_all_for_user(&user.user_id).await
    }

    pub async fn delete_all_for_user(user: &User) -> Result<Vec<UserContact>, sqlx::Error> {
        user_contact::delete_all_for_user(&user.user_id).await
    }
}
