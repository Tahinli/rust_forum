pub mod comment;
pub mod interaction;
pub mod post;
pub mod role;
pub mod user;

use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Connection, Pool, Postgres};
use tokio::time::sleep;

use crate::DatabaseConfig;

pub async fn set_database_up(database_connection: &Pool<Postgres>) {
    sqlx::migrate!().run(database_connection).await.unwrap();
}
pub async fn establish_connection() -> Pool<Postgres> {
    let database_config = DatabaseConfig::default();
    let connection_string = format!(
        "{}://{}:{}@{}/{}",
        database_config.backend,
        database_config.username,
        database_config.password,
        database_config.address,
        database_config.database
    );
    PgPoolOptions::new()
        .max_connections(database_config.connection_pool_size)
        .test_before_acquire(false)
        .connect(&connection_string)
        .await
        .unwrap()
}

pub async fn is_alive(database_connection: &Pool<Postgres>) -> bool {
    tokio::select! {
        database_connection = database_connection.acquire() => {
            match database_connection {
                Ok(mut database_connection) => {
                    match database_connection.ping().await {
                        Ok(_) => true,
                        Err(_) => false,
                    }
                },
                Err(_) => false,
            }
        }
        _ = sleep(Duration::from_secs(1)) => false,
    }
}
