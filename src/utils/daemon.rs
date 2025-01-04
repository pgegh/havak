//! A collection of utility functions for creating and managing daemons.

/// Creates a child process, makes it a daemon, and exits the parent process.
pub fn daemonize() -> std::io::Result<()> {
    fork_the_process()?;
    execute_system_call_setsid()
}

/// Creates a child process and exits the parent process.
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

/// Creates a new Unix session and makes the calling process a session leader.
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
