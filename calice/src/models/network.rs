use std::net::{Shutdown, TcpStream};

#[derive(Debug)]
pub struct Network {
    server_address: String,
    port: String,
}

impl Network {
    pub fn new(server_address: String, port: String) -> Network {
        Network {
            server_address,
            port,
        }
    }

    pub fn get_fulladdress(&self) -> String {
        format!("{}:{}", self.server_address, self.port)
    }

    pub fn send_message() {
        todo!()
    }

    pub fn read_message() {
        todo!()
    }

    pub fn close_connection(stream: &mut TcpStream) {
        let _ = stream.shutdown(Shutdown::Both);
    }
}
