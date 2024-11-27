use std::{
    fmt::Write,
    io,
    net::{Ipv4Addr, TcpStream},
};

use log::{debug, error, info, warn};
use openssl::x509::X509;

use crate::{
    types::communication::{CertificateState, Communication, PasswordState},
    utils::{config::Config, input::input},
};

use super::{encrypt::Encrypt, file_write, network::Network};

pub struct Client {
    pub network: Network,
    pub password_auth: bool,
}

impl Client {
    pub fn new(address: Ipv4Addr, port: &str, password_auth: bool) -> Client {
        Client {
            network: Network::new(address, port),
            password_auth,
        }
    }

    pub fn run_client(&self) -> Result<(), io::Error> {
        if self.password_auth {
            self.run_client_with_password_auth()
        } else {
            self.run_client_with_certificate_auth()
        }
    }

    fn run_client_with_password_auth(&self) -> Result<(), io::Error> {
        let mut stream = match self.connect_to_server() {
            Ok(s) => s,
            Err(err) => return Err(io::Error::new(io::ErrorKind::ConnectionRefused, err)),
        };

        info!("Connected to server");

        let password = match self.send_password(&mut stream) {
            Ok(p) => p,
            Err(err) => {
                if err.kind() == io::ErrorKind::ConnectionRefused {
                    return Err(err);
                }
                return Err(io::Error::new(io::ErrorKind::PermissionDenied, err));
            }
        };

        let key = Encrypt::derive_key_from_password(&password, 10000);

        let mut hex_key = String::new();
        for byte in key {
            write!(hex_key, "{:02x}", byte).expect("Failed to write to string");
        }
        debug!("Key in hesadecimal: {}", hex_key);

        match Network::communication(stream, key) {
            Ok(_) => warn!("The server is disconnected"),
            Err(err) => return Err(err),
        }

        Ok(())
    }

    fn run_client_with_certificate_auth(&self) -> Result<(), io::Error> {
        let config = Config::read();

        let admin_server_cert = file_write::read_server_certificate(&config.config_path)?;
        let p_key = file_write::read_pvt_key(&config.config_path)?;
        let user_cert = file_write::read_self_certificate(&config.config_path)?;

        let mut stream = match self.connect_to_server() {
            Ok(s) => s,
            Err(err) => return Err(io::Error::new(io::ErrorKind::ConnectionRefused, err)),
        };

        let mut hex_key = String::new();
        for byte in p_key.private_key_to_pkcs8()? {
            write!(hex_key, "{:02x}", byte).expect("Failed to write to string");
        }
        debug!("Private Key in hesadecimal: {}", hex_key);

        let mut hex_key = String::new();
        for byte in user_cert.public_key()?.public_key_to_der()? {
            write!(hex_key, "{:02x}", byte).expect("Failed to write to string");
        }
        debug!("Private Key in hesadecimal: {}", hex_key);

        info!("Server authentication");

        let server_cert = match self.send_certificate(&mut stream, &user_cert) {
            Ok(c) => c,
            Err(err) => {
                if err.kind() == io::ErrorKind::ConnectionRefused {
                    return Err(err);
                }
                return Err(io::Error::new(io::ErrorKind::PermissionDenied, err));
            }
        };

        // TODO: Can be cool to check the server cert also

        match Network::communication_async(stream, server_cert, p_key) {
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
            password = input("Please enter the conversation password:")?;

            let hash = Encrypt::hash(&password)?;

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

    fn send_certificate(
        &self,
        stream: &mut TcpStream,
        user_cert: &X509,
    ) -> Result<X509, io::Error> {
        Network::send_certificate(stream, user_cert).expect("Failled to send certificate");

        match Network::read_message(stream) {
            Ok((communication, data)) => match communication {
                Communication::CommunicationCertificate(comm_cert) => {
                    debug!("{:?}", comm_cert);
                    match comm_cert.certificate_state {
                        CertificateState::Incorrect => {
                            info!("Certificate incorrect");
                            return Err(io::Error::new(
                                io::ErrorKind::ConnectionRefused,
                                "Connection Refused, invalid certificate",
                            ));
                        }
                        CertificateState::Correct => {
                            info!("Certificate correct");
                            let server_cert = match X509::from_pem(&data) {
                                Ok(c) => c,
                                Err(e) => {
                                    error!("Unable to read server's certificate {}", e);
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        "Unable to read server's certificate... ",
                                    ));
                                }
                            };

                            return Ok(server_cert);
                        }
                        _ => {
                            return Err(io::Error::new(
                                io::ErrorKind::Other,
                                "Wrong data received, not a certificate",
                            ))
                        }
                    }
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Wrong data received, not a certificate",
                    ))
                }
            },
            Err(err) => return Err(err),
        }
    }
}
