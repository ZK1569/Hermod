use std::{
    io,
    net::{Ipv4Addr, TcpListener, TcpStream},
    thread,
};

use log::{debug, info, warn};

use crate::types::communication::{Communication, CommunicationText};

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

    pub fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
        info!("Someone has connected to the server");

        // FIX: Peut avoir une meilleur solution
        let mut stream_clone = stream.try_clone()?;

        let handle_message = thread::spawn(move || -> Result<(), io::Error> {
            loop {
                match Network::read_message(&mut stream) {
                    Ok((communication, data)) => match communication {
                        Communication::CommunicationText(_comm_text) => {
                            let message = String::from_utf8_lossy(&data);
                            println!("client: {}", message)
                        }
                        Communication::CommunicationFile(_comm_file) => {
                            debug!("Un fichier recu")
                        }
                        Communication::CommunicationCertificate(_comm_cert) => {
                            debug!("Un cert recu")
                        }
                    },
                    Err(err) => return Err(err),
                }
            }
        });

        let _ = thread::spawn(move || loop {
            let init_message = CommunicationText {};

            let enum_network = Communication::CommunicationText(init_message);

            print!("> ");
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("failed to readline");
            let mut data_tmp: Vec<u8> = guess.as_bytes().to_vec();

            Network::send_message(&mut stream_clone, &enum_network, &mut data_tmp).unwrap();
        });

        match handle_message.join() {
            Ok(thread) => match thread {
                Ok(_) => {}
                Err(err) => {
                    if err.kind() == io::ErrorKind::ConnectionAborted {
                        warn!("End of communication");
                        return Ok(());
                    } else if err.kind() == io::ErrorKind::InvalidData {
                        warn!("Message lost");
                        return Err(err);
                    }
                    return Err(io::Error::new(io::ErrorKind::Other, err));
                }
            },
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "An error has occurred on the thread",
                ));
            }
        };

        Ok(())
    }
}
