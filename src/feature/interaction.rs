use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::database::interaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct Interaction {
    pub id: i64,
    pub name: String,
}

impl Interaction {
    pub async fn create(
        name: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Interaction, sqlx::Error> {
        interaction::create(name, database_connection).await
    }

    pub async fn read(
        id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Interaction, sqlx::Error> {
        interaction::read(id, database_connection).await
    }

    pub async fn update(
        id: &i64,
        name: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Interaction, sqlx::Error> {
        interaction::update(id, name, database_connection).await
    }

    pub async fn delete(
        id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Interaction, sqlx::Error> {
        interaction::delete(id, database_connection).await
    }

    pub async fn read_all(
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<Interaction>, sqlx::Error> {
        interaction::read_all(database_connection).await
    }
}
