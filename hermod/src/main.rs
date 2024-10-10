use env_logger::Env;
use utils::starter;

mod models;
mod utils;

fn main() {
    starter::start_message();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let _config = utils::config::Config::read();
}
