use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::database::contact;

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub id: i64,
    pub name: String,
}

impl Contact {
    pub async fn create(
        name: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Contact, sqlx::Error> {
        contact::create(name, database_connection).await
    }

    pub async fn read(
        id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Contact, sqlx::Error> {
        contact::read(id, database_connection).await
    }

    pub async fn update(
        id: &i64,
        name: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Contact, sqlx::Error> {
        contact::update(id, name, database_connection).await
    }

    pub async fn delete(
        id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Contact, sqlx::Error> {
        contact::delete(id, database_connection).await
    }

    pub async fn read_all(
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<Contact>, sqlx::Error> {
        contact::read_all(database_connection).await
    }
}
