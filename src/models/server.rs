use std::{
    io,
    net::{Ipv4Addr, TcpListener},
};

use log::{error, info, warn};

use super::network::Network;

pub struct Server {
    pub network: Network,
}

impl Server {
    pub fn new(address: Ipv4Addr, port: &str) -> Server {
        Server {
            network: Network::new(address, port),
        }
    }

    pub fn start_server(&self) -> Result<TcpListener, io::Error> {
        TcpListener::bind(self.network.get_fulladdress())
    }

    pub fn run_sever(&self) -> Result<(), io::Error> {
        let listener_result = self.start_server();

        let listener = match listener_result {
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

        // TODO: Change to have only one client connected
        for stream in listener.incoming() {
            info!("A new customer is connected ...");

            match stream {
                Ok(s) => match Network::communication(s) {
                    Ok(_) => warn!("The customer has left the conversation"),
                    Err(err) => return Err(err),
                },
                Err(err) => {
                    error!("A strange customer tried to connect... \n{}", err)
                }
            }
        }

        Ok(())
    }
}
