use serde::{Deserialize, Serialize};

use crate::database::role;

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub role_id: i64,
    pub name: String,
}

impl Role {
    pub async fn create(name: &String) -> Result<Role, sqlx::Error> {
        role::create(name).await
    }

    pub async fn read(role_id: &i64) -> Result<Role, sqlx::Error> {
        role::read(role_id).await
    }

    pub async fn update(role_id: &i64, name: &String) -> Result<Role, sqlx::Error> {
        role::update(role_id, name).await
    }

    pub async fn delete(role_id: &i64) -> Result<Role, sqlx::Error> {
        role::delete(role_id).await
    }

    pub async fn read_all() -> Result<Vec<Role>, sqlx::Error> {
        role::read_all().await
    }
}
