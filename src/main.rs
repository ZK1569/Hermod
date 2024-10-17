use std::{io, process};

use env_logger::Env;
use log::{debug, error, info};
use models::{client::Client, network::Network, server::Server};
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
        "info"
    }))
    .init();

    starter::start_message();
    debug!("Inputed commands : {}", command);

    match command.execution_mod {
        commands::ExecMod::Server(server_info) => {
            let ip = match Network::get_local_ip() {
                Ok(ip) => ip,
                Err(err) => {
                    error!("The ip address is not accessible, please check that you are on a network... {}", err);
                    process::exit(1);
                }
            };
            let server = Server::new(ip, &server_info.port.to_string());

            match server.run_sever() {
                Ok(_) => info!("No errors encountered"),
                Err(err) => {
                    if err.kind() == io::ErrorKind::ConnectionRefused {
                        error!("Server failed to start... \n{err}");
                        process::exit(1);
                    }
                    error!("An error has occurred... \n{}", err)
                }
            }
        }
        commands::ExecMod::Client(client_info) => {
            let client = Client::new(client_info.address, &client_info.port.to_string());

            match client.run_client() {
                Ok(_) => info!("No errors encountered"),
                Err(err) => {
                    if err.kind() == io::ErrorKind::ConnectionRefused {
                        error!("Server connection failure... \n{err}");
                        process::exit(1);
                    }
                    error!("An error has occurred... \n{}", err)
                }
            }
        }
    }
}