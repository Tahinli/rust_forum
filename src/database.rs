use std::{sync::LazyLock, time::Duration};

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tokio::time::sleep;

use crate::DatabaseConfig;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn establish_connection() {
    let database_config = DatabaseConfig::default();

    DB.connect::<Ws>(database_config.address).await.unwrap();
    DB.signin(Root {
        username: &database_config.username,
        password: &database_config.password,
    })
    .await
    .unwrap();
    DB.use_ns(database_config.namespace).await.unwrap();
    DB.use_db(database_config.database).await.unwrap();
}

pub async fn is_alive() -> bool {
    tokio::select! {
        alive_status = DB.health() => {
            match alive_status {
                Ok(_) => true,
                Err(_) => false,
            }
        },
        _ = sleep(Duration::from_secs(1)) => false
    }
}

pub async fn create_post() {}

pub async fn read_post() {}

pub async fn update_post() {}

pub async fn delete_post() {}
