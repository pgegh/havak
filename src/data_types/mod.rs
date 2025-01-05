//! A collection of custom data types.

use crate::utils::ipc_signals::create_sigterm_received_flag;
use std::sync::{atomic, atomic::AtomicBool, Arc};

/// A structure representing the server state.
pub struct ServerState {
    sigterm_received: Arc<AtomicBool>,
}

impl ServerState {
    pub fn new() -> std::io::Result<ServerState> {
        let sigterm_received = create_sigterm_received_flag()?;
        Ok(ServerState { sigterm_received })
    }

    /// Check the presence of any exit request.
    pub fn exit_requested(&self) -> bool {
        if self.sigterm_received.load(atomic::Ordering::Relaxed) {
            true
        } else {
            false
        }
    }
}
