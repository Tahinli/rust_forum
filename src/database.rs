pub mod interaction;
pub mod message;
pub mod post;
pub mod user;

use std::time::Duration;

use sea_orm::{Database, DatabaseConnection};
use tokio::time::sleep;

use crate::DatabaseConfig;

pub async fn establish_connection() -> DatabaseConnection {
    let database_config = DatabaseConfig::default();
    let connection_string = format!(
        "{}://{}:{}@{}/{}",
        database_config.backend,
        database_config.username,
        database_config.password,
        database_config.address,
        database_config.database
    );
    Database::connect(connection_string).await.unwrap()
}

pub async fn is_alive() -> bool {
    tokio::select! {

        _ = sleep(Duration::from_secs(1)) => false,
    }
}
