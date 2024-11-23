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
    Certificate(Certificate),
}

pub struct ServerMod {
    pub port: u16,
    pub localhost: bool,
    pub password: bool,
}

pub struct ClientMod {
    pub address: Ipv4Addr,
    pub port: u16,
    pub password: bool,
}

#[derive(Debug)]
pub struct Certificate {
    pub action: CertificateActions,
}

#[derive(Debug)]
pub enum CertificateActions {
    New,
    See(CertificateToSee),
    Delete,
}

#[derive(Debug)]
pub struct CertificateToSee {
    pub file_path: String,
}

impl fmt::Display for ExecMod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecMod::Server(s) => {
                write!(
                    f,
                    "Server mode - port: {}, localhost: {}, password_auth: {}",
                    s.port, s.localhost, s.password
                )
            }
            ExecMod::Client(c) => {
                write!(
                    f,
                    "Client mode - target: {}, port: {}, password_auth: {}",
                    c.address, c.port, c.password
                )
            }
            ExecMod::Certificate(_c) => {
                write!(f, "Certificate")
            }
        }
    }
}

impl fmt::Display for CommandArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, debug: {}", self.execution_mod, self.debug)
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
                )
                .arg(
                    Arg::new("localhost")
                        .short('l')
                        .long("localhost")
                        .help("Execute the server as localhost for debugging purposes")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("password")
                        .long("password")
                        .help("Authenticates customer just by password")
                        .action(ArgAction::SetTrue),
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
                )
                .arg(
                    Arg::new("password")
                        .long("password")
                        .help("Authenticates to server just by using password")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("certificate")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .about("Command for certificates")
                .subcommand(Command::new("new").about("Generate a new certificate"))
                .subcommand(
                    Command::new("see")
                        .arg(Arg::new("name").required(true).index(1))
                        .about("Displays certificate information"),
                )
                .subcommand(Command::new("delete").about("Deleting your local certificate")),
        )
        .get_matches();

    let exec: ExecMod = match matches.subcommand() {
        Some(("server", sub_matches)) => {
            let port = match sub_matches.get_one::<u16>("port") {
                Some(p) => p.clone(),
                None => 8080,
            };

            let localhost = sub_matches.get_flag("localhost");

            let password = sub_matches.get_flag("password");

            ExecMod::Server(ServerMod {
                port,
                localhost,
                password,
            })
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

            let password = sub_matches.get_flag("password");

            ExecMod::Client(ClientMod {
                address,
                port,
                password,
            })
        }
        Some(("certificate", sub_matches)) => match sub_matches.subcommand() {
            Some(("new", _sub_matches)) => ExecMod::Certificate(Certificate {
                action: CertificateActions::New,
            }),
            Some(("see", sub_matches)) => {
                let file_name = match sub_matches.get_one::<String>("name") {
                    Some(n) => n.clone(),
                    None => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Certificate not specified",
                        ))
                    }
                };
                ExecMod::Certificate(Certificate {
                    action: CertificateActions::See(CertificateToSee {
                        file_path: file_name,
                    }),
                })
            }
            Some(("delete", _sub_matches)) => ExecMod::Certificate(Certificate {
                action: CertificateActions::Delete,
            }),
            _ => unreachable!("an unknown command is used for certificates"),
        },
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
