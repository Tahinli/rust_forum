pub mod database;
pub mod error;
pub mod feature;
pub mod mail;
pub mod routing;
pub mod server;
pub mod utils;

use std::sync::LazyLock;

use utils::naive_toml_parser;

pub static SERVER_CONFIG: LazyLock<ServerConfig> = LazyLock::new(ServerConfig::default);

const DATABASE_CONFIG_FILE_LOCATION: &str = "./configs/database_config.toml";
const SERVER_CONFIG_FILE_LOCATION: &str = "./configs/server_config.toml";

#[derive(Debug)]
pub struct DatabaseConfig {
    pub address: String,
    pub username: String,
    pub password: String,
    pub database: String,
    pub backend: String,
    pub connection_pool_size: u32,
}
impl Default for DatabaseConfig {
    fn default() -> Self {
        let (header, mut database_configs) = naive_toml_parser(DATABASE_CONFIG_FILE_LOCATION);

        if header == "[database_config]" {
            Self {
                address: database_configs.pop_front().unwrap().parse().unwrap(),
                username: database_configs.pop_front().unwrap().parse().unwrap(),
                password: database_configs.pop_front().unwrap().parse().unwrap(),
                database: database_configs.pop_front().unwrap().parse().unwrap(),
                backend: database_configs.pop_front().unwrap().parse().unwrap(),
                connection_pool_size: database_configs.pop_front().unwrap().parse().unwrap(),
            }
        } else {
            panic!("Database Config File Must Include [database_config] at the First Line")
        }
    }
}

#[derive(Debug)]
pub struct ServerConfig {
    pub address: String,
    pub otp_time_limit: usize,
    pub login_token_expiration_time_limit: usize,
    pub login_token_refresh_time_limit: usize,
    pub concurrency_limit: usize,
    pub post_length_limit: usize,
    pub comment_length_limit: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let (header, mut server_configs) = naive_toml_parser(SERVER_CONFIG_FILE_LOCATION);
        let value_or_max = |value: String| value.parse().map_or(usize::MAX, |value| value);
        let value_or_semaphore_max = |value: String| {
            value
                .parse()
                .map_or(tokio::sync::Semaphore::MAX_PERMITS, |value| value)
        };

        if header == "[server_config]" {
            Self {
                address: server_configs.pop_front().unwrap().parse().unwrap(),
                otp_time_limit: value_or_max(server_configs.pop_front().unwrap()),
                login_token_expiration_time_limit: value_or_max(
                    server_configs.pop_front().unwrap(),
                ),
                login_token_refresh_time_limit: value_or_max(server_configs.pop_front().unwrap()),
                concurrency_limit: value_or_semaphore_max(server_configs.pop_front().unwrap()),
                post_length_limit: server_configs.pop_front().unwrap().parse().unwrap(),
                comment_length_limit: server_configs.pop_front().unwrap().parse().unwrap(),
            }
        } else {
            panic!("Server Config File Must Include [server_config] at the First Line")
        }
    }
}
