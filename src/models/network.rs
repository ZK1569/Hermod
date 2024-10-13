use std::{
    io::{self, Read, Write},
    net::{IpAddr, Ipv4Addr, Shutdown, TcpStream},
};

use local_ip_address::local_ip;
use log::{error, trace};

use crate::types::communication::Communication;

#[derive(Debug)]
pub struct Network {
    server_address: Ipv4Addr,
    port: String,
}

impl Network {
    pub fn new(server_address: Ipv4Addr, port: &str) -> Network {
        Network {
            server_address,
            port: port.to_string(),
        }
    }

    pub fn get_fulladdress(&self) -> String {
        format!("{}:{}", self.server_address, self.port)
    }

    pub fn send_message(
        stream: &mut TcpStream,
        communication: &Communication,
        data: &[u8],
    ) -> Result<String, io::Error> {
        let json_message = serde_json::to_string(&communication)?;

        let json_message_size = json_message.len() as u32;
        let data_message_size = data.len() as u32;
        let total_message_size: u32 = json_message_size + data_message_size;

        stream.write_all(&total_message_size.to_be_bytes())?;
        stream.write_all(&json_message_size.to_be_bytes())?;
        stream.write_all(&json_message.as_bytes())?;
        stream.write_all(data)?;

        Ok(json_message)
    }

    pub fn read_message(stream: &mut TcpStream) -> Result<(Communication, Vec<u8>), io::Error> {
        let mut total_len_buf = [0; 4];
        match stream.read_exact(&mut total_len_buf) {
            Ok(_) => trace!("Received total message size"),
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    "Communication done",
                ));
            }
        };
        let total_message_size = u32::from_be_bytes(total_len_buf);

        let mut json_len_buf = [0; 4];
        match stream.read_exact(&mut json_len_buf) {
            Ok(_) => trace!("Received json message size"),
            Err(err) => {
                error!("The json message size could not be received {}", err);
                return Err(err);
            }
        };
        let json_message_size = u32::from_be_bytes(json_len_buf);

        if total_message_size < json_message_size {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Json message size if bigger than total message size",
            ));
        }

        let data_message_size = total_message_size - json_message_size;

        let mut sbuf = vec![0_u8; json_message_size as usize];
        match stream.read_exact(&mut sbuf) {
            Ok(_) => trace!("Received json"),
            Err(err) => {
                error!("The json message could not be received {}", err);
                return Err(err);
            }
        };
        let s = String::from_utf8_lossy(&sbuf);

        trace!("Json received : {s}");

        let communication_request = serde_json::from_str(&s);
        let communication = match communication_request {
            Ok(r) => r,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Message received by server cannot be deserialized",
                ));
            }
        };

        let mut data = vec![0_u8; data_message_size as usize];
        if let Err(e) = stream.read_exact(&mut data) {
            error!("Failed to read binary data: {}", e);
            return Err(e.into());
        }

        Ok((communication, data))
    }

    pub fn close_connection(stream: &mut TcpStream) {
        let _ = stream.shutdown(Shutdown::Both);
    }

    pub fn get_local_ip() -> Result<Ipv4Addr, io::Error> {
        let address = match local_ip() {
            Ok(a) => a,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::NotConnected,
                    "You are not on a local network",
                ))
            }
        };
        let ipv4: Ipv4Addr = match address {
            IpAddr::V4(ipv4) => ipv4,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Your ip address is not ipv4",
                ))
            }
        };

        Ok(ipv4)
    }
}
