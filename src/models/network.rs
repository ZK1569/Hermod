use std::{
    io::{self, Read, Write},
    net::{IpAddr, Ipv4Addr, Shutdown, TcpStream},
    thread,
};

use local_ip_address::local_ip;
use log::{debug, error, trace, warn};
use openssl::{
    pkey::{PKey, Private},
    x509::X509,
};

use crate::types::communication::{
    CertificateState, Communication, CommunicationCertificate, CommunicationPassword,
    CommunicationText, PasswordState,
};

use super::encrypt::Encrypt;

#[derive(Debug)]
pub struct Network {
    pub address: Ipv4Addr,
    pub port: String,
}

impl Network {
    pub fn new(address: Ipv4Addr, port: &str) -> Network {
        Network {
            address,
            port: port.to_string(),
        }
    }

    pub fn get_fulladdress(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }

    pub fn communication(mut stream: TcpStream, key: [u8; 32]) -> Result<(), io::Error> {
        // FIX: Peut avoir une meilleur solution que try_clone()
        let mut stream_clone = stream.try_clone()?;

        let handle_message = thread::spawn(move || -> Result<(), io::Error> {
            loop {
                match Network::read_message(&mut stream) {
                    Ok((communication, data)) => match communication {
                        Communication::CommunicationText(_comm_text) => {
                            let message = Encrypt::decrypt_message(&data, &key);
                            let message = String::from_utf8_lossy(&message).to_string();
                            println!("other: {}", message)
                        }
                        Communication::CommunicationFile(_comm_file) => {
                            // TODO: Download file
                            debug!("File received")
                        }
                        Communication::CommunicationCertificate(_comm_cert) => {
                            // TODO: Check cert
                            debug!("Cert received")
                        }
                        Communication::CommunicationPassword(_comm_password) => {
                            debug!("Password received")
                        }
                    },
                    Err(err) => {
                        if err.kind() == io::ErrorKind::InvalidData {
                            warn!("Message lost");
                            continue;
                        }
                        return Err(err);
                    }
                }
            }
        });

        let _handle_input = thread::spawn(move || loop {
            let init_message = CommunicationText {};

            let enum_network = Communication::CommunicationText(init_message);

            let mut message = String::new();

            io::stdin()
                .read_line(&mut message)
                .expect("failed to readline");

            message.pop(); // INFO: Delete the '\n' at the end

            let mut data_tmp = Encrypt::encrypt_message(&message.as_bytes(), &key);

            Network::send_message(&mut stream_clone, &enum_network, &mut data_tmp).unwrap();
        });

        match handle_message.join() {
            Ok(thread) => match thread {
                Ok(_) => {}
                Err(err) => {
                    if err.kind() == io::ErrorKind::ConnectionAborted {
                        return Ok(());
                    } else if err.kind() == io::ErrorKind::InvalidData {
                        return Err(err);
                    }
                    return Err(io::Error::new(io::ErrorKind::Other, err));
                }
            },
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "An error has occurred on the thread",
                ));
            }
        };

        // INFO: The input thread is not checked

        Ok(())
    }

    pub fn communication_async(
        mut stream: TcpStream,
        client_cert: X509,
        private_key: PKey<Private>,
    ) -> Result<(), io::Error> {
        todo!()
    }

    pub fn send_message(
        stream: &mut TcpStream,
        communication: &Communication,
        data: &[u8],
    ) -> Result<String, io::Error> {
        let json_message = serde_json::to_string(&communication)?;

        let json_message_size = json_message.len() as u32;
        let data_message_size = data.len() as u32;
        let total_message_size: u32 = json_message_size + data_message_size;

        stream.write_all(&total_message_size.to_be_bytes())?;
        stream.write_all(&json_message_size.to_be_bytes())?;
        stream.write_all(&json_message.as_bytes())?;
        stream.write_all(data)?;

        Ok(json_message)
    }

    pub fn read_message(stream: &mut TcpStream) -> Result<(Communication, Vec<u8>), io::Error> {
        let mut total_len_buf = [0; 4];
        match stream.read_exact(&mut total_len_buf) {
            Ok(_) => trace!("Received total message size"),
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    "Communication done",
                ));
            }
        };
        let total_message_size = u32::from_be_bytes(total_len_buf);

        let mut json_len_buf = [0; 4];
        match stream.read_exact(&mut json_len_buf) {
            Ok(_) => trace!("Received json message size"),
            Err(err) => {
                error!("The json message size could not be received {}", err);
                return Err(err);
            }
        };
        let json_message_size = u32::from_be_bytes(json_len_buf);

        if total_message_size < json_message_size {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Json message size if bigger than total message size",
            ));
        }

        let data_message_size = total_message_size - json_message_size;

        let mut sbuf = vec![0_u8; json_message_size as usize];
        match stream.read_exact(&mut sbuf) {
            Ok(_) => trace!("Received json"),
            Err(err) => {
                error!("The json message could not be received {}", err);
                return Err(err);
            }
        };
        let s = String::from_utf8_lossy(&sbuf);

        trace!("Json received : {s}");

        let communication_request = serde_json::from_str(&s);
        let communication = match communication_request {
            Ok(r) => r,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Message received by server cannot be deserialized",
                ));
            }
        };

        let mut data = vec![0_u8; data_message_size as usize];
        if let Err(e) = stream.read_exact(&mut data) {
            error!("Failed to read binary data: {}", e);
            return Err(e.into());
        }

        Ok((communication, data))
    }

    pub fn close_connection(stream: &mut TcpStream) {
        let _ = stream.shutdown(Shutdown::Both);
    }

    pub fn get_local_ip() -> Result<Ipv4Addr, io::Error> {
        let address = match local_ip() {
            Ok(a) => a,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::NotConnected,
                    "You are not on a local network",
                ))
            }
        };
        let ipv4: Ipv4Addr = match address {
            IpAddr::V4(ipv4) => ipv4,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Your ip address is not ipv4",
                ))
            }
        };

        Ok(ipv4)
    }

    pub fn send_password(stream: &mut TcpStream, hash: &[u8; 32]) -> Result<(), io::Error> {
        let password_communication = CommunicationPassword {
            password_state: PasswordState::Submition,
        };
        let enum_network = Communication::CommunicationPassword(password_communication);

        let _ = Network::send_message(stream, &enum_network, hash)?;
        Ok(())
    }

    pub fn password_response(
        stream: &mut TcpStream,
        validity: PasswordState,
    ) -> Result<(), io::Error> {
        let password_communication = CommunicationPassword {
            password_state: validity,
        };
        let enum_network = Communication::CommunicationPassword(password_communication);
        let data: [u8; 0] = [0; 0];
        let _ = Network::send_message(stream, &enum_network, &data)?;
        Ok(())
    }

    pub fn certificate_response(
        stream: &mut TcpStream,
        validity: CertificateState,
        user_certificate: Option<&X509>,
    ) -> Result<(), io::Error> {
        let certificate_communication = CommunicationCertificate {
            certificate_state: validity,
        };

        let data: Vec<u8>;
        if user_certificate.is_some() {
            data = user_certificate.unwrap().to_pem()?
        } else {
            data = [0; 0].to_vec();
        }

        let enum_network = Communication::CommunicationCertificate(certificate_communication);
        let _ = Network::send_message(stream, &enum_network, &data)?;
        Ok(())
    }
}
