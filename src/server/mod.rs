//! The core module of `havak-server`.

use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

/// Starts the execution of the sever logic.
pub fn start_sever() -> std::io::Result<()> {
    // Redirect stdout and stderr to a log file
    let log_file = File::create("temp/daemon.log")?;
    let mut log_writer = std::io::BufWriter::new(log_file);

    // Example of some long-running work the daemon does
    loop {
        writeln!(
            log_writer,
            "Daemon running at: {:?}",
            std::time::SystemTime::now()
        )?;
        log_writer.flush()?;
        thread::sleep(Duration::from_secs(1));
    }
}
