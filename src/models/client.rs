use std::{
    io,
    net::{Ipv4Addr, TcpStream},
};

use log::{info, warn};

use super::network::Network;

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
        let stream = match self.connect_to_server() {
            Ok(s) => s,
            Err(err) => return Err(io::Error::new(io::ErrorKind::ConnectionRefused, err)),
        };

        info!("Connected to server");

        match Network::communication(stream) {
            Ok(_) => warn!("The server is disconnected"),
            Err(err) => return Err(err),
        }

        Ok(())
    }

    fn connect_to_server(&self) -> Result<TcpStream, io::Error> {
        TcpStream::connect(self.network.get_fulladdress())
    }
}
