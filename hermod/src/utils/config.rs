use log::{debug, trace, warn};
use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Debug)]
pub struct Config {
    pub port: String,
    pub log_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigToml {
    server: Option<ConfigTomlServer>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigTomlServer {
    port: Option<String>,
    log_level: Option<String>,
}

impl Config {
    pub fn read() -> Self {
        let config_filepaths: [&str; 2] = ["./hermod/config.toml", "./config.toml"];

        let mut content: String = "".to_owned();

        for filepath in config_filepaths {
            let result: Result<String, io::Error> = fs::read_to_string(filepath);

            match result {
                Ok(e) => {
                    content = e;
                    break;
                }
                Err(_) => {}
            }
        }

        trace!("File values {content}");

        let config_toml_result = toml::from_str(&content);
        let config_toml = match config_toml_result {
            Ok(r) => r,
            Err(_) => ConfigToml { server: None },
        };

        debug!("{:?}", config_toml);

        let (port, log_level): (String, String) = match config_toml.server {
            Some(server) => {
                let server_port: String = server.port.unwrap_or_else(|| {
                    warn!("Missing field port in table server.");
                    "8080".to_owned()
                });

                let log_level: String = server.log_level.unwrap_or_else(|| {
                    warn!("Missing field port in table server.");
                    "debug".to_owned()
                });

                (server_port, log_level)
            }
            None => {
                warn!("Missing table server, default value will be use.");
                ("8080".to_owned(), "debug".to_owned())
            }
        };

        Config { port, log_level }
    }
}
