use std::process;

use env_logger::Env;
use log::error;
use models::server::Server;

mod models;
mod utils;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let config = utils::config::Config::read();

    let server = Server::new(config.port);

    let listener_result = server.start_server();

    let listener = match listener_result {
        Ok(r) => r,
        Err(err) => {
            error!("Server failed to start ... \n{err}");
            process::exit(1);
        }
    };
}
