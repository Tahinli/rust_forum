pub mod comment;
pub mod comment_interaction;
pub mod contact;
pub mod interaction;
pub mod login;
pub mod post;
pub mod post_interaction;
pub mod role;
pub mod user;
pub mod user_contact;

use std::{sync::LazyLock, time::Duration};

use sqlx::{postgres::PgPoolOptions, Connection, Pool, Postgres};
use tokio::time::sleep;

use crate::DatabaseConfig;

static DATABASE_CONNECTIONS: LazyLock<Pool<Postgres>> = LazyLock::new(establish_connection);

pub async fn set_database_up(database_connection: &Pool<Postgres>) {
    sqlx::migrate!().run(database_connection).await.unwrap();
}
pub fn establish_connection() -> Pool<Postgres> {
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
        .connect_lazy(&connection_string)
        .unwrap()
}

pub async fn is_alive() -> bool {
    tokio::select! {
        database_connection = DATABASE_CONNECTIONS.acquire() => {
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
