pub mod database;
pub mod utils;

use utils::naive_toml_parser;

const DATABASE_CONFIG_FILE_LOCATION: &str = "./configs/database_config.toml";
const SERVER_CONFIG_FILE_LOCATION: &str = "./configs/server_config.toml";

#[derive(Debug)]
pub struct DatabaseConfig {
    pub address: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}
impl Default for DatabaseConfig {
    fn default() -> Self {
        let (header, mut database_configs) = naive_toml_parser(DATABASE_CONFIG_FILE_LOCATION);

        if header == "[database_config]" {
            Self {
                database: database_configs.pop().unwrap(),
                namespace: database_configs.pop().unwrap(),
                password: database_configs.pop().unwrap(),
                username: database_configs.pop().unwrap(),
                address: database_configs.pop().unwrap(),
            }
        } else {
            panic!("Database Config File Must Include [database_config] at the First Line")
        }
    }
}

#[derive(Debug)]
pub struct ServerConfig {
    pub address: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let (header, mut server_configs) = naive_toml_parser(SERVER_CONFIG_FILE_LOCATION);

        if header == "[server_config]" {
            Self {
                address: server_configs.pop().unwrap(),
            }
        } else {
            panic!("Server Config File Must Include [server_config] at the First Line")
        }
    }
}
