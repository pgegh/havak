//! The core module of `havak-server`.

use crate::data_types::ServerState;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

/// Starts the execution of the sever logic.
pub fn start_sever() -> std::io::Result<()> {
    let server_state = ServerState::new()?;

    // Redirect stdout and stderr to a log file
    let log_file = File::create("temp/daemon.log")?;
    let mut log_writer = std::io::BufWriter::new(log_file);

    // Example of some long-running work the daemon does
    while !server_state.exit_requested() {
        writeln!(
            log_writer,
            "Daemon running at: {:?}",
            std::time::SystemTime::now()
        )?;
        log_writer.flush()?;
        thread::sleep(Duration::from_secs(1));
    }

    writeln!(log_writer, "SIGTERM was received")?;
    log_writer.flush()?;
    Ok(())
}
