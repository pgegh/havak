use log::{error, info};
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn daemonize() -> std::io::Result<()> {
    //Fork the process
    let pid = unsafe { libc::fork() };
    if pid < 0 {
        // Fork failed
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Fork failed",
        ));
    }
    if pid > 0 {
        // Parent process exits, leaving the child to run as a daemon
        std::process::exit(0);
    }

    // Child process continues here
    // Create a new session and detach from terminal
    unsafe { libc::setsid() };

    // Redirect stdout and stderr to a log file
    let log_file = File::create("temp/daemon.log")?;
    let mut log_writer = std::io::BufWriter::new(log_file);

    // Example of some long-running work the daemon does
    loop {
        writeln!(
            log_writer,
            "Daemon running at: {:?}",
            std::time::SystemTime::now()
        )
        .unwrap();
        log_writer.flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    // Initialize logger
    env_logger::init();

    match daemonize() {
        Ok(_) => info!("Daemon started successfully."),
        Err(e) => error!("Failed to start daemon: {}", e),
    }
}
