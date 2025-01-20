use serde::{Deserialize, Serialize};

use crate::database::contact;

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub id: i64,
    pub name: String,
}

impl Contact {
    pub async fn create(name: &String) -> Result<Contact, sqlx::Error> {
        contact::create(name).await
    }

    pub async fn read(id: &i64) -> Result<Contact, sqlx::Error> {
        contact::read(id).await
    }

    pub async fn update(id: &i64, name: &String) -> Result<Contact, sqlx::Error> {
        contact::update(id, name).await
    }

    pub async fn delete(id: &i64) -> Result<Contact, sqlx::Error> {
        contact::delete(id).await
    }

    pub async fn read_all() -> Result<Vec<Contact>, sqlx::Error> {
        contact::read_all().await
    }
}
