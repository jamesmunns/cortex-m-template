//! Interrupts

use cortex_m::{exception, Handler};

/// Interrupt handlers
#[repr(C)]
pub struct Handlers {
    // TODO update this struct to list the interrupts specific to your device
    // Make sure they are specified in the right order!
    // Use `exceptions::Exceptions` as a reference
    /// Unspecified interrupts
    pub unspecified: [Handler; 240],
}

/// Default interrupt handlers
pub const DEFAULT_HANDLERS: Handlers = Handlers {
    // TODO you'll have to update this as well
    unspecified: [exception::default_handler; 240],
};
