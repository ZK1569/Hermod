use core::fmt;
use std::{
    io,
    net::{AddrParseError, Ipv4Addr},
    ops::RangeInclusive,
};

use clap::{command, Arg, ArgAction, Command};

pub struct CommandArg {
    pub execution_mod: ExecMod,
    pub debug: bool,
}

pub enum ExecMod {
    Server(ServerMod),
    Client(ClientMod),
}

pub struct ServerMod {
    pub port: u16,
}

pub struct ClientMod {
    pub address: Ipv4Addr,
    pub port: u16,
}

impl fmt::Display for ExecMod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecMod::Server(s) => {
                write!(f, "Server mode - port: {}", s.port)
            }
            ExecMod::Client(c) => {
                write!(f, "Client mode - target: {}, port: {}", c.address, c.port)
            }
        }
    }
}

impl fmt::Display for CommandArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} debug: {}", self.execution_mod, self.debug)
    }
}

pub fn get_commands() -> Result<CommandArg, io::Error> {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Displays more information on internal status")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("server")
                // TODO: Change the about info
                .about("Run as server")
                .arg(
                    Arg::new("port")
                        .short('p')
                        .long("port")
                        .help("The port on which the server is running")
                        .default_value("8080")
                        .value_parser(port_in_range),
                ),
        )
        .subcommand(
            Command::new("client")
                // TODO: Change the about info
                .about("Run as client")
                .arg(
                    Arg::new("address")
                        .short('a')
                        .long("address")
                        .required(true)
                        .help("The ip address of the machine to connect to")
                        .value_parser(check_id_address),
                )
                .arg(
                    Arg::new("port")
                        .short('p')
                        .long("port")
                        .help("The host port on which the server is running")
                        .default_value("8080")
                        .value_parser(port_in_range),
                ),
        )
        .get_matches();

    let exec: ExecMod = match matches.subcommand() {
        Some(("server", sub_matches)) => {
            let port = match sub_matches.get_one::<u16>("port") {
                Some(p) => p.clone(),
                None => 8080,
            };

            ExecMod::Server(ServerMod { port })
        }
        Some(("client", sub_matches)) => {
            let address = match sub_matches.get_one::<Ipv4Addr>("address") {
                Some(a) => a.clone(),
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "User did not provide correct ip address",
                    ))
                }
            };
            let port = match sub_matches.get_one::<u16>("port") {
                Some(p) => p.clone(),
                None => 8080,
            };

            ExecMod::Client(ClientMod { address, port })
        }
        _ => unreachable!("an unknown command is used"),
    };

    Ok(CommandArg {
        execution_mod: exec,
        debug: matches.get_flag("debug"),
    })
}

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}

fn check_id_address(addres: &str) -> Result<Ipv4Addr, io::Error> {
    let test: Result<Ipv4Addr, AddrParseError> = addres.parse();
    match test {
        Ok(add) => Ok(add),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Ip address not valid",
        )),
    }
}
