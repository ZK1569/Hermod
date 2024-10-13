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
    pub fn new(port: &str) -> Server {
        Server {
            network: Network::new(Ipv4Addr::new(127, 0, 0, 1), port),
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

        let ip = match Network::get_local_ip() {
            Ok(ip) => ip,
            Err(err) => {
                error!("{}", err);
                return Err(io::Error::new(io::ErrorKind::AddrNotAvailable, err));
            }
        };

        info!(
            "Your server is running on address: {} port: {}",
            ip, self.network.port
        );

        for stream in listener.incoming() {
            info!("A new customer is connected ...");

            match stream {
                Ok(s) => match Network::communication(s) {
                    Ok(_) => warn!("The customer has left the conversation"),
                    Err(err) => {
                        if err.kind() == io::ErrorKind::InvalidData {
                            warn!("Message lost");
                            continue;
                        } else if err.kind() == io::ErrorKind::Other {
                            error!("Unknown error")
                        }
                        error!(
                                "An unexpected error occurred during communication with the client... \n{}",
                                err
                            );
                        return Err(io::Error::new(io::ErrorKind::Other, err));
                    }
                },
                Err(err) => {
                    error!("A strange customer tried to connect... \n{}", err)
                }
            }
        }

        Ok(())
    }
}
