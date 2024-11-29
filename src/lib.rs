use std::{fs::File, io::Read};

pub mod database;

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
        let mut database_config_file = File::open("./configs/database_config.toml").unwrap();
        let mut database_configs = String::default();
        database_config_file
            .read_to_string(&mut database_configs)
            .unwrap();
        let database_configs = database_configs
            .lines()
            .map(|line| line.trim_end())
            .collect::<Vec<&str>>();

        if database_configs[0] == "[database_config]" {
            Self {
                address: database_configs[1]
                    .split_once('=')
                    .unwrap()
                    .1
                    .replace('"', "")
                    .trim()
                    .to_string(),
                username: database_configs[2]
                    .split_once('=')
                    .unwrap()
                    .1
                    .replace('"', "")
                    .trim()
                    .to_string(),
                password: database_configs[3]
                    .split_once('=')
                    .unwrap()
                    .1
                    .replace('"', "")
                    .trim()
                    .to_string(),
                namespace: database_configs[4]
                    .split_once('=')
                    .unwrap()
                    .1
                    .replace('"', "")
                    .trim()
                    .to_string(),
                database: database_configs[5]
                    .split_once('=')
                    .unwrap()
                    .1
                    .replace('"', "")
                    .trim()
                    .to_string(),
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
        let mut server_config_file = File::open("./configs/server_config.toml").unwrap();
        let mut server_configs = String::default();
        server_config_file
            .read_to_string(&mut server_configs)
            .unwrap();
        let server_configs = server_configs
            .lines()
            .map(|line| line.trim_end())
            .collect::<Vec<&str>>();
        if server_configs[0] == "[server_config]" {
            Self {
                address: server_configs[1]
                    .split_once('=')
                    .unwrap()
                    .1
                    .replace('"', "")
                    .trim()
                    .to_string(),
            }
        } else {
            panic!("Server Config File Must Include [server_config] at the First Line")
        }
    }
}
