use std::{error::Error, io};

use app::App;
use log::{debug, error, info, warn};
use models::{api::ServerApi, client::Client, encrypt::Encrypt, file_write, server::Server};
use ratatui::{
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::CrosstermBackend,
    Terminal,
};

mod app;
mod models;
mod tests;
mod types;
mod ui;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.messages.push((true, "pour le test".to_string()));
    app.messages.push((false, "pour le test 2".to_string()));
    app.messages.push((true, "pour le test".to_string()));
    app.messages.push((true, "pour le test".to_string()));
    app.messages.push((false, "pour le test".to_string()));
    app.messages.push((true, "pour le test".to_string()));
    let _res = App::run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

// fn main() {
//     let command_result = commands::get_commands();
//     let command = match command_result {
//         Ok(c) => c,
//         Err(e) => {
//             error!("Something went wrong ... {}", e);
//             process::exit(1);
//         }
//     };
//
//     env_logger::Builder::from_env(Env::default().default_filter_or(if command.debug {
//         "debug"
//     } else {
//         "info"
//     }))
//     .init();
//
//     starter::start_message();
//     debug!("Inputed commands : {}", command);
//
//     let config = Config::read();
//
//     let _ = create_dir_all(&config.config_path);
//
//     debug!("config data {:?}", config);
//
//     match command.execution_mod {
//         commands::ExecMod::Server(server_info) => {
//             let server = match Server::new(
//                 &server_info.port.to_string(),
//                 server_info.localhost,
//                 server_info.password,
//             ) {
//                 Ok(s) => s,
//                 Err(err) => {
//                     error!("{}", err);
//                     process::exit(1);
//                 }
//             };
//
//             match server.run_sever() {
//                 Ok(_) => info!("No errors encountered"),
//                 Err(err) => {
//                     if err.kind() == io::ErrorKind::ConnectionRefused {
//                         error!("Server failed to start... \n{err}");
//                         process::exit(1);
//                     }
//                     error!("An error has occurred... \n{}", err)
//                 }
//             }
//         }
//         commands::ExecMod::Client(client_info) => {
//             let client = Client::new(
//                 client_info.address,
//                 &client_info.port.to_string(),
//                 client_info.password,
//             );
//
//             match client.run_client() {
//                 Ok(_) => info!("No errors encountered"),
//                 Err(err) => {
//                     if err.kind() == io::ErrorKind::ConnectionRefused {
//                         error!("Server connection failure... \n{err}");
//                         process::exit(1);
//                     } else if err.kind() == io::ErrorKind::PermissionDenied {
//                         error!(
//                             "It is not possible to connect to the server because... {}",
//                             err
//                         );
//                     }
//                     error!("An error has occurred... \n{}", err)
//                 }
//             }
//         }
//         commands::ExecMod::Certificate(cert_action) => match cert_action.action {
//             CertificateActions::New => {
//                 let username = match input::input("Full name: ") {
//                     Ok(u) => u,
//                     Err(err) => {
//                         error!("Error reading user input... {}", err);
//                         process::exit(1);
//                     }
//                 };
//                 let email = match input::input("Email: ") {
//                     Ok(u) => u,
//                     Err(err) => {
//                         error!("Error reading user input... {}", err);
//                         process::exit(1);
//                     }
//                 };
//                 let country = match input::input("Country [CA]: ") {
//                     Ok(mut u) => {
//                         if u.len() != 2 {
//                             error!("Country name not valid, default used [CA]");
//                             u = "CA".to_owned();
//                         }
//                         u
//                     }
//                     Err(err) => {
//                         error!("Error reading user input... {}", err);
//                         process::exit(1);
//                     }
//                 };
//                 let locality = match input::input("Locality Name: ") {
//                     Ok(u) => u,
//                     Err(err) => {
//                         error!("Error reading user input... {}", err);
//                         process::exit(1);
//                     }
//                 };
//                 let (cert, key_pair) =
//                     match Encrypt::mk_ca_cert(&username, &email, &country, &locality) {
//                         Ok((c, k)) => (c, k),
//                         Err(err) => {
//                             error!(
//                                 "Something went wrong with the certificate generation... {}",
//                                 err
//                             );
//                             process::exit(1);
//                         }
//                     };
//
//                 let signed_cert = match tokio::runtime::Runtime::new()
//                     .unwrap()
//                     .block_on(async { ServerApi::signe_certificate(&cert).await })
//                 {
//                     Ok(c) => c,
//                     Err(err) => {
//                         error!("Error occurred during certificate signing: {}", err);
//                         process::exit(1);
//                     }
//                 };
//
//                 if let Err(e) = file_write::save_certificate(&signed_cert, &config.config_path) {
//                     error!("Error will saving the user's certificate... {}", e);
//                 }
//                 if let Err(e) = file_write::save_pvt_key(key_pair, &config.config_path) {
//                     error!("Error will saving the user's private key... {}", e);
//                 }
//
//                 let server_cert = match tokio::runtime::Runtime::new()
//                     .unwrap()
//                     .block_on(async { ServerApi::get_server_certificate().await })
//                 {
//                     Ok(c) => c,
//                     Err(err) => {
//                         error!("An error occurred when requesting the server certificate... {err}");
//                         process::exit(1);
//                     }
//                 };
//
//                 if let Err(e) =
//                     file_write::save_server_certificate(&server_cert, &config.config_path)
//                 {
//                     error!("Error will saving the admin server's certificate... {}", e);
//                 }
//             }
//
//             CertificateActions::Delete => {
//                 if let Err(e) = file_write::delete_certificate(&config.config_path) {
//                     warn!("There was an error when trying to delete the certificate file... {e}");
//                 }
//                 if let Err(e) = file_write::delete_pvt_key(&config.config_path) {
//                     warn!("There cas an error when trying to delete the private key file... {e}");
//                 }
//             }
//
//             CertificateActions::See(certificate) => {
//                 let file_path: String;
//                 if certificate.file_path.starts_with(|c| c == '/') {
//                     file_path = certificate.file_path.clone();
//                 } else {
//                     file_path = config.current_path.clone() + "/" + &certificate.file_path;
//                 }
//                 debug!("path for cert : {}", file_path);
//                 match file_write::read_certificate(&file_path) {
//                     Ok(cert) => info!("{}: \n{:#?}", certificate.file_path, cert),
//                     Err(e) => error!("Unable to display the certificate... {e}"),
//                 };
//             }
//         },
//     }
// }
