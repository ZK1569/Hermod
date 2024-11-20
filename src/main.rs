use std::{io, process};

use env_logger::Env;
use log::{debug, error, info};
use models::{client::Client, encrypt::Encrypt, file_write, server::Server};
use utils::{
    commands::{self, CertificateActions},
    input, starter,
};

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
                    } else if err.kind() == io::ErrorKind::PermissionDenied {
                        error!(
                            "It is not possible to connect to the server because... {}",
                            err
                        );
                    }
                    error!("An error has occurred... \n{}", err)
                }
            }
        }
        commands::ExecMod::Certificate(cert_action) => match cert_action.action {
            CertificateActions::New => {
                let username = match input::input("Full name ") {
                    Ok(u) => u,
                    Err(err) => {
                        error!("Error reading user input... {}", err);
                        process::exit(1);
                    }
                };
                //  country / localisasty
                let email = match input::input("Email ") {
                    Ok(u) => u,
                    Err(err) => {
                        error!("Error reading user input... {}", err);
                        process::exit(1);
                    }
                };
                let country = match input::input("Country [CA]") {
                    Ok(mut u) => {
                        if u.len() != 2 {
                            error!("Country name not valid, default used [CA]");
                            u = "CA".to_owned();
                        }
                        u
                    }
                    Err(err) => {
                        error!("Error reading user input... {}", err);
                        process::exit(1);
                    }
                };
                let locality = match input::input("Locality Name ") {
                    Ok(u) => u,
                    Err(err) => {
                        error!("Error reading user input... {}", err);
                        process::exit(1);
                    }
                };
                let (cert, key_pair) =
                    match Encrypt::mk_ca_cert(&username, &email, &country, &locality) {
                        Ok((c, k)) => (c, k),
                        Err(err) => {
                            error!(
                                "Something went wrong with the certificate generation... {}",
                                err
                            );
                            process::exit(1);
                        }
                    };

                if let Err(e) = file_write::save_certificate(cert) {
                    error!("Error will saving the user's certificate... {}", e);
                }
                if let Err(e) = file_write::save_pvt_key(key_pair) {
                    error!("Error will saving the user's private key... {}", e);
                }
            }

            CertificateActions::Delete => {}
            CertificateActions::See(_) => {}
        },
    }
}
