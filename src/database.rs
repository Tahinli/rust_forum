pub mod interaction;
pub mod post;
pub mod user;

pub type SurrealUserReturn = Result<Option<User>, surrealdb::Error>;
pub type SurrealCountReturn = Result<Option<u128>, surrealdb::Error>;

use std::{sync::LazyLock, time::Duration};

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tokio::time::sleep;

use crate::{feature::user::User, DatabaseConfig};

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
    DB.query("DEFINE INDEX email ON TABLE user FIELDS email UNIQUE;").await.unwrap();
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
