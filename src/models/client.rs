use std::{
    io,
    net::{Ipv4Addr, TcpStream},
};

use log::{debug, error, info, warn};

use crate::types::communication::{Communication, CommunicationPassword, PasswordState};

use super::{encrypt::Enrypt, network::Network};

pub struct Client {
    pub network: Network,
}

impl Client {
    pub fn new(address: Ipv4Addr, port: &str) -> Client {
        Client {
            network: Network::new(address, port),
        }
    }

    pub fn run_client(&self) -> Result<(), io::Error> {
        let mut stream = match self.connect_to_server() {
            Ok(s) => s,
            Err(err) => return Err(io::Error::new(io::ErrorKind::ConnectionRefused, err)),
        };

        info!("Connected to server");

        println!("Please enter the conversation password: ");
        let password = match self.send_password(&mut stream) {
            Ok(p) => p,
            Err(err) => {
                if err.kind() == io::ErrorKind::ConnectionRefused {
                    return Err(err);
                }
                return Err(io::Error::new(io::ErrorKind::PermissionDenied, err));
            }
        };

        match Network::communication(stream) {
            Ok(_) => warn!("The server is disconnected"),
            Err(err) => return Err(err),
        }

        Ok(())
    }

    fn connect_to_server(&self) -> Result<TcpStream, io::Error> {
        TcpStream::connect(self.network.get_fulladdress())
    }

    fn send_password(&self, stream: &mut TcpStream) -> Result<String, io::Error> {
        let mut password = String::new();
        let mut password_error = 0;
        while password_error <= 3 {
            password = String::new();
            io::stdin()
                .read_line(&mut password)
                .expect("Failed to read password");

            password.pop();

            let hash = Enrypt::hash(&password)?;

            Network::send_password(stream, &hash).expect("Failled to send password");

            match Network::read_message(stream) {
                Ok((communication, _)) => match communication {
                    Communication::CommunicationPassword(communication_password) => {
                        debug!("{:?}", communication_password);
                        match communication_password.password_state {
                            PasswordState::Incorrect => {
                                info!("Password Incorrect");
                                password_error = password_error + 1;
                                continue;
                            }
                            PasswordState::Correct => {
                                info!("Correct password");
                                return Ok(password);
                            }
                            PasswordState::Failed => {
                                return Err(io::Error::new(
                                    io::ErrorKind::ConnectionRefused,
                                    "Connection Refused, invalid password",
                                ));
                            }
                            _ => {
                                return Err(io::Error::new(
                                    io::ErrorKind::Other,
                                    "Wrong data received for password",
                                ))
                            }
                        }
                    }
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "Wrong data received, not a password",
                        ))
                    }
                },
                Err(err) => return Err(err),
            }
        }
        Ok(password)
    }
}
