use serde::{Deserialize, Serialize};

use crate::database::user_contact;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserContact {
    pub user_id: i64,
    pub contact_id: i64,
    pub contact_value: String,
}

impl UserContact {
    pub async fn create(
        user_id: &i64,
        contact_id: &i64,
        contact_value: &String,
    ) -> Result<UserContact, sqlx::Error> {
        user_contact::create(user_id, contact_id, contact_value).await
    }

    pub async fn read(user_id: &i64, contact_id: &i64) -> Result<UserContact, sqlx::Error> {
        user_contact::read(user_id, contact_id).await
    }

    pub async fn update(
        user_id: &i64,
        contact_id: &i64,
        contact_value: &String,
    ) -> Result<UserContact, sqlx::Error> {
        user_contact::update(user_id, contact_id, contact_value).await
    }

    pub async fn delete(user_id: &i64, contact_id: &i64) -> Result<UserContact, sqlx::Error> {
        user_contact::delete(user_id, contact_id).await
    }

    pub async fn read_all_for_user(user_id: &i64) -> Result<Vec<UserContact>, sqlx::Error> {
        user_contact::read_all_for_user(user_id).await
    }

    pub async fn delete_all_for_user(user_id: &i64) -> Result<Vec<UserContact>, sqlx::Error> {
        user_contact::delete_all_for_user(user_id).await
    }
}
