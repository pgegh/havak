use env_logger;
use libc;
use log::{error, info};

pub mod server;

fn main() {
    env_logger::init();

    match daemonize() {
        Ok(_) => info!("Daemon started successfully."),
        Err(e) => error!("Failed to start daemon: {}", e),
    }

    match crate::server::start_sever() {
        Ok(_) => info!("Server executed successfully."),
        Err(e) => error!("Failed to execute the server correctly: {}", e),
    }
}

fn daemonize() -> std::io::Result<()> {
    fork_the_process()?;
    execute_system_call_setsid()
}

fn fork_the_process() -> std::io::Result<()> {
    let pid = unsafe { libc::fork() };
    if pid < 0 {
        // Fork failed
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Fork failed",
        ))
    } else if pid > 0 {
        // Parent process exits, leaving the child to run as a daemon
        std::process::exit(0)
    } else {
        // Child process continues here.
        Ok(())
    }
}

fn execute_system_call_setsid() -> std::io::Result<()> {
    let sid = unsafe { libc::setsid() };
    if sid < 0 {
        // Setsid failed
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Setsid failed",
        ))
    } else {
        Ok(())
    }
}
