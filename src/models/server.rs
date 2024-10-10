use std::{
    io,
    net::{TcpListener, TcpStream},
};

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

    pub fn handle_client(stream: &mut TcpStream) -> Result<Communication, io::Error> {
        todo!("Client handler")
    }
}
