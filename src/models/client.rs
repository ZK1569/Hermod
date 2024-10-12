use crate::types::communication::{Communication, CommunicationText};
use std::{
    io,
    net::{Ipv4Addr, TcpStream},
};

use super::network::Network;

pub struct Client {
    pub network: Network,
}

impl Client {
    pub fn new(address: Ipv4Addr, port: String) -> Client {
        Client {
            network: Network::new(address.to_string(), port),
        }
    }

    pub fn run_client(&self) -> Result<(), io::Error> {
        let mut stream = self.connect_to_server()?;

        self.send_tmp_message(&mut stream)?;

        Ok(())
    }

    fn connect_to_server(&self) -> Result<TcpStream, io::Error> {
        TcpStream::connect(self.network.get_fulladdress())
    }

    fn send_tmp_message(&self, stream: &mut TcpStream) -> Result<(), io::Error> {
        let init_message = CommunicationText {};

        let enum_network = Communication::CommunicationText(init_message);

        let message = "Ceci est un mesage de test".to_owned();
        let mut data_tmp: Vec<u8> = message.as_bytes().to_vec();

        Network::send_message(stream, &enum_network, &mut data_tmp)?;
        Ok(())
    }
}
