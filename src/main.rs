use std::process;

use env_logger::Env;
use log::{debug, error, warn};
use models::server::Server;
use utils::{commands, starter};

mod models;
mod tests;
mod types;
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

    match command.execution_mod {
        commands::ExecMod::Server(server_info) => {
            let server = Server::new(server_info.port.to_string());

            let listener_result = server.start_server();

            let listener = match listener_result {
                Ok(r) => r,
                Err(err) => {
                    error!("Server failed to start ... \n{err}");
                    process::exit(1);
                }
            };
            for stream in listener.incoming() {
                match stream {
                    Ok(mut s) => match Server::handle_client(&mut s) {
                        Ok(_) => {}
                        Err(err) => warn!("Data received from the client has a probleme! {}", err),
                    },
                    Err(err) => {
                        warn!("Something went wrong with a client ! {}", err)
                    }
                }
            }
        }
        commands::ExecMod::Client(client) => {
            todo!()
        }
    }
}
