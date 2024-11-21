use home::home_dir;
use log::{debug, trace, warn};
use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Debug)]
pub struct Config {
    pub user_home_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigToml {
    home_path: Option<ConfigFileSave>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFileSave {
    dir: Option<String>,
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
            Err(_) => ConfigToml { home_path: None },
        };

        debug!("{:?}", config_toml);

        let config_path: String = match config_toml.home_path {
            Some(file) => {
                let config_path: String = file.dir.unwrap_or_else(|| {
                    warn!("Missing field dir in table config_path.");
                    home_dir
                });

                config_path
            }
            None => {
                warn!("Missing table config_path, default value will be use.");
                home_dir
            }
        };

        Config {
            user_home_path: config_path,
        }
    }
}
