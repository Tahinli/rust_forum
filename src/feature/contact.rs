use serde::{Deserialize, Serialize};

use crate::database::contact;

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub contact_id: i64,
    pub name: String,
}

impl Contact {
    pub async fn create(name: &String) -> Result<Contact, sqlx::Error> {
        contact::create(name).await
    }

    pub async fn read(contact_id: &i64) -> Result<Contact, sqlx::Error> {
        contact::read(contact_id).await
    }

    pub async fn update(contact_id: &i64, name: &String) -> Result<Contact, sqlx::Error> {
        contact::update(contact_id, name).await
    }

    pub async fn delete(contact_id: &i64) -> Result<Contact, sqlx::Error> {
        contact::delete(contact_id).await
    }

    pub async fn read_all() -> Result<Vec<Contact>, sqlx::Error> {
        contact::read_all().await
    }
}
