//! A collection of functions related to inter-process communication (IPC) signals.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

/// Creates a flag that indicates whether SIGTERM is received.
pub fn create_sigterm_received_flag() -> std::io::Result<Arc<AtomicBool>> {
    let sigterm = Arc::new(AtomicBool::new(false));
    match signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&sigterm)) {
        Ok(_) => Ok(sigterm),
        Err(e) => Err(e),
    }
}
