use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::database::routing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Routing {
    pub id: i64,
    pub endpoint: String,
}

impl Routing {
    pub async fn create(
        endpoint: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Routing, sqlx::Error> {
        routing::create(endpoint, database_connection).await
    }

    pub async fn read(
        id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Routing, sqlx::Error> {
        routing::read(id, database_connection).await
    }

    pub async fn update(
        id: &i64,
        endpoint: &String,
        database_connection: &Pool<Postgres>,
    ) -> Result<Routing, sqlx::Error> {
        routing::update(id, endpoint, database_connection).await
    }

    pub async fn delete(
        id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Routing, sqlx::Error> {
        routing::delete(id, database_connection).await
    }

    pub async fn read_all(
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<Routing>, sqlx::Error> {
        routing::read_all(database_connection).await
    }

    pub async fn delete_all(
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<Routing>, sqlx::Error> {
        routing::delete_all(database_connection).await
    }
}
