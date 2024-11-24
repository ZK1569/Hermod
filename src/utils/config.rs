use home::home_dir;
use log::{debug, trace, warn};
use serde::{Deserialize, Serialize};
use std::{env::current_dir, fs, io, process};

#[derive(Debug)]
pub struct Config {
    pub user_home_path: String,
    pub config_path: String,
    pub current_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigToml {
    config_path: Option<ConfigFileSave>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFileSave {
    home_path: Option<String>,
}

impl Config {
    pub fn read() -> Self {
        let home_dir = match home_dir() {
            Some(p) => p.to_string_lossy().to_string(),
            None => {
                warn!("User home path unreachable, default value user (./)");
                "./".to_owned()
            }
        };

        let config_filepath: [&str; 1] = ["./config.toml"];

        let mut content: String = "".to_owned();

        for filepath in config_filepath {
            let result: Result<String, io::Error> = fs::read_to_string(filepath);

            match result {
                Ok(e) => {
                    content = e;
                    break;
                }
                Err(_) => {}
            };
        }

        trace!("Value of file {content}");

        let config_toml: ConfigToml = match toml::from_str(&content) {
            Ok(r) => r,
            Err(_) => ConfigToml { config_path: None },
        };

        debug!("{:?}", config_toml);

        let config_path: String = match config_toml.config_path {
            Some(file) => {
                let config_path: String = file.home_path.unwrap_or_else(|| {
                    warn!("Missing field home_path in table config_path.");
                    home_dir
                });

                config_path
            }
            None => {
                warn!("Missing table config_path, default value will be use.");
                home_dir
            }
        };

        let current_path = match current_dir() {
            Ok(c) => c.to_string_lossy().to_string(),
            Err(e) => {
                warn!("It was not possible to obtain the execution path... {e}");
                process::exit(1)
            }
        };

        Config {
            user_home_path: config_path.clone(),
            config_path: config_path + "/.config/hermod",
            current_path,
        }
    }
}
