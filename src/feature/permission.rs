use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::database::permission;

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub id: i64,
    pub name: String,
}

impl Permission {
    pub async fn create(
        name: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Permission, sqlx::Error> {
        permission::create(name, database_connection).await
    }

    pub async fn read(
        id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Permission, sqlx::Error> {
        permission::read(id, database_connection).await
    }

    pub async fn update(
        id: &i64,
        name: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Permission, sqlx::Error> {
        permission::update(id, name, database_connection).await
    }

    pub async fn delete(
        id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Permission, sqlx::Error> {
        permission::delete(id, database_connection).await
    }

    pub async fn read_all(
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<Permission>, sqlx::Error> {
        permission::read_all(database_connection).await
    }
}
