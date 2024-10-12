use std::{
    io,
    net::{TcpListener, TcpStream},
};

use log::{debug, error};

use crate::types::communication::Communication;

use super::network::Network;

pub struct Server {
    pub network: Network,
}

impl Server {
    pub fn new(port: String) -> Server {
        Server {
            network: Network::new("127.0.0.1".to_owned(), port),
        }
    }

    pub fn start_server(&self) -> Result<TcpListener, io::Error> {
        TcpListener::bind(self.network.get_fulladdress())
    }

    pub fn handle_client(stream: &mut TcpStream) -> Result<(), io::Error> {
        let communication_result = Network::read_message(stream);

        match communication_result {
            Ok((communication, data)) => match (communication) {
                Communication::CommunicationText(comm_text) => {
                    debug!("Un text recu");
                    let message = String::from_utf8_lossy(&data);
                    debug!("le message est {}", message)
                }
                Communication::CommunicationFile(comm_file) => {
                    debug!("Un text recu")
                }
                Communication::CommunicationCertificate(comm_cert) => {
                    debug!("Un text recu")
                }
            },
            Err(err) => {
                error!("A message received but there has been an error ...");
            }
        }

        // TODO: Display message recived
        Ok(())
    }
}
