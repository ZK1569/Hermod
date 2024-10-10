use std::process;

use env_logger::Env;
use log::{debug, error};
use utils::{commands, starter};

mod models;
mod utils;

fn main() {
    let command_result = commands::get_commands();
    let command = match command_result {
        Ok(c) => c,
        Err(e) => {
            error!("Something went wrong ... {}", e);
            process::exit(1);
        }
    };

    env_logger::Builder::from_env(Env::default().default_filter_or(if command.debug {
        "debug"
    } else {
        "warn"
    }))
    .init();

    starter::start_message();
    debug!("Inputed commands : {}", command);
}
