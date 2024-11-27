use std::{
    fmt::Write,
    io,
    net::{Ipv4Addr, TcpListener, TcpStream},
};

use log::{debug, error, info, warn};
use openssl::x509::X509;

use crate::{
    models::{
        encrypt::{self, Encrypt},
        file_write, network,
    },
    types::communication::{CertificateState, Communication, PasswordState},
    utils::{
        config::Config,
        input::{self},
    },
};

use super::network::Network;

const MAX_PASSWORD_ERRORS: u8 = 3;

pub struct Server {
    pub network: Network,
    pub password_auth: bool,
}

impl Server {
    pub fn new(port: &str, localhost: bool, password_auth: bool) -> Result<Server, io::Error> {
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
            password_auth,
        })
    }

    fn start_server(&self) -> Result<TcpListener, io::Error> {
        TcpListener::bind(self.network.get_fulladdress())
    }

    pub fn run_sever(&self) -> Result<(), io::Error> {
        if self.password_auth {
            self.run_server_with_password_auth()
        } else {
            self.run_server_with_certificate()
        }
    }

    fn run_server_with_password_auth(&self) -> Result<(), io::Error> {
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

        let mut hex_key = String::new();
        for byte in key {
            write!(hex_key, "{:02x}", byte).expect("Failed to write to string");
        }
        debug!("Key in hesadecimal: {}", hex_key);

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
                info!("A new client ({}) is connected ...", addr);
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

    fn run_server_with_certificate(&self) -> Result<(), io::Error> {
        let config = Config::read();
        let server_cert = file_write::read_server_certificate(&config.config_path)?;
        let p_key = file_write::read_pvt_key(&config.config_path)?;
        let user_cert = file_write::read_self_certificate(&config.config_path)?;

        let listener = match self.start_server() {
            Ok(r) => r,
            Err(err) => {
                error!("Server failed to start... \n{err}");
                return Err(io::Error::new(io::ErrorKind::ConnectionRefused, err));
            }
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

        info!(
            "Your server is running on address: {} port: {}",
            self.network.address, self.network.port
        );

        match listener.accept() {
            Ok((mut socket, addr)) => {
                info!("A new client ({}) is connected ...", addr);
                match Server::check_certificate_and_get_user_cert(
                    &mut socket,
                    &server_cert,
                    &user_cert,
                ) {
                    Ok(client_cert) => {
                        if Server::do_enable_connection(&client_cert)? {
                            match Network::communication_async(socket, client_cert, p_key) {
                                Ok(_) => warn!("The client has left the conversation"),
                                Err(err) => return Err(err),
                            }
                        }
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
        let password =
            input::input("Choose a password that will ensure the security of the conversation:")
                .expect("Failed to read password");

        debug!("the password is >{}<", password);

        let hash = Encrypt::hash(&password)?;

        Ok((hash, password))
    }

    fn check_password(stream: &mut TcpStream, hash: [u8; 32]) -> Result<(), io::Error> {
        // FIX: This function has too much responsibility, check the password and return a response
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

    fn check_certificate_and_get_user_cert(
        stream: &mut TcpStream,
        server_cert: &X509,
        user_cert: &X509,
    ) -> Result<X509, io::Error> {
        // FIX: This function has too much responsibility, check the certificate and return a response
        match Network::read_message(stream) {
            Ok((communication, data)) => match communication {
                Communication::CommunicationCertificate(_comm_certificate) => {
                    debug!("Certificate received");

                    let client_cert = match X509::from_pem(&data) {
                        Ok(c) => c,
                        Err(e) => {
                            error!("Unable to read client certificate {}", e);
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "Unable to read client certificate... ",
                            ));
                        }
                    };

                    if encrypt::Encrypt::certificate_check_signature(server_cert, &client_cert) {
                        info!("Correct certificate received");
                        let _ = Network::certificate_response(
                            stream,
                            CertificateState::Correct,
                            Some(user_cert),
                        );
                        Ok(client_cert)
                    } else {
                        error!("Certificate with the incorrect signature");
                        Network::certificate_response(stream, CertificateState::Incorrect, None)?;
                        return Err(io::Error::new(
                            io::ErrorKind::PermissionDenied,
                            "Invalid certificate",
                        ));
                    }
                }
                _ => {
                    Network::certificate_response(stream, CertificateState::Incorrect, None)?;
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Did not received a certificate",
                    ));
                }
            },
            Err(err) => {
                error!("Error certificate... {err}");
                Network::certificate_response(stream, CertificateState::Incorrect, None)?;
                return Err(err);
            }
        }
    }

    fn do_enable_connection(cert: &X509) -> Result<bool, io::Error> {
        let cert_name = match Encrypt::get_name_on_certificate(cert)? {
            Some(c) => c,
            None => "no_name_found_on_certificate".to_owned(),
        };

        let cert_email = match Encrypt::get_email_on_certificate(cert)? {
            Some(m) => m,
            None => "no_email_found_on_certificate".to_owned(),
        };

        info!(
            "Would you like to start communicating with >> {} <<",
            cert_name
        );
        info!("Email: {}", cert_email);

        let user_res = input::input("[Y/n]")?;
        if user_res.starts_with('Y') || user_res.starts_with('y') {
            return Ok(true);
        }
        Ok(false)
    }
}
