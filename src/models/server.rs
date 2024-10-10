use std::{
    io::Error,
    net::{Shutdown, TcpListener, TcpStream},
};

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

    pub fn start_server(&self) -> Result<TcpListener, Error> {
        TcpListener::bind(self.network.get_fulladdress())
    }

    pub fn close_connection(stream: &mut TcpStream) {
        let _ = stream.shutdown(Shutdown::Both);
    }
}
