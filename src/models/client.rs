use std::{io, net::TcpStream};

use log::{info, warn};

use crate::types::communication::{Communication, CommunicationText};

use super::network::Network;

pub struct Client {
    pub network: Network,
}

impl Client {
    pub fn new(address: String, port: String) -> Client {
        Client {
            network: Network::new(address, port),
        }
    }

    pub fn run_client(&self) -> Result<(), io::Error> {
        match self.send_tmp_message() {
            Ok(_) => info!("Message envoyer "),
            Err(_) => info!("Message pas envoyer"),
        }

        Ok(())
    }

    fn connect_to_server(&self) -> Result<TcpStream, io::Error> {
        TcpStream::connect(self.network.get_fulladdress())
    }

    fn send_tmp_message(&self) -> Result<(), io::Error> {
        let mut stream = self.connect_to_server()?;

        let init_message = CommunicationText {};

        let enum_network = Communication::CommunicationText(init_message);

        let message = "Ceci est un mesage de test".to_owned();
        let mut data_tmp: Vec<u8> = message.as_bytes().to_vec();

        Network::send_message(&mut stream, &enum_network, &mut data_tmp)?;
        Ok(())
    }
}
