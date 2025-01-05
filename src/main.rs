//! `havak-server` is the backend of *Havak*, a solution for documenting and managing collections.

use env_logger;
use log::{error, info};

mod server;
mod utils;
mod data_types;

fn main() {
    env_logger::init();

    match utils::daemon::daemonize() {
        Ok(_) => info!("Daemon started successfully."),
        Err(e) => error!("Failed to start daemon: {}", e),
    }

    match server::start_sever() {
        Ok(_) => info!("Server executed successfully."),
        Err(e) => error!("Failed to execute the server correctly: {}", e),
    }
}
