use std::process;

use env_logger::Env;
use log::{debug, error, info, warn};
use models::{client::Client, server::Server};
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
                    error!("Server failed to start... \n{err}");
                    process::exit(1);
                }
            };
            for stream in listener.incoming() {
                match stream {
                    Ok(mut s) => match Server::handle_client(&mut s) {
                        Ok(_) => {}
                        Err(err) => warn!(
                            "An unexpected error occurred during communication with the client... \n{}",
                            err
                        ),
                    },
                    Err(err) => {
                        warn!("A strange customer tried to connect... \n{}", err)
                    }
                }
            }
        }
        commands::ExecMod::Client(client_info) => {
            let client = Client::new(
                client_info.address.to_string(),
                client_info.port.to_string(),
            );

            match client.run_client() {
                Ok(_) => info!("No errors encountered"),
                Err(e) => info!("An error has occurred... \n{}", e),
            }
        }
    }
}
