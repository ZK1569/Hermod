use std::{
    io,
    net::{TcpListener, TcpStream},
    thread,
};

use log::{debug, error};

use crate::types::communication::{Communication, CommunicationText};

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

    pub fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
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

    // pub fn handle_client(stream: &mut TcpStream) -> Result<(), io::Error> {
    //     let communication_result = Network::read_message(stream);
    //
    //     match communication_result {
    //         Ok((communication, data)) => match (communication) {
    //             Communication::CommunicationText(comm_text) => {
    //                 debug!("Un text recu");
    //                 let message = String::from_utf8_lossy(&data);
    //                 debug!("le message est {}", message)
    //             }
    //             Communication::CommunicationFile(comm_file) => {
    //                 debug!("Un text recu")
    //             }
    //             Communication::CommunicationCertificate(comm_cert) => {
    //                 debug!("Un text recu")
    //             }
    //         },
    //         Err(err) => {
    //             error!("A message received but there has been an error ...");
    //         }
    //     }
    //
    //     // TODO: Display message recived
    //     Ok(())
    // }
}
