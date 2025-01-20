use serde::{Deserialize, Serialize};

use crate::database::role;

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
}

impl Role {
    pub async fn create(name: &String) -> Result<Role, sqlx::Error> {
        role::create(name).await
    }

    pub async fn read(id: &i64) -> Result<Role, sqlx::Error> {
        role::read(id).await
    }

    pub async fn update(id: &i64, name: &String) -> Result<Role, sqlx::Error> {
        role::update(id, name).await
    }

    pub async fn delete(id: &i64) -> Result<Role, sqlx::Error> {
        role::delete(id).await
    }

    pub async fn read_all() -> Result<Vec<Role>, sqlx::Error> {
        role::read_all().await
    }
}
