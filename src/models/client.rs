use log::{debug, error};

use crate::types::communication::{Communication, CommunicationText};
use std::{
    io,
    net::{Ipv4Addr, TcpStream},
    thread,
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
        let stream = self.connect_to_server()?;

        let _ = Client::communication_with_client(stream);

        // self.send_tmp_message(&mut stream)?;
        // self.read_tmp_message(&mut stream)?;

        Ok(())
    }

    pub fn communication_with_client(mut stream: TcpStream) -> Result<(), io::Error> {
        let mut stream_clone = stream.try_clone()?;

        let handle_message = thread::spawn(move || loop {
            match Network::read_message(&mut stream) {
                Ok((communication, data)) => match communication {
                    Communication::CommunicationText(_comm_text) => {
                        debug!("Un text recu");
                        let message = String::from_utf8_lossy(&data);
                        debug!("le message est {}", message)
                    }
                    Communication::CommunicationFile(_comm_file) => {
                        debug!("Un text recu")
                    }
                    Communication::CommunicationCertificate(_comm_cert) => {
                        debug!("Un text recu")
                    }
                },
                Err(_err) => {
                    error!("A message received but there has been an error ...");
                }
            }
        });

        let handle_input = thread::spawn(move || loop {
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

        loop {
            if handle_message.is_finished() && handle_input.is_finished() {
                break;
            }
        }

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

    fn read_tmp_message(&self, stream: &mut TcpStream) -> Result<(), io::Error> {
        match Network::read_message(stream) {
            Ok((communication, data)) => match (communication) {
                Communication::CommunicationText(comm_text) => {
                    debug!("Un text recu");
                    let message = String::from_utf8_lossy(&data);
                    debug!("le message est {}", message)
                }
                Communication::CommunicationFile(comm_file) => {
                    debug!("Un text recu");
                }
                Communication::CommunicationCertificate(comm_cert) => {
                    debug!("Un text recu");
                }
            },
            Err(err) => {
                error!("A message received but there has been an error ...");
            }
        }

        Ok(())
    }
}
