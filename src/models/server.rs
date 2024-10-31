use std::{
    io,
    net::{Ipv4Addr, TcpListener, TcpStream},
};

use log::{debug, error, info, warn};

use crate::{
    models::encrypt::Encrypt,
    types::communication::{Communication, PasswordState},
};

use super::network::Network;

const MAX_PASSWORD_ERRORS: u8 = 3;

pub struct Server {
    pub network: Network,
}

impl Server {
    pub fn new(port: &str, localhost: bool) -> Result<Server, io::Error> {
        let ip = if localhost {
            Ipv4Addr::new(127, 0, 0, 1)
        } else {
            match Network::get_local_ip() {
                Ok(ip) => ip,
                Err(err) => {
                    debug!("{}", err);
                    return Err(io::Error::new(io::ErrorKind::AddrNotAvailable, "The ip address is not accessible, please check that you are on a network..." ));
                }
            }
        };
        Ok(Server {
            network: Network::new(ip, port),
        })
    }

    pub fn start_server(&self) -> Result<TcpListener, io::Error> {
        TcpListener::bind(self.network.get_fulladdress())
    }

    pub fn run_sever(&self) -> Result<(), io::Error> {
        let (hash, password) = match Server::choose_password() {
            Ok((h, p)) => (h, p),
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "It was not possible to calculate the password hash",
                ))
            }
        };

        let key = Encrypt::derive_key_from_password(&password, 10000);

        debug!("Key: {:?}", key);

        let listener = match self.start_server() {
            Ok(r) => r,
            Err(err) => {
                error!("Server failed to start... \n{err}");
                return Err(io::Error::new(io::ErrorKind::ConnectionRefused, err));
            }
        };

        info!(
            "Your server is running on address: {} port: {}",
            self.network.address, self.network.port
        );

        match listener.accept() {
            Ok((mut socket, addr)) => {
                info!("A new customer ({}) is connected ...", addr);
                match Server::check_password(&mut socket, hash) {
                    Ok(_) => {
                        match Network::communication(socket, key) {
                            Ok(_) => warn!("The client has left the conversation"),
                            Err(err) => return Err(err),
                        };
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            Err(e) => {
                error!("{}", e);
            }
        }

        Ok(())
    }

    fn choose_password() -> Result<([u8; 32], String), io::Error> {
        println!("Choose a password that will ensure the security of the conversation: ");

        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read password");

        password.pop(); // INFO: Delete the '\n' at the end

        debug!("the password is >{}<", password);

        let hash = Encrypt::hash(&password)?;

        Ok((hash, password))
    }

    fn check_password(stream: &mut TcpStream, hash: [u8; 32]) -> Result<(), io::Error> {
        let mut errors = 0;
        while errors <= MAX_PASSWORD_ERRORS - 1 {
            match Network::read_message(stream) {
                Ok((communication, data)) => match communication {
                    Communication::CommunicationPassword(_comm_password) => {
                        debug!("Password received {:?}", data);
                        if data == hash {
                            info!("Correct password received");
                            Network::password_response(stream, PasswordState::Correct)?;
                            return Ok(());
                        }
                        if errors < MAX_PASSWORD_ERRORS - 1 {
                            info!("Incorrect password received");
                            Network::password_response(stream, PasswordState::Incorrect)?;
                        }
                        errors = errors + 1;
                    }
                    _ => {
                        Network::password_response(stream, PasswordState::Failed)?;
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Did not receive password",
                        ));
                    }
                },
                Err(err) => {
                    error!("Error password... {}", err);
                    Network::password_response(stream, PasswordState::Failed)?;
                    return Err(err);
                }
            }
        }
        Network::password_response(stream, PasswordState::Failed)?;
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Invalid password",
        ))
    }
}
