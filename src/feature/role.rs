use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::database::role;

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
}

impl Role {
    pub async fn create(
        name: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Role, sqlx::Error> {
        role::create(name, database_connection).await
    }

    pub async fn read(id: &i64, database_connection: &Pool<Postgres>) -> Result<Role, sqlx::Error> {
        role::read(id, database_connection).await
    }

    pub async fn update(
        id: &i64,
        name: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Role, sqlx::Error> {
        role::update(id, name, database_connection).await
    }

    pub async fn delete(
        id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Role, sqlx::Error> {
        role::delete(id, database_connection).await
    }

    pub async fn read_all(database_connection: &Pool<Postgres>) -> Result<Vec<Role>, sqlx::Error> {
        role::read_all(database_connection).await
    }
}
