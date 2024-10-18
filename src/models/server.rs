use std::{
    io,
    net::{Ipv4Addr, TcpListener},
};

use log::{debug, error, info, warn};

use super::network::Network;

pub struct Server {
    pub network: Network,
}

impl Server {
    pub fn new(port: &str, localhost: bool) -> Result<Server, io::Error> {
        let ip = if localhost {
            Ipv4Addr::new(127, 0, 0, 1)
        } else {
            match Network::get_local_ip() {
                Ok(ip) => ip,
                Err(err) => {
                    debug!("{}", err);
                    return Err(io::Error::new(io::ErrorKind::AddrNotAvailable, "The ip address is not accessible, please check that you are on a network..." ));
                }
            }
        };
        Ok(Server {
            network: Network::new(ip, port),
        })
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

        match listener.accept() {
            Ok((socket, addr)) => {
                info!("A new customer ({}) is connected ...", addr);
                match Network::communication(socket) {
                    Ok(_) => warn!("The customer has left the conversation"),
                    Err(err) => return Err(err),
                };
            }
            Err(e) => {
                error!("{}", e);
            }
        }

        Ok(())
    }
}
