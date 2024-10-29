use std::{io, process};

use env_logger::Env;
use log::{debug, error, info};
use models::{client::Client, server::Server};
use openssl::{
    encrypt::Encrypter,
    pkey::PKey,
    rsa::{Padding, Rsa},
};
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

    //-----------------------------------

    let keypair = Rsa::generate(2048).unwrap();
    let keypair = PKey::from_rsa(keypair).unwrap();

    let data = b"hello, world!";

    // Encrypt the data with RSA PKCS1
    let mut encrypter = Encrypter::new(&keypair).unwrap();
    encrypter.set_rsa_padding(Padding::PKCS1).unwrap();
    // Create an output buffer
    let buffer_len = encrypter.encrypt_len(data).unwrap();
    let mut encrypted = vec![0; buffer_len];
    // Encrypt and truncate the buffer
    let encrypted_len = encrypter.encrypt(data, &mut encrypted).unwrap();
    encrypted.truncate(encrypted_len);
    debug!("{:?}", keypair);
    debug!("{:?}", encrypted);

    //-----------------------------------

    starter::start_message();
    debug!("Inputed commands : {}", command);

    match command.execution_mod {
        commands::ExecMod::Server(server_info) => {
            let server = match Server::new(&server_info.port.to_string(), server_info.localhost) {
                Ok(s) => s,
                Err(err) => {
                    error!("{}", err);
                    process::exit(1);
                }
            };

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
