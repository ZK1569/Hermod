use std::{
    io,
    net::{Ipv4Addr, TcpStream},
};

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
        let stream = self.connect_to_server()?;

        let _ = Network::communication(stream);

        Ok(())
    }

    fn connect_to_server(&self) -> Result<TcpStream, io::Error> {
        TcpStream::connect(self.network.get_fulladdress())
    }
}
