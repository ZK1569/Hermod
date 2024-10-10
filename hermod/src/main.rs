use std::process;

use env_logger::Env;
use log::{debug, error};
use utils::{commands, starter};

mod models;
mod utils;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let command_result = commands::get_commands();
    let command = match command_result {
        Ok(c) => c,
        Err(e) => {
            error!("Something went wrong ... {}", e);
            process::exit(1);
        }
    };

    starter::start_message();
    let _config = utils::config::Config::read();
    debug!("info : {}", command);
}
